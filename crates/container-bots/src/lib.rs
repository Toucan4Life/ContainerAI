//! Computer opponents for Container.
//!
//! An opponent sees what its seat sees and the actions it may take, and picks one. Every opponent
//! the UI offers is listed in [`OPPONENTS`] and built with [`create`].

use container_engine::action::Action;
use container_engine::observation::Observation;

pub mod consortium;
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
pub const OPPONENTS: [OpponentInfo; 5] = [
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
        _ => None,
    }
}

#[cfg(test)]
mod tests;
