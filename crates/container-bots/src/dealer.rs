//! Dealer, Broker trained further to deal with its opponents, not only to win.
//!
//! The weights are `runs/inter_B/final.pt` exported by `training/model.py::export_weights`: Broker's net,
//! trained 45 minutes more in league self-play with `--interaction-weight 0.5`, a reward that is half
//! the usual one and half a score for buying from opponents, bidding on their deals, selling to them and
//! pricing containers where a buyer cannot lose. It keeps Broker's two rules: it never repays a loan in the
//! turn it took it, and it opens every delivery auction with a bid of at least $3 (all its cash, if less).

use crate::neural::Neural;
use crate::nets;

pub const ID: &str = "dealer";

/// The net in [`nets`].
pub const NET: &str = "dealer";

/// The least Dealer bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

pub fn new(seed: u64) -> Neural {
    Neural::new(nets::get(NET), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID)
}
