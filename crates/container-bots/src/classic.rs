//! Classic, a net trained from scratch under the classic rule: nobody calls the Bank.
//!
//! The weights are `runs/classic_net/final.pt` exported by `training/model.py::export_weights`
//! (`--classic --no-same-turn-repay --scripted-share 0.8 --trader-share 0.7`, 30 minutes). It has never
//! met a Bank auction, so the deployed net may not start or outbid one, and it never repays a loan the
//! turn it takes it. It plays its best action with the repetition guard of [`Neural`]: without it about
//! a quarter of its games froze in the trainer's final eval.

use crate::neural::Neural;
use crate::nets;

pub const ID: &str = "classic";

/// The net in [`nets`].
pub const NET: &str = "classic";

pub fn new(seed: u64) -> Neural {
    Neural::new(nets::get(NET), seed, 0.0).without_bank().without_same_turn_repay()
}
