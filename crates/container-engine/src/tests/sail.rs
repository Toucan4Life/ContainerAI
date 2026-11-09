use super::{
    actions_while_playing, new_game, pass_turn, set_cash, set_holding, set_lots, set_ship,
    set_warehouses,
};
use crate::action::{Action, Building, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;
use crate::state::{AuctionKind, PlayerId, ShipLocation};

fn game() -> Game {
    new_game(3, GameLength::Standard, 1)
}

fn dock(g: &mut Game, location: ShipLocation) {
    let me = g.current_player();
    g.state.players[me].ship_location = location;
}

fn location(g: &Game, p: PlayerId) -> ShipLocation {
    g.state.players[p].ship_location
}

fn sails(g: &Game) -> Vec<Action> {
    g.legal_actions()
        .into_iter()
        .filter(|a| matches!(a, Action::Sail(_)))
        .collect()
}

fn buy(lot: usize, color: Color) -> Action {
    Action::Buy { lot, color }
}

fn ship(g: &Game, p: PlayerId) -> ColorCounts {
    g.state.players[p].ship
}

fn cash(g: &Game, p: PlayerId) -> u32 {
    g.state.players[p].cash
}

/// The active player and the seat after them, whose harbor holds `stock` (lot, color, count).
/// The buyer has `cash`, and the ship is still in the ocean.
fn harbor_game(stock: &[(usize, Color, u8)], cash: u32) -> (Game, PlayerId, PlayerId) {
    let mut g = game();
    let buyer = g.current_player();
    let seller = (buyer + 1) % 3;
    let total: u8 = stock.iter().map(|&(_, _, n)| n).sum();
    set_warehouses(&mut g, seller, total.max(1));
    set_lots(&mut g, seller, District::Harbor, stock);
    set_cash(&mut g, buyer, cash);
    (g, buyer, seller)
}

/// As `harbor_game`, with the buyer's ship already docked at the seller's harbor.
fn docked_game(stock: &[(usize, Color, u8)], cash: u32) -> (Game, PlayerId, PlayerId) {
    let (mut g, buyer, seller) = harbor_game(stock, cash);
    g.state.players[buyer].ship_location = ShipLocation::Harbor(seller);
    (g, buyer, seller)
}

#[test]
fn from_the_ocean_the_ship_can_reach_the_bank_and_every_opponents_harbor() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        let me = g.current_player();
        let mut expected = vec![Action::Sail(ShipLocation::Bank)];
        expected.extend(
            (0..players)
                .filter(|&p| p != me)
                .map(|p| Action::Sail(ShipLocation::Harbor(p))),
        );
        assert_eq!(sails(&g), expected, "{players} players");
    }
}

#[test]
fn nobody_can_sail_to_their_own_harbor_or_to_an_empty_seat() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        let me = g.current_player();
        assert!(!g.is_legal(Action::Sail(ShipLocation::Harbor(me))));
        for seat in players..8 {
            assert!(
                !g.is_legal(Action::Sail(ShipLocation::Harbor(seat))),
                "seat {seat}"
            );
        }
    }
}

#[test]
fn from_anywhere_but_the_ocean_the_ship_can_only_go_to_the_ocean() {
    let other = (game().current_player() + 1) % 3;
    for place in [
        ShipLocation::Bank,
        ShipLocation::Island,
        ShipLocation::Harbor(other),
    ] {
        let mut g = game();
        dock(&mut g, place);
        assert_eq!(
            sails(&g),
            vec![Action::Sail(ShipLocation::Ocean)],
            "{place:?}"
        );
    }
}

