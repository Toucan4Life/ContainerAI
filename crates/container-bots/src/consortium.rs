//! Consortium, the strongest net the population study ends on (`docs/equilibrium_spec.md`, section 9.4),
//! built into the crate.
//!
//! It is psro08, the best response trained against the refined equilibrium of the nine-strategy game,
//! which the ten-strategy game keeps in the support at every table size. It plays the action it rates
//! best with the repetition guard, the `guarded` mode the study measured it in. The weights are
//! `populations/psro/psro08.pt`, exported by `training/model.py::export_weights`.

use std::sync::{Arc, OnceLock};

use crate::neural::{Net, Neural};

pub const ID: &str = "consortium";

/// The built-in net, parsed on first use and shared by every seat.
fn net() -> Arc<Net> {
    static NET: OnceLock<Arc<Net>> = OnceLock::new();
    NET.get_or_init(|| {
        Arc::new(
            Net::from_bytes(include_bytes!("../weights/psro08.bin"))
                .expect("the built-in weights are valid"),
        )
    })
    .clone()
}

pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0)
}
