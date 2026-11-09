use super::{
    actions_while_playing, finish_arrange, new_game, set_cash, set_factories, set_factory_lots,
    set_lots, set_warehouses,
};
use crate::action::{Action, Building, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;
use crate::state::PlayerId;

fn game() -> Game {
    new_game(3, GameLength::Standard, 1)
}

/// The active player buys from the next seat, who holds exactly `stock` (lot, color, count) in the factory district
/// and owns enough factories to store it. The buyer has the given cash and warehouses and an empty harbor.
fn buying_game(
    stock: &[(usize, Color, u8)],
    cash: u32,
    warehouses: u8,
) -> (Game, PlayerId, PlayerId) {
    let mut g = game();
    let buyer = g.current_player();
    let seller = (buyer + 1) % 3;
    let total: u8 = stock.iter().map(|&(_, _, n)| n).sum();
    let factories = usize::from(total).div_ceil(2).max(1);
    set_factories(&mut g, seller, &Color::ALL[..factories]);
    set_factory_lots(&mut g, seller, stock);
    set_cash(&mut g, buyer, cash);
    set_warehouses(&mut g, buyer, warehouses);
    (g, buyer, seller)
}

fn buy(lot: usize, color: Color) -> Action {
    Action::Buy { lot, color }
}

fn hand(g: &Game) -> ColorCounts {
    match g.phase {
        Phase::Arrange(a) => {
            assert_eq!(a.district, District::Harbor);
            a.hand
        }
        other => panic!("expected the harbor arrangement, got {other:?}"),
    }
}

fn cash(g: &Game, p: PlayerId) -> u32 {
    g.state.players[p].cash
}

fn purchases(g: &Game) -> Vec<Action> {
    g.legal_actions()
        .into_iter()
        .filter(|a| matches!(a, Action::FactoryPurchase(_)))
        .collect()
}

/// The hand and cash at the end of every complete sequence of purchases.
fn outcomes(g: &Game, out: &mut Vec<(ColorCounts, u32)>) {
    match g.phase {
        Phase::Buy(_) => {
            let legal = g.legal_actions();
            assert!(!legal.is_empty(), "dead end while buying");
            for a in legal {
                let mut next = g.clone();
                next.apply(a).unwrap();
                outcomes(&next, out);
            }
        }
        Phase::Arrange(a) => out.push((a.hand, cash(g, g.current_player()))),
        other => panic!("unexpected phase {other:?}"),
    }
}

#[test]
fn factory_purchase_is_offered_for_every_opponent_with_something_to_sell() {
    let g = game();
    let me = g.current_player();
    let expected: Vec<Action> = (0..3)
        .filter(|&p| p != me)
        .map(Action::FactoryPurchase)
        .collect();
    assert_eq!(purchases(&g), expected);
}

#[test]
fn nobody_can_buy_from_themselves_or_from_an_empty_seat() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        let me = g.current_player();
        assert!(!g.is_legal(Action::FactoryPurchase(me)));
        for seat in players..8 {
            assert!(!g.is_legal(Action::FactoryPurchase(seat)), "seat {seat}");
        }
    }
}

#[test]
fn an_opponent_without_containers_cannot_be_bought_from() {
    let (g, _, seller) = buying_game(&[], 20, 1);
    assert!(!g.is_legal(Action::FactoryPurchase(seller)));
}

#[test]
fn the_buyer_must_afford_at_least_one_container() {
    let (mut g, buyer, seller) = buying_game(&[(1, Color::Red, 1)], 1, 1);
    assert!(
        !g.is_legal(Action::FactoryPurchase(seller)),
        "$2 container, $1 in hand"
    );
    set_cash(&mut g, buyer, 2);
    assert!(g.is_legal(Action::FactoryPurchase(seller)));
}

#[test]
fn the_buyer_needs_room_in_the_harbor() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 1)], 20, 1);
    assert!(g.is_legal(Action::FactoryPurchase(seller)));
    set_lots(&mut g, buyer, District::Harbor, &[(0, Color::Blue, 1)]);
    assert!(
        !g.is_legal(Action::FactoryPurchase(seller)),
        "the harbor is full"
    );
}

