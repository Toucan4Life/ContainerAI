//! A batch of games that Python plays through, one decision at a time.
//!
//! Each seat of each game is either a scripted bot (played here) or a *slot*: a decision that Python
//! answers, usually with a neural net. The calls go: `reset` the games, `advance` them to the next
//! decision of a slot, `observe` what the slots see, then `step` with the choices and repeat.
//!
//! Seat kinds in `reset`: `-1` random, `-2` simple, `-3` merchant, `-4` trader (see `merchant::Trader`), `-5` discounter and `-6` bargain (cheap sellers, see `container_bots::ARCHETYPES`), `-7` solitaire, `-8` producer, `-9` showcase and `-10` shipper (see `container_bots::HEURISTICS`), `-11` to `-19` the evaluation-only price-lot variants of `container_bots::archetype::VARIANTS`, `>= 0` a slot, whose number tells
//! Python which policy answers.

use std::os::raw::c_int;

use container_bots::features::{
    ACTION_DIM, MAX_ACTIONS, STATE_DIM, STATE_DIM_MEM, encode_action, encode_state, encode_state_mem, thin,
};
use container_bots::memory::Memory;
use container_bots::merchant::Merchant;
use container_bots::{Opponent, create, is_same_turn_repay};
use container_engine::action::{Action, Building, District};
use container_engine::color::{Color, NUM_COLORS};
use container_engine::config::{GameConfig, GameLength, MAX_PLAYERS, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::phase::Phase;
use container_engine::state::{AuctionKind, PlayerId, ShipLocation};

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
/// in a lot of each price ($1 to $6). From column `MOVES` on, one counter per kind of move.
pub const INTERACTION: usize = MOVES + MOVE_NAMES.len();
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
// Strategy diagnostics, not rewards; `training/strategy_metrics.py` gives the formulas.
/// The seat's Main decisions, and those in which Produce was legal.
const MAIN_DECISIONS: usize = 13;
const PRODUCE_CHANCES: usize = 14;
/// Produce actions taken, and the containers they put in the factory district.
const PRODUCES: usize = 15;
const PRODUCED: usize = 16;
/// Summed over the seat's Main decisions: containers on its lots, those priced at most `COMPETITIVE_PRICE`,
/// and the distinct colors among those.
const OFFERED: usize = 17;
const OFFERED_COMPETITIVE: usize = 18;
const COMPETITIVE_COLORS: usize = 19;
/// Containers bought from opponents and sold to them, by color (five columns each).
const BOUGHT_COLOR: usize = 20;
const SOLD_COLOR: usize = 25;
/// Trips to the Island, the containers on the ship then, and the subsidy the supply paid the seat for
/// accepted bids (money created).
const DELIVERIES: usize = 30;
const SHIPPED: usize = 31;
const SUBSIDY: usize = 32;
/// Per delivery bid (a runoff bid is another one), summed: the bidder's allowance for the cargo (`bid_allowance`),
/// and the money bid above that (a runoff bid counts only what it adds to the money on the table).
const BID_VALUE: usize = 33;
const OVERBID: usize = 34;
/// A container left on a ship scores $3, so a buyer cannot lose at this price or less.
const COMPETITIVE_PRICE: u32 = 3;

/// Trips to the Island by number of containers on the ship (1 to 5, columns `CARGO_SIZE..CARGO_SIZE + 5`), the
/// cash the deliverer received for accepted deliveries of each size (twice the winning bid, same layout), and the
/// distinct colors of all the cargoes delivered.
const CARGO_SIZE: usize = 35;
const CARGO_CASH: usize = 40;
const CARGO_COLORS: usize = 45;
/// Summed over Main decisions: factories owned, warehouses owned, and containers laid out in the harbor district.
const FACTORIES_HELD: usize = 46;
const WAREHOUSES_HELD: usize = 47;
const HARBOR_OFFERED: usize = 48;
/// Builds that the build script (`cenv_set_build_script`) made for the seat.
const SCRIPT_BUILDS: usize = 49;

/// Per bid in a delivery auction: the bidder's cash before the bid (in the bidder's row), and in the deliverer's row
/// the bids made by rivals, and how many of them could afford $3 and $10 at that moment.
const BID_CASH: usize = 50;
const RIVAL_BIDS: usize = 51;
const RIVAL_CASH_3: usize = 52;
const RIVAL_CASH_10: usize = 53;

/// Cash flows that no other counter sees, for the ledger of `training/ledger.py`: the delivery auctions the seat won as
/// bidder and what it paid for them, what it paid the Bank to buy its own cargo out, and the interest due when its turns
/// begin ($1 per loan held; a seat that cannot pay it is seized instead, so due is paid except in a default).
const DEALS_WON: usize = 54;
const DEAL_PAID: usize = 55;
const BUYOUT_PAID: usize = 56;
const INTEREST: usize = 57;

/// The first column of the move counters.
const MOVES: usize = 58;

/// Every action a seat plays, in any phase and whether it chose it or it was forced, falls in exactly one of
/// these columns, so the block adds up to the seat's actions in the game. The names go to `cenv_move_names`.
macro_rules! moves {
    ($($variant:ident => $name:literal,)+) => {
        #[derive(Clone, Copy)]
        enum Move { $($variant),+ }
        const MOVE_NAMES: &[&str] = &[$($name),+];
        const MOVE_NAMES_C: &str = concat!($($name, ","),+, "\0");
    };
}

moves! {
    // The main action of a turn.
    Produce => "produce",
    BuildFactory => "build_factory",
    BuildWarehouse => "build_warehouse",
    FactoryPurchase => "factory_purchase",
    HarborPurchase => "harbor_purchase",
    StartLoading => "start_loading",
    SailOcean => "sail_ocean",
    SailBank => "sail_bank",
    SailHarbor => "sail_harbor",
    SailIsland => "sail_island",
    RepriceFactory => "reprice_factory",
    RepriceHarbor => "reprice_harbor",
    Pass => "pass",
    CallContainerLot => "call_container_lot",
    CallCashLot => "call_cash_lot",
    OutbidContainerLot => "outbid_container_lot",
    OutbidCashLot => "outbid_cash_lot",
    RequestLoan => "request_loan",
    RepayLoan => "repay_loan",
    // The decisions inside an action, and the answers to the others'.
    ProduceColor => "produce_color",
    BuyFactory => "buy_factory",
    BuyHarbor => "buy_harbor",
    DoneBuying => "done_buying",
    Load => "load",
    DoneLoading => "done_loading",
    Place => "place",
    Bid => "bid",
    Accept => "accept",
    BuyOut => "buy_out",
    Seize => "seize",
    Deal => "deal",
    BidCash => "bid_cash",
    BidContainer => "bid_container",
    DoneBidding => "done_bidding",
}

/// The kind of move `action` is in the current phase. Repricing a factory district with nothing in it is a pass.
fn move_of(game: &Game, action: Action) -> Move {
    let me = &game.state.players[game.state.active_player];
    match action {
        Action::Produce => Move::Produce,
        Action::ProduceColor(_) => Move::ProduceColor,
        Action::Build(Building::Factory(_)) => Move::BuildFactory,
        Action::Build(Building::Warehouse) => Move::BuildWarehouse,
        Action::FactoryPurchase(_) => Move::FactoryPurchase,
        Action::Buy { .. } => match game.phase {
            Phase::Buy(b) if b.district == District::Harbor => Move::BuyHarbor,
            _ => Move::BuyFactory,
        },
        Action::DoneBuying => Move::DoneBuying,
        Action::Sail(ShipLocation::Ocean) => Move::SailOcean,
        Action::Sail(ShipLocation::Bank) => Move::SailBank,
        Action::Sail(ShipLocation::Harbor(_)) => Move::SailHarbor,
        Action::Sail(ShipLocation::Island) => Move::SailIsland,
        Action::HarborPurchase if me.ship_location == ShipLocation::Bank => Move::StartLoading,
        Action::HarborPurchase => Move::HarborPurchase,
        Action::Load(_) => Move::Load,
        Action::DoneLoading => Move::DoneLoading,
        Action::Reprice(District::Harbor) => Move::RepriceHarbor,
        Action::Reprice(District::Factory) if me.stock(District::Factory) == 0 => Move::Pass,
        Action::Reprice(District::Factory) => Move::RepriceFactory,
        Action::Place(_) => Move::Place,
        Action::Bid(_) => Move::Bid,
        Action::Accept(_) => Move::Accept,
        Action::BuyOut => Move::BuyOut,
        Action::RequestLoan => Move::RequestLoan,
        Action::RepayLoan => Move::RepayLoan,
        Action::Seize { .. } => Move::Seize,
        Action::Deal(_) => Move::Deal,
        Action::StartAuction { kind: AuctionKind::ContainerLot, .. } => Move::CallContainerLot,
        Action::StartAuction { kind: AuctionKind::CashLot, .. } => Move::CallCashLot,
        Action::Outbid(AuctionKind::ContainerLot) => Move::OutbidContainerLot,
        Action::Outbid(AuctionKind::CashLot) => Move::OutbidCashLot,
        Action::BidCash(_) => Move::BidCash,
        Action::BidContainer { .. } => Move::BidContainer,
        Action::DoneBidding => Move::DoneBidding,
    }
}

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
    legal: &[Action],
    action: Action,
) {
    inter[game.current_player()][MOVES + move_of(game, action) as usize] += 1;
    if game.phase == Phase::Main {
        count_main(game, inter, legal, action);
    }
    memory.record(game.current_player(), &game.phase, action);
    if game.phase == Phase::Main {
        *last_reprice = match action {
            Action::Reprice(district) => Some((game.state.active_player, district, game.turn.actions_left.saturating_sub(1))),
            _ => None,
        };
    }
    match (game.phase, action) {
        (Phase::Decide(d), Action::Accept(winner)) => {
            inter[d.deliverer][SUBSIDY] += d.top as i32;
            let size = game.state.players[d.deliverer].ship.total() as usize;
            if (1..=5).contains(&size) {
                inter[d.deliverer][CARGO_CASH + size - 1] += 2 * d.top as i32;
            }
            traded[d.deliverer] += d.top as i32;
            traded[winner] += d.top as i32;
            inter[winner][DEALS_WON] += 1;
            inter[winner][DEAL_PAID] += d.top as i32;
        }
        (Phase::Decide(d), Action::BuyOut) => inter[d.deliverer][BUYOUT_PAID] += d.top as i32,
        (Phase::Buy(b), Action::Buy { lot, color }) => {
            let buyer = game.state.active_player;
            let price = b.district.lot_price(lot) as i32;
            inter[buyer][BOUGHT] += 1;
            inter[buyer][SPENT] += price;
            inter[b.seller][SOLD] += 1;
            inter[b.seller][EARNED] += price;
            inter[buyer][BOUGHT_COLOR + color.index()] += 1;
            inter[b.seller][SOLD_COLOR + color.index()] += 1;
        }
        (Phase::Bid(b), Action::Bid(amount)) => {
            let value = bid_allowance(game, b.bidder, b.deliverer);
            let (before, after) = (b.bids[b.bidder], b.bids[b.bidder] + amount);
            inter[b.bidder][BIDS] += 1;
            inter[b.bidder][BID_MONEY] += amount as i32;
            inter[b.bidder][REAL_BIDS] += (amount >= REAL_BID) as i32;
            inter[b.bidder][BID_VALUE] += value as i32;
            let cash = game.state.players[b.bidder].cash;
            inter[b.bidder][BID_CASH] += cash as i32;
            inter[b.deliverer][RIVAL_BIDS] += 1;
            inter[b.deliverer][RIVAL_CASH_3] += (cash >= 3) as i32;
            inter[b.deliverer][RIVAL_CASH_10] += (cash >= 10) as i32;
            inter[b.bidder][OVERBID] += (after.saturating_sub(value) - before.saturating_sub(value)) as i32;
        }
        (Phase::Arrange(a), Action::Place(lot)) => {
            let price = a.district.lot_price(lot) as usize;
            inter[a.owner][PLACED_AT + price - 1] += 1;
        }
        _ => {}
    }
    let next = (game.state.active_player + 1) % game.state.num_players;
    let due = i32::from(game.state.players[next].loans);
    game.apply(action).expect("legal action");
    // The turn passed to `next`, who paid interest on the loans it held.
    if game.state.active_player == next {
        inter[next][INTEREST] += due;
    }
}

