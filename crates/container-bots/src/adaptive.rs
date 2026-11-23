//! Adaptive: one net trained from scratch (seed 2 of the adaptive-generalist runs; the first 17% of its
//! budget copied Merchant) to play well against Producer, Showcase and Shipper, the three scripted
//! heuristics, which filled 70% of the opponent seats in training (the rest were earlier copies of itself).
//! It was never trained against Peddler, Magnate or Consortium.
//!
//! It plays under Peddler v2's five rules, which bound its seat in training and bind it here: no same-turn
//! loan repay, a first bid of at least $3, and the three table rules of `Neural::with_table_rules`.

use std::sync::{Arc, OnceLock};

use crate::neural::{Net, Neural};

pub const ID: &str = "adaptive";

/// The least Adaptive bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

fn net() -> Arc<Net> {
    static NET: OnceLock<Arc<Net>> = OnceLock::new();
    NET.get_or_init(|| {
        Arc::new(
            Net::from_bytes(include_bytes!("../weights/adaptive.bin")).expect("the built-in weights are valid"),
        )
    })
    .clone()
}

pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID).with_table_rules()
}
