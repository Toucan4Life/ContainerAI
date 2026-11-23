//! Consortium, the strongest net the population study ends on (`docs/equilibrium_spec.md`, section 9.4),
//! shown in the UI as Banker.
//!
//! It is psro08, the best response trained against the refined equilibrium of the nine-strategy game,
//! which the ten-strategy game keeps in the support at every table size. It plays the action it rates
//! best with the repetition guard, the `guarded` mode the study measured it in. The weights are
//! `populations/psro/psro08.pt`, exported by `training/model.py::export_weights`.

use crate::neural::Neural;
use crate::nets;

pub const ID: &str = "consortium";

/// The net in [`nets`].
pub const NET: &str = "psro08";

pub fn new(seed: u64) -> Neural {
    Neural::new(nets::get(NET), seed, 0.0)
}
