//! Computer opponents for Container.
//!
//! An opponent sees what its seat sees and the actions it may take, and picks one. Every opponent
//! the UI offers is listed in [`OPPONENTS`] and built with [`create`].

use container_engine::action::Action;
use container_engine::observation::Observation;

pub mod adaptive;
pub mod archetype;
pub mod broker;
pub mod classic;
pub mod consortium;
pub mod dealer;
pub mod features;
pub mod haggler;
pub mod heuristic;
pub mod magnate;
pub mod memory;
pub mod merchant;
pub mod neural;
pub mod peddler;
pub mod peddler2;
pub mod random;
pub mod solitaire;

pub use heuristic::Heuristic;
pub use merchant::Merchant;
pub use neural::{Net, Neural};
pub use random::Random;

/// A player that is not a human.
pub trait Opponent {
    /// Picks one of `legal`, the actions the game accepts from `obs.viewer` now. `legal` is never
    /// empty while the game is going.
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action;

    /// Tells the opponent about an action any seat has just chosen, `obs` being that seat's view
    /// before it. Called for every action of the game, the opponent's own included. Opponents
    /// without memory ignore it.
    fn observe(&mut self, _mover: usize, _obs: &Observation, _action: Action) {}
}

/// Whether `action` repays a loan on a turn in which its player has already requested one. The two
/// cancel out, so a net that does both has only spent its free actions. Training can forbid it for
/// a seat (`container-env`) and a deployed [`Neural`] can filter it, from the same rule.
pub fn is_same_turn_repay(loans_requested: u8, action: Action) -> bool {
    loans_requested > 0 && action == Action::RepayLoan
}

/// How the UI lists an opponent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpponentInfo {
    /// Stable identifier, passed to [`create`].
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// Rating on the ladder of `training/ladder.json`, anchored at Random = 0. Magnate's is its
    /// `anchors rate` result, as it is not an anchor itself.
    pub elo: u32,
}

