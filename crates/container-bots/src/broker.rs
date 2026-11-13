//! Broker, a net trained from scratch to win while dealing with its opponents.
//!
//! The weights are `runs/scratch_constrained/final.pt` exported by `training/model.py::export_weights`
//! (`--no-same-turn-repay --min-bid 3 --min-trade 45 --scripted-share 0.7`, 30 minutes). It was trained
//! under two rules that the deployed net keeps: it never repays a loan in the turn it took it, and it
//! opens every delivery auction with a bid of at least $3 (all its cash, if less). It plays its best
//! action with the repetition guard of [`Neural`].

use std::sync::{Arc, OnceLock};

use crate::neural::{Net, Neural};

pub const ID: &str = "broker";

/// The least Broker bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

fn net() -> Arc<Net> {
    static NET: OnceLock<Arc<Net>> = OnceLock::new();
    NET.get_or_init(|| {
        Arc::new(
            Net::from_bytes(include_bytes!("../weights/broker.bin")).expect("the built-in weights are valid"),
        )
    })
    .clone()
}

pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID)
}
