//! A neural opponent: the policy net trained in `training/`, run in plain Rust.
//!
//! The weights file is written by `training/model.py::export_weights`. The net scores every legal action
//! from the state and the action's features; the opponent plays the best one, or samples from the scores
//! at a temperature when it is meant to be weaker or less predictable.

use std::collections::HashMap;
use std::sync::Arc;

use container_engine::action::{Action, District};
use container_engine::state::{SHIP_CAPACITY, ShipLocation};
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::rng::Rng;

use crate::{Opponent, is_same_turn_repay};
use crate::features::{ACTION_DIM, MAX_ACTIONS, STATE_DIM, STATE_DIM_MEM, encode_action, encode_state, encode_state_mem, thin};
use crate::memory::Memory;

const MAGIC: &[u8; 4] = b"CNTW";

// `Neural::tried` keeps one bit per action the net considers.
const _: () = assert!(MAX_ACTIONS <= u64::BITS as usize);

/// A fully connected layer, weights stored out x in.
#[derive(Clone, Debug)]
struct Layer {
    inputs: usize,
    w: Vec<f32>,
    b: Vec<f32>,
}

impl Layer {
    fn apply(&self, x: &[f32], out: &mut [f32]) {
        for (o, (row, bias)) in out.iter_mut().zip(self.w.chunks(self.inputs).zip(&self.b)) {
            *o = *bias + row.iter().zip(x).map(|(w, x)| w * x).sum::<f32>();
        }
    }

    fn apply_relu(&self, x: &[f32], out: &mut [f32]) {
        self.apply(x, out);
        out.iter_mut().for_each(|v| *v = v.max(0.0));
    }
}

/// The trained weights. Shared between opponents, which only read them.
#[derive(Clone, Debug)]
pub struct Net {
    hidden: usize,
    /// Whether the net reads the opponent memory after the usual state.
    memory: bool,
    t1: Layer,
    t2: Layer,
    ws: Layer,
    wa: Layer,
    h1: Layer,
    h2: Layer,
    v1: Layer,
    v2: Layer,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetError {
    BadMagic,
    Truncated,
    WrongShape,
}

impl Net {
    pub fn from_bytes(bytes: &[u8]) -> Result<Net, NetError> {
        if bytes.len() < 16 || &bytes[..4] != MAGIC {
            return Err(NetError::BadMagic);
        }
        let word = |i: usize| u32::from_le_bytes(bytes[4 + 4 * i..8 + 4 * i].try_into().unwrap()) as usize;
        let (state, action, hidden) = (word(0), word(1), word(2));
        if (state != STATE_DIM && state != STATE_DIM_MEM) || action != ACTION_DIM {
            return Err(NetError::WrongShape);
        }
        let mut pos = 16;
        let mut layer = |inputs: usize, outputs: usize| -> Result<Layer, NetError> {
            let n = outputs * inputs + outputs;
            let end = pos + 4 * n;
            let data = bytes.get(pos..end).ok_or(NetError::Truncated)?;
            let floats: Vec<f32> = data
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                .collect();
            pos = end;
            let (w, b) = floats.split_at(outputs * inputs);
            Ok(Layer {
                inputs,
                w: w.to_vec(),
                b: b.to_vec(),
            })
        };
        let net = Net {
            hidden,
            memory: state == STATE_DIM_MEM,
            t1: layer(state, hidden)?,
            t2: layer(hidden, hidden)?,
            ws: layer(hidden, hidden)?,
            wa: layer(action, hidden)?,
            h1: layer(hidden, hidden)?,
            h2: layer(hidden, 1)?,
            v1: layer(hidden, hidden / 2)?,
            v2: layer(hidden / 2, 1)?,
        };
        if pos != bytes.len() {
            return Err(NetError::WrongShape);
        }
        Ok(net)
    }

    /// Whether the net reads the opponent memory.
    pub fn reads_memory(&self) -> bool {
        self.memory
    }

