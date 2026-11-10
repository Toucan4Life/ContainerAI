//! Magnate, the strongest trained net, built into the crate.
//!
//! The weights are `runs/defense_run2_vs_exploiter_30m/final.pt` exported by
//! `training/model.py::export_weights`. To ship a stronger net, replace `weights/magnate.bin` and
//! update its rating in `OPPONENTS`.
//!
//! Magnate plays its best action, except that it never plays the same action twice from an
//! identical input until it has tried the others (see [`Neural`]). Without that it froze about one
//! game in thirty at a mixed table, and many more at tables with Random.

use std::sync::{Arc, OnceLock};

use crate::neural::{Net, Neural};

pub const ID: &str = "magnate";

const WEIGHTS: &[u8] = include_bytes!("../weights/magnate.bin");

/// The built-in net, parsed on first use and shared by every Magnate.
pub fn net() -> Arc<Net> {
    static NET: OnceLock<Arc<Net>> = OnceLock::new();
    NET.get_or_init(|| Arc::new(Net::from_bytes(WEIGHTS).expect("the built-in weights are valid")))
        .clone()
}

/// A Magnate. The seed makes no difference: it never samples.
pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0)
}
