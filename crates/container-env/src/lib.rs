//! A batch of games that Python plays through, one decision at a time.
//!
//! Each seat of each game is either a scripted bot (played here) or a *slot*: a decision that Python
//! answers, usually with a neural net. The calls go: `reset` the games, `advance` them to the next
//! decision of a slot, `observe` what the slots see, then `step` with the choices and repeat.
//!
//! Seat kinds in `reset`: `-1` random, `-2` simple, `-3` merchant, `-4` trader (see `merchant::Trader`), `-5` discounter and `-6` bargain (cheap sellers, see `container_bots::ARCHETYPES`), `-7` solitaire, `>= 0` a slot, whose number tells
//! Python which policy answers.

use std::os::raw::c_int;

use container_bots::features::{
    ACTION_DIM, MAX_ACTIONS, STATE_DIM, STATE_DIM_MEM, encode_action, encode_state, encode_state_mem, thin,
};
use container_bots::memory::Memory;
use container_bots::merchant::Merchant;
use container_bots::{Opponent, create, is_same_turn_repay};
use container_engine::action::{Action, District};
use container_engine::config::{GameConfig, GameLength, MAX_PLAYERS, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::phase::Phase;
use container_engine::state::{AuctionKind, ShipLocation};

/// The default horizon: a game that has run this many actions is stalled and ends without a result.
/// Real games take 300-1,900 actions (the observed maximum, over 5,292 finished games); a longer limit
/// only lets a stalling policy pile up stored decisions. `cenv_set_horizon` changes it, so that an
/// experiment can measure with another horizon, but training and evaluation use this one.
const STALL: usize = STALL_HORIZON;

/// `cenv_reset_ex` game lengths.
fn length_of(code: c_int) -> GameLength {
    match code {
        0 => GameLength::Short,
        2 => GameLength::Extended,
        _ => GameLength::Standard,
    }
}

/// Counters per seat of how it deals with the others, in game order of the columns of `cenv_interaction`:
/// containers bought from opponents and what they cost, containers sold to opponents and what they brought,
/// delivery bids placed and their money, bids of at least `REAL_BID`, and the containers the seat laid out
/// in a lot of each price ($1 to $6).
pub const INTERACTION: usize = 16;
const BOUGHT: usize = 0;
const SPENT: usize = 1;
const SOLD: usize = 2;
const EARNED: usize = 3;
const BIDS: usize = 4;
const BID_MONEY: usize = 5;
const REAL_BIDS: usize = 6;
const PLACED_AT: usize = 7; // $1 is column 7, $6 is column 12
/// A bid below this is a pass, not an offer for the cargo.
const REAL_BID: u32 = 2;

type Interaction = [[i32; INTERACTION]; MAX_PLAYERS];

/// The seat, district and actions left after the last Reprice, while it is the latest main action.
type LastReprice = Option<(usize, District, u8)>;

/// Applies `action`, crediting both seats of a delivery that is sold to a player with its bid, and counting
/// the purchases, bids and prices of the seats.
fn apply(
    game: &mut Game,
    traded: &mut [i32; MAX_PLAYERS],
    inter: &mut Interaction,
    memory: &mut Memory,
    last_reprice: &mut LastReprice,
    action: Action,
) {
    memory.record(game.current_player(), &game.phase, action);
    if game.phase == Phase::Main {
        *last_reprice = match action {
            Action::Reprice(district) => Some((game.state.active_player, district, game.turn.actions_left.saturating_sub(1))),
            _ => None,
        };
    }
    match (game.phase, action) {
        (Phase::Decide(d), Action::Accept(winner)) => {
            traded[d.deliverer] += d.top as i32;
            traded[winner] += d.top as i32;
        }
        (Phase::Buy(b), Action::Buy { lot, .. }) => {
            let buyer = game.state.active_player;
            let price = b.district.lot_price(lot) as i32;
            inter[buyer][BOUGHT] += 1;
            inter[buyer][SPENT] += price;
            inter[b.seller][SOLD] += 1;
            inter[b.seller][EARNED] += price;
        }
        (Phase::Bid(b), Action::Bid(amount)) => {
            inter[b.bidder][BIDS] += 1;
            inter[b.bidder][BID_MONEY] += amount as i32;
            inter[b.bidder][REAL_BIDS] += (amount >= REAL_BID) as i32;
        }
        (Phase::Arrange(a), Action::Place(lot)) => {
            let price = a.district.lot_price(lot) as usize;
            inter[a.owner][PLACED_AT + price - 1] += 1;
        }
        _ => {}
    }
    game.apply(action).expect("legal action");
}

/// A small code for the phase of a pending decision, for audits: the Bank bid is split by what the
/// bidder is bidding with.
fn phase_code(game: &Game) -> c_int {
    match game.phase {
        Phase::Main => 0,
        Phase::Produce(_) => 1,
        Phase::Buy(_) => 2,
        Phase::Load(_) => 3,
        Phase::Arrange(_) => 4,
        Phase::Bid(_) => 5,
        Phase::Decide(_) => 6,
        Phase::Seize(_) => 7,
        Phase::Distribute(_) => 8,
        Phase::BankBid(b) if b.kind == AuctionKind::ContainerLot => 9,
        Phase::BankBid(_) => 10,
        Phase::GameOver => 11,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Idle,
    Pending,
    Done,
    Stalled,
}

/// The rules imposed on the seats of a slot. Each names the slot it binds, if any.
#[derive(Clone, Copy, Default)]
struct Rules {
    /// The slot that may not call the Bank (start or outbid an auction).
    no_bank: Option<i32>,
    /// Nobody calls the Bank, whatever their kind: the classic rule.
    no_bank_all: bool,
    /// The slot that, on sailing to a harbor, must buy something there.
    buy_at_harbor: Option<i32>,
    /// The slot that may not repay a loan in a turn in which it requested one.
    no_same_turn_repay: Option<i32>,
    /// The slot that may not reprice the same district twice in a row within a turn.
    no_double_reprice: Option<i32>,
    /// The slot that may only sail to the Bank when it has a container there to load (and room for it).
    bank_needs_load: Option<i32>,
    /// The slot that must bid at least `min_bid` (or all its cash, if less) in the first round of a delivery auction.
    min_bid: Option<i32>,
    min_bid_amount: u32,
    /// The slot that may not lay a container out in a lot dearer than `max_price_amount`.
    max_price: Option<i32>,
    max_price_amount: u32,
    /// `(slot, like)`: the seats of `slot` play under the rules that bind slot `like`.
    mirror: Option<(i32, i32)>,
}

impl Rules {
    /// Whether any rule binds the seats of this slot.
    fn bind(&self, kind: i32) -> bool {
        self.no_bank_all
            || [self.no_bank, self.buy_at_harbor, self.no_same_turn_repay, self.min_bid, self.max_price, self.no_double_reprice, self.bank_needs_load].contains(&Some(kind))
    }
}

struct Env {
    game: Option<Game>,
    kinds: [i32; MAX_PLAYERS],
    scripted: Vec<Option<Box<dyn Opponent + Send>>>,
    teacher: Option<Box<dyn Opponent + Send>>,
    legal: Vec<Action>,
    thinned: Vec<usize>,
    actions: usize,
    /// Per seat, the money of the player-to-player deliveries it took part in (the accepted bids), as
    /// deliverer or as the winning bidder.
    traded: [i32; MAX_PLAYERS],
    /// Per seat, how it dealt with the others in the game so far (see `INTERACTION`).
    inter: Interaction,
    /// What the seats have done in the game so far, for nets that read it.
    memory: Memory,
    last_reprice: LastReprice,
    /// Actions after which a running game is declared stalled.
    horizon: usize,
    players: usize,
    status: Status,
    rules: Rules,
}

impl Env {
    fn new() -> Self {
        Self {
            game: None,
            kinds: [-1; MAX_PLAYERS],
            scripted: Vec::new(),
            teacher: None,
            legal: Vec::new(),
            thinned: Vec::new(),
            actions: 0,
            traded: [0; MAX_PLAYERS],
            inter: [[0; INTERACTION]; MAX_PLAYERS],
            memory: Memory::default(),
            last_reprice: None,
            horizon: STALL,
            players: 0,
            status: Status::Idle,
            rules: Rules::default(),
        }
    }

    fn reset(&mut self, players: usize, seed: u64, kinds: &[i32]) {
        self.reset_with(players, seed, GameLength::Standard, kinds);
    }

    fn reset_with(&mut self, players: usize, seed: u64, length: GameLength, kinds: &[i32]) {
        self.game = Game::new(GameConfig::new(players).with_length(length).with_seed(seed)).ok();
        self.kinds = [-1; MAX_PLAYERS];
        self.kinds[..players].copy_from_slice(&kinds[..players]);
        self.scripted = (0..players)
            .map(|seat| match kinds[seat] {
                -1 => create("random", seed ^ (seat as u64 + 1)),
                -2 => create("simple", seed ^ (seat as u64 + 1)),
                -3 => create("merchant", seed ^ (seat as u64 + 1)),
                -4 => Some(Box::new(Merchant::trader(seed ^ (seat as u64 + 1))) as Box<dyn Opponent + Send>),
                -5 => create("discounter", seed ^ (seat as u64 + 1)),
                -6 => create("bargain", seed ^ (seat as u64 + 1)),
                -7 => create("solitaire", seed ^ (seat as u64 + 1)),
                _ => None,
            })
            .collect();
        self.teacher = create("merchant", seed ^ 0x7EAC4E5);
        self.actions = 0;
        self.traded = [0; MAX_PLAYERS];
        self.inter = [[0; INTERACTION]; MAX_PLAYERS];
        self.memory = Memory::default();
        self.last_reprice = None;
        self.players = players;
        self.status = Status::Idle;
    }

    /// Plays scripted seats and forced moves until a slot has a real choice, or the game ends.
    fn advance(&mut self) {
        if self.status != Status::Idle {
            return;
        }
        let Some(game) = self.game.as_mut() else { return };
        loop {
            if game.is_terminal() {
                self.status = Status::Done;
                return;
            }
            if self.actions >= self.horizon {
                self.status = Status::Stalled;
                return;
            }
            let seat = game.current_player();
            let mut legal = game.legal_actions();
            let kind = self.kinds[seat];
            let kind = self.rules.mirror.filter(|&(slot, _)| slot == kind).map_or(kind, |(_, like)| like);
            if self.rules.bind(kind) {
                let rules = self.rules;
                let allowed: Vec<Action> = legal
                    .iter()
                    .copied()
                    .filter(|&a| !forbidden(rules, game, self.last_reprice, kind, a))
                    .collect();
                // Never leave a seat without a move.
                if !allowed.is_empty() {
                    legal = allowed;
                }
            }
            let action = if let Some(bot) = self.scripted[seat].as_mut() {
                let obs = game.observation(seat).expect("seat");
                bot.choose(&obs, &legal)
            } else if legal.len() == 1 {
                legal[0]
            } else {
                self.thinned = thin(legal.len());
                self.legal = legal;
                self.status = Status::Pending;
                return;
            };
            apply(game, &mut self.traded, &mut self.inter, &mut self.memory, &mut self.last_reprice, action);
            self.actions += 1;
        }
    }

    fn observe(&self, state: &mut [f32], acts: &mut [f32], mask: &mut [u8]) -> Option<usize> {
        let game = self.game.as_ref()?;
        if self.status != Status::Pending {
            return None;
        }
        let seat = game.current_player();
        let obs = game.observation(seat)?;
        if state_dim() == STATE_DIM_MEM {
            encode_state_mem(&obs, &self.memory, state);
        } else {
            encode_state(&obs, state);
        }
        for (k, &i) in self.thinned.iter().enumerate() {
            encode_action(&obs, self.legal[i], &mut acts[k * ACTION_DIM..(k + 1) * ACTION_DIM]);
            mask[k] = 1;
        }
        Some(seat)
    }

    fn step(&mut self, choice: usize) {
        if self.status != Status::Pending {
            return;
        }
        let action = self.legal[self.thinned[choice.min(self.thinned.len() - 1)]];
        apply(self.game.as_mut().expect("game"), &mut self.traded, &mut self.inter, &mut self.memory, &mut self.last_reprice, action);
        self.actions += 1;
        self.status = Status::Idle;
    }

    /// What the merchant would do here, as an index into the thinned list.
    fn teach(&mut self) -> Option<usize> {
        if self.status != Status::Pending {
            return None;
        }
        let game = self.game.as_ref()?;
        let obs = game.observation(game.current_player())?;
        let pick = self.teacher.as_mut()?.choose(&obs, &self.legal);
        let at = self.legal.iter().position(|&a| a == pick)?;
        // The thinned action nearest to the teacher's choice.
        (0..self.thinned.len()).min_by_key(|&k| self.thinned[k].abs_diff(at))
    }
}

/// Whether the rules imposed on this slot rule `action` out.
fn forbidden(rules: Rules, game: &Game, last_reprice: LastReprice, kind: i32, action: Action) -> bool {
    if (rules.no_bank_all || rules.no_bank == Some(kind)) && matches!(action, Action::StartAuction { .. } | Action::Outbid(_)) {
        return true;
    }
    if rules.no_same_turn_repay == Some(kind) && is_same_turn_repay(game.turn.loans_requested, action) {
        return true;
    }
    if rules.min_bid == Some(kind) {
        if let (Phase::Bid(b), Action::Bid(amount)) = (game.phase, action) {
            let cash = game.state.players[b.bidder].cash;
            if !b.runoff && amount < rules.min_bid_amount.min(cash) {
                return true;
            }
        }
    }
    if let (Some(slot), Phase::Arrange(a), Action::Place(lot)) = (rules.max_price, game.phase, action) {
        if slot == kind && a.district.lot_price(lot) > rules.max_price_amount {
            return true;
        }
    }
    if rules.no_double_reprice == Some(kind) {
        if let (Phase::Main, Action::Reprice(d)) = (game.phase, action) {
            if last_reprice == Some((game.state.active_player, d, game.turn.actions_left)) {
                return true;
            }
        }
    }
    if rules.bank_needs_load == Some(kind) && game.phase == Phase::Main && action == Action::Sail(ShipLocation::Bank) {
        let mut after = game.clone();
        if after.apply(action).is_ok() && !matches!(after.phase, Phase::Load(_)) {
            return true;
        }
    }
    if rules.buy_at_harbor != Some(kind) {
        return false;
    }
    match (game.phase, action) {
        // Sailing to a harbor must open a purchase, so there has to be something to buy there.
        (Phase::Main, Action::Sail(ShipLocation::Harbor(_))) => {
            let mut after = game.clone();
            after.apply(action).is_ok() && !matches!(after.phase, Phase::Buy(_))
        }
        // The free purchase on arrival may not be declined.
        (Phase::Buy(b), Action::DoneBuying) => {
            b.optional && b.district == District::Harbor && b.bought.is_empty()
        }
        _ => false,
    }
}

pub struct Batch {
    envs: Vec<Env>,
    threads: usize,
}

impl Batch {
    fn each<F: Fn(usize, &mut Env) + Sync>(&mut self, f: F) {
        let chunk = self.envs.len().div_ceil(self.threads.max(1)).max(1);
        std::thread::scope(|scope| {
            for (c, envs) in self.envs.chunks_mut(chunk).enumerate() {
                let f = &f;
                scope.spawn(move || {
                    for (i, env) in envs.iter_mut().enumerate() {
                        f(c * chunk + i, env);
                    }
                });
            }
        });
    }
}

/// A raw pointer that may cross threads; every thread touches only its own env's part of the buffer.
#[derive(Clone, Copy)]
struct Shared<T>(*mut T);
unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}

#[unsafe(no_mangle)]
pub extern "C" fn cenv_new(n: usize, threads: usize) -> *mut Batch {
    let envs = (0..n).map(|_| Env::new()).collect();
    Box::into_raw(Box::new(Batch { envs, threads }))
}

/// # Safety
/// `b` comes from `cenv_new` and is not used afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_free(b: *mut Batch) {
    drop(unsafe { Box::from_raw(b) });
}

