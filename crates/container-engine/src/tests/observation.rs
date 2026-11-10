use super::delivery::{arrive, at_decision, bid_all, delivery_game};
use super::{
    actions_while_playing, drain_supply, new_game, pass_turn, set_cash, set_cash_lot,
    set_container_lot, set_factories, set_factory_lots, set_holding, set_loans, set_lots,
    set_scoring_area, set_ship, set_warehouses,
};
use crate::action::{Action, District};
use crate::color::{Color, ColorCounts};
use crate::config::{GameLength, MAX_PLAYERS};
use crate::game::Game;
use crate::observation::{BiddingView, Observation, PhaseView, PlayerView};
use crate::phase::{Bidding, Phase, Turn};
use crate::rng::Rng;
use crate::scoring::ScoringCard;
use crate::state::{Auction, AuctionKind, Bid, ContainerBid, PlayerId, PlayerState, ShipLocation};

fn seen(g: &Game, viewer: PlayerId) -> Observation {
    g.observation(viewer).expect("a seat at the table")
}

fn bids_seen(g: &Game, viewer: PlayerId) -> BiddingView {
    match seen(g, viewer).phase {
        PhaseView::Bid(view) => view,
        other => panic!("expected a delivery auction, saw {other:?}"),
    }
}

fn flags(seats: &[PlayerId]) -> [bool; MAX_PLAYERS] {
    let mut flags = [false; MAX_PLAYERS];
    for &seat in seats {
        flags[seat] = true;
    }
    flags
}

fn table(bids: &[(PlayerId, u32)]) -> [u32; MAX_PLAYERS] {
    let mut table = [0; MAX_PLAYERS];
    for &(seat, amount) in bids {
        table[seat] = amount;
    }
    table
}

fn some(seat: usize, shift: usize, n: u8) -> ColorCounts {
    let mut counts = ColorCounts::default();
    counts[Color::ALL[(seat + shift) % 5]] = n;
    counts
}

/// Every player owns something different in every place, so that no view can mix two places up.
fn a_busy_table() -> Game {
    let mut g = new_game(4, GameLength::Standard, 3);
    let docks = [
        ShipLocation::Ocean,
        ShipLocation::Island,
        ShipLocation::Bank,
        ShipLocation::Harbor(1),
    ];
    for (p, dock) in docks.into_iter().enumerate() {
        let k = p as u8;
        set_cash(&mut g, p, 12 + 5 * u32::from(k));
        set_factories(&mut g, p, &[Color::ALL[p], Color::ALL[p + 1]]);
        set_warehouses(&mut g, p, k + 1);
        set_factory_lots(
            &mut g,
            p,
            &[
                (p, Color::ALL[p], 1),
                ((p + 1) % 4, Color::ALL[(p + 3) % 5], 2),
            ],
        );
        set_lots(
            &mut g,
            p,
            District::Harbor,
            &[((p + 2) % 5, Color::ALL[(p + 4) % 5], 1 + k)],
        );
        set_ship(&mut g, p, some(p, 1, 2));
        set_holding(&mut g, p, some(p, 2, 1));
        set_scoring_area(&mut g, p, some(p, 3, 3));
        set_loans(&mut g, p, k);
        let player = &mut g.state.players[p];
        player.ship_location = dock;
        player.reserve_factory = k + 2;
        player.reserve_harbor = k + 3;
    }
    g
}

fn assert_on_the_table(view: &PlayerView, p: &PlayerState, context: &str) {
    assert_eq!(view.loans, p.loans, "{context}: loans");
    assert_eq!(view.factories, p.factories, "{context}: factories");
    assert_eq!(view.warehouses, p.warehouses, "{context}: warehouses");
    assert_eq!(view.factory_lots, p.factory_lots, "{context}: factory lots");
    assert_eq!(view.harbor_lots, p.harbor_lots, "{context}: harbor lots");
    assert_eq!(
        view.ship_location, p.ship_location,
        "{context}: ship location"
    );
    assert_eq!(view.ship, p.ship, "{context}: ship");
    assert_eq!(view.holding, p.holding, "{context}: holding area");
    assert_eq!(view.scoring_area, p.scoring_area, "{context}: scoring area");
    assert_eq!(
        view.reserve_factory, p.reserve_factory,
        "{context}: factory reserve"
    );
    assert_eq!(
        view.reserve_harbor, p.reserve_harbor,
        "{context}: harbor reserve"
    );
}

