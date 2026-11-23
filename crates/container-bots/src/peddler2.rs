//! Peddler v2: Peddler trained 20 more minutes (a warm start from its weights, no imitation) in a league
//! that seats Solitaire, Merchant and a frozen copy of Peddler, none of which Peddler had been trained
//! against. The reward is Peddler's: 0.8 interaction (buying from opponents, bids on their deals, selling
//! to them, fair prices) plus 0.2 win share alone, no score margin.
//!
//! The rules are Peddler's, bound to its seat in training and here (`Neural::with_table_rules`): it never
//! reprices the district it has just repriced in the same turn, it sails to an opponent's harbor only to
//! buy there (and then buys), and it sails to the Bank only with a container to pick up and room for it.
//! It also keeps Dealer's two rules: no same-turn loan repay, and a first bid of at least $3.

use crate::neural::Neural;
use crate::nets;

pub const ID: &str = "peddler2";

/// The net in [`nets`].
pub const NET: &str = "peddler2";

/// The least Peddler v2 bids in the first round of a delivery auction.
const MIN_BID: u32 = 3;

pub fn new(seed: u64) -> Neural {
    Neural::new(nets::get(NET), seed, 0.0).without_same_turn_repay().with_min_bid(MIN_BID).with_table_rules()
}