/// Forbids the seats of slot `slot` to call the Bank in every env; a negative `slot` lifts the ban.
/// Takes effect from the next `advance`.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_ban_bank(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let ban = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.no_bank = ban;
    }
}

/// Forbids every seat, scripted or not, to call the Bank (the classic rule); `on == 0` lifts it.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_ban_bank_all(b: *mut Batch, on: c_int) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.no_bank_all = on != 0;
    }
}

/// Makes the seats of slot `slot` buy something whenever they sail to a harbor (they may only sail to
/// one where something can be bought, and may not decline the purchase); a negative `slot` lifts it.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_require_harbor_purchase(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let rule = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.buy_at_harbor = rule;
    }
}

/// Forbids the seats of slot `slot` to reprice the same district twice in a row within a turn; a negative
/// `slot` lifts the rule.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_forbid_double_reprice(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let rule = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.no_double_reprice = rule;
    }
}

/// Makes the seats of slot `slot` play under the rules that bind slot `like`, for a frozen copy of a net
/// that was trained under them; a negative `slot` lifts it.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_mirror_rules(b: *mut Batch, slot: c_int, like: c_int) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.mirror = (slot >= 0).then_some((slot, like));
    }
}

/// Lets the seats of slot `slot` sail to the Bank only to load a container; a negative `slot` lifts the rule.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_require_bank_load(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let rule = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.bank_needs_load = rule;
    }
}