// Who is looking

#[test]
fn only_the_seats_at_the_table_can_observe() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        for seat in 0..players {
            assert!(g.observation(seat).is_some(), "seat {seat} of {players}");
        }
        assert!(g.observation(players).is_none());
        assert!(g.observation(usize::MAX).is_none());
    }
}

#[test]
fn an_observation_says_who_it_is_for_who_is_active_and_who_must_decide() {
    let (mut g, deliverer) = delivery_game(4, 10);
    let o = arrive(&mut g);

    for viewer in 0..4 {
        let obs = seen(&g, viewer);
        assert_eq!(obs.viewer, viewer);
        assert_eq!(obs.num_players, 4);
        assert_eq!(obs.active_player, deliverer);
        assert_eq!(obs.current_player, o[0]);
        assert_eq!(obs.players.len(), 4);
    }
}

#[test]
fn an_observation_names_the_first_player_apart_from_the_active_one() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let first = g.state.first_player;
    pass_turn(&mut g);

    for viewer in 0..3 {
        let obs = seen(&g, viewer);
        assert_eq!(obs.first_player, first);
        assert_eq!(obs.active_player, (first + 1) % 3);
    }
}

// Hands

#[test]
fn a_player_sees_their_own_cash_and_scoring_card() {
    let mut g = new_game(4, GameLength::Standard, 1);
    for p in 0..4 {
        set_cash(&mut g, p, 11 + 3 * p as u32);
    }

    for viewer in 0..4 {
        let view = seen(&g, viewer).players[viewer];
        assert_eq!(view.cash, Some(11 + 3 * viewer as u32));
        assert_eq!(
            view.scoring_card,
            Some(g.state.players[viewer].scoring_card)
        );
    }
}

#[test]
fn a_player_cannot_see_the_cash_or_the_scoring_card_of_the_others() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        for viewer in 0..players {
            let obs = seen(&g, viewer);
            assert_eq!(obs.players.len(), players);
            for (seat, view) in obs.players.iter().enumerate() {
                if seat != viewer {
                    assert_eq!(view.cash, None, "cash of {seat} seen by {viewer}");
                    assert_eq!(view.scoring_card, None, "card of {seat} seen by {viewer}");
                }
            }
        }
    }
}

#[test]
fn the_cash_and_the_scoring_cards_are_revealed_once_the_game_is_over() {
    let mut g = new_game(4, GameLength::Standard, 1);
    for p in 0..4 {
        set_cash(&mut g, p, 5 + p as u32);
    }
    g.phase = Phase::GameOver;

    for viewer in 0..4 {
        let obs = seen(&g, viewer);
        assert_eq!(obs.phase, PhaseView::Open(Phase::GameOver));
        for (seat, view) in obs.players.iter().enumerate() {
            assert_eq!(view.cash, Some(5 + seat as u32), "cash of {seat}");
            assert_eq!(
                view.scoring_card,
                Some(g.state.players[seat].scoring_card),
                "card of {seat}"
            );
        }
    }
}

// The table

#[test]
fn the_board_of_every_player_is_public() {
    let g = a_busy_table();

    for viewer in 0..4 {
        let obs = seen(&g, viewer);
        for (seat, view) in obs.players.iter().enumerate() {
            let context = format!("player {seat} seen by {viewer}");
            assert_on_the_table(view, &g.state.players[seat], &context);
        }
    }
}