    /// Logit of each action (one row of `ACTION_DIM` numbers per action) and the state's value.
    pub fn forward(&self, state: &[f32], actions: &[f32]) -> (Vec<f32>, f32) {
        let n = self.hidden;
        let mut a = vec![0.0; n];
        let mut h = vec![0.0; n];
        self.t1.apply_relu(state, &mut a);
        self.t2.apply_relu(&a, &mut h);
        let mut hs = vec![0.0; n];
        self.ws.apply(&h, &mut hs);

        let mut z = vec![0.0; n];
        let mut y = vec![0.0; n];
        let mut o = [0.0];
        let logits = actions
            .chunks(ACTION_DIM)
            .map(|row| {
                self.wa.apply(row, &mut z);
                z.iter_mut().zip(&hs).for_each(|(z, s)| *z = (*z + s).max(0.0));
                self.h1.apply_relu(&z, &mut y);
                self.h2.apply(&y, &mut o);
                o[0]
            })
            .collect();

        let mut v = vec![0.0; n / 2];
        self.v1.apply_relu(&h, &mut v);
        self.v2.apply(&v, &mut o);
        (logits, o[0])
    }
}

/// An opponent driven by a [`Net`].
///
/// A net that always plays its best action can freeze a table. When that action leaves the net's own
/// input as it was, the net plays it again forever, and a game only ends once somebody produces. So
/// a greedy net does not play the same action twice from the same input until it has played all the
/// others. Sampling needs no such rule.
#[derive(Clone, Debug)]
pub struct Neural {
    net: Arc<Net>,
    rng: Rng,
    temperature: f32,
    /// For each input a greedy net has met, a bit for every action already played from it.
    tried: HashMap<u64, u64>,
    /// Whether the net never sees a repayment on a turn it requested a loan (see [`is_same_turn_repay`]).
    no_same_turn_repay: bool,
    /// The least the net bids in the first round of a delivery auction (all its cash, if less).
    min_bid: u32,
    /// The dearest lot, in dollars, the net lays a container out in (0 for no limit).
    max_price: u32,
    /// Whether the net never sees a Bank call (starting or outbidding an auction).
    no_bank: bool,
    /// Whether sailing to a harbor must open a purchase that is then not declined.
    harbor_must_buy: bool,
    /// Whether the net sails to the Bank only with a container to load there (and room for it).
    bank_needs_load: bool,
    /// Whether the net never reprices the district it has just repriced, in the same turn.
    no_double_reprice: bool,
    /// District and actions left after the net's last main action, if that was a Reprice.
    last_reprice: Option<(District, u8)>,
    /// What the seats have done so far in this game, read by a net with memory.
    memory: Memory,
}

impl Neural {
    /// Plays the net's best action when `temperature` is 0, and otherwise samples from the scores
    /// divided by it.
    pub fn new(net: Arc<Net>, seed: u64, temperature: f32) -> Self {
        Self {
            net,
            rng: Rng::new(seed),
            temperature,
            tried: HashMap::new(),
            no_same_turn_repay: false,
            min_bid: 0,
            max_price: 0,
            no_bank: false,
            harbor_must_buy: false,
            bank_needs_load: false,
            no_double_reprice: false,
            last_reprice: None,
            memory: Memory::default(),
        }
    }

    /// Plays under the table rules a net trained with `--buy-at-harbor --bank-needs-load --no-double-reprice`
    /// knows: sailing to a player means buying from them, sailing to the Bank means loading there, and
    /// no repricing the same district twice in a row within a turn.
    pub fn with_table_rules(mut self) -> Self {
        self.harbor_must_buy = true;
        self.bank_needs_load = true;
        self.no_double_reprice = true;
        self
    }

    /// Takes repaying a loan off the table on a turn in which the net has requested one. A net
    /// trained with `--no-same-turn-repay` must play under this filter; one that was not would keep
    /// the loans it took, as it counts on repaying them.
    pub fn without_same_turn_repay(mut self) -> Self {
        self.no_same_turn_repay = true;
        self
    }

    /// Takes calling the Bank off the table, as `--classic` does in training. A net trained under it
    /// has never met the Bank, so it must play under the same rule. The engine still allows the call
    /// to the other seats.
    pub fn without_bank(mut self) -> Self {
        self.no_bank = true;
        self
    }

    /// Takes bids under `amount` off the table in the first round of a delivery auction, as
    /// `--min-bid` does in training. A net trained with it must play under it.
    pub fn with_min_bid(mut self, amount: u32) -> Self {
        self.min_bid = amount;
        self
    }