/// Forbids the seats of slot `slot` to repay a loan in a turn in which they requested one, in every
/// env; a negative `slot` lifts the rule.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_forbid_same_turn_repay(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let rule = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.no_same_turn_repay = rule;
    }
}

/// Seats of `slot` must open a delivery auction with a bid of at least `amount` (their whole cash if less).
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_min_bid(b: *mut Batch, slot: c_int, amount: u32) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.min_bid = (slot >= 0).then_some(slot);
        env.rules.min_bid_amount = amount;
    }
}

/// Seats of `slot` may not lay a container out in a lot that costs more than `amount` (unless every
/// other way of laying it out is closed too).
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_max_price(b: *mut Batch, slot: c_int, amount: u32) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.max_price = (slot >= 0).then_some(slot);
        env.rules.max_price_amount = amount;
    }
}

static MEMORY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Length of the state rows: with the opponent memory ([`Memory`]) appended or not.
fn state_dim() -> usize {
    if MEMORY.load(std::sync::atomic::Ordering::Relaxed) { STATE_DIM_MEM } else { STATE_DIM }
}

/// Makes the states carry the opponent memory (`on` non-zero); call before `cenv_dims`.
#[unsafe(no_mangle)]
pub extern "C" fn cenv_set_memory(on: c_int) {
    MEMORY.store(on != 0, std::sync::atomic::Ordering::Relaxed);
}

