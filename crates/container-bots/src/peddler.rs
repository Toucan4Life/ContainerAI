//! Peddler, Haggler's recipe played under three table rules: a net trained from scratch (no warm start from
//! another net; the first 17% of its budget copied Merchant) to deal with its opponents. Reward: 0.8
//! interaction (buying from opponents, bids on their deals, selling to them, fair prices) plus 0.2 win share
//! alone, no score margin; 35% of the training seats were cheap-seller archetypes.
//!
//! The three rules bound its seat in training and bind it here (`Neural::with_table_rules`): it never
//! reprices the district it has just repriced in the same turn, it sails to an opponent's harbor only to
//! buy there (and then buys), and it sails to the Bank only with a container to pick up and room for it.
//! It also keeps Dealer's two rules: no same-turn loan repay, and a first bid of at least $3.

use crate::neural::Neural;
use crate::nets;

pub const ID: &str = "peddler";

/// The net in [`nets`].
pub const NET: &str = "peddler";

/// The least Peddler bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

pub fn new(seed: u64) -> Neural {
    Neural::new(nets::get(NET), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID).with_table_rules()
}