/// Every opponent that can be selected.
pub const OPPONENTS: [OpponentInfo; 16] = [
    OpponentInfo {
        id: random::ID,
        name: "Random",
        description: "Plays any legal action, chosen at random.",
        elo: 0,
    },
    OpponentInfo {
        id: heuristic::ID,
        name: "Simple",
        description: "Produces, delivers and buys by a short list of fixed priorities.",
        elo: 263,
    },
    OpponentInfo {
        id: merchant::ID,
        name: "Merchant",
        description: "Simple's priorities, tuned in self-play: it also builds, borrows and bids by cargo.",
        elo: 932,
    },
    OpponentInfo {
        id: solitaire::ID,
        name: "Solitaire",
        description: "The bot of the solitaire variant: a fixed order of sail, produce, buy from a factory, buy from a harbor, reprice and build, with a d20 for every price and every purchase. When it has nothing else to do it rolls a d20 and calls the Bank on 11+ for the fullest container lot, and it outbids an opponent's cash bid on 16+. Rated 1200 on the Random, Simple and Merchant ladder; against the nets it is far weaker (about 270 ELO above Random, where Magnate is about 870).",
        elo: 1200,
    },
    OpponentInfo {
        id: archetype::SHIPPER,
        name: "Shipper",
        description: "Lives off deliveries: it buys the cheapest cargo it can find, sails whenever that pays and bids hard for the cargo of others, usually with one or two containers aboard. It builds factories and warehouses, borrows, and lists its harbor stock at $2. A hand-set script, not a net: it never starts or outbids a Bank auction, and it can go broke and then sit idle for the rest of the game. Rated 1860 on the ladder of the nets and 1470 on the Random, Simple and Merchant ladder; against the ladder's strongest nets it outscores one of them in 11 to 18% of the comparisons (an even share is 50%).",
        elo: 1860,
    },
    OpponentInfo {
        id: peddler::ID,
        name: "Peddler",
        description: "Haggler's recipe (trained from scratch, 80% dealing with opponents and 20% winning), trained under three table rules that it keeps: it sails to an opponent's harbor only to buy there, and then buys; it sails to the Bank only with a container to pick up; and it never reprices the same district twice in a row in a turn. It lists about two containers in three at $3 or less, buys from opponents about as often as Magnate and Consortium, and delivers more often than the other nets (11 a game against 8). It beats Merchant every time and Solitaire at three and four players, but is barely ahead of four Solitaires at five, and with two Solitaires at three players it sometimes cannot end the game (4% of such tables). It loses to Magnate, Dealer and Consortium. Also keeps Dealer's two rules. The rules bind this bot only, not its opponents, so its rating, from its training evaluation, is only roughly comparable.",
        elo: 1900,
    },
    OpponentInfo {
        id: archetype::SHOWCASE,
        name: "Showcase",
        description: "Keeps a harbor stocked with many colors and resells what it buys from factories. It builds mostly warehouses, lists its stock at $2 in the factory and $3 in the harbor, and takes no loans. A hand-set script, not a net: it never starts or outbids a Bank auction. Rated 1911 on the ladder of the nets and 1451 on the Random, Simple and Merchant ladder; against the ladder's strongest nets it outscores one of them in 17 to 23% of the comparisons (an even share is 50%).",
        elo: 1911,
    },
    OpponentInfo {
        id: haggler::ID,
        name: "Haggler",
        description: "A net trained from scratch to trade rather than to beat the strong nets: its reward was 80% dealing with opponents (buying from them, bidding on their deliveries, selling to them, pricing at $3 or less) and 20% winning, with no score margin. It lists about three containers in four at $3 or less. It beats Merchant and the solitaire variants easily and loses to Magnate, Dealer and Consortium. Same two rules as Dealer.",
        elo: 2000,
    },
    OpponentInfo {
        id: archetype::PRODUCER,
        name: "Producer",
        description: "Produces more than anyone else and sells it cheap, at $1 from its factories. It builds factories and warehouses, borrows, and ships only part of its stock. A hand-set script, not a net: it never starts or outbids a Bank auction, and it can go broke and then sit idle for the rest of the game. Rated 2033 on the ladder of the nets and 1509 on the Random, Simple and Merchant ladder; against the ladder's strongest nets it outscores one of them in 26 to 38% of the comparisons (an even share is 50%).",
        elo: 2033,
    },
    OpponentInfo {
        id: broker::ID,
        name: "Broker",
        description: "A net trained from scratch to win and to deal with its opponents. It bids at least $3 on every delivery, buys from opponents, never repays a loan the turn it takes it, and plays the action it rates best without repeating itself in an identical position. Weaker than Magnate.",
        elo: 2150,
    },
    OpponentInfo {
        id: dealer::ID,
        name: "Dealer",
        description: "Broker trained further to deal, not only to win. It buys from opponents' factories and harbors, bids on their deals, and lists about four containers in ten at $3 or less, a price at which a buyer cannot lose, where Broker lists one in twenty. Same rules and about the same strength as Broker.",
        elo: 2170,
    },
    OpponentInfo {
        id: peddler2::ID,
        name: "Peddler v2",
        description: "Peddler trained 20 more minutes in a league that seats Solitaire, Merchant and a frozen copy of Peddler, with the same reward (80% dealing with opponents, 20% winning) and the same three table rules, which it keeps: it sails to an opponent's harbor only to buy there, and then buys; it sails to the Bank only with a container to pick up; and it never reprices the same district twice in a row in a turn. It lists four containers in five at $3 or less, buys from opponents 15 times a game (Magnate and Consortium 16), sells to them 17 times (they sell 13 to 14) and delivers 11 times a game (they deliver 7 to 8). It beats Merchant in 99.8% of the games and Solitaire in at least 96% at every table size. Against the strong nets an even share is 26%: it wins 52% of the games against Magnate, 37% against Consortium and 27% against Dealer (Peddler wins 6% against the three together). With two Solitaires at three players it sometimes cannot end the game (2% of such tables). Also keeps Dealer's two rules. The rules bind this bot only, not its opponents, so its rating, from its training evaluation, is only roughly comparable.",
        elo: 2225,
    },
    OpponentInfo {
        id: classic::ID,
        name: "Classic",
        description: "A net trained from scratch for the classic rule, in which nobody calls the Bank: it never starts or outbids a Bank auction, so it trades with its opponents instead. Never repays a loan the turn it takes it. Its rating is from its training evaluation, played against opponents that could not call the Bank either, so it is only roughly comparable.",
        elo: 2230,
    },
    OpponentInfo {
        id: adaptive::ID,
        name: "Adaptive",
        description: "A net trained from scratch for two hours to play well against three scripted players (a producer, a showcase seller and a shipper) rather than to imitate a style; it changes how it buys, sells and delivers with the table. It builds no factory or warehouse, produces little, buys at harbors and ships fuller cargoes than the other nets (three containers a delivery on average). It beats the three scripted players most of the time, but loses clearly to Peddler v2, which it never met in training. Plays under Peddler v2's rules (no same-turn loan repay, a first bid of at least $3, and the three table rules); the rules bind this bot only, so its rating, from its training evaluation, is only roughly comparable.",
        elo: 2241,
    },
    OpponentInfo {
        id: magnate::ID,
        name: "Magnate",
        description: "A neural net trained by imitating Merchant, then by self-play against earlier versions of itself. It plays the action it rates best and never repeats itself in an identical position.",
        elo: 2280,
    },
    OpponentInfo {
        id: consortium::ID,
        name: "Banker",
        description: "The last net of the population study (Consortium in the reports and on the ladder), a best response trained against the strongest nets before it. It plays the Bank: it starts about ten Bank auctions a game, for container lots and cash lots, and it borrows. It builds no factory or warehouse and lists its own containers in the top lots, at the highest prices, but it still buys from opponents (about 18 containers a game at a table of scripted players). It plays the action it rates best and never repeats itself in an identical position. About 100 ELO above Magnate when they share a table.",
        elo: 2378,
    },
];