/// Writes `[state dim, ACTION_DIM, MAX_ACTIONS]` into `out`.
///
/// # Safety
/// `out` points to three `u32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_dims(out: *mut u32) {
    let out = unsafe { std::slice::from_raw_parts_mut(out, 3) };
    out.copy_from_slice(&[state_dim() as u32, ACTION_DIM as u32, MAX_ACTIONS as u32]);
}

/// Starts a new game in env `i`. `kinds` holds one seat kind per player.
///
/// # Safety
/// `b` is live, `i` is in range and `kinds` points to `players` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_reset(
    b: *mut Batch,
    i: usize,
    players: usize,
    seed: u64,
    kinds: *const c_int,
) {
    let batch = unsafe { &mut *b };
    let kinds = unsafe { std::slice::from_raw_parts(kinds, players) };
    batch.envs[i].reset(players, seed, kinds);
}

/// Like `cenv_reset`, with the game length: 0 short, 1 standard, 2 extended.
///
/// # Safety
/// `b` is live, `i` is in range and `kinds` points to `players` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_reset_ex(
    b: *mut Batch,
    i: usize,
    players: usize,
    seed: u64,
    length: c_int,
    kinds: *const c_int,
) {
    let batch = unsafe { &mut *b };
    let kinds = unsafe { std::slice::from_raw_parts(kinds, players) };
    batch.envs[i].reset_with(players, seed, length_of(length), kinds);
}

/// Sets the number of actions after which a running game is declared stalled, in every env. It
/// applies from the next `advance`, also to games already under way.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_horizon(b: *mut Batch, horizon: usize) {
    for env in &mut unsafe { &mut *b }.envs {
        env.horizon = horizon;
    }
}

/// Plays every env up to its next decision.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_advance(b: *mut Batch) {
    unsafe { &mut *b }.each(|_, env| env.advance());
}

/// Writes each env's pending decision; rows of envs with nothing pending are left as they were.
/// Buffers are `[n, STATE_DIM]`, `[n, MAX_ACTIONS, ACTION_DIM]`
/// and `[n, MAX_ACTIONS]`, and padding is zeroed; `slot` and `seat` get -1 where nothing is pending.
///
/// # Safety
/// `b` is live and every buffer has the size above.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_observe(
    b: *mut Batch,
    state: *mut f32,
    acts: *mut f32,
    mask: *mut u8,
    slot: *mut c_int,
    seat: *mut c_int,
) {
    let batch = unsafe { &mut *b };
    let (state, acts, mask) = (Shared(state), Shared(acts), Shared(mask));
    let (slot, seat) = (Shared(slot), Shared(seat));
    batch.each(move |i, env| {
        let (state, acts, mask, slot, seat) = (state, acts, mask, slot, seat);
        let (s, a, m) = unsafe {
            (
                std::slice::from_raw_parts_mut(state.0.add(i * state_dim()), state_dim()),
                std::slice::from_raw_parts_mut(
                    acts.0.add(i * MAX_ACTIONS * ACTION_DIM),
                    MAX_ACTIONS * ACTION_DIM,
                ),
                std::slice::from_raw_parts_mut(mask.0.add(i * MAX_ACTIONS), MAX_ACTIONS),
            )
        };
        m.fill(0);
        if env.status == Status::Pending {
            s.fill(0.0);
            a.fill(0.0);
        }
        let who = env.observe(s, a, m);
        unsafe {
            *seat.0.add(i) = who.map_or(-1, |w| w as c_int);
            *slot.0.add(i) = who.map_or(-1, |w| env.kinds[w]);
        }
    });
}

/// Applies one choice per env (an index into the thinned action list; ignored where nothing is pending).
///
/// # Safety
/// `b` is live and `choice` points to one value per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_step(b: *mut Batch, choice: *const c_int) {
    let batch = unsafe { &mut *b };
    let choice = Shared(choice as *mut c_int);
    batch.each(move |i, env| {
        let choice = choice;
        env.step(unsafe { *choice.0.add(i) }.max(0) as usize)
    });
}

/// Writes what the merchant would choose in each pending decision, or -1.
///
/// # Safety
/// `b` is live and `out` points to one value per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_teacher(b: *mut Batch, out: *mut c_int) {
    let batch = unsafe { &mut *b };
    let out = Shared(out);
    batch.each(move |i, env| {
        let out = out;
        unsafe { *out.0.add(i) = env.teach().map_or(-1, |k| k as c_int) };
    });
}