/// What a bound seat may have on the table for the cargo on the deliverer's ship: every container at the value
/// of the bidder's own secret card, with a full set of colors assumed and nothing discarded, but never less than
/// `BID_ALLOWANCE_FLOOR`, so that a small cargo can still be bid on and the table minimum never conflicts with it.
fn bid_allowance(game: &Game, bidder: PlayerId, deliverer: PlayerId) -> u32 {
    game.state.players[bidder].scoring_card.cargo_value(&game.state.players[deliverer].ship).max(BID_ALLOWANCE_FLOOR)
}

const BID_ALLOWANCE_FLOOR: u32 = 5;

/// The strategy counters of a Main decision: what the seat offers, whether it could produce and did, and
/// whether it sets sail for the Island.
fn count_main(game: &Game, inter: &mut Interaction, legal: &[Action], action: Action) {
    let seat = game.state.active_player;
    let me = &game.state.players[seat];
    let row = &mut inter[seat];
    row[MAIN_DECISIONS] += 1;
    row[FACTORIES_HELD] += me.factory_count() as i32;
    row[WAREHOUSES_HELD] += me.warehouses as i32;
    row[HARBOR_OFFERED] += me.harbor_stock() as i32;
    let mut colors = [false; NUM_COLORS];
    for (district, lots) in [(District::Factory, &me.factory_lots[..]), (District::Harbor, &me.harbor_lots[..])] {
        for (lot, counts) in lots.iter().enumerate() {
            row[OFFERED] += counts.total() as i32;
            if district.lot_price(lot) <= COMPETITIVE_PRICE {
                row[OFFERED_COMPETITIVE] += counts.total() as i32;
                for color in Color::ALL {
                    colors[color.index()] |= counts[color] > 0;
                }
            }
        }
    }
    row[COMPETITIVE_COLORS] += colors.iter().filter(|&&c| c).count() as i32;
    if legal.contains(&Action::Produce) {
        row[PRODUCE_CHANCES] += 1;
    }
    match action {
        Action::Produce => {
            let candidates = me.factory_colors().filter(|&c| game.state.supply.containers[c] > 0).count() as u32;
            row[PRODUCES] += 1;
            row[PRODUCED] += candidates.min(me.factory_room()) as i32;
        }
        Action::Sail(ShipLocation::Island) => {
            row[DELIVERIES] += 1;
            row[SHIPPED] += me.ship.total() as i32;
            let size = me.ship.total() as usize;
            if (1..=5).contains(&size) {
                row[CARGO_SIZE + size - 1] += 1;
            }
            row[CARGO_COLORS] += Color::ALL.iter().filter(|&&c| me.ship[c] > 0).count() as i32;
        }
        _ => {}
    }
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
    /// The slot that may not have more on the table in a delivery auction than the cargo is worth on its own secret
    /// card (every container at its value, a full set assumed, nothing discarded), or `BID_ALLOWANCE_FLOOR` if that is more.
    bid_within_value: Option<i32>,
    /// `(slot, like)`: the seats of `slot` play under the rules that bind slot `like`.
    mirror: Option<(i32, i32)>,
    /// `(mask, like)`: the net slots `1000 + k` with bit `k` set in `mask` play under the rules that bind slot `like`
    /// (several frozen nets trained under the same rules at one table).
    mirror_set: Option<(u32, i32)>,
    /// The build script of a slot (`cenv_set_build_script`). (`cenv_set_build_script`).
    build_script: Option<BuildScript>,
    /// Probe: every seat whose kind is not this slot bids for a delivery by the value of the cargo on its own card
    /// (`cenv_set_value_bidder`): the percentage of `cargo_value`, so that rivals pay more for a bigger cargo.
    value_bidder: Option<(i32, u32)>,
    /// With `value_bidder`: a value bidder that lacks the cash for its target is lent the difference for the auction,
    /// and the loan is taken back once the auction is settled. It separates what rivals can pay from what the cargo is worth.
    lend_bidders: bool,
    /// The slot that may not sail to the Island with fewer than this many containers on the ship.
    min_cargo: Option<(i32, u32)>,
}