    /// Takes laying a container out in a lot dearer than `amount` off the table, as `--max-price` does in
    /// training, so that the net's containers are offered at a price someone may pay. A net trained with it
    /// must play under it.
    pub fn with_max_price(mut self, amount: u32) -> Self {
        self.max_price = amount;
        self
    }

    /// Whether the net may not play `action` here, under the rules it was trained with.
    fn forbidden(&self, obs: &Observation, action: Action) -> bool {
        if self.no_bank && matches!(action, Action::StartAuction { .. } | Action::Outbid(_)) {
            return true;
        }
        if self.no_same_turn_repay && is_same_turn_repay(obs.turn.loans_requested, action) {
            return true;
        }
        if let (PhaseView::Open(Phase::Main), Some(me)) = (obs.phase, obs.players.get(obs.viewer)) {
            match action {
                Action::Reprice(d) if self.no_double_reprice => {
                    if self.last_reprice == Some((d, obs.turn.actions_left)) {
                        return true;
                    }
                }
                Action::Sail(ShipLocation::Bank) if self.bank_needs_load => {
                    if me.holding.is_empty() || SHIP_CAPACITY <= me.ship.total() {
                        return true;
                    }
                }
                Action::Sail(ShipLocation::Harbor(owner)) if self.harbor_must_buy => {
                    let cash = me.cash.unwrap_or(0);
                    let can_buy = owner != obs.viewer
                        && owner < obs.num_players
                        && me.ship.total() < SHIP_CAPACITY
                        && obs.players[owner]
                            .harbor_lots
                            .iter()
                            .enumerate()
                            .any(|(lot, c)| !c.is_empty() && District::Harbor.lot_price(lot) <= cash);
                    if !can_buy {
                        return true;
                    }
                }
                _ => {}
            }
        }
        if let (PhaseView::Open(Phase::Buy(b)), Action::DoneBuying) = (obs.phase, action) {
            if self.harbor_must_buy && b.optional && b.district == District::Harbor && b.bought.is_empty() {
                return true;
            }
        }
        if let (PhaseView::Open(Phase::Arrange(a)), Action::Place(lot)) = (obs.phase, action) {
            if self.max_price > 0 && a.district.lot_price(lot) > self.max_price {
                return true;
            }
        }
        if let (PhaseView::Bid(b), Action::Bid(amount)) = (obs.phase, action) {
            let cash = obs.players[obs.viewer].cash.unwrap_or(u32::MAX);
            return !b.runoff && amount < self.min_bid.min(cash);
        }
        false
    }

    /// The best-scoring action not yet played from this input. Once all of them have been played,
    /// the input starts afresh.
    fn best_untried(&mut self, logits: &[f32], input: u64) -> usize {
        let tried = self.tried.entry(input).or_insert(0);
        let best = |skip: u64| {
            logits
                .iter()
                .enumerate()
                .filter(|&(k, _)| skip >> k & 1 == 0)
                .max_by(|a, b| a.1.total_cmp(b.1))
                .map(|(k, _)| k)
        };
        let pick = best(*tried).unwrap_or_else(|| {
            *tried = 0;
            best(0).unwrap_or(0)
        });
        *tried |= 1 << pick;
        pick
    }
}

/// A hash of everything the net sees. Equal inputs get equal choices, so this tells a situation the
/// net has been in from a new one.
fn fingerprint(state: &[f32], acts: &[f32]) -> u64 {
    state.iter().chain(acts).fold(0xcbf2_9ce4_8422_2325, |hash, x| {
        x.to_bits()
            .to_le_bytes()
            .iter()
            .fold(hash, |hash, &byte| (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3))
    })
}

impl Opponent for Neural {
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action {
        let barred = |&a: &Action| self.forbidden(obs, a);
        let allowed: Vec<Action>;
        let legal = if legal.iter().any(barred) {
            allowed = legal.iter().copied().filter(|a| !barred(a)).collect();
            // Never leave the seat without a move.
            if allowed.is_empty() { legal } else { &allowed }
        } else {
            legal
        };
        let kept = thin(legal.len());
        let mut state = vec![0.0; if self.net.memory { STATE_DIM_MEM } else { STATE_DIM }];
        if self.net.memory {
            encode_state_mem(obs, &self.memory, &mut state);
        } else {
            encode_state(obs, &mut state);
        }
        let mut acts = vec![0.0; kept.len() * ACTION_DIM];
        for (k, &i) in kept.iter().enumerate() {
            encode_action(obs, legal[i], &mut acts[k * ACTION_DIM..(k + 1) * ACTION_DIM]);
        }
        let (logits, _) = self.net.forward(&state, &acts);
        let pick = if self.temperature <= 0.0 {
            self.best_untried(&logits, fingerprint(&state, &acts))
        } else {
            let top = logits.iter().cloned().fold(f32::MIN, f32::max);
            let w: Vec<f64> = logits
                .iter()
                .map(|l| (((l - top) / self.temperature) as f64).exp())
                .collect();
            let mut x = (self.rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64 * w.iter().sum::<f64>();
            let mut k = 0;
            while k + 1 < w.len() && x >= w[k] {
                x -= w[k];
                k += 1;
            }
            k
        };
        let action = legal[kept[pick]];
        if obs.phase == PhaseView::Open(Phase::Main) {
            self.last_reprice = match action {
                Action::Reprice(d) => Some((d, obs.turn.actions_left.saturating_sub(1))),
                _ => None,
            };
        }
        action
    }

    fn observe(&mut self, _mover: usize, obs: &Observation, action: Action) {
        if let (true, PhaseView::Open(phase)) = (self.net.memory, &obs.phase) {
            self.memory.record(_mover, phase, action);
        }
    }
}

#[cfg(test)]
mod tests {
    use container_engine::config::GameConfig;
    use container_engine::game::Game;