/// Writes each env's status (0 idle, 1 pending, 2 done, 3 stalled) and final scores `[n, MAX_PLAYERS]`
/// (zero until the game is done), plus the number of players.
///
/// # Safety
/// `b` is live; `status` and `players` hold one value per env and `scores` `MAX_PLAYERS` per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_results(
    b: *mut Batch,
    status: *mut c_int,
    players: *mut c_int,
    scores: *mut c_int,
) {
    unsafe { results(b, status, players, scores, std::ptr::null_mut(), std::ptr::null_mut()) }
}

/// Like `cenv_results`, plus for each env a bit mask of the winners (bit `p` set when seat `p` won,
/// zero until the game is done; the engine's tie-break on factory stock is applied) and the number of
/// actions the game has taken so far, forced moves included.
///
/// # Safety
/// `b` is live; `status`, `players`, `winners` and `actions` hold one value per env and `scores`
/// `MAX_PLAYERS` per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_results_ex(
    b: *mut Batch,
    status: *mut c_int,
    players: *mut c_int,
    scores: *mut c_int,
    winners: *mut c_int,
    actions: *mut c_int,
) {
    unsafe { results(b, status, players, scores, winners, actions) }
}

/// Writes for each env the money of the deliveries each seat sold or won so far: `MAX_PLAYERS` per env.
///
/// # Safety
/// `b` is live; `traded` holds `MAX_PLAYERS` values per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_traded(b: *mut Batch, traded: *mut c_int) {
    let batch = unsafe { &*b };
    for (i, env) in batch.envs.iter().enumerate() {
        for p in 0..MAX_PLAYERS {
            unsafe { *traded.add(i * MAX_PLAYERS + p) = env.traded[p] };
        }
    }
}

/// Writes for each env the interaction counters of every seat: `MAX_PLAYERS * INTERACTION` per env (see
/// `INTERACTION` for the columns).
///
/// # Safety
/// `b` is live; `out` holds `MAX_PLAYERS * INTERACTION` values per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_interaction(b: *mut Batch, out: *mut c_int) {
    let batch = unsafe { &*b };
    for (i, env) in batch.envs.iter().enumerate() {
        for p in 0..MAX_PLAYERS {
            for k in 0..INTERACTION {
                unsafe { *out.add((i * MAX_PLAYERS + p) * INTERACTION + k) = env.inter[p][k] };
            }
        }
    }
}

/// Writes for each env the final score of every seat as cash, island, leftovers and loans (a positive
/// amount, as in `ScoreBreakdown`): `MAX_PLAYERS * 4` per env, zero unless the game is done.
///
/// # Safety
/// `b` is live; `out` holds `MAX_PLAYERS * 4` values per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_breakdown(b: *mut Batch, out: *mut c_int) {
    let batch = unsafe { &*b };
    for (i, env) in batch.envs.iter().enumerate() {
        let results = match env.status {
            Status::Done => env.game.as_ref().and_then(|g| g.results()),
            _ => None,
        };
        for p in 0..MAX_PLAYERS {
            let parts = match results.as_ref().and_then(|r| r.scores.get(p)) {
                Some(sc) => [sc.cash as c_int, sc.island as c_int, sc.leftovers as c_int, sc.loans as c_int],
                None => [0; 4],
            };
            for (k, v) in parts.into_iter().enumerate() {
                unsafe { *out.add((i * MAX_PLAYERS + p) * 4 + k) = v };
            }
        }
    }
}

/// Writes for each env with a pending decision the number of legal actions before thinning and a
/// phase code (see `phase_code`); -1 in both where nothing is pending.
///
/// # Safety
/// `b` is live; `legal` and `phase` hold one value per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_decision_info(b: *mut Batch, legal: *mut c_int, phase: *mut c_int) {
    let batch = unsafe { &mut *b };
    for (i, env) in batch.envs.iter().enumerate() {
        let (n, p) = match (env.status, env.game.as_ref()) {
            (Status::Pending, Some(game)) => (env.legal.len() as c_int, phase_code(game)),
            _ => (-1, -1),
        };
        unsafe {
            *legal.add(i) = n;
            *phase.add(i) = p;
        }
    }
}

