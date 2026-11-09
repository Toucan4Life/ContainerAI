use crate::cards::{BLUFF_CARDS, DENOMINATIONS, can_show, card_counts};

/// For every amount up to `max`, the numbers of cards it can be made of, found by trying every card
/// that could be the last one laid down.
fn by_trying_every_combination(max: usize) -> Vec<Vec<bool>> {
    let mut table = vec![vec![false; max + 1]; max + 1];
    table[0][0] = true;
    for amount in 1..=max {
        for denomination in DENOMINATIONS {
            let Some(rest) = amount.checked_sub(denomination as usize) else {
                continue;
            };
            for cards in 1..=max {
                if table[rest][cards - 1] {
                    table[amount][cards] = true;
                }
            }
        }
    }
    table
}

fn counts_in(table: &[Vec<bool>], amount: usize) -> Vec<u32> {
    (0..table[amount].len() as u32)
        .filter(|&cards| table[amount][cards as usize])
        .collect()
}

#[test]
fn the_cash_cards_are_ones_twos_fives_tens_and_twenties() {
    assert_eq!(DENOMINATIONS, [1, 2, 5, 10, 20]);
}

#[test]
fn every_player_has_two_bluff_cards() {
    assert_eq!(BLUFF_CARDS, 2);
}

#[test]
fn no_cash_is_no_cards_and_any_cash_is_at_least_one_card() {
    assert!(can_show(0, 0));
    assert_eq!(card_counts(0), vec![0]);
    assert!(!can_show(0, 1));
    assert!(!can_show(0, 2));
    for amount in 1..60 {
        assert!(!can_show(amount, 0), "${amount} is not no cards");
        assert!(!card_counts(amount).contains(&0));
    }
}

#[test]
fn small_amounts_lie_on_the_table_as_the_cards_that_add_up_to_them() {
    // Worked out by hand: $5 is one card, or 2+2+1, 2+1+1+1 and 1+1+1+1+1, but never two cards;
    // $10 is 10, 5+5, 5+2+2+1, ..., but no three cards add up to it.
    let table: [(u32, &[u32]); 9] = [
        (1, &[1]),
        (2, &[1, 2]),
        (3, &[2, 3]),
        (4, &[2, 3, 4]),
        (5, &[1, 3, 4, 5]),
        (6, &[2, 3, 4, 5, 6]),
        (7, &[2, 3, 4, 5, 6, 7]),
        (8, &[3, 4, 5, 6, 7, 8]),
        (10, &[1, 2, 4, 5, 6, 7, 8, 9, 10]),
    ];
    for (amount, counts) in table {
        assert_eq!(card_counts(amount), counts, "${amount}");
        for cards in 0..=amount + 2 {
            assert_eq!(
                can_show(amount, cards),
                counts.contains(&cards),
                "${amount} as {cards} cards"
            );
        }
    }
}

#[test]
fn twenty_dollars_can_be_any_number_of_cards_up_to_twenty() {
    assert_eq!(card_counts(20), (1..=20).collect::<Vec<u32>>());
}

#[test]
fn card_counts_agree_with_trying_every_combination() {
    let table = by_trying_every_combination(1400);
    for amount in 0..=1400 {
        assert_eq!(
            card_counts(amount as u32),
            counts_in(&table, amount),
            "${amount}"
        );
    }
}

#[test]
fn can_show_agrees_with_trying_every_combination() {
    let table = by_trying_every_combination(1400);
    let amounts = (0..=260).chain([499, 500, 501, 937, 1000, 1306, 1399, 1400]);
    for amount in amounts {
        for cards in 0..=amount + 3 {
            let expected = table[amount].get(cards).copied().unwrap_or(false);
            assert_eq!(
                can_show(amount as u32, cards as u32),
                expected,
                "${amount} as {cards} cards"
            );
        }
    }
}

#[test]
fn the_fewest_cards_are_the_biggest_ones_and_the_most_are_all_ones() {
    for amount in 1..=1400u32 {
        let counts = card_counts(amount);
        let small = amount % 5;
        let greedy = amount / 20 + (amount % 20) / 10 + (amount % 10) / 5 + small / 2 + small % 2;
        assert_eq!(counts.first(), Some(&greedy), "fewest for ${amount}");
        assert_eq!(counts.last(), Some(&amount), "most for ${amount}");
    }
}

#[test]
fn counts_are_listed_once_each_from_the_fewest_up() {
    for amount in 0..=300 {
        let counts = card_counts(amount);
        assert!(counts.windows(2).all(|pair| pair[0] < pair[1]), "${amount}");
    }
}

#[test]
fn absurd_card_counts_are_refused_without_overflowing() {
    for cards in [u32::MAX, u32::MAX - 1, 1 << 31] {
        assert!(!can_show(5, cards));
        assert!(!can_show(0, cards));
        assert!(!can_show(1306, cards));
    }
}
