use crate::color::{Color, ColorCounts};
use crate::state::{Auction, AuctionKind, BANK_LOTS, BankState, Bid};
use Color::{Blue, Green, Red, Yellow};

fn counts(items: &[(Color, u8)]) -> ColorCounts {
    let mut tally = ColorCounts::default();
    for &(color, n) in items {
        tally[color] += n;
    }
    tally
}

fn bank(cash_lots: [u32; 3]) -> BankState {
    BankState {
        cash_lots,
        ..BankState::default()
    }
}

/// A bank with an auction token on one lot. Who holds it and what they bid does not matter here.
fn with_token(mut b: BankState, kind: AuctionKind, lot: usize) -> BankState {
    b.auctions[kind] = Some(Auction {
        lot,
        holder: 0,
        bid: Bid::Cash(0),
    });
    b
}

#[test]
fn a_payment_is_dealt_out_from_lot_one_going_left_to_right() {
    let cases = [
        (0, [1, 2, 3]),
        (1, [2, 2, 3]),
        (2, [2, 3, 3]),
        (3, [2, 3, 4]),
        (4, [3, 3, 4]),
        (5, [3, 4, 4]),
        (6, [3, 4, 5]),
        (7, [4, 4, 5]),
    ];
    for (amount, expected) in cases {
        let mut b = bank([1, 2, 3]);
        b.receive_cash(amount);
        assert_eq!(b.cash_lots, expected, "paying {amount}");
    }
}

#[test]
fn every_payment_starts_again_at_lot_one() {
    let mut b = bank([0, 0, 0]);
    b.receive_cash(1);
    b.receive_cash(1);
    assert_eq!(b.cash_lots, [2, 0, 0]);
}

#[test]
fn a_payment_adds_exactly_its_amount_as_evenly_as_possible() {
    for amount in 0..40 {
        let mut b = bank([5, 0, 9]);
        b.receive_cash(amount);
        let added = [b.cash_lots[0] - 5, b.cash_lots[1], b.cash_lots[2] - 9];
        assert_eq!(added.iter().sum::<u32>(), amount);
        let (min, max) = (added.iter().min().unwrap(), added.iter().max().unwrap());
        assert!(max - min <= 1, "paying {amount}: {added:?}");
        assert!(added[0] >= added[1] && added[1] >= added[2], "{added:?}");
    }
}

#[test]
fn a_cash_payment_leaves_the_container_lots_alone() {
    let mut b = bank([1, 2, 3]);
    b.container_lots[0].0[2] = 4;
    let before = b.container_lots;
    b.receive_cash(5);
    assert_eq!(b.container_lots, before);
}

#[test]
fn a_cash_lot_with_an_auction_token_gets_nothing() {
    // The rulebook's example: $2 of interest with a token on cash lot II is $1 to lot I and $1 to lot III.
    let mut b = with_token(bank([1, 2, 3]), AuctionKind::CashLot, 1);
    b.receive_cash(2);
    assert_eq!(b.cash_lots, [2, 2, 4]);
}

#[test]
fn an_odd_payment_gives_its_extra_dollar_to_the_first_lot_still_open() {
    let cases = [
        (0, [1, 4, 4]), // $2 to lot II, $1 to lot III
        (1, [3, 2, 4]), // $2 to lot I, $1 to lot III
        (2, [3, 3, 3]), // $2 to lot I, $1 to lot II
    ];
    for (token, expected) in cases {
        let mut b = with_token(bank([1, 2, 3]), AuctionKind::CashLot, token);
        b.receive_cash(3);
        assert_eq!(b.cash_lots, expected, "token on lot {token}");
    }
}

#[test]
fn a_payment_around_a_token_adds_exactly_its_amount_to_the_other_lots() {
    for token in 0..BANK_LOTS {
        for amount in 0..40 {
            let mut b = with_token(bank([5, 0, 9]), AuctionKind::CashLot, token);
            let before = b.cash_lots;
            b.receive_cash(amount);

            assert_eq!(
                b.cash_lots[token], before[token],
                "token {token}, paying {amount}"
            );
            let added: Vec<u32> = (0..BANK_LOTS)
                .filter(|&lot| lot != token)
                .map(|lot| b.cash_lots[lot] - before[lot])
                .collect();
            assert_eq!(added.iter().sum::<u32>(), amount, "token {token}");
            assert!(
                added[0] >= added[1] && added[0] - added[1] <= 1,
                "token {token}, paying {amount}: {added:?}"
            );
        }
    }
}

#[test]
fn an_auction_of_a_container_lot_does_not_change_where_cash_goes() {
    let mut b = with_token(bank([1, 2, 3]), AuctionKind::ContainerLot, 0);
    b.receive_cash(3);
    assert_eq!(b.cash_lots, [2, 3, 4]);
}

