//! The opponent that has no idea what it is doing.

use container_engine::action::Action;
use container_engine::observation::Observation;
use container_engine::rng::Rng;

use crate::Opponent;

pub const ID: &str = "random";

/// Picks uniformly among the legal actions.
#[derive(Clone, Debug)]
pub struct Random {
    rng: Rng,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rng::new(seed),
        }
    }
}

impl Opponent for Random {
    fn choose(&mut self, _: &Observation, legal: &[Action]) -> Action {
        legal[self.rng.below(legal.len() as u32) as usize]
    }
}