/// # Safety
/// As `cenv_results_ex`, where null `winners` and `actions` are skipped.
unsafe fn results(
    b: *mut Batch,
    status: *mut c_int,
    players: *mut c_int,
    scores: *mut c_int,
    winners: *mut c_int,
    actions: *mut c_int,
) {
    let batch = unsafe { &mut *b };
    for (i, env) in batch.envs.iter().enumerate() {
        let s = match env.status {
            Status::Idle => 0,
            Status::Pending => 1,
            Status::Done => 2,
            Status::Stalled => 3,
        };
        unsafe {
            *status.add(i) = s;
            *players.add(i) = env.players as c_int;
            if !actions.is_null() {
                *actions.add(i) = env.actions as c_int;
            }
            if !winners.is_null() {
                *winners.add(i) = 0;
            }
            for p in 0..MAX_PLAYERS {
                *scores.add(i * MAX_PLAYERS + p) = 0;
            }
            if env.status == Status::Done {
                if let Some(r) = env.game.as_ref().and_then(|g| g.results()) {
                    for (p, sc) in r.scores.iter().enumerate() {
                        *scores.add(i * MAX_PLAYERS + p) = sc.total;
                    }
                    if !winners.is_null() {
                        *winners.add(i) = r.winners.iter().fold(0, |mask, &p| mask | 1 << p);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plays whole games with every seat a slot choosing at random, and counts the decisions that
    /// offered a Call Bank action.
    fn bank_offers(no_bank: Option<i32>) -> usize {
        let mut offers = 0;
        for seed in 0..20 {
            let mut env = Env::new();
            env.rules.no_bank = no_bank;
            env.reset(3, seed, &[0, 0, 0]);
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let calls = |a: &&Action| matches!(a, Action::StartAuction { .. } | Action::Outbid(_));
                offers += env.thinned.iter().filter(|&&i| calls(&&env.legal[i])).count();
                env.step((env.actions * 7 + seed as usize) % env.thinned.len());
            }
        }
        offers
    }

    #[test]
    fn a_banned_slot_is_never_offered_a_bank_call() {
        assert!(bank_offers(None) > 0, "random play should be offered the Bank sometimes");
        assert_eq!(bank_offers(Some(0)), 0);
    }

    #[test]
    fn the_classic_rule_stops_scripted_seats_calling_the_bank_too() {
        let cash_taken = |classic: bool| {
            let mut taken = 0;
            for seed in 0..10 {
                let mut env = Env::new();
                env.rules.no_bank_all = classic;
                env.reset(4, seed, &[-3, -3, -3, -3]);
                let before = env.game.as_ref().unwrap().state.bank.cash_lots;
                play_out(&mut env, 1);
                let after = env.game.as_ref().unwrap().state.bank.cash_lots;
                taken += usize::from(before != after);
            }
            taken
        };
        assert!(cash_taken(false) > 0, "merchants call the Bank");
        assert_eq!(cash_taken(true), 0);
    }

    /// Plays random games with every seat a slot, and counts the decisions that offered a repayment:
    /// in a turn that had already requested a loan, and in all.
    fn repay_offers(rule: Option<i32>) -> (usize, usize) {
        let (mut same_turn, mut all) = (0, 0);
        for seed in 0..30 {
            let mut env = Env::new();
            env.rules.no_same_turn_repay = rule;
            env.reset(3, seed, &[0, 0, 0]);
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let requested = env.game.as_ref().unwrap().turn.loans_requested > 0;
                let repays = env.thinned.iter().filter(|&&i| env.legal[i] == Action::RepayLoan).count();
                all += repays;
                if requested {
                    same_turn += repays;
                }
                env.step((env.actions * 7 + seed as usize) % env.thinned.len());
            }
        }
        (same_turn, all)
    }

    #[test]
    fn a_slot_with_the_repay_rule_is_never_offered_to_repay_a_loan_it_just_requested() {
        let (same_turn, all) = repay_offers(None);
        assert!(same_turn > 0, "random play should be able to repay in the turn it borrowed");
        let (same_turn, kept) = repay_offers(Some(0));
        assert_eq!(same_turn, 0);
        assert!(kept > 0, "the rule must still allow repaying in a later turn");
        assert!(kept <= all);
    }

    #[test]
    fn a_sold_delivery_credits_its_deliverer_and_its_winning_bidder_with_the_bid() {
        let (mut sold, mut games_with_trade) = (0, 0);
        for seed in 0..20 {
            let mut env = Env::new();
            env.reset(3, seed, &[0, 0, 0]);
            let mut step = 0usize;
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let before = env.traded;
                let accepted = match env.game.as_ref().unwrap().phase {
                    Phase::Decide(d) => Some(d.top as i32),
                    _ => None,
                };
                step += 1;
                env.step((step * 5 + seed as usize) % env.thinned.len());
                let gain: i32 = (0..MAX_PLAYERS).map(|p| env.traded[p] - before[p]).sum();
                match accepted {
                    Some(top) => assert!(gain == 0 || gain == 2 * top, "an accepted bid credits two seats"),
                    None => assert_eq!(gain, 0),
                }
                sold += (gain > 0) as usize;
            }
            games_with_trade += (env.traded.iter().sum::<i32>() > 0) as usize;
            assert_eq!(env.traded[3..], [0; MAX_PLAYERS - 3]);
        }
        assert!(sold > 0 && games_with_trade > 0, "random play should sell some deliveries to players");
    }

    #[test]
    fn a_slot_with_a_minimum_bid_never_opens_an_auction_below_it() {
        let (mut bids, mut low) = (0, 0);
        for min in [0, 3] {
            for seed in 0..20 {
                let mut env = Env::new();
                if min > 0 {
                    env.rules.min_bid = Some(0);
                    env.rules.min_bid_amount = min;
                }
                env.reset(3, seed, &[0, 0, 0]);
                let mut step = 0usize;
                loop {
                    env.advance();
                    if env.status != Status::Pending {
                        break;
                    }
                    let game = env.game.as_ref().unwrap();
                    if let Phase::Bid(b) = game.phase {
                        let cash = game.state.players[b.bidder].cash;
                        for &i in &env.thinned {
                            if let Action::Bid(a) = env.legal[i] {
                                if min > 0 && !b.runoff {
                                    bids += 1;
                                    low += (a < min.min(cash)) as usize;
                                }
                            }
                        }
                    }
                    step += 1;
                    env.step((step * 3 + seed as usize) % env.thinned.len());
                }
            }
        }
        assert!(bids > 0, "random play should reach delivery auctions");
        assert_eq!(low, 0);
    }

    #[test]
    fn a_slot_with_a_price_cap_lays_nothing_out_above_it_and_purchases_have_two_sides() {
        let (mut places, mut dear, mut bought, mut top_lot) = (0, 0, 0, 0);
        for cap in [0, 3] {
            for seed in 0..20 {
                let mut env = Env::new();
                if cap > 0 {
                    env.rules.max_price = Some(0);
                    env.rules.max_price_amount = cap;
                }
                env.reset(3, seed, &[0, 0, 0]);
                let mut step = 0usize;
                loop {
                    env.advance();
                    if env.status != Status::Pending {
                        break;
                    }
                    let game = env.game.as_ref().unwrap();
                    if let Phase::Arrange(a) = game.phase {
                        for &i in &env.thinned {
                            if let Action::Place(lot) = env.legal[i] {
                                if cap > 0 {
                                    places += 1;
                                    dear += (a.district.lot_price(lot) > cap) as usize;
                                }
                            }
                        }
                    }
                    step += 1;
                    env.step((step * 3 + seed as usize) % env.thinned.len());
                }
                let seat = |k: usize| env.inter.iter().map(|row| row[k]).sum::<i32>();
                assert_eq!(seat(BOUGHT), seat(SOLD), "every purchase has a buyer and a seller");
                assert_eq!(seat(SPENT), seat(EARNED));
                bought += seat(BOUGHT);
                if cap == 0 {
                    top_lot += seat(PLACED_AT + 5);
                }
            }
        }
        assert!(places > 0, "random play should lay containers out");
        assert_eq!(dear, 0);
        assert!(bought > 0, "random play should buy from opponents");
        assert!(top_lot > 0, "the counters see the dearest lot being used when nothing forbids it");
    }

    /// Plays random games with every seat a slot, and counts harbor arrivals that bought nothing.
    fn empty_harbor_visits(rule: Option<i32>) -> (usize, usize) {
        let (mut visits, mut empty) = (0, 0);
        for seed in 0..30 {
            let mut env = Env::new();
            env.rules.buy_at_harbor = rule;
            env.reset(3, seed, &[0, 0, 0]);
            let mut step = 0usize;
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let game = env.game.as_ref().unwrap();
                let at_harbor = |g: &Game| {
                    matches!(g.observation(g.current_player()).map(|o| o.players[o.viewer].ship_location),
                             Some(ShipLocation::Harbor(_)))
                };
                let was_main = game.phase == Phase::Main;
                let me = game.current_player();
                let was_docked = at_harbor(game);
                step += 1;
                env.step((step * 7 + seed as usize) % env.thinned.len());
                let game = env.game.as_ref().unwrap();
                if was_main && !was_docked && game.current_player() == me && at_harbor(game) {
                    visits += 1;
                    // The arrival did not open a purchase.
                    if !matches!(game.phase, Phase::Buy(_)) {
                        empty += 1;
                    }
                }
            }
        }
        (visits, empty)
    }

    #[test]
    fn a_slot_with_the_harbor_rule_only_sails_where_it_can_buy() {
        let (visits, empty) = empty_harbor_visits(None);
        assert!(visits > 0 && empty > 0, "random play should reach empty harbors: {visits} {empty}");
        let (visits, empty) = empty_harbor_visits(Some(0));
        assert!(visits > 0, "the rule must still allow sailing to harbors");
        assert_eq!(empty, 0);
    }

    /// Plays random games where every seat is a slot under the two new rules; counts the Bank trips that
    /// load nothing and the back-to-back reprices of one district.
    fn rule_breaches(on: bool, slot: i32) -> (usize, usize, usize, usize) {
        let (mut bank, mut empty_bank, mut reprices, mut repeats) = (0, 0, 0, 0);
        for seed in 0..40 {
            let mut env = Env::new();
            if on {
                env.rules.bank_needs_load = Some(0);
                env.rules.no_double_reprice = Some(0);
                env.rules.mirror = (slot != 0).then_some((slot, 0));
            }
            env.reset(3, seed, &[slot; 3]);
            let mut step = 0usize;
            let mut prev: Option<(usize, District, u8)> = None;
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let game = env.game.as_ref().unwrap();
                let main = game.phase == Phase::Main;
                let (me, left) = (game.current_player(), game.turn.actions_left);
                step += 1;
                let pick = (step * 7 + seed as usize) % env.thinned.len();
                let action = env.legal[env.thinned[pick]];
                if main {
                    if let Action::Reprice(d) = action {
                        reprices += 1;
                        repeats += usize::from(prev == Some((me, d, left)));
                        prev = Some((me, d, left - 1));
                    } else {
                        prev = None;
                    }
                    if action == Action::Sail(ShipLocation::Bank) {
                        bank += 1;
                        let mut after = game.clone();
                        after.apply(action).unwrap();
                        empty_bank += usize::from(!matches!(after.phase, Phase::Load(_)));
                    }
                }
                env.step(pick);
            }
        }
        (bank, empty_bank, reprices, repeats)
    }

    #[test]
    fn the_reprice_and_bank_rules_bind_a_slot() {
        let (bank, empty_bank, reprices, repeats) = rule_breaches(false, 0);
        assert!(bank > 0 && empty_bank > 0 && reprices > 0 && repeats > 0, "random play breaks both rules");
        let (bank, empty_bank, reprices, repeats) = rule_breaches(true, 0);
        assert!(bank > 0 && reprices > 0, "the rules still allow both actions");
        assert_eq!((empty_bank, repeats), (0, 0));
    }

    #[test]
    fn a_mirrored_slot_plays_under_the_rules_of_another() {
        let (bank, _, reprices, _) = rule_breaches(false, 5);
        assert!(bank > 0 && reprices > 0);
        let (bank, empty_bank, reprices, repeats) = rule_breaches(true, 5);
        assert!(bank > 0 && reprices > 0, "the rules still allow both actions");
        assert_eq!((empty_bank, repeats), (0, 0));
    }

    #[test]
    fn solitaire_seats_play_a_game_out() {
        let mut env = Env::new();
        env.reset(3, 9, &[-7, -7, -7]);
        play_out(&mut env, 1);
        assert_eq!(env.status, Status::Done);
    }

    #[test]
    fn the_harbor_rule_forbids_declining_the_free_purchase() {
        for seed in 0..30 {
            let mut env = Env::new();
            env.rules.buy_at_harbor = Some(0);
            env.reset(3, seed, &[0, 0, 0]);
            let mut step = 0usize;
            loop {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                let game = env.game.as_ref().unwrap();
                if let Phase::Buy(b) = game.phase {
                    if b.optional && b.district == District::Harbor && b.bought.is_empty() {
                        assert!(
                            env.thinned.iter().all(|&i| env.legal[i] != Action::DoneBuying),
                            "declining the free harbor purchase was offered"
                        );
                    }
                }
                step += 1;
                env.step((step * 5 + seed as usize) % env.thinned.len());
            }
        }
    }

    /// Plays one game with every seat a slot choosing pseudo-randomly, until it ends or stalls.
    fn play_out(env: &mut Env, salt: usize) {
        let mut step = 0usize;
        loop {
            env.advance();
            if env.status != Status::Pending {
                break;
            }
            step += 1;
            env.step((step * 7 + salt) % env.thinned.len());
        }
    }

    #[test]
    fn a_game_stalls_at_the_horizon_and_not_before() {
        let mut env = Env::new();
        env.horizon = 40;
        env.reset(3, 5, &[0, 0, 0]);
        play_out(&mut env, 1);
        assert_eq!(env.status, Status::Stalled);
        assert_eq!(env.actions, 40);

        let mut env = Env::new();
        env.reset(3, 5, &[-3, -3, -3]);
        play_out(&mut env, 1);
        assert_eq!(env.status, Status::Done, "merchants finish well within the default horizon");
        assert!(env.actions < STALL);
    }

    #[test]
    fn traders_finish_their_games_at_every_table_size() {
        for players in 3..=5 {
            for seed in 0..10 {
                let mut env = Env::new();
                env.reset(players, seed, &vec![-4; players]);
                play_out(&mut env, 1);
                assert_eq!(env.status, Status::Done, "{players} traders, seed {seed}");
            }
        }
    }

    #[test]
    fn the_length_reaches_the_engine() {
        for (code, expected) in [(0, GameLength::Short), (1, GameLength::Standard), (2, GameLength::Extended)] {
            let mut env = Env::new();
            env.reset_with(4, 9, length_of(code), &[-3, -3, -3, -3]);
            assert_eq!(env.game.as_ref().unwrap().config.length, expected);
        }
        let mut env = Env::new();
        env.reset(4, 9, &[-3, -3, -3, -3]);
        assert_eq!(env.game.as_ref().unwrap().config.length, GameLength::Standard);
    }

    #[test]
    fn results_report_the_winners_and_the_action_count() {
        let mut batch = Batch { envs: vec![Env::new(), Env::new()], threads: 1 };
        batch.envs[0].reset(4, 11, &[-3, -3, -2, -2]);
        batch.envs[1].horizon = 10;
        batch.envs[1].reset(3, 12, &[0, 0, 0]);
        for env in &mut batch.envs {
            play_out(env, 3);
        }
        let (mut status, mut players, mut scores) = ([0; 2], [0; 2], [0; 2 * MAX_PLAYERS]);
        let (mut winners, mut actions) = ([-1; 2], [-1; 2]);
        let b: *mut Batch = &mut batch;
        unsafe {
            cenv_results_ex(b, status.as_mut_ptr(), players.as_mut_ptr(), scores.as_mut_ptr(),
                            winners.as_mut_ptr(), actions.as_mut_ptr());
        }
        assert_eq!((status[0], players[0]), (2, 4));
        let done = batch.envs[0].game.as_ref().unwrap().results().unwrap();
        let mask = done.winners.iter().fold(0, |m, &p| m | 1 << p);
        assert_eq!(winners[0], mask);
        assert!(mask != 0);
        let best = done.scores.iter().map(|s| s.total).max().unwrap();
        for (p, s) in done.scores.iter().enumerate() {
            assert_eq!(scores[p], s.total);
            assert!(winners[0] >> p & 1 == 0 || s.total == best, "a winner has the top score");
        }
        assert_eq!(actions[0], batch.envs[0].actions as c_int);
        assert_eq!((status[1], winners[1], actions[1]), (3, 0, 10), "a stalled game has no winner");
    }

    #[test]
    fn the_first_pending_decision_belongs_to_the_first_player() {
        let mut seen = [false; 4];
        for seed in 0..40u64 {
            let mut env = Env::new();
            env.reset(4, seed, &[0, 0, 0, 0]);
            env.advance();
            assert_eq!((env.status, env.actions), (Status::Pending, 0));
            let game = env.game.as_ref().unwrap();
            assert_eq!(game.current_player(), game.state.first_player);
            seen[game.state.first_player] = true;
        }
        assert!(seen.iter().all(|&s| s), "the first player is drawn from every seat");
    }

    #[test]
    fn decision_info_reports_the_legal_count_before_thinning() {
        let mut batch = Batch { envs: vec![Env::new()], threads: 1 };
        let (mut legal, mut phase) = ([0; 1], [0; 1]);
        let b: *mut Batch = &mut batch;
        batch.envs[0].reset(3, 2, &[0, 0, 0]);
        unsafe { cenv_decision_info(b, legal.as_mut_ptr(), phase.as_mut_ptr()) };
        assert_eq!((legal[0], phase[0]), (-1, -1), "nothing is pending before `advance`");
        let mut saw_bid = false;
        for seed in 0..20u64 {
            batch.envs[0].reset(3, seed, &[0, 0, 0]);
            for step in 0..3000usize {
                batch.envs[0].advance();
                if batch.envs[0].status != Status::Pending {
                    break;
                }
                unsafe { cenv_decision_info(b, legal.as_mut_ptr(), phase.as_mut_ptr()) };
                let env = &batch.envs[0];
                assert_eq!(legal[0] as usize, env.legal.len());
                assert!(env.thinned.len() <= MAX_ACTIONS);
                saw_bid |= phase[0] == 5;
                let pick = (step * 7 + seed as usize) % env.thinned.len();
                batch.envs[0].step(pick);
            }
        }
        assert!(saw_bid, "random games hold delivery auctions");
    }
}