/// What a value bidder would have on the table in all in this auction: `percent` of the cargo's value on its card.
fn bid_target(rules: Rules, game: &Game, kind: i32) -> Option<u32> {
    let (_, percent) = rules.value_bidder.filter(|&(slot, _)| slot != kind)?;
    let Phase::Bid(b) = game.phase else { return None };
    let value = game.state.players[b.bidder].scoring_card.cargo_value(&game.state.players[b.deliverer].ship);
    Some(value * percent / 100)
}

/// The bid a value bidder makes: the most the legal bids allow up to its target (counting what it already has on
/// the table), else the lowest legal bid.
fn script_bid(rules: Rules, game: &Game, kind: i32, legal: &[Action]) -> Option<Action> {
    let target = bid_target(rules, game, kind)?;
    let Phase::Bid(b) = game.phase else { return None };
    let placed = b.bids[b.bidder];
    let bids = legal.iter().filter_map(|a| if let Action::Bid(x) = a { Some(*x) } else { None });
    let pick = bids.clone().filter(|&x| placed + x <= target).max().or_else(|| bids.min())?;
    Some(Action::Bid(pick))
}

/// Investment probe: seats of `slot` build `count` buildings of one kind at their first Main decision that comes
/// after `after_main` of their own, where the build is legal and leaves at least `reserve` cash. Everything else
/// is left to the seat's policy.
#[derive(Clone, Copy)]
struct BuildScript {
    slot: i32,
    warehouse: bool,
    count: i32,
    after_main: i32,
    reserve: u32,
}