#[test]
fn the_bank_the_supply_and_the_turn_are_public() {
    let mut g = a_busy_table();
    set_container_lot(&mut g, 0, ColorCounts([1, 0, 2, 0, 0]));
    set_container_lot(&mut g, 2, ColorCounts([0, 3, 0, 0, 1]));
    set_cash_lot(&mut g, 1, 7);
    g.state.bank.auctions[AuctionKind::ContainerLot] = Some(Auction {
        lot: 1,
        holder: 2,
        bid: Bid::Cash(4),
    });
    g.state.bank.auctions[AuctionKind::CashLot] = Some(Auction {
        lot: 2,
        holder: 3,
        bid: Bid::Containers(ContainerBid {
            factory: ColorCounts([0, 0, 1, 0, 0]),
            harbor: ColorCounts([0, 1, 0, 0, 1]),
        }),
    });
    g.turn = Turn {
        actions_left: 1,
        produced: true,
        loans_requested: 2,
        loans_repaid: 1,
        bank_called: true,
        won_auction: true,
    };

    for viewer in 0..4 {
        let obs = seen(&g, viewer);
        assert_eq!(obs.bank, g.state.bank);
        assert_eq!(obs.supply, g.state.supply);
        assert_eq!(obs.turn, g.turn);
    }
}

#[test]
fn everybody_knows_when_it_is_the_last_turn() {
    let mut g = new_game(3, GameLength::Standard, 1);
    for viewer in 0..3 {
        assert!(!seen(&g, viewer).last_turn);
    }

    drain_supply(&mut g, Color::Red);
    for viewer in 0..3 {
        assert!(
            !seen(&g, viewer).last_turn,
            "one empty color is not the end"
        );
    }

    drain_supply(&mut g, Color::Blue);
    for viewer in 0..3 {
        assert!(seen(&g, viewer).last_turn);
    }
}

// Delivery bids

#[test]
fn a_bid_is_face_down_for_everybody_still_to_bid_and_for_the_deliverer() {
    let (mut g, deliverer) = delivery_game(4, 10);
    let o = arrive(&mut g);
    for viewer in 0..4 {
        let view = bids_seen(&g, viewer);
        assert_eq!(view.deliverer, deliverer);
        assert_eq!(view.bidder, o[0]);
        assert_eq!(view.contenders, flags(&o));
        assert!(!view.runoff);
        assert_eq!(view.base, 0);
        assert_eq!(view.placed, flags(&[]));
    }

    bid_all(&mut g, &[3]);
    for viewer in [o[1], o[2], deliverer] {
        let view = bids_seen(&g, viewer);
        assert_eq!(view.bidder, o[1]);
        assert_eq!(view.bids, table(&[]), "viewer {viewer}");
        assert_eq!(view.placed, flags(&[o[0]]), "viewer {viewer}");
    }

    bid_all(&mut g, &[4]);
    for viewer in [o[2], deliverer] {
        let view = bids_seen(&g, viewer);
        assert_eq!(view.bidder, o[2]);
        assert_eq!(view.bids, table(&[]), "viewer {viewer}");
        assert_eq!(view.placed, flags(&[o[0], o[1]]), "viewer {viewer}");
    }
}

#[test]
fn a_player_sees_their_own_face_down_bid() {
    let (mut g, _) = delivery_game(4, 10);
    let o = arrive(&mut g);
    bid_all(&mut g, &[3, 4]);

    assert_eq!(bids_seen(&g, o[0]).bids, table(&[(o[0], 3)]));
    assert_eq!(bids_seen(&g, o[1]).bids, table(&[(o[1], 4)]));
    assert_eq!(bids_seen(&g, o[2]).bids, table(&[]));
}