    use super::*;

    const WEIGHTS: &[u8] = include_bytes!("../tests/fixtures/parity.bin");
    const REFERENCE: &str = include_str!("../tests/fixtures/parity.txt");

    fn floats(line: &str) -> Vec<f32> {
        line.split_whitespace().map(|x| x.parse().unwrap()).collect()
    }

    /// The opening position of a three-player game, and the actions open in it.
    fn opening() -> (Observation, Vec<Action>) {
        let game = Game::new(GameConfig::new(3).with_seed(1)).unwrap();
        let obs = game.observation(game.current_player()).unwrap();
        let legal = game.legal_actions();
        assert!(legal.len() <= MAX_ACTIONS && legal.len() > 2);
        (obs, legal)
    }

    #[test]
    fn a_greedy_net_plays_its_best_action_at_first() {
        let net = Arc::new(Net::from_bytes(WEIGHTS).unwrap());
        let (obs, legal) = opening();
        let mut state = vec![0.0; STATE_DIM];
        encode_state(&obs, &mut state);
        let acts: Vec<f32> = legal
            .iter()
            .flat_map(|&a| {
                let mut row = vec![0.0; ACTION_DIM];
                encode_action(&obs, a, &mut row);
                row
            })
            .collect();
        let (logits, _) = net.forward(&state, &acts);
        let best = (0..legal.len()).max_by(|&a, &b| logits[a].total_cmp(&logits[b])).unwrap();
        assert_eq!(Neural::new(net, 0, 0.0).choose(&obs, &legal), legal[best]);
    }

    #[test]
    fn a_greedy_net_tries_the_other_actions_before_repeating_one() {
        let net = Arc::new(Net::from_bytes(WEIGHTS).unwrap());
        let (obs, legal) = opening();
        let mut bot = Neural::new(net, 0, 0.0);
        let first = bot.choose(&obs, &legal);
        let mut played = vec![first];
        for _ in 1..legal.len() {
            let next = bot.choose(&obs, &legal);
            assert!(!played.contains(&next), "{next:?} was played twice");
            played.push(next);
        }
        assert_eq!(bot.choose(&obs, &legal), first, "every action has had its turn");
    }