/// The build the script makes in this Main decision, if any: a factory in the color with the most supply left.
fn script_build(rules: Rules, game: &Game, row: &[i32; INTERACTION], kind: i32, legal: &[Action]) -> Option<Action> {
    let s = rules.build_script.filter(|s| s.slot == kind)?;
    if game.phase != Phase::Main || row[SCRIPT_BUILDS] >= s.count || row[MAIN_DECISIONS] < s.after_main {
        return None;
    }
    let me = &game.state.players[game.state.active_player];
    let cost = if s.warehouse {
        container_engine::build::warehouse_build_cost(u32::from(me.warehouses))
    } else {
        container_engine::build::factory_build_cost(me.factory_count())
    }?;
    if me.cash < cost + s.reserve {
        return None;
    }
    legal
        .iter()
        .copied()
        .filter(|a| match a {
            Action::Build(Building::Warehouse) => s.warehouse,
            Action::Build(Building::Factory(_)) => !s.warehouse,
            _ => false,
        })
        .max_by_key(|a| match a {
            Action::Build(Building::Factory(c)) => game.state.supply.containers[*c],
            _ => 0,
        })
}

impl Rules {
    /// Whether any rule binds the seats of this slot.
    fn bind(&self, kind: i32) -> bool {
        self.no_bank_all
            || self.min_cargo.is_some_and(|(slot, _)| slot == kind)
            || [self.no_bank, self.buy_at_harbor, self.no_same_turn_repay, self.min_bid, self.max_price, self.bid_within_value, self.no_double_reprice, self.bank_needs_load].contains(&Some(kind))
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
    /// Per seat, the cash lent to a value bidder for the delivery auction under way (`Rules::lend_bidders`).
    lent: [u32; MAX_PLAYERS],
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
            lent: [0; MAX_PLAYERS],
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
                -8 => create("producer", seed ^ (seat as u64 + 1)),
                -9 => create("showcase", seed ^ (seat as u64 + 1)),
                -10 => create("shipper", seed ^ (seat as u64 + 1)),
                k @ -19..=-11 => container_bots::archetype::variant((-11 - k) as usize, seed ^ (seat as u64 + 1))
                    .map(|a| Box::new(a) as Box<dyn Opponent + Send>),
                _ => None,
            })
            .collect();
        self.teacher = create("merchant", seed ^ 0x7EAC4E5);
        self.actions = 0;
        self.traded = [0; MAX_PLAYERS];
        self.inter = [[0; INTERACTION]; MAX_PLAYERS];
        self.memory = Memory::default();
        self.last_reprice = None;
        self.lent = [0; MAX_PLAYERS];
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
            // The loan to a value bidder ends with its auction.
            for (seat, lent) in self.lent.iter_mut().enumerate().filter(|(_, l)| **l > 0) {
                if !matches!(game.phase, Phase::Bid(_) | Phase::Decide(_)) {
                    let cash = &mut game.state.players[seat].cash;
                    *cash -= (*lent).min(*cash);
                    *lent = 0;
                }
            }
            if game.is_terminal() {
                self.status = Status::Done;
                return;
            }
            if self.actions >= self.horizon {
                self.status = Status::Stalled;
                return;
            }
            let seat = game.current_player();
            if let (true, Some(target), Phase::Bid(b)) = (self.rules.lend_bidders, bid_target(self.rules, game, self.kinds[seat]), game.phase) {
                let cash = &mut game.state.players[b.bidder].cash;
                let lack = target.saturating_sub(b.bids[b.bidder]).saturating_sub(*cash);
                *cash += lack;
                self.lent[b.bidder] += lack;
            }
            let mut legal = game.legal_actions();
            let kind = self.kinds[seat];
            let kind = self.rules.mirror.filter(|&(slot, _)| slot == kind).map_or(kind, |(_, like)| like);
            let kind = match self.rules.mirror_set {
                Some((mask, like)) if kind >= 1000 && kind < 1032 && mask >> (kind - 1000) & 1 == 1 => like,
                _ => kind,
            };
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
            let built = script_build(self.rules, game, &self.inter[seat], self.kinds[seat], &legal)
                .or_else(|| script_bid(self.rules, game, self.kinds[seat], &legal));
            if built.is_some() && game.phase == Phase::Main {
                self.inter[seat][SCRIPT_BUILDS] += 1;
            }
            let action = if let Some(a) = built {
                a
            } else if let Some(bot) = self.scripted[seat].as_mut() {
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
            apply(game, &mut self.traded, &mut self.inter, &mut self.memory, &mut self.last_reprice, &legal, action);
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
        apply(self.game.as_mut().expect("game"), &mut self.traded, &mut self.inter, &mut self.memory, &mut self.last_reprice, &self.legal, action);
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
    if let (Phase::Bid(b), Action::Bid(amount)) = (game.phase, action) {
        let floor = rules.min_bid_amount.min(game.state.players[b.bidder].cash);
        if rules.min_bid == Some(kind) && !b.runoff && amount < floor {
            return true;
        }
        if rules.bid_within_value == Some(kind) && b.bids[b.bidder] + amount > bid_allowance(game, b.bidder, b.deliverer) {
            return true;
        }
    }
    if let (Some(slot), Phase::Arrange(a), Action::Place(lot)) = (rules.max_price, game.phase, action) {
        if slot == kind && a.district.lot_price(lot) > rules.max_price_amount {
            return true;
        }
    }
    if let (Some((slot, n)), Phase::Main, Action::Sail(ShipLocation::Island)) = (rules.min_cargo, game.phase, action) {
        if slot == kind && game.state.players[game.state.active_player].ship.total() < n {
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

/// Makes the net slots `1000 + k` with bit `k` set in `mask` play under the rules that bind slot `like`; a zero mask lifts it.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_mirror_rules_set(b: *mut Batch, mask: u32, like: c_int) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.mirror_set = (mask != 0).then_some((mask, like));
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

/// Seats of `slot` may not put more money on the table in a delivery auction than the cargo is worth on their
/// own scoring card (`bid_allowance`: every container at its value, a full set of colors assumed, none discarded,
/// and at least `BID_ALLOWANCE_FLOOR`); a negative `slot` lifts the rule.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_require_bid_within_value(b: *mut Batch, slot: c_int) {
    let batch = unsafe { &mut *b };
    let rule = (slot >= 0).then_some(slot);
    for env in &mut batch.envs {
        env.rules.bid_within_value = rule;
    }
}

static MEMORY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Probe: every seat except those of `slot` bids for deliveries by `percent` of the cargo's value on its own card (0
/// lifts it; pass `i32::MIN` for everyone). With `lend` non-zero a bidder that lacks the cash for its bid is lent the
/// difference until the auction is settled.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_value_bidder(b: *mut Batch, slot: c_int, percent: u32, lend: u32) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.value_bidder = (percent > 0).then_some((slot, percent));
        env.rules.lend_bidders = lend != 0;
    }
}