#[test]
fn what_others_bid_face_down_makes_no_difference_to_what_a_seat_sees() {
    let (mut low, deliverer) = delivery_game(4, 10);
    let (mut high, _) = delivery_game(4, 10);
    let o = arrive(&mut low);
    arrive(&mut high);

    bid_all(&mut low, &[3]);
    bid_all(&mut high, &[7]);
    for viewer in [o[1], o[2], deliverer] {
        assert_eq!(seen(&low, viewer), seen(&high, viewer), "viewer {viewer}");
    }
    assert_ne!(seen(&low, o[0]), seen(&high, o[0]));

    bid_all(&mut low, &[4]);
    bid_all(&mut high, &[1]);
    for viewer in [o[2], deliverer] {
        assert_eq!(seen(&low, viewer), seen(&high, viewer), "viewer {viewer}");
    }
    assert_ne!(seen(&low, o[1]), seen(&high, o[1]));
}

#[test]
fn the_revealed_bids_of_a_runoff_are_public_but_not_what_is_added_to_them() {
    let (mut g, deliverer, o) = at_decision(4, 10, &[5, 2, 5]);
    for viewer in 0..4 {
        let view = bids_seen(&g, viewer);
        assert!(view.runoff);
        assert_eq!(view.bidder, o[0]);
        assert_eq!(view.contenders, flags(&[o[0], o[2]]));
        assert_eq!(view.base, 5, "viewer {viewer}");
        assert_eq!(view.bids, table(&[(o[0], 5), (o[2], 5)]), "viewer {viewer}");
        assert_eq!(view.placed, flags(&[]), "viewer {viewer}");
    }

    bid_all(&mut g, &[2]);
    for viewer in [o[1], o[2], deliverer] {
        let view = bids_seen(&g, viewer);
        assert_eq!(view.bidder, o[2]);
        assert_eq!(view.bids, table(&[(o[0], 5), (o[2], 5)]), "viewer {viewer}");
        assert_eq!(view.placed, flags(&[o[0]]), "viewer {viewer}");
    }
    assert_eq!(
        bids_seen(&g, o[0]).bids,
        table(&[(o[0], 7), (o[2], 5)]),
        "the bidder knows what they added"
    );
}

#[test]
fn what_is_added_face_down_in_a_runoff_makes_no_difference_to_what_a_seat_sees() {
    let (mut low, deliverer, o) = at_decision(4, 10, &[5, 2, 5]);
    let (mut high, _, _) = at_decision(4, 10, &[5, 2, 5]);

    bid_all(&mut low, &[2]);
    bid_all(&mut high, &[5]);

    for viewer in [o[1], o[2], deliverer] {
        assert_eq!(seen(&low, viewer), seen(&high, viewer), "viewer {viewer}");
    }
    assert_ne!(seen(&low, o[0]), seen(&high, o[0]));
}

// Whole games

/// The seats that have already bid in the current round of a delivery auction: the contenders from the
/// deliverer's left up to the bidder.
fn bid_already(g: &Game, bidding: &Bidding) -> Vec<PlayerId> {
    let n = g.state.num_players;
    let mut seats = Vec::new();
    let mut seat = (bidding.deliverer + 1) % n;
    while seat != bidding.bidder {
        if bidding.contenders[seat] {
            seats.push(seat);
        }
        seat = (seat + 1) % n;
    }
    seats
}

/// The same game with everything that the other seats keep from `viewer` changed: their cash, their
/// scoring cards, the cash they have face down on the table, and the seed that dealt it all.
fn with_other_hands_changed(g: &Game, viewer: PlayerId) -> Game {
    let mut other = g.clone();
    other.config.seed ^= 0x5eed;
    let seats: Vec<PlayerId> = (0..g.state.num_players)
        .filter(|&seat| seat != viewer)
        .collect();
    let cards: Vec<ScoringCard> = seats
        .iter()
        .map(|&seat| g.state.players[seat].scoring_card)
        .collect();
    for (i, &seat) in seats.iter().enumerate() {
        other.state.players[seat].cash += 7;
        other.state.players[seat].scoring_card = cards[(i + 1) % cards.len()];
    }
    if let Phase::Bid(mut bidding) = g.phase {
        for &seat in &seats {
            if bidding.contenders[seat] {
                bidding.bids[seat] += 5;
            }
        }
        other.phase = Phase::Bid(bidding);
    }
    other
}