#[test]
fn choosing_the_seller_opens_the_buying_node() {
    let stock = [
        (0, Color::Red, 1),
        (1, Color::Red, 1),
        (1, Color::Blue, 1),
        (3, Color::Green, 1),
    ];
    let (mut g, _, seller) = buying_game(&stock, 3, 3);

    g.apply(Action::FactoryPurchase(seller)).unwrap();

    assert!(matches!(g.phase, Phase::Buy(_)));
    assert_eq!(
        g.legal_actions(),
        vec![buy(0, Color::Red), buy(1, Color::Red), buy(1, Color::Blue)],
        "the $4 container is out of reach and at least one purchase is required"
    );
    assert_eq!(g.turn.actions_left, 2, "the action is still being resolved");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn buying_pays_the_seller_the_lot_price_and_moves_the_container_into_hand() {
    let (mut g, buyer, seller) = buying_game(&[(2, Color::Yellow, 1)], 10, 2);
    let seller_cash = cash(&g, seller);

    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(2, Color::Yellow)).unwrap();

    assert_eq!(cash(&g, buyer), 10 - 3);
    assert_eq!(cash(&g, seller), seller_cash + 3);
    assert!(g.state.players[seller].factory_lots[2].is_empty());
    assert_eq!(hand(&g), ColorCounts([0, 0, 1, 0, 0]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn each_factory_lot_has_its_own_price() {
    for lot in 0..4 {
        let (mut g, buyer, seller) = buying_game(&[(lot, Color::Red, 1)], 20, 1);
        g.apply(Action::FactoryPurchase(seller)).unwrap();
        g.apply(buy(lot, Color::Red)).unwrap();
        assert_eq!(cash(&g, buyer), 20 - (lot as u32 + 1), "lot {lot}");
    }
}

#[test]
fn several_containers_can_be_bought_in_one_action() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 2), (1, Color::Blue, 1)], 20, 3);

    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    g.apply(buy(1, Color::Blue)).unwrap();

    assert_eq!(hand(&g), ColorCounts([2, 1, 0, 0, 0]));
    assert_eq!(cash(&g, buyer), 20 - 1 - 1 - 2);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn buying_may_stop_early_once_something_was_bought() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 2), (1, Color::Blue, 1)], 20, 3);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();

    assert!(matches!(g.phase, Phase::Buy(_)));
    assert!(g.legal_actions().contains(&Action::DoneBuying));
    g.apply(Action::DoneBuying).unwrap();

    assert_eq!(hand(&g), ColorCounts([1, 0, 0, 0, 0]));
}

#[test]
fn at_least_one_container_must_be_bought() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 2)], 20, 3);
    g.apply(Action::FactoryPurchase(seller)).unwrap();

    assert!(!g.is_legal(Action::DoneBuying));
    assert!(!g.legal_actions().contains(&Action::DoneBuying));
    assert!(g.apply(Action::DoneBuying).is_err());
}

#[test]
fn the_harbor_limit_caps_the_purchase() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 4)], 20, 2);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();

    assert_eq!(hand(&g).total(), 2, "no third container fits");
    assert_eq!(g.state.players[seller].factory_lots[0][Color::Red], 2);
}

#[test]
fn stored_containers_count_against_the_harbor_limit() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 4)], 20, 3);
    set_lots(&mut g, buyer, District::Harbor, &[(2, Color::Green, 1)]);

    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();

    assert_eq!(
        hand(&g),
        ColorCounts([2, 0, 0, 0, 1]),
        "two bought plus the stored one fill the harbor"
    );
}

#[test]
fn cash_caps_the_purchase() {
    let (mut g, buyer, seller) = buying_game(&[(1, Color::Red, 4)], 5, 4);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(1, Color::Red)).unwrap();
    g.apply(buy(1, Color::Red)).unwrap();

    assert_eq!(hand(&g).total(), 2, "$1 left cannot buy a $2 container");
    assert_eq!(cash(&g, buyer), 1);
}

#[test]
fn the_harbor_is_priced_again_with_the_new_containers_and_the_old_ones() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 1)], 20, 3);
    set_lots(&mut g, buyer, District::Harbor, &[(0, Color::Green, 1)]);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    assert_eq!(hand(&g), ColorCounts([1, 0, 0, 0, 1]));

    g.apply(Action::Place(4)).unwrap();
    g.apply(Action::Place(2)).unwrap();

    let lots = g.state.players[buyer].harbor_lots;
    assert_eq!(lots[4], ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(lots[2], ColorCounts([0, 0, 0, 0, 1]));
    assert!(lots[0].is_empty());
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_factory_purchase_is_one_action() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 1)], 20, 1);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    assert_eq!(g.turn.actions_left, 2);

    finish_arrange(&mut g);

    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.current_player(), buyer);
    assert_eq!(g.phase, Phase::Main);
}

