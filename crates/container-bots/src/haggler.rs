//! Haggler, a net trained from scratch (no imitation of another bot, no warm start) to deal with its
//! opponents. Reward: 0.8 interaction (buying from opponents, bids on their deals, selling to them,
//! fair prices) plus 0.2 win share alone, no score margin; 35% of the training seats were cheap-seller
//! archetypes. It keeps Dealer's two rules: no same-turn loan repay, and a first bid of at least $3.

use std::sync::{Arc, OnceLock};

use crate::neural::{Net, Neural};

pub const ID: &str = "haggler";

/// The least Haggler bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

fn net() -> Arc<Net> {
    static NET: OnceLock<Arc<Net>> = OnceLock::new();
    NET.get_or_init(|| {
        Arc::new(
            Net::from_bytes(include_bytes!("../weights/haggler.bin")).expect("the built-in weights are valid"),
        )
    })
    .clone()
}

pub fn new(seed: u64) -> Neural {
    Neural::new(net(), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID)
}