/// Probe: seats of `slot` may not sail to the Island with fewer than `containers` on the ship (waived if nothing else
/// is legal); a negative `slot` lifts it.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_min_cargo(b: *mut Batch, slot: c_int, containers: u32) {
    let batch = unsafe { &mut *b };
    for env in &mut batch.envs {
        env.rules.min_cargo = (slot >= 0).then_some((slot, containers));
    }
}

/// Investment probe: seats of `slot` build `count` factories (`warehouse` 0) or warehouses (non-zero) at their first
/// Main decision after `after_main` of their own where the build is legal and leaves `reserve` cash; a negative
/// `slot` lifts the script.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_set_build_script(
    b: *mut Batch,
    slot: c_int,
    warehouse: c_int,
    count: c_int,
    after_main: c_int,
    reserve: u32,
) {
    let batch = unsafe { &mut *b };
    let script = (slot >= 0).then_some(BuildScript { slot, warehouse: warehouse != 0, count, after_main, reserve });
    for env in &mut batch.envs {
        env.rules.build_script = script;
    }
}

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

/// The number of columns per seat of `cenv_interaction`, so that a caller can tell a stale library.
#[unsafe(no_mangle)]
pub extern "C" fn cenv_interaction_width() -> usize {
    INTERACTION
}

/// The names of the move counters of `cenv_interaction`, which start at column `MOVES`: comma separated
/// and NUL terminated.
#[unsafe(no_mangle)]
pub extern "C" fn cenv_move_names() -> *const std::os::raw::c_char {
    MOVE_NAMES_C.as_ptr().cast()
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
    fn the_allowance_is_the_cargo_on_the_bidders_own_card_and_never_under_the_floor() {
        use container_engine::color::ColorCounts;
        use container_engine::scoring::ScoringCard;
        let mut env = Env::new();
        env.reset(3, 0, &[0, 0, 0]);
        let game = env.game.as_mut().unwrap();
        // Seat 2 delivers. Seat 0 values red at $10, white at $4 and yellow at $6; seat 1 values red at $6, white at $10, yellow at $2.
        for (seat, ten) in [(0, Color::Red), (1, Color::White), (2, Color::Blue)] {
            game.state.players[seat].scoring_card = ScoringCard { ten };
        }
        let floor = BID_ALLOWANCE_FLOOR;
        for (ship, seat_0, seat_1) in [
            ([3, 0, 0, 0, 0], 30, 18),
            ([1, 1, 1, 1, 1], 32, 32),
            ([0, 0, 1, 0, 0], 6, floor),
            ([0, 0, 0, 1, 0], floor, 10),
            ([0, 0, 0, 0, 0], floor, floor),
        ] {
            game.state.players[2].ship = ColorCounts(ship);
            assert_eq!([bid_allowance(game, 0, 2), bid_allowance(game, 1, 2)], [seat_0, seat_1], "ship {ship:?}");
        }
    }

    #[test]
    fn a_slot_bound_to_the_value_of_the_cargo_never_has_more_on_the_table_than_it_is_worth() {
        // Indexed by whether the slot is bound; the minimum bid of $3 binds it either way.
        let (mut bids, mut over, mut counted, mut floored) = ([0usize; 2], [0usize; 2], [0i32; 2], 0usize);
        for bound in [false, true] {
            for seed in 0..30 {
                let mut env = Env::new();
                env.rules.min_bid = Some(0);
                env.rules.min_bid_amount = 3;
                if bound {
                    env.rules.bid_within_value = Some(0);
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
                        let allowance = bid_allowance(game, b.bidder, b.deliverer);
                        let worth = game.state.players[b.bidder].scoring_card.cargo_value(&game.state.players[b.deliverer].ship);
                        for &i in &env.thinned {
                            if let Action::Bid(a) = env.legal[i] {
                                bids[bound as usize] += 1;
                                over[bound as usize] += (b.bids[b.bidder] + a > allowance) as usize;
                                floored += (bound && b.bids[b.bidder] + a > worth) as usize;
                            }
                        }
                    }
                    step += 1;
                    env.step((step * 3 + seed as usize) % env.thinned.len());
                }
                counted[bound as usize] += (0..3).map(|p| env.inter[p][OVERBID]).sum::<i32>();
            }
        }
        assert!(bids[1] > 0, "random play should reach delivery auctions");
        assert!(floored > 0, "a cargo worth less than the floor can still be bid on up to the floor");
        assert!(over[0] > 0 && counted[0] > 0, "an unbound slot does bid more than the cargo is worth");
        assert_eq!((over[1], counted[1]), (0, 0));
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
    fn the_heuristic_seats_play_a_game_out_at_every_table_size() {
        for (players, seed) in [(3, 3), (4, 4), (5, 5)] {
            let kinds: Vec<i32> = (0..players).map(|seat| -8 - (seat as i32 % 3)).collect();
            let mut env = Env::new();
            env.reset(players, seed, &kinds);
            play_out(&mut env, 1);
            assert_eq!(env.status, Status::Done, "{players} players");
        }
    }

    #[test]
    fn the_stock_a_seat_ends_with_is_what_its_counters_leave_over() {
        // `training/gate.py` and the EGTA report derive the unsold stock as 1 + produced + bought - sold - shipped.
        // The Bank is the one source of containers no counter follows, and the heuristic seats never call it.
        for (players, seed) in (3..=5).flat_map(|players| (0..8).map(move |seed| (players, seed))) {
            let kinds: Vec<i32> = (0..players).map(|seat| -8 - ((seat as i32 + seed as i32) % 3)).collect();
            let mut env = Env::new();
            env.reset(players, seed, &kinds);
            play_out(&mut env, 1);
            let game = env.game.as_ref().unwrap();
            for (seat, row) in env.inter.iter().enumerate().take(players) {
                let me = &game.state.players[seat];
                let held = (me.factory_stock() + me.harbor_stock() + me.ship.total() + me.holding.total()) as i32;
                assert_eq!(held, 1 + row[PRODUCED] + row[BOUGHT] - row[SOLD] - row[SHIPPED], "seat {seat} of {players}");
            }
        }
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
    fn the_subsidy_counter_matches_the_money_the_engine_minted() {
        let mut subsidised = 0;
        for seed in 0..20 {
            let mut env = Env::new();
            env.reset(3, seed, &[0, 0, 0]);
            play_out(&mut env, seed as usize + 1);
            let minted = env.game.as_ref().unwrap().state.cash_minted as i32;
            let subsidy: i32 = env.inter.iter().map(|row| row[SUBSIDY]).sum();
            assert!(minted >= subsidy && (minted - subsidy) % container_engine::loans::LOAN_AMOUNT as i32 == 0, "the rest is loans");
            subsidised += subsidy;
        }
        assert!(subsidised > 0, "random play should sell deliveries to players");
    }

    #[test]
    fn the_deal_counters_match_the_deliveries_and_interest_is_counted_when_a_turn_begins() {
        let column = |name: &str| MOVES + MOVE_NAMES.iter().position(|&n| n == name).unwrap();
        let (mut won, mut bought_out) = (0, 0);
        for seed in 0..20 {
            let mut env = Env::new();
            env.reset(3, seed, &[0, 0, 0]);
            play_out(&mut env, seed as usize + 1);
            let sum = |c: usize| env.inter.iter().map(|row| row[c]).sum::<i32>();
            assert_eq!(sum(DEAL_PAID), sum(SUBSIDY), "the winner pays the bid that the supply matches");
            assert_eq!(sum(DEALS_WON), sum(column("accept")));
            won += sum(DEALS_WON);
            bought_out += sum(BUYOUT_PAID);
            for row in &env.inter {
                assert!(row[column("buy_out")] > 0 || row[BUYOUT_PAID] == 0);
            }
        }
        assert!(won > 0 && bought_out > 0, "random play sells some deliveries and buys some out");

        let mut env = Env::new();
        env.reset(3, 4, &[0, 0, 0]);
        let game = env.game.as_mut().unwrap();
        let first = game.state.active_player;
        let next = (first + 1) % 3;
        game.state.players[next].loans = 2;
        for _ in 0..50 {
            if game.state.active_player != first {
                break;
            }
            let legal = game.legal_actions();
            apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, legal[0]);
        }
        assert_eq!(game.state.active_player, next);
        assert_eq!(env.inter[next][INTEREST], 2, "$1 per loan held when the turn begins");
        assert_eq!(env.inter[first][INTEREST] + env.inter[(first + 2) % 3][INTEREST], 0);
    }

    #[test]
    fn the_shipped_counter_is_the_cargo_that_reaches_the_scoring_area() {
        use container_engine::color::ColorCounts;
        fn play(env: &mut Env, action: Action) {
            let game = env.game.as_mut().unwrap();
            let legal = game.legal_actions();
            assert!(legal.contains(&action), "{action:?}");
            apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, action);
        }
        let mut env = Env::new();
        env.reset(3, 0, &[0, 0, 0]);
        let game = env.game.as_mut().unwrap();
        let seller = game.state.active_player;
        let (first, second) = ((seller + 1) % 3, (seller + 2) % 3);
        game.state.players[seller].ship = ColorCounts([2, 1, 0, 0, 0]);
        game.state.players[seller].ship_location = ShipLocation::Ocean;

        play(&mut env, Action::Sail(ShipLocation::Island));
        play(&mut env, Action::Bid(2));
        play(&mut env, Action::Bid(0));
        play(&mut env, Action::Accept(first));

        let game = env.game.as_ref().unwrap();
        assert_eq!(game.state.players[first].scoring_area.total(), 3);
        assert_eq!(game.state.players[second].scoring_area.total() + game.state.players[seller].scoring_area.total(), 0);
        assert_eq!([env.inter[seller][DELIVERIES], env.inter[seller][SHIPPED]], [1, 3]);
        assert_eq!(env.inter[first][SHIPPED] + env.inter[second][SHIPPED], 0, "only the deliverer ships");
        let row = &env.inter[seller];
        assert_eq!(row[CARGO_SIZE..CARGO_SIZE + 5], [0, 0, 1, 0, 0], "a 3-container cargo is in the third bin");
        assert_eq!(row[CARGO_CASH..CARGO_CASH + 5], [0, 0, 4, 0, 0], "the deliverer got twice the winning bid");
        assert_eq!(row[CARGO_COLORS], 2);
    }

    #[test]
    fn auction_cash_counters_count_each_rival_bid_once_and_never_more_than_the_cash() {
        let mut env = Env::new();
        env.reset(3, 0, &[-3, -3, -3]);
        env.advance();
        assert_eq!(env.status, Status::Done, "scripted merchants play the game out");
        let sum = |c: usize| env.inter.iter().map(|row| row[c]).sum::<i32>();
        assert!(sum(BIDS) > 0, "the game had delivery auctions");
        assert_eq!(sum(RIVAL_BIDS), sum(BIDS), "every bid is a rival's bid in the deliverer's row");
        assert!(sum(RIVAL_CASH_10) <= sum(RIVAL_CASH_3) && sum(RIVAL_CASH_3) <= sum(RIVAL_BIDS));
        assert!(sum(BID_CASH) >= sum(BID_MONEY), "a bid is never more than the bidder's cash");
    }

    #[test]
    fn the_build_script_builds_once_with_cash_to_spare_and_leaves_the_rest() {
        for warehouse in [false, true] {
            let mut env = Env::new();
            env.reset(3, 0, &[0, 1, 1]);
            env.rules.build_script = Some(BuildScript { slot: 0, warehouse, count: 1, after_main: 0, reserve: 5 });
            let (f, w) = (MOVES + Move::BuildFactory as usize, MOVES + Move::BuildWarehouse as usize);
            for _ in 0..400 {
                env.advance();
                if env.status != Status::Pending {
                    break;
                }
                env.step(0);
            }
            let row = &env.inter[0];
            let scripted = if warehouse { w } else { f };
            assert_eq!(row[SCRIPT_BUILDS], 1, "built once, as cash allowed");
            assert!(row[scripted] >= 1);
            assert_eq!(env.inter[1][SCRIPT_BUILDS], 0, "only the slot's seats");
        }
    }

    #[test]
    fn value_bidders_bid_more_for_a_bigger_cargo_and_the_excepted_slot_is_left_alone() {
        use container_engine::color::ColorCounts;
        let mut totals = Vec::new();
        for ship in [ColorCounts([1, 0, 0, 0, 0]), ColorCounts([1, 1, 1, 0, 0])] {
            let mut env = Env::new();
            env.reset(3, 0, &[0, 1, 1]);
            env.rules.value_bidder = Some((0, 50));
            env.advance();
            let game = env.game.as_mut().unwrap();
            let seat = game.state.active_player;
            game.state.players[seat].ship = ship;
            game.state.players[seat].ship_location = ShipLocation::Ocean;
            let legal = game.legal_actions();
            apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, Action::Sail(ShipLocation::Island));
            let mut sum = 0;
            for _ in 0..2 {
                let Phase::Bid(b) = game.phase else { panic!("bidding") };
                let legal = game.legal_actions();
                let kind = env.kinds[b.bidder];
                match script_bid(env.rules, game, kind, &legal) {
                    Some(Action::Bid(x)) => {
                        assert_ne!(kind, 0, "the excepted slot is not scripted");
                        sum += x;
                        apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, Action::Bid(x));
                    }
                    _ => {
                        assert_eq!(kind, 0);
                        apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, Action::Bid(0));
                    }
                }
            }
            totals.push(sum);
        }
        assert!(totals[1] > totals[0], "{totals:?}");
    }

    #[test]
    fn a_lent_bidder_bids_its_whole_target_and_the_loan_is_taken_back_with_the_auction() {
        use container_engine::color::ColorCounts;
        let cargo = ColorCounts([5, 0, 0, 0, 0]);
        let mut env = Env::new();
        env.reset(3, 0, &[0, 1, 1]);
        env.rules.value_bidder = Some((i32::MIN, 100));
        env.rules.lend_bidders = true;
        env.advance();
        let game = env.game.as_mut().unwrap();
        let deliverer = game.state.active_player;
        let rivals: Vec<usize> = (0..3).filter(|&s| s != deliverer).collect();
        game.state.players[deliverer].ship = cargo;
        game.state.players[deliverer].ship_location = ShipLocation::Ocean;
        for &r in &rivals {
            game.state.players[r].cash = 1;
        }
        let before = game.state.players[deliverer].cash;
        let legal = game.legal_actions();
        apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, Action::Sail(ShipLocation::Island));
        env.status = Status::Idle;
        env.advance();

        let game = env.game.as_ref().unwrap();
        let Phase::Decide(d) = game.phase else { panic!("the bids are in") };
        let targets: Vec<u32> = rivals.iter().map(|&s| game.state.players[s].scoring_card.cargo_value(&cargo)).collect();
        assert!(targets.iter().all(|&t| t > 1), "the rivals could not afford their targets: {targets:?}");
        assert_eq!(d.top, *targets.iter().max().unwrap(), "every rival bids its whole target");
        assert!(env.lent.iter().sum::<u32>() > 0, "the loan stands until the auction is settled");

        let winner = (0..3).find(|&s| d.tied[s]).unwrap();
        let legal = env.legal.clone();
        let game = env.game.as_mut().unwrap();
        apply(game, &mut env.traded, &mut env.inter, &mut env.memory, &mut env.last_reprice, &legal, Action::Accept(winner));
        env.status = Status::Idle;
        env.advance();
        let game = env.game.as_ref().unwrap();
        assert_eq!(env.lent, [0; MAX_PLAYERS], "the loan is taken back");
        for &r in &rivals {
            assert_eq!(game.state.players[r].cash, u32::from(r != winner), "seat {r}: back to its own cash, the winner has paid it all");
        }
        assert_eq!(game.state.players[deliverer].cash, before + 2 * d.top, "the deliverer is paid twice the top bid");
    }

    #[test]
    fn a_slot_with_a_minimum_cargo_does_not_sail_to_the_island_with_less() {
        let mut env = Env::new();
        env.reset(3, 0, &[0, 1, 1]);
        env.rules.min_cargo = Some((0, 2));
        let mut checked = 0;
        for _ in 0..3000 {
            env.advance();
            if env.status != Status::Pending {
                break;
            }
            let game = env.game.as_ref().unwrap();
            let seat = game.current_player();
            if env.kinds[seat] == 0 && game.phase == Phase::Main && env.legal.len() > 1 {
                let island = env.legal.iter().filter(|&&a| a == Action::Sail(ShipLocation::Island)).count();
                let others = env.legal.iter().any(|&a| a != Action::Sail(ShipLocation::Island));
                if game.state.players[seat].ship.total() < 2 && others {
                    assert_eq!(island, 0, "offered an Island sail with one container");
                }
                checked += others as usize;
            }
            env.step(0);
        }
        assert!(checked > 0, "the slot must have had Main decisions");
    }

    #[test]
    fn every_kind_of_action_is_its_own_move() {
        use container_engine::phase::Buying;
        let mut game = Game::new(GameConfig::new(3).with_seed(1)).unwrap();
        let (c, active) = (Color::ALL[0], game.state.active_player);
        let mut seen = std::collections::HashSet::new();
        let mut check = |game: &Game, action: Action, expected: &'static str| {
            assert_eq!(MOVE_NAMES[move_of(game, action) as usize], expected, "{action:?}");
            seen.insert(expected);
        };
        for (action, expected) in [
            (Action::Produce, "produce"),
            (Action::ProduceColor(c), "produce_color"),
            (Action::Build(Building::Factory(c)), "build_factory"),
            (Action::Build(Building::Warehouse), "build_warehouse"),
            (Action::FactoryPurchase(1), "factory_purchase"),
            (Action::Buy { lot: 0, color: c }, "buy_factory"),
            (Action::DoneBuying, "done_buying"),
            (Action::Sail(ShipLocation::Ocean), "sail_ocean"),
            (Action::Sail(ShipLocation::Bank), "sail_bank"),
            (Action::Sail(ShipLocation::Harbor(1)), "sail_harbor"),
            (Action::Sail(ShipLocation::Island), "sail_island"),
            (Action::HarborPurchase, "harbor_purchase"),
            (Action::Load(c), "load"),
            (Action::DoneLoading, "done_loading"),
            (Action::Reprice(District::Harbor), "reprice_harbor"),
            (Action::Place(0), "place"),
            (Action::Bid(3), "bid"),
            (Action::Accept(1), "accept"),
            (Action::BuyOut, "buy_out"),
            (Action::RequestLoan, "request_loan"),
            (Action::RepayLoan, "repay_loan"),
            (Action::Seize { lot: 0, color: c }, "seize"),
            (Action::Deal(c), "deal"),
            (Action::StartAuction { kind: AuctionKind::ContainerLot, lot: 0 }, "call_container_lot"),
            (Action::StartAuction { kind: AuctionKind::CashLot, lot: 0 }, "call_cash_lot"),
            (Action::Outbid(AuctionKind::ContainerLot), "outbid_container_lot"),
            (Action::Outbid(AuctionKind::CashLot), "outbid_cash_lot"),
            (Action::BidCash(5), "bid_cash"),
            (Action::BidContainer { district: District::Factory, lot: 0, color: c }, "bid_container"),
            (Action::DoneBidding, "done_bidding"),
        ] {
            check(&game, action, expected);
        }
        // What the move is depends on what the district holds, where the ship is and what is being bought from.
        game.state.players[active].factory_lots = Default::default();
        check(&game, Action::Reprice(District::Factory), "pass");
        game.state.players[active].factory_lots[0][c] += 1;
        check(&game, Action::Reprice(District::Factory), "reprice_factory");
        game.state.players[active].ship_location = ShipLocation::Bank;
        check(&game, Action::HarborPurchase, "start_loading");
        let buying = Buying { seller: 1, district: District::Harbor, bought: Default::default(), next: 0, optional: false };
        game.phase = Phase::Buy(buying);
        check(&game, Action::Buy { lot: 0, color: c }, "buy_harbor");
        assert_eq!(seen.len(), MOVE_NAMES.len(), "every column is some action's move");
        assert!(MOVE_NAMES_C.starts_with(MOVE_NAMES[0]) && MOVE_NAMES_C.ends_with(",\0"));
    }

    #[test]
    fn the_moves_of_a_game_add_up_to_its_actions_and_agree_with_the_other_counters() {
        let column = |name: &str| MOVES + MOVE_NAMES.iter().position(|&n| n == name).unwrap();
        let (mut at_factories, mut at_harbors) = (0, 0);
        for seed in 0..20 {
            let mut env = Env::new();
            env.reset(3, seed, &[0, 0, 0]);
            play_out(&mut env, seed as usize + 1);
            let moves: usize = env.inter.iter().map(|row| row[MOVES..].iter().sum::<i32>() as usize).sum();
            assert_eq!(moves, env.actions, "forced moves included");
            for row in &env.inter {
                assert_eq!(row[column("buy_factory")] + row[column("buy_harbor")], row[BOUGHT]);
                assert_eq!(row[column("produce")], row[PRODUCES]);
                assert_eq!(row[column("sail_island")], row[DELIVERIES]);
                assert_eq!(row[column("bid")], row[BIDS]);
                assert_eq!(row[column("place")], row[PLACED_AT..PLACED_AT + 6].iter().sum::<i32>());
                at_factories += row[column("buy_factory")];
                at_harbors += row[column("buy_harbor")];
            }
        }
        assert!(at_factories > 0 && at_harbors > 0, "random play buys at both districts");
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