/// Scripted economic archetypes for evaluation. They are not offered in the UI: they exist to test
/// whether a net notices and uses the offers other kinds of players make.
pub const ARCHETYPES: [&str; 3] = ["discounter", "bargain", "trader"];

/// The three hand-set heuristics of the heuristic-first program (see [`archetype`]). They are listed
/// in [`OPPONENTS`] like any other bot.
pub const HEURISTICS: [&str; 3] = [archetype::PRODUCER, archetype::SHOWCASE, archetype::SHIPPER];

/// Builds the opponent with this identifier. A seed gives the same choices every time.
pub fn create(id: &str, seed: u64) -> Option<Box<dyn Opponent + Send>> {
    if let Some(path) = id.strip_prefix("net:") {
        return candidate(path, seed, false);
    }
    // A net trained under the three table rules (see `Neural::with_table_rules`).
    if let Some(path) = id.strip_prefix("netr:") {
        return candidate(path, seed, true);
    }
    match id {
        random::ID => Some(Box::new(Random::new(seed))),
        heuristic::ID => Some(Box::new(Heuristic::new(seed))),
        merchant::ID => Some(Box::new(Merchant::new(seed))),
        solitaire::ID => Some(Box::new(solitaire::Solitaire::new(seed))),
        magnate::ID => Some(Box::new(magnate::new(seed))),
        consortium::ID => Some(Box::new(consortium::new(seed))),
        broker::ID => Some(Box::new(broker::new(seed))),
        dealer::ID => Some(Box::new(dealer::new(seed))),
        haggler::ID => Some(Box::new(haggler::new(seed))),
        peddler::ID => Some(Box::new(peddler::new(seed))),
        peddler2::ID => Some(Box::new(peddler2::new(seed))),
        adaptive::ID => Some(Box::new(adaptive::new(seed))),
        classic::ID => Some(Box::new(classic::new(seed))),
        archetype::PRODUCER => Some(Box::new(archetype::Archetype::producer(seed))),
        archetype::SHOWCASE => Some(Box::new(archetype::Archetype::showcase(seed))),
        archetype::SHIPPER => Some(Box::new(archetype::Archetype::shipper(seed))),
        // Prices everything at $1 (factory) and buys at any price.
        "discounter" => Some(Box::new(Merchant::seller(seed, 4, 0))),
        // Prices around $2 and will not pay more than $3.
        "bargain" => Some(Box::new(Merchant::seller(seed, 3, 1))),
        "trader" => Some(Box::new(Merchant::trader(seed))),
        _ => None,
    }
}

/// An unreleased candidate net (`net:PATH.bin`), played under Dealer's rules: at least $3 to open a
/// delivery auction and no repayment in the turn a loan was taken. `netr:PATH.bin` adds the three table
/// rules (see `Neural::with_table_rules`). For evaluation only; the file is read once per process and
/// shared by every seat.
fn candidate(path: &str, seed: u64, table_rules: bool) -> Option<Box<dyn Opponent + Send>> {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock};
    static NETS: OnceLock<Mutex<HashMap<String, Arc<Net>>>> = OnceLock::new();
    let mut nets = NETS.get_or_init(Default::default).lock().ok()?;
    if !nets.contains_key(path) {
        let net = Net::from_bytes(&std::fs::read(path).ok()?).ok()?;
        nets.insert(path.to_string(), Arc::new(net));
    }
    let net = nets[path].clone();
    let bot = Neural::new(net, seed, 0.0).without_same_turn_repay().with_min_bid(3);
    Some(Box::new(if table_rules { bot.with_table_rules() } else { bot }))
}

#[cfg(test)]
mod tests;
