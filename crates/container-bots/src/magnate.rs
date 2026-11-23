//! Magnate, the strongest trained net, built into the crate.
//!
//! The weights are `runs/defense_run2_vs_exploiter_30m/final.pt` exported by
//! `training/model.py::export_weights`. To ship a stronger net, replace `weights/magnate.bin` and
//! update its rating in `OPPONENTS`.
//!
//! Magnate plays its best action, except that it never plays the same action twice from an
//! identical input until it has tried the others (see [`Neural`]). Without that it froze about one
//! game in thirty at a mixed table, and many more at tables with Random.

use std::sync::Arc;

use crate::neural::{Net, Neural};
use crate::nets;

pub const ID: &str = "magnate";

/// The net in [`nets`].
pub const NET: &str = "magnate";

/// The net, shared by every Magnate.
pub fn net() -> Arc<Net> {
    nets::get(NET)
}

/// A Magnate. The seed makes no difference: it never samples.
pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0)
}