#[test]
fn a_factory_purchase_can_be_taken_twice_in_a_turn() {
    let (mut g, buyer, seller) = buying_game(&[(0, Color::Red, 2)], 20, 2);
    for _ in 0..2 {
        g.apply(Action::FactoryPurchase(seller)).unwrap();
        g.apply(buy(0, Color::Red)).unwrap();
        if matches!(g.phase, Phase::Buy(_)) {
            g.apply(Action::DoneBuying).unwrap();
        }
        finish_arrange(&mut g);
    }
    assert_ne!(g.current_player(), buyer, "two actions end the turn");
    assert_eq!(g.state.players[buyer].harbor_stock(), 2);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn containers_in_hand_mid_purchase_still_count() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 3)], 20, 3);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();

    assert!(matches!(g.phase, Phase::Buy(_)));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn other_actions_are_illegal_while_buying() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 3)], 20, 3);
    g.apply(Action::FactoryPurchase(seller)).unwrap();

    for a in [
        Action::Produce,
        Action::Build(Building::Warehouse),
        Action::Reprice(District::Factory),
        Action::FactoryPurchase(seller),
        Action::Place(0),
        buy(1, Color::Red),
    ] {
        assert!(!g.is_legal(a), "{a:?}");
        assert!(g.apply(a).is_err(), "{a:?}");
    }
}

#[test]
fn buying_actions_are_illegal_at_the_main_node() {
    let g = game();
    assert!(!g.is_legal(buy(0, Color::Red)));
    assert!(!g.is_legal(Action::DoneBuying));
}

#[test]
fn lots_beyond_the_factory_district_do_not_exist() {
    let (mut g, _, seller) = buying_game(&[(0, Color::Red, 1)], 20, 3);
    g.apply(Action::FactoryPurchase(seller)).unwrap();
    assert!(
        !g.is_legal(buy(4, Color::Red)),
        "a factory district has four lots"
    );
}

#[test]
fn a_rejected_purchase_leaves_the_game_unchanged() {
    let mut g = game();
    let me = g.current_player();
    let before = g.clone();
    assert!(g.apply(Action::FactoryPurchase(me)).is_err());
    assert_eq!(g, before);
}

#[test]
fn every_feasible_purchase_is_reachable_exactly_once() {
    let stock = [
        (0, Color::Red, 1),
        (1, Color::Red, 1),
        (1, Color::Blue, 1),
        (3, Color::Green, 1),
    ];
    let prices = [1u32, 2, 2, 4];
    for cash in [1, 3, 5, 20] {
        for room in 1..=4u8 {
            let (mut g, _, seller) = buying_game(&stock, cash, room);
            g.apply(Action::FactoryPurchase(seller)).unwrap();
            let mut found = Vec::new();
            outcomes(&g, &mut found);

            // Every non-empty set of the four distinct containers that fits the cash and the room.
            let expected = (1u32..16)
                .filter(|mask| {
                    let cost: u32 = (0..4)
                        .filter(|i| mask >> i & 1 == 1)
                        .map(|i| prices[i])
                        .sum();
                    mask.count_ones() <= u32::from(room) && cost <= cash
                })
                .count();
            assert_eq!(found.len(), expected, "cash {cash}, room {room}");
            assert!(found.iter().all(|(hand, _)| !hand.is_empty()));
        }
    }
}

#[test]
fn random_play_with_many_purchases_keeps_every_invariant() {
    for seed in 0..4 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            let mut rng = Rng::new(seed + 50);
            for _ in 0..600 {
                let Some(legal) = actions_while_playing(&g) else {
                    break;
                };
                // Buy and produce whenever possible, so containers keep changing hands.
                let preferred: Vec<Action> = legal
                    .iter()
                    .copied()
                    .filter(|a| matches!(a, Action::FactoryPurchase(_) | Action::Produce))
                    .collect();
                let pool = if preferred.is_empty() {
                    &legal
                } else {
                    &preferred
                };
                let pick = pool[rng.below(pool.len() as u32) as usize];
                g.apply(pick).unwrap();
                assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
            }
        }
    }
}
