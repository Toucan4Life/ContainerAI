//! The cash cards of a delivery bid: how an amount of cash lies on the table as cards, and the $0
//! bluff cards that disguise how many cards it is.

use std::ops::RangeInclusive;

/// What the cash cards are worth. Change is free and the cards never run out.
pub const DENOMINATIONS: [u32; 5] = [1, 2, 5, 10, 20];

/// The $0 bluff cards each player has. They come back to the hand after every bid.
pub const BLUFF_CARDS: u8 = 2;

/// Whether `amount` can lie on the table as exactly `cards` cash cards.
pub fn can_show(amount: u32, cards: u32) -> bool {
    (0..=amount / 5).any(|fives| counts_with_fives(amount, fives).contains(&cards))
}

/// Every number of cards `amount` can lie on the table as, from the fewest up.
pub fn card_counts(amount: u32) -> Vec<u32> {
    let mut counts = Vec::new();
    let mut next = 0;
    // More fives means both fewer cards at the very least and fewer at the very most, so counting
    // down from the most fives lists the ranges from the lowest up.
    for fives in (0..=amount / 5).rev() {
        let (fewest, most) = counts_with_fives(amount, fives).into_inner();
        counts.extend(fewest.max(next)..=most);
        next = most + 1;
    }
    counts
}

/// The numbers of cards when `fives` fives' worth of the cash are in $5, $10 and $20 cards and the
/// rest is in $1 and $2 cards.
///
/// Splitting a card into smaller ones adds exactly one card at a time (a $20 into two $10s, a $10
/// into two $5s, a $2 into two $1s), so every number between the fewest and the most is possible.
fn counts_with_fives(amount: u32, fives: u32) -> RangeInclusive<u32> {
    let rest = amount - 5 * fives;
    let fewest = fewest_cards_for_fives(fives) + rest.div_ceil(2);
    let most = fives + rest;
    fewest..=most
}

/// The fewest $5, $10 and $20 cards that make `fives` fives, which are worth 1, 2 and 4 of them.
fn fewest_cards_for_fives(fives: u32) -> u32 {
    fives / 4 + [0, 1, 1, 2][(fives % 4) as usize]
}