    #[test]
    fn a_net_without_same_turn_repay_never_repays_a_loan_it_requested_that_turn() {
        fn picks(mut bot: Neural, obs: &Observation, legal: &[Action]) -> Vec<Action> {
            (0..legal.len()).map(|_| bot.choose(obs, legal)).collect()
        }
        let net = Arc::new(Net::from_bytes(WEIGHTS).unwrap());
        let (mut obs, mut legal) = opening();
        legal.push(Action::RepayLoan);
        obs.turn.loans_requested = 1;
        let plain = picks(Neural::new(net.clone(), 0, 0.0), &obs, &legal);
        assert!(plain.contains(&Action::RepayLoan), "the guard plays every action in turn");
        let filtered = picks(Neural::new(net.clone(), 0, 0.0).without_same_turn_repay(), &obs, &legal);
        assert!(!filtered.contains(&Action::RepayLoan));
        obs.turn.loans_requested = 0;
        let allowed = picks(Neural::new(net, 0, 0.0).without_same_turn_repay(), &obs, &legal);
        assert!(allowed.contains(&Action::RepayLoan), "nothing was requested, so repaying is fine");
    }

    #[test]
    fn the_table_rules_bar_exactly_the_sails_that_open_nothing() {
        use crate::solitaire::Solitaire;
        let net = Arc::new(Net::from_bytes(include_bytes!("../weights/peddler.bin")).unwrap());
        let checker = Neural::new(net.clone(), 0, 0.0).with_table_rules();
        let (mut opens, mut idle) = (0, 0);
        for seed in 0..4 {
            let mut game = Game::new(GameConfig::new(4).with_seed(seed)).unwrap();
            let mut players: Vec<Box<dyn Opponent>> = (0..4)
                .map(|s| -> Box<dyn Opponent> {
                    if s < 2 {
                        Box::new(Neural::new(net.clone(), s, 0.0).with_table_rules())
                    } else {
                        Box::new(Solitaire::new(s))
                    }
                })
                .collect();
            for _ in 0..2_500 {
                if game.is_terminal() {
                    break;
                }
                let viewer = game.current_player();
                let obs = game.observation(viewer).unwrap();
                let legal = game.legal_actions();
                if obs.phase == PhaseView::Open(Phase::Main) {
                    for &action in &legal {
                        if !matches!(action, Action::Sail(ShipLocation::Bank | ShipLocation::Harbor(_))) {
                            continue;
                        }
                        let mut after = game.clone();
                        after.apply(action).unwrap();
                        let does_something = matches!(after.phase, Phase::Buy(_) | Phase::Load(_));
                        assert_eq!(checker.forbidden(&obs, action), !does_something, "{action:?}");
                        if does_something { opens += 1 } else { idle += 1 }
                    }
                }
                let action = players[viewer].choose(&obs, &legal);
                game.apply(action).unwrap();
            }
        }
        assert!(opens > 0 && idle > 0, "{opens} {idle}");
    }

    #[test]
    fn a_new_position_has_no_history() {
        let net = Arc::new(Net::from_bytes(WEIGHTS).unwrap());
        let (obs, legal) = opening();
        let game = Game::new(GameConfig::new(4).with_seed(2)).unwrap();
        let other = game.observation(game.current_player()).unwrap();
        let other_legal = game.legal_actions();
        let mut bot = Neural::new(net.clone(), 0, 0.0);
        for _ in 0..3 {
            bot.choose(&obs, &legal);
        }
        assert_eq!(
            bot.choose(&other, &other_legal),
            Neural::new(net, 0, 0.0).choose(&other, &other_legal)
        );
    }

    #[test]
    fn the_net_matches_the_numpy_reference() {
        let net = Net::from_bytes(WEIGHTS).unwrap();
        let mut lines = REFERENCE.lines();
        let n: usize = lines.next().unwrap().parse().unwrap();
        let state = floats(lines.next().unwrap());
        let acts: Vec<f32> = (0..n).flat_map(|_| floats(lines.next().unwrap())).collect();
        let want_logits = floats(lines.next().unwrap());
        let want_value = floats(lines.next().unwrap())[0];
        let (logits, value) = net.forward(&state, &acts);
        assert_eq!(logits.len(), n);
        for (got, want) in logits.iter().zip(&want_logits) {
            assert!((got - want).abs() < 1e-4, "{got} vs {want}");
        }
        assert!((value - want_value).abs() < 1e-4);
    }

    #[test]
    fn bad_files_are_refused() {
        assert_eq!(Net::from_bytes(b"nope").unwrap_err(), NetError::BadMagic);
        assert_eq!(
            Net::from_bytes(&WEIGHTS[..WEIGHTS.len() - 4]).unwrap_err(),
            NetError::Truncated
        );
    }
}