#[derive(Default)]
struct Coverage {
    face_down_bids: u32,
    face_down_runoff_bids: u32,
    finished: u32,
}

fn check_every_view(g: &Game, coverage: &mut Coverage) {
    let n = g.state.num_players;
    for viewer in 0..n {
        let obs = seen(g, viewer);
        assert_eq!(obs.viewer, viewer);
        assert_eq!(obs.num_players, n);
        assert_eq!(obs.first_player, g.state.first_player);
        assert_eq!(obs.active_player, g.state.active_player);
        assert_eq!(obs.current_player, g.current_player());
        assert_eq!(obs.bank, g.state.bank);
        assert_eq!(obs.supply, g.state.supply);
        assert_eq!(obs.turn, g.turn);
        assert_eq!(obs.last_turn, g.is_last_turn());
        assert_eq!(obs.players.len(), n);
        for (seat, view) in obs.players.iter().enumerate() {
            assert_on_the_table(view, &g.state.players[seat], &format!("player {seat}"));
            let hand_is_known = seat == viewer || g.is_terminal();
            let cash = hand_is_known.then_some(g.state.players[seat].cash);
            let card = hand_is_known.then_some(g.state.players[seat].scoring_card);
            assert_eq!(view.cash, cash, "cash of {seat} seen by {viewer}");
            assert_eq!(view.scoring_card, card, "card of {seat} seen by {viewer}");
        }
        match g.phase {
            Phase::Bid(bidding) => {
                let PhaseView::Bid(view) = obs.phase else {
                    panic!(
                        "a delivery auction must be seen as one, saw {:?}",
                        obs.phase
                    );
                };
                assert_eq!(view.deliverer, bidding.deliverer);
                assert_eq!(view.bidder, bidding.bidder);
                assert_eq!(view.contenders, bidding.contenders);
                assert_eq!(view.runoff, bidding.runoff);
                assert_eq!(view.base, bidding.base);
                let placed = bid_already(g, &bidding);
                assert_eq!(view.placed, flags(&placed));
                for seat in 0..MAX_PLAYERS {
                    let on_the_table = if seat == viewer {
                        bidding.bids[seat]
                    } else if bidding.contenders[seat] {
                        bidding.base
                    } else {
                        0
                    };
                    assert_eq!(
                        view.bids[seat], on_the_table,
                        "bid of {seat} seen by {viewer}"
                    );
                }
                if placed.iter().any(|&seat| seat != viewer) {
                    if bidding.runoff {
                        coverage.face_down_runoff_bids += 1;
                    } else {
                        coverage.face_down_bids += 1;
                    }
                }
            }
            phase => assert_eq!(obs.phase, PhaseView::Open(phase)),
        }

        if g.is_terminal() {
            coverage.finished += 1;
            continue;
        }
        let changed = with_other_hands_changed(g, viewer);
        assert_eq!(
            obs,
            seen(&changed, viewer),
            "seat {viewer} sees what the others keep to themselves in {:?}",
            g.phase
        );
        let mut richer = g.clone();
        richer.state.players[viewer].cash += 1;
        assert_ne!(
            obs,
            seen(&richer, viewer),
            "seat {viewer} sees its own cash"
        );
    }
}

#[test]
fn random_play_never_shows_a_seat_what_the_others_keep_to_themselves() {
    let mut coverage = Coverage::default();
    for seed in 0..6 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            let mut rng = Rng::new(seed + 130);
            for _ in 0..1500 {
                check_every_view(&g, &mut coverage);
                let Some(legal) = actions_while_playing(&g) else {
                    break;
                };
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
                                | Action::Accept(_)
                                | Action::BuyOut
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
            }
            check_every_view(&g, &mut coverage);
        }
    }
    assert!(coverage.face_down_bids > 0, "no bid was ever face down");
    assert!(
        coverage.face_down_runoff_bids > 0,
        "no runoff bid was ever face down"
    );
    assert!(coverage.finished > 0, "no game ever finished");
}
