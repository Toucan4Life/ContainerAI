//! Computer opponents for Container.
//!
//! An opponent sees what its seat sees and the actions it may take, and picks one. Every opponent
//! the UI offers is listed in [`OPPONENTS`] and built with [`create`].

use container_engine::action::Action;
use container_engine::observation::Observation;

pub mod heuristic;
pub mod random;

pub use heuristic::Heuristic;
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
}

/// Every opponent that can be selected.
pub const OPPONENTS: [OpponentInfo; 2] = [
    OpponentInfo {
        id: random::ID,
        name: "Random",
        description: "Plays any legal action, chosen at random.",
    },
    OpponentInfo {
        id: heuristic::ID,
        name: "Simple",
        description: "Produces, delivers and buys by a short list of fixed priorities.",
    },
];

/// Builds the opponent with this identifier. A seed gives the same choices every time.
pub fn create(id: &str, seed: u64) -> Option<Box<dyn Opponent + Send>> {
    match id {
        random::ID => Some(Box::new(Random::new(seed))),
        heuristic::ID => Some(Box::new(Heuristic::new(seed))),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
