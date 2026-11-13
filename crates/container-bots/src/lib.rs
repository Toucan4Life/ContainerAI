//! Computer opponents for Container.
//!
//! An opponent sees what its seat sees and the actions it may take, and picks one. Every opponent
//! the UI offers is listed in [`OPPONENTS`] and built with [`create`].

use container_engine::action::Action;
use container_engine::observation::Observation;

pub mod broker;
pub mod classic;
pub mod consortium;
pub mod dealer;
pub mod features;
pub mod heuristic;
pub mod magnate;
pub mod merchant;
pub mod neural;
pub mod random;

pub use heuristic::Heuristic;
pub use merchant::Merchant;
pub use neural::{Net, Neural};
pub use random::Random;

/// A player that is not a human.
pub trait Opponent {
    /// Picks one of `legal`, the actions the game accepts from `obs.viewer` now. `legal` is never
    /// empty while the game is going.
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action;
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
pub const OPPONENTS: [OpponentInfo; 8] = [
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
        id: classic::ID,
        name: "Classic",
        description: "A net trained from scratch for the classic rule, in which nobody calls the Bank: it never starts or outbids a Bank auction, so it trades with its opponents instead. Never repays a loan the turn it takes it. Its rating is from its training evaluation, played against opponents that could not call the Bank either, so it is only roughly comparable.",
        elo: 2230,
    },
    OpponentInfo {
        id: magnate::ID,
        name: "Magnate",
        description: "A neural net trained by imitating Merchant, then by self-play against earlier versions of itself. It plays the action it rates best and never repeats itself in an identical position.",
        elo: 2280,
    },
    OpponentInfo {
        id: consortium::ID,
        name: "Consortium",
        description: "The last net of the population study, a best response trained against the strongest nets before it. It plays the action it rates best and never repeats itself in an identical position. About 100 ELO above Magnate when they share a table.",
        elo: 2378,
    },
];

/// Builds the opponent with this identifier. A seed gives the same choices every time.
pub fn create(id: &str, seed: u64) -> Option<Box<dyn Opponent + Send>> {
    match id {
        random::ID => Some(Box::new(Random::new(seed))),
        heuristic::ID => Some(Box::new(Heuristic::new(seed))),
        merchant::ID => Some(Box::new(Merchant::new(seed))),
        magnate::ID => Some(Box::new(magnate::new(seed))),
        consortium::ID => Some(Box::new(consortium::new(seed))),
        broker::ID => Some(Box::new(broker::new(seed))),
        dealer::ID => Some(Box::new(dealer::new(seed))),
        classic::ID => Some(Box::new(classic::new(seed))),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