#[test]
fn sailing_to_the_ocean_is_one_action() {
    let mut g = game();
    let me = g.current_player();
    dock(&mut g, ShipLocation::Bank);

    g.apply(Action::Sail(ShipLocation::Ocean)).unwrap();

    assert_eq!(location(&g, me), ShipLocation::Ocean);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn moving_between_boards_takes_two_actions_and_fits_in_one_turn() {
    let mut g = game();
    let me = g.current_player();
    dock(&mut g, ShipLocation::Harbor((me + 1) % 3));

    g.apply(Action::Sail(ShipLocation::Ocean)).unwrap();
    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    assert_eq!(location(&g, me), ShipLocation::Bank);
    assert_ne!(g.current_player(), me, "both actions are used up");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_ship_keeps_its_place_between_turns() {
    let mut g = game();
    let me = g.current_player();
    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();
    g.apply(Action::Reprice(District::Factory)).unwrap();
    while g.phase != Phase::Main {
        g.apply(g.legal_actions()[0]).unwrap();
    }
    assert_eq!(location(&g, me), ShipLocation::Bank);
    assert_ne!(g.current_player(), me);
}

#[test]
fn the_bank_can_be_reached_with_nothing_to_collect() {
    let mut g = game();
    let me = g.current_player();

    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    assert_eq!(location(&g, me), ShipLocation::Bank);
    assert_eq!(g.phase, Phase::Main, "nothing to load, so no anchor choice");
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn loading_at_the_bank_moves_containers_from_holding_to_the_ship_for_free() {
    let mut g = game();
    let me = g.current_player();
    set_holding(&mut g, me, ColorCounts([2, 1, 0, 0, 0]));
    let before = cash(&g, me);

    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    assert!(matches!(g.phase, Phase::Load(_)));
    assert_eq!(
        g.legal_actions(),
        vec![
            Action::Load(Color::Red),
            Action::Load(Color::Blue),
            Action::DoneLoading
        ]
    );
    assert_eq!(
        g.turn.actions_left, 2,
        "the anchor action is part of the sail"
    );
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::Load(Color::Red)).unwrap();
    g.apply(Action::Load(Color::Red)).unwrap();
    g.apply(Action::Load(Color::Blue)).unwrap();

    assert_eq!(ship(&g, me), ColorCounts([2, 1, 0, 0, 0]));
    assert!(g.state.players[me].holding.is_empty());
    assert_eq!(cash(&g, me), before);
    assert_eq!(g.phase, Phase::Main, "nothing is left to load");
    assert_eq!(g.turn.actions_left, 1, "an anchor action is free");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_free_load_at_the_bank_can_be_skipped() {
    let mut g = game();
    let me = g.current_player();
    set_holding(&mut g, me, ColorCounts([1, 0, 0, 0, 0]));
    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    g.apply(Action::DoneLoading).unwrap();

    assert!(ship(&g, me).is_empty());
    assert_eq!(g.state.players[me].holding, ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn loading_stops_when_the_ship_is_full() {
    let mut g = game();
    let me = g.current_player();
    set_ship(&mut g, me, ColorCounts([0, 0, 0, 0, 4]));
    set_holding(&mut g, me, ColorCounts([3, 0, 0, 0, 0]));
    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    g.apply(Action::Load(Color::Red)).unwrap();

    assert_eq!(ship(&g, me).total(), 5);
    assert_eq!(g.state.players[me].holding, ColorCounts([2, 0, 0, 0, 0]));
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn nothing_is_offered_at_the_bank_with_a_full_ship() {
    let mut g = game();
    let me = g.current_player();
    set_ship(&mut g, me, ColorCounts([0, 0, 0, 0, 5]));
    set_holding(&mut g, me, ColorCounts([1, 0, 0, 0, 0]));

    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn colors_are_loaded_in_color_order() {
    let mut g = game();
    let me = g.current_player();
    set_holding(&mut g, me, ColorCounts([1, 1, 1, 0, 0]));
    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();
    g.apply(Action::Load(Color::Blue)).unwrap();

    assert_eq!(
        g.legal_actions(),
        vec![Action::Load(Color::Yellow), Action::DoneLoading,],
        "Red is behind us and Blue has none left"
    );
}

#[test]
fn docked_at_the_bank_a_harbor_purchase_loads_the_holding_area_for_free() {
    let mut g = game();
    let me = g.current_player();
    dock(&mut g, ShipLocation::Bank);
    set_holding(&mut g, me, ColorCounts([1, 0, 2, 0, 0]));
    let before = cash(&g, me);
    assert!(g.legal_actions().contains(&Action::HarborPurchase));

    g.apply(Action::HarborPurchase).unwrap();

    assert!(matches!(g.phase, Phase::Load(_)));
    assert!(
        !g.is_legal(Action::DoneLoading),
        "a paid action must load something"
    );
    g.apply(Action::Load(Color::Red)).unwrap();
    assert!(g.is_legal(Action::DoneLoading));
    g.apply(Action::Load(Color::Yellow)).unwrap();
    assert_eq!(g.turn.actions_left, 2);
    g.apply(Action::Load(Color::Yellow)).unwrap();

    assert_eq!(ship(&g, me), ColorCounts([1, 0, 2, 0, 0]));
    assert_eq!(cash(&g, me), before);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(
        g.turn.actions_left, 1,
        "loading at the bank is a normal action"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_harbor_purchase_at_the_bank_needs_something_to_load_and_room_for_it() {
    let mut g = game();
    let me = g.current_player();
    dock(&mut g, ShipLocation::Bank);
    assert!(!g.is_legal(Action::HarborPurchase), "holding area is empty");

    set_holding(&mut g, me, ColorCounts([1, 0, 0, 0, 0]));
    assert!(g.is_legal(Action::HarborPurchase));

    set_ship(&mut g, me, ColorCounts([0, 0, 0, 0, 5]));
    assert!(!g.is_legal(Action::HarborPurchase), "the ship is full");
}

/// The errata's own scenario: the ship is already docked when the auction is won, so there is no
/// arrival to load on.
#[test]
fn a_ship_docked_at_the_bank_loads_the_containers_of_an_auction_it_won() {
    let mut g = game();
    let me = g.current_player();
    let lot = g.state.bank.container_lots[0];
    assert!(lot.total() > 0);

    g.apply(Action::Sail(ShipLocation::Bank)).unwrap();
    assert_eq!(g.phase, Phase::Main, "nothing to collect on arrival");
    g.apply(Action::StartAuction {
        kind: AuctionKind::ContainerLot,
        lot: 0,
    })
    .unwrap();
    g.apply(Action::BidCash(5)).unwrap();
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), me);
    assert_eq!(
        location(&g, me),
        ShipLocation::Bank,
        "the ship stays docked"
    );
    assert_eq!(g.state.players[me].holding, lot, "the lot is won and held");
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert!(g.is_legal(Action::HarborPurchase));

    g.apply(Action::HarborPurchase).unwrap();
    for color in Color::ALL {
        for _ in 0..lot[color] {
            g.apply(Action::Load(color)).unwrap();
        }
    }

    assert_eq!(ship(&g, me), lot);
    assert!(g.state.players[me].holding.is_empty());
    assert_eq!(cash(&g, me), 15, "loading costs nothing");
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1, "it was one action");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn loading_actions_are_illegal_elsewhere() {
    let mut g = game();
    let me = g.current_player();
    set_holding(&mut g, me, ColorCounts([1, 0, 0, 0, 0]));
    for c in Color::ALL {
        assert!(!g.is_legal(Action::Load(c)));
    }
    assert!(!g.is_legal(Action::DoneLoading));
    assert!(g.apply(Action::Load(Color::Red)).is_err());
}

#[test]
fn arriving_at_a_harbor_offers_a_free_purchase_that_can_be_skipped() {
    let (mut g, buyer, seller) = harbor_game(&[(0, Color::Red, 1), (2, Color::Blue, 1)], 20);

    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();

    assert_eq!(location(&g, buyer), ShipLocation::Harbor(seller));
    assert!(matches!(g.phase, Phase::Buy(_)));
    assert_eq!(
        g.legal_actions(),
        vec![buy(0, Color::Red), buy(2, Color::Blue), Action::DoneBuying]
    );
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::DoneBuying).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
    assert!(ship(&g, buyer).is_empty());
    assert_eq!(cash(&g, buyer), 20);
}

#[test]
fn the_free_purchase_pays_the_harbor_owner_and_loads_the_ship() {
    let (mut g, buyer, seller) = harbor_game(&[(1, Color::Green, 1), (4, Color::Red, 2)], 20);
    let seller_cash = cash(&g, seller);

    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
    g.apply(buy(1, Color::Green)).unwrap();
    g.apply(buy(4, Color::Red)).unwrap();
    assert_eq!(
        g.check_invariants(),
        Ok(()),
        "containers in hand still count"
    );
    g.apply(Action::DoneBuying).unwrap();

    assert_eq!(cash(&g, buyer), 20 - 3 - 6);
    assert_eq!(cash(&g, seller), seller_cash + 3 + 6);
    assert_eq!(ship(&g, buyer), ColorCounts([1, 0, 0, 0, 1]));
    let lots = g.state.players[seller].harbor_lots;
    assert!(lots[1].is_empty());
    assert_eq!(lots[4], ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(
        g.turn.actions_left, 1,
        "the purchase was a free anchor action"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn harbor_lots_are_priced_two_to_six() {
    for lot in 0..5 {
        let (mut g, buyer, seller) = harbor_game(&[(lot, Color::Red, 1)], 20);
        g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
        g.apply(buy(lot, Color::Red)).unwrap();
        assert_eq!(cash(&g, buyer), 20 - (lot as u32 + 2), "lot {lot}");
    }
}

#[test]
fn a_harbor_has_five_lots() {
    let (mut g, _, seller) = harbor_game(&[(0, Color::Red, 1), (4, Color::Red, 1)], 20);
    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
    assert!(!g.is_legal(buy(5, Color::Red)));
    assert!(g.is_legal(buy(4, Color::Red)));
}

#[test]
fn the_ship_capacity_caps_a_harbor_purchase() {
    let (mut g, buyer, seller) = harbor_game(&[(0, Color::Red, 5)], 30);
    set_ship(&mut g, buyer, ColorCounts([0, 0, 0, 2, 0]));
    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
    for _ in 0..3 {
        g.apply(buy(0, Color::Red)).unwrap();
    }

    assert_eq!(ship(&g, buyer).total(), 5);
    assert_eq!(g.state.players[seller].harbor_lots[0][Color::Red], 2);
    assert_eq!(
        g.phase,
        Phase::Main,
        "the ship is full, so the purchase ended"
    );
}

#[test]
fn cash_caps_a_harbor_purchase() {
    let (mut g, buyer, seller) = harbor_game(&[(0, Color::Red, 4)], 5);
    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();
    g.apply(buy(0, Color::Red)).unwrap();

    assert_eq!(cash(&g, buyer), 1);
    assert_eq!(ship(&g, buyer).total(), 2);
    assert_eq!(g.phase, Phase::Main);
}

#[test]
fn arriving_at_a_harbor_with_nothing_to_buy_ends_the_action_at_once() {
    // An empty harbor, a full ship, and too little cash for the cheapest container.
    type Case = (Vec<(usize, Color, u8)>, u32, u8);
    let cases: [Case; 3] = [
        (vec![], 20, 0),
        (vec![(0, Color::Red, 1)], 20, 5),
        (vec![(0, Color::Red, 1)], 1, 0),
    ];
    for (stock, cash, aboard) in cases {
        let (mut g, buyer, seller) = harbor_game(&stock, cash);
        set_ship(&mut g, buyer, ColorCounts([0, 0, 0, 0, aboard]));

        g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();

        assert_eq!(g.phase, Phase::Main, "{stock:?} {cash} {aboard}");
        assert_eq!(g.turn.actions_left, 1);
        assert_eq!(location(&g, buyer), ShipLocation::Harbor(seller));
    }
}

#[test]
fn docked_at_a_harbor_a_purchase_costs_an_action_and_must_buy_something() {
    let (mut g, buyer, seller) = docked_game(&[(0, Color::Red, 2)], 20);
    assert!(g.legal_actions().contains(&Action::HarborPurchase));

    g.apply(Action::HarborPurchase).unwrap();

    assert!(matches!(g.phase, Phase::Buy(_)));
    assert!(
        !g.is_legal(Action::DoneBuying),
        "a paid action must buy something"
    );
    assert_eq!(g.turn.actions_left, 2);

    g.apply(buy(0, Color::Red)).unwrap();
    assert!(g.is_legal(Action::DoneBuying));
    g.apply(Action::DoneBuying).unwrap();

    assert_eq!(ship(&g, buyer), ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(cash(&g, buyer), 18);
    assert_eq!(g.state.players[seller].harbor_lots[0][Color::Red], 1);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_paid_harbor_purchase_needs_something_to_buy() {
    let (mut g, buyer, _) = docked_game(&[(0, Color::Red, 1)], 20);
    assert!(g.is_legal(Action::HarborPurchase));

    set_cash(&mut g, buyer, 1);
    assert!(
        !g.is_legal(Action::HarborPurchase),
        "a $2 container costs more than $1"
    );
    set_cash(&mut g, buyer, 20);
    assert!(g.is_legal(Action::HarborPurchase));

    set_ship(&mut g, buyer, ColorCounts([0, 0, 0, 0, 5]));
    assert!(!g.is_legal(Action::HarborPurchase), "the ship is full");
}

#[test]
fn a_paid_harbor_purchase_needs_a_stocked_harbor() {
    let (g, _, _) = docked_game(&[], 20);
    assert!(!g.is_legal(Action::HarborPurchase));
}

#[test]
fn a_harbor_purchase_needs_a_docked_ship() {
    for place in [ShipLocation::Ocean, ShipLocation::Island] {
        let (mut g, buyer, _) = harbor_game(&[(0, Color::Red, 1)], 20);
        g.state.players[buyer].ship_location = place;
        assert!(!g.is_legal(Action::HarborPurchase), "{place:?}");
    }
}

#[test]
fn other_actions_are_illegal_while_buying_at_a_harbor() {
    let (mut g, _, seller) = harbor_game(&[(0, Color::Red, 2)], 20);
    g.apply(Action::Sail(ShipLocation::Harbor(seller))).unwrap();
    for a in [
        Action::Produce,
        Action::Build(Building::Warehouse),
        Action::Sail(ShipLocation::Ocean),
        Action::HarborPurchase,
        Action::FactoryPurchase(seller),
        Action::Reprice(District::Factory),
        Action::Load(Color::Red),
        Action::DoneLoading,
    ] {
        assert!(!g.is_legal(a), "{a:?}");
    }
}

/// The ship and cash at the end of every complete sequence of purchases.
fn harbor_outcomes(g: &Game, buyer: PlayerId, out: &mut Vec<(ColorCounts, u32)>) {
    match g.phase {
        Phase::Buy(_) => {
            let legal = g.legal_actions();
            assert!(!legal.is_empty(), "dead end while buying");
            for a in legal {
                let mut next = g.clone();
                next.apply(a).unwrap();
                harbor_outcomes(&next, buyer, out);
            }
        }
        _ => out.push((ship(g, buyer), cash(g, buyer))),
    }
}

#[test]
fn every_feasible_harbor_purchase_is_reachable_exactly_once() {
    let stock = [
        (0, Color::Red, 1),
        (1, Color::Red, 1),
        (1, Color::Blue, 1),
        (4, Color::Green, 1),
    ];
    let prices = [2u32, 3, 3, 6];
    for cash in [2, 4, 5, 8, 30] {
        for room in 1..=5u8 {
            for free_anchor in [true, false] {
                let (mut g, buyer, seller) = harbor_game(&stock, cash);
                set_ship(&mut g, buyer, ColorCounts([0, 0, 5 - room, 0, 0]));
                let start = if free_anchor {
                    Action::Sail(ShipLocation::Harbor(seller))
                } else {
                    g.state.players[buyer].ship_location = ShipLocation::Harbor(seller);
                    Action::HarborPurchase
                };
                g.apply(start).unwrap();
                let mut found = Vec::new();
                harbor_outcomes(&g, buyer, &mut found);

                // Every set of the four distinct containers that fits the cash and the room.
                // Buying nothing is only allowed for the free anchor action.
                let expected = (0u32..16)
                    .filter(|mask| {
                        let cost: u32 = (0..4)
                            .filter(|i| mask >> i & 1 == 1)
                            .map(|i| prices[i])
                            .sum();
                        let count = mask.count_ones();
                        count <= u32::from(room) && cost <= cash && (free_anchor || count > 0)
                    })
                    .count();
                assert_eq!(
                    found.len(),
                    expected,
                    "cash {cash}, room {room}, free {free_anchor}"
                );
            }
        }
    }
}

/// The ship and holding area at the end of every complete sequence of loads.
fn load_outcomes(g: &Game, me: PlayerId, out: &mut Vec<(ColorCounts, ColorCounts)>) {
    match g.phase {
        Phase::Load(_) => {
            let legal = g.legal_actions();
            assert!(!legal.is_empty(), "dead end while loading");
            for a in legal {
                let mut next = g.clone();
                next.apply(a).unwrap();
                load_outcomes(&next, me, out);
            }
        }
        _ => out.push((ship(g, me), g.state.players[me].holding)),
    }
}

#[test]
fn every_feasible_load_is_reachable_exactly_once() {
    for room in 1..=5u8 {
        for free_anchor in [true, false] {
            let mut g = game();
            let me = g.current_player();
            set_ship(&mut g, me, ColorCounts([0, 0, 0, 5 - room, 0]));
            set_holding(&mut g, me, ColorCounts([2, 1, 1, 0, 0]));
            let start = if free_anchor {
                Action::Sail(ShipLocation::Bank)
            } else {
                dock(&mut g, ShipLocation::Bank);
                Action::HarborPurchase
            };
            g.apply(start).unwrap();
            let mut found = Vec::new();
            load_outcomes(&g, me, &mut found);

            let mut expected = 0;
            for red in 0..=2u8 {
                for blue in 0..=1u8 {
                    for yellow in 0..=1u8 {
                        let count = red + blue + yellow;
                        if count <= room && (free_anchor || count > 0) {
                            expected += 1;
                        }
                    }
                }
            }
            assert_eq!(found.len(), expected, "room {room}, free {free_anchor}");
        }
    }
}

#[test]
fn random_play_that_favours_ships_keeps_every_invariant() {
    for seed in 0..4 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            let mut rng = Rng::new(seed + 70);
            for _ in 0..800 {
                let Some(legal) = actions_while_playing(&g) else {
                    break;
                };
                // Move containers around: produce, buy at factories, then sail and buy at harbors.
                let preferred: Vec<Action> = legal
                    .iter()
                    .copied()
                    .filter(|a| {
                        matches!(
                            a,
                            Action::Produce
                                | Action::FactoryPurchase(_)
                                | Action::Sail(_)
                                | Action::HarborPurchase
                                | Action::Buy { .. }
                                | Action::Load(_)
                        )
                    })
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