#[test]
fn containers_are_dealt_one_lot_at_a_time_wrapping_around_a_container_lot_token() {
    let cases = [
        (None, [0, 1, 2, 0, 1, 2, 0]),
        (Some(0), [1, 2, 1, 2, 1, 2, 1]),
        (Some(1), [0, 2, 0, 2, 0, 2, 0]),
        (Some(2), [0, 1, 0, 1, 0, 1, 0]),
    ];
    for (token, expected) in cases {
        let b = match token {
            Some(lot) => with_token(BankState::default(), AuctionKind::ContainerLot, lot),
            None => BankState::default(),
        };
        let dealt: Vec<usize> = (0..expected.len())
            .map(|n| b.container_lot_for(n))
            .collect();
        assert_eq!(dealt, expected, "token on {token:?}");
    }
}

#[test]
fn an_auction_of_a_cash_lot_does_not_change_where_containers_go() {
    let b = with_token(BankState::default(), AuctionKind::CashLot, 1);
    let dealt: Vec<usize> = (0..6).map(|n| b.container_lot_for(n)).collect();
    assert_eq!(dealt, [0, 1, 2, 0, 1, 2]);
}

#[test]
fn a_container_payment_is_dealt_one_at_a_time_color_by_color_from_lot_one() {
    let mut b = BankState::default();
    b.receive_containers(&counts(&[(Red, 1), (Blue, 1), (Yellow, 2), (Green, 1)]));
    assert_eq!(b.container_lots[0], counts(&[(Red, 1), (Yellow, 1)]));
    assert_eq!(b.container_lots[1], counts(&[(Blue, 1), (Green, 1)]));
    assert_eq!(b.container_lots[2], counts(&[(Yellow, 1)]));
}

#[test]
fn a_container_payment_is_added_to_what_the_lots_already_hold() {
    let mut b = BankState {
        container_lots: [
            counts(&[(Red, 2)]),
            counts(&[(Red, 1), (Blue, 3)]),
            counts(&[(Green, 4)]),
        ],
        ..BankState::default()
    };
    b.receive_containers(&counts(&[(Red, 2), (Blue, 1)]));
    assert_eq!(
        b.container_lots,
        [
            counts(&[(Red, 3)]),
            counts(&[(Red, 2), (Blue, 3)]),
            counts(&[(Blue, 1), (Green, 4)]),
        ]
    );
}

#[test]
fn a_container_payment_skips_the_lot_being_auctioned() {
    let cases = [(0, [0, 2, 1]), (1, [2, 0, 1]), (2, [2, 1, 0])];
    for (token, expected) in cases {
        let mut b = with_token(BankState::default(), AuctionKind::ContainerLot, token);
        b.receive_containers(&counts(&[(Red, 3)]));
        assert_eq!(
            b.container_lots.map(|lot| lot[Red]),
            expected,
            "token on lot {token}"
        );
    }
}

#[test]
fn an_auction_of_a_cash_lot_does_not_change_where_a_container_payment_goes() {
    let mut b = with_token(BankState::default(), AuctionKind::CashLot, 1);
    b.receive_containers(&counts(&[(Red, 3)]));
    assert_eq!(b.container_lots.map(|lot| lot[Red]), [1, 1, 1]);
}

#[test]
fn a_container_payment_adds_exactly_its_containers_as_evenly_as_possible_around_a_token() {
    let payments = [
        counts(&[]),
        counts(&[(Red, 1)]),
        counts(&[(Red, 4)]),
        counts(&[(Blue, 3), (Green, 2)]),
        counts(&[(Red, 2), (Blue, 2), (Yellow, 2), (Green, 2)]),
        counts(&[(Red, 5), (Blue, 5)]),
    ];
    for token in [None, Some(0), Some(1), Some(2)] {
        for payment in &payments {
            let mut b = match token {
                Some(lot) => with_token(BankState::default(), AuctionKind::ContainerLot, lot),
                None => BankState::default(),
            };
            b.receive_containers(payment);

            let mut per_color = ColorCounts::default();
            for lot in &b.container_lots {
                per_color.add_all(lot);
            }
            assert_eq!(&per_color, payment, "token {token:?}");

            let open: Vec<u32> = (0..BANK_LOTS)
                .filter(|&lot| Some(lot) != token)
                .map(|lot| b.container_lots[lot].total())
                .collect();
            if let Some(lot) = token {
                assert_eq!(b.container_lots[lot].total(), 0, "token {token:?}");
            }
            assert!(
                open.windows(2).all(|w| w[0] >= w[1] && w[0] - w[1] <= 1),
                "token {token:?}, paying {payment:?}: {open:?}"
            );
        }
    }
}

#[test]
fn a_container_payment_changes_nothing_but_the_container_lots() {
    let mut b = with_token(bank([1, 2, 3]), AuctionKind::CashLot, 1);
    let before = b;
    b.receive_containers(&counts(&[(Red, 2), (Green, 3)]));
    assert_eq!(b.cash_lots, before.cash_lots);
    assert_eq!(b.auctions, before.auctions);
}

#[test]
fn an_empty_container_payment_changes_nothing() {
    let mut b = with_token(bank([1, 2, 3]), AuctionKind::ContainerLot, 2);
    b.container_lots[1] = counts(&[(Blue, 2)]);
    let before = b;
    b.receive_containers(&ColorCounts::default());
    assert_eq!(b, before);
}
