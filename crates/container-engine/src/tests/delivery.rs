use super::{actions_while_playing, new_game, pass_turn, set_cash, set_ship};
use crate::action::{Action, Building, District};
use crate::color::{Color, ColorCounts};
use crate::config::{GameLength, MAX_PLAYERS};
use crate::game::Game;
use crate::phase::{Bidding, Phase};
use crate::rng::Rng;
use crate::state::{PlayerId, ShipLocation};

const CARGO: ColorCounts = ColorCounts([2, 1, 0, 0, 0]);

fn cash(g: &Game, p: PlayerId) -> u32 {
    g.state.players[p].cash
}

fn bank_cash(g: &Game) -> [u32; 3] {
    g.state.bank.cash_lots
}

/// What the auction offers: the legal actions other than taking a loan, which is always on the side.
pub(super) fn choices(g: &Game) -> Vec<Action> {
    g.legal_actions()
        .into_iter()
        .filter(|a| *a != Action::RequestLoan)
        .collect()
}

fn bidding(g: &Game) -> bool {
    matches!(g.phase, Phase::Bid(_))
}

fn runoff(g: &Game) -> bool {
    matches!(g.phase, Phase::Bid(b) if b.runoff)
}

fn deciding(g: &Game) -> bool {
    matches!(g.phase, Phase::Decide(_))
}

fn dock(g: &mut Game, place: ShipLocation) {
    let me = g.current_player();
    g.state.players[me].ship_location = place;
}

/// The opponents in the order they bid: clockwise, starting with the seat to the deliverer's left.
fn order(me: PlayerId, players: usize) -> Vec<PlayerId> {
    (1..players).map(|k| (me + k) % players).collect()
}

/// The active player has `CARGO` on a ship in the ocean, and every player has `start` cash.
pub(super) fn delivery_game(players: usize, start: u32) -> (Game, PlayerId) {
    delivery_game_after(players, start, 0)
}

/// As `delivery_game`, but `turns` turns go by first: the deliverer is that many seats on from the
/// player who started.
pub(super) fn delivery_game_after(players: usize, start: u32, turns: usize) -> (Game, PlayerId) {
    let mut g = new_game(players, GameLength::Standard, 1);
    for _ in 0..turns {
        pass_turn(&mut g);
    }
    let me = g.current_player();
    set_ship(&mut g, me, CARGO);
    for p in 0..players {
        set_cash(&mut g, p, start);
    }
    (g, me)
}

/// Sails to the Island and returns the opponents in bidding order.
pub(super) fn arrive(g: &mut Game) -> Vec<PlayerId> {
    let me = g.current_player();
    g.apply(Action::Sail(ShipLocation::Island)).unwrap();
    order(me, g.state.num_players)
}

/// Places the bids of the next players due to bid, in the order the game asks.
pub(super) fn bid_all(g: &mut Game, bids: &[u32]) {
    for &amount in bids {
        assert!(bidding(g), "a bid was due");
        g.apply(Action::Bid(amount)).unwrap();
    }
}

/// Everyone has `start` cash and bids as given, leaving the game at the deliverer's decision.
pub(super) fn at_decision(
    players: usize,
    start: u32,
    bids: &[u32],
) -> (Game, PlayerId, Vec<PlayerId>) {
    let (mut g, me) = delivery_game(players, start);
    let opponents = arrive(&mut g);
    bid_all(&mut g, bids);
    (g, me, opponents)
}

// Reaching the Island

#[test]
fn the_island_cannot_be_reached_with_an_empty_ship() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = g.current_player();
    assert!(!g.is_legal(Action::Sail(ShipLocation::Island)));
    assert!(g.apply(Action::Sail(ShipLocation::Island)).is_err());

    set_ship(&mut g, me, CARGO);
    assert!(g.is_legal(Action::Sail(ShipLocation::Island)));
}

#[test]
fn a_single_container_is_enough_to_deliver() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = g.current_player();
    set_ship(&mut g, me, ColorCounts([0, 0, 0, 0, 1]));
    assert!(g.is_legal(Action::Sail(ShipLocation::Island)));
}

#[test]
fn the_island_is_only_reached_from_the_ocean() {
    let other = (new_game(3, GameLength::Standard, 1).current_player() + 1) % 3;
    for place in [
        ShipLocation::Bank,
        ShipLocation::Island,
        ShipLocation::Harbor(other),
    ] {
        let (mut g, _) = delivery_game(3, 10);
        dock(&mut g, place);
        assert!(!g.is_legal(Action::Sail(ShipLocation::Island)), "{place:?}");
    }
}

#[test]
fn arriving_starts_a_delivery_auction_with_the_cargo_still_aboard() {
    let (mut g, me) = delivery_game(3, 10);

    g.apply(Action::Sail(ShipLocation::Island)).unwrap();

    assert_eq!(g.state.players[me].ship_location, ShipLocation::Island);
    assert_eq!(g.state.players[me].ship, CARGO);
    assert!(bidding(&g));
    assert!(!runoff(&g));
    assert_eq!(g.check_invariants(), Ok(()));
}

// Bidding

#[test]
fn opponents_bid_one_after_the_other_clockwise_from_the_deliverers_left() {
    for players in 3..=5 {
        let (mut g, me) = delivery_game(players, 10);
        let expected = arrive(&mut g);
        let mut asked = Vec::new();
        for amount in 0..players - 1 {
            asked.push(g.current_player());
            // Distinct bids, so that nobody ties.
            g.apply(Action::Bid(amount as u32)).unwrap();
        }
        assert_eq!(asked, expected, "{players} players");
        assert!(!asked.contains(&me));
        assert!(deciding(&g));
    }
}

#[test]
fn a_bid_is_any_amount_up_to_the_cash_in_hand() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();
    set_cash(&mut g, bidder, 7);

    let expected: Vec<Action> = (0..=7).map(Action::Bid).collect();
    assert_eq!(choices(&g), expected);
}

#[test]
fn a_bid_above_the_cash_in_hand_is_rejected() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();
    set_cash(&mut g, bidder, 7);
    let before = g.clone();

    assert!(g.apply(Action::Bid(8)).is_err());
    assert_eq!(g, before);
}

#[test]
fn a_player_without_cash_can_only_bid_nothing() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();
    set_cash(&mut g, bidder, 0);

    assert_eq!(choices(&g), vec![Action::Bid(0)]);
}

#[test]
fn nothing_but_bids_is_legal_while_bidding() {
    let (mut g, me) = delivery_game(3, 10);
    let opponents = arrive(&mut g);
    for action in [
        Action::Produce,
        Action::Build(Building::Warehouse),
        Action::Build(Building::Factory(Color::Green)),
        Action::FactoryPurchase(opponents[0]),
        Action::HarborPurchase,
        Action::Sail(ShipLocation::Ocean),
        Action::Reprice(District::Factory),
        Action::Place(0),
        Action::DoneBuying,
        Action::DoneLoading,
        Action::Accept(opponents[0]),
        Action::BuyOut,
    ] {
        assert!(!g.is_legal(action), "{action:?}");
    }
    assert!(choices(&g).iter().all(|a| matches!(a, Action::Bid(_))));
    assert_ne!(g.current_player(), me);
}

#[test]
fn a_bid_leaves_the_hand_and_stays_on_the_table() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();

    g.apply(Action::Bid(4)).unwrap();

    assert_eq!(cash(&g, bidder), 6);
    assert_eq!(g.check_invariants(), Ok(()), "the bid is still in play");
}

#[test]
fn a_bid_is_final_and_the_next_opponent_bids_at_once() {
    let (mut g, _) = delivery_game(3, 10);
    let o = arrive(&mut g);

    g.apply(Action::Bid(5)).unwrap();

    assert_eq!(g.current_player(), o[1]);
    assert_eq!(choices(&g), (0..=10).map(Action::Bid).collect::<Vec<_>>());
    assert_eq!(cash(&g, o[0]), 5, "the first bid cannot be taken back");
    assert_eq!(g.check_invariants(), Ok(()));
}

// The winner

#[test]
fn the_highest_bid_wins_and_everyone_else_gets_their_bid_back() {
    let (g, me, o) = at_decision(3, 10, &[4, 6]);

    assert!(deciding(&g));
    assert_eq!(g.current_player(), me);
    assert_eq!(cash(&g, o[0]), 10, "the lower bid is returned");
    assert_eq!(cash(&g, o[1]), 4, "the winning bid stays on the table");
    assert_eq!(choices(&g), vec![Action::Accept(o[1]), Action::BuyOut]);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_deliverer_decides_and_nobody_can_pick_another_winner() {
    let (g, me, o) = at_decision(3, 10, &[4, 6]);

    for action in [
        Action::Accept(o[0]),
        Action::Accept(me),
        Action::Bid(0),
        Action::Produce,
        Action::Sail(ShipLocation::Ocean),
        Action::Reprice(District::Factory),
    ] {
        assert!(!g.is_legal(action), "{action:?}");
    }
}

// Accepting

#[test]
fn accepting_pays_the_deliverer_the_bid_and_an_equal_subsidy() {
    let (mut g, me, o) = at_decision(3, 10, &[4, 6]);
    let minted = g.state.cash_minted;

    g.apply(Action::Accept(o[1])).unwrap();

    assert_eq!(cash(&g, me), 10 + 6 + 6);
    assert_eq!(cash(&g, o[1]), 4);
    assert_eq!(cash(&g, o[0]), 10);
    assert_eq!(
        g.state.cash_minted - minted,
        6,
        "the subsidy comes from the supply"
    );
    assert_eq!(bank_cash(&g), [1, 2, 3], "the Bank is not involved");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn accepting_delivers_the_cargo_to_the_winners_scoring_area() {
    let (mut g, me, o) = at_decision(3, 10, &[4, 6]);

    g.apply(Action::Accept(o[1])).unwrap();

    assert_eq!(g.state.players[o[1]].scoring_area, CARGO);
    assert!(g.state.players[me].ship.is_empty());
    assert!(g.state.players[me].scoring_area.is_empty());
    assert!(g.state.players[o[0]].scoring_area.is_empty());
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_turn_ends_right_after_the_auction() {
    for buy_out in [false, true] {
        let (mut g, me, o) = at_decision(3, 10, &[4, 6]);
        g.apply(if buy_out {
            Action::BuyOut
        } else {
            Action::Accept(o[1])
        })
        .unwrap();

        assert_eq!(g.current_player(), (me + 1) % 3, "buy out: {buy_out}");
        assert_eq!(g.phase, Phase::Main);
        assert_eq!(g.turn.actions_left, 2);
        assert!(!g.turn.produced);
    }
}

#[test]
fn the_ship_stays_at_the_island_until_it_sails_back() {
    let (mut g, me, o) = at_decision(3, 10, &[4, 6]);
    g.apply(Action::Accept(o[1])).unwrap();
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), me);
    assert_eq!(g.state.players[me].ship_location, ShipLocation::Island);
    let sails: Vec<Action> = g
        .legal_actions()
        .into_iter()
        .filter(|a| matches!(a, Action::Sail(_)))
        .collect();
    assert_eq!(sails, vec![Action::Sail(ShipLocation::Ocean)]);
}

// Buying out

#[test]
fn buying_out_keeps_the_cargo_and_pays_the_bank() {
    let (mut g, me, o) = at_decision(3, 10, &[4, 6]);
    let minted = g.state.cash_minted;

    g.apply(Action::BuyOut).unwrap();

    assert_eq!(g.state.players[me].scoring_area, CARGO);
    assert!(g.state.players[me].ship.is_empty());
    assert_eq!(cash(&g, me), 10 - 6);
    assert_eq!(cash(&g, o[1]), 10, "the highest bidder gets the bid back");
    assert_eq!(cash(&g, o[0]), 10);
    assert!(g.state.players[o[1]].scoring_area.is_empty());
    assert_eq!(
        bank_cash(&g),
        [3, 4, 5],
        "6 spread evenly over the cash lots"
    );
    assert_eq!(g.state.cash_minted, minted, "no subsidy for a buyout");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_buyout_payment_is_dealt_out_from_lot_one() {
    let (mut g, _, _) = at_decision(3, 10, &[4, 2]);

    g.apply(Action::BuyOut).unwrap();

    assert_eq!(bank_cash(&g), [3, 3, 4]);
}

#[test]
fn a_buyout_needs_the_cash_for_the_winning_bid() {
    for (deliverer_cash, buyout_legal) in [(5, false), (6, true)] {
        let (mut g, me) = delivery_game(3, 10);
        set_cash(&mut g, me, deliverer_cash);
        let o = arrive(&mut g);
        bid_all(&mut g, &[4, 6]);

        let mut expected = vec![Action::Accept(o[1])];
        if buyout_legal {
            expected.push(Action::BuyOut);
        }
        assert_eq!(choices(&g), expected, "cash {deliverer_cash}");
    }
}

// Ties and the runoff

#[test]
fn a_tie_starts_a_runoff_that_begins_afresh() {
    let (g, me, o) = at_decision(3, 10, &[5, 5]);

    let mut tied = [false; MAX_PLAYERS];
    tied[o[0]] = true;
    tied[o[1]] = true;
    assert_eq!(g.phase, Phase::Bid(Bidding::runoff(me, o[0], tied, 5)));
    assert_eq!((cash(&g, o[0]), cash(&g, o[1])), (5, 5));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_tie_for_the_highest_bid_starts_a_runoff_between_the_tied_bidders_only() {
    let (mut g, _, o) = at_decision(4, 10, &[5, 2, 5]);

    assert!(runoff(&g));
    assert_eq!(g.current_player(), o[0]);
    g.apply(Action::Bid(0)).unwrap();
    assert_eq!(g.current_player(), o[2], "the third bidder sits out");
    assert_eq!(cash(&g, o[1]), 10, "the lower bid was returned");
    assert_eq!(cash(&g, o[0]), 5, "tied bids stay on the table");
    assert_eq!(cash(&g, o[2]), 5);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn runoff_bids_are_added_from_the_cash_still_in_hand() {
    let (g, _, _) = at_decision(4, 10, &[5, 2, 5]);

    let expected: Vec<Action> = (0..=5).map(Action::Bid).collect();
    assert_eq!(choices(&g), expected);
}

#[test]
fn the_highest_total_wins_the_runoff() {
    let (mut g, me, o) = at_decision(3, 10, &[5, 5]);
    assert!(runoff(&g));
    let minted = g.state.cash_minted;

    bid_all(&mut g, &[2, 1]);

    assert!(deciding(&g));
    assert_eq!(choices(&g), vec![Action::Accept(o[0]), Action::BuyOut]);
    assert_eq!(cash(&g, o[0]), 10 - 7);
    assert_eq!(cash(&g, o[1]), 10, "the runoff loser gets everything back");

    g.apply(Action::Accept(o[0])).unwrap();
    assert_eq!(cash(&g, me), 10 + 7 + 7);
    assert_eq!(g.state.cash_minted - minted, 7);
    assert_eq!(g.state.players[o[0]].scoring_area, CARGO);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_tie_after_the_runoff_lets_the_deliverer_pick_the_winner() {
    let (mut g, me, o) = at_decision(3, 10, &[5, 5]);
    bid_all(&mut g, &[1, 1]);

    assert!(deciding(&g), "there is only one runoff");
    let mut expected = vec![Action::Accept(o[0]), Action::Accept(o[1])];
    expected.sort_by_key(|a| match a {
        Action::Accept(p) => *p,
        _ => unreachable!(),
    });
    expected.push(Action::BuyOut);
    assert_eq!(choices(&g), expected);

    g.apply(Action::Accept(o[1])).unwrap();
    assert_eq!(cash(&g, o[1]), 10 - 6);
    assert_eq!(cash(&g, o[0]), 10);
    assert_eq!(cash(&g, me), 10 + 6 + 6);
    assert_eq!(g.state.players[o[1]].scoring_area, CARGO);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_buyout_after_a_tie_returns_every_tied_bid() {
    let (mut g, me, o) = at_decision(3, 10, &[5, 5]);
    bid_all(&mut g, &[1, 1]);

    g.apply(Action::BuyOut).unwrap();

    assert_eq!(cash(&g, o[0]), 10);
    assert_eq!(cash(&g, o[1]), 10);
    assert_eq!(cash(&g, me), 10 - 6);
    assert_eq!(bank_cash(&g), [3, 4, 5]);
    assert_eq!(g.state.players[me].scoring_area, CARGO);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn bids_of_nothing_all_round_are_a_tie() {
    let (mut g, me, o) = at_decision(3, 10, &[0, 0]);
    assert!(runoff(&g));
    bid_all(&mut g, &[0, 0]);
    assert!(deciding(&g));
    assert!(choices(&g).contains(&Action::Accept(o[0])));
    assert!(choices(&g).contains(&Action::Accept(o[1])));

    let minted = g.state.cash_minted;
    g.apply(Action::Accept(o[0])).unwrap();

    assert_eq!(
        cash(&g, me),
        10,
        "a zero bid pays nothing, and so does the subsidy"
    );
    assert_eq!(g.state.cash_minted, minted);
    assert_eq!(g.state.players[o[0]].scoring_area, CARGO);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_buyout_at_zero_costs_nothing() {
    let (mut g, me, _) = at_decision(3, 10, &[0, 0]);
    bid_all(&mut g, &[0, 0]);
    assert!(g.is_legal(Action::BuyOut));

    g.apply(Action::BuyOut).unwrap();

    assert_eq!(cash(&g, me), 10);
    assert_eq!(bank_cash(&g), [1, 2, 3]);
    assert_eq!(g.state.players[me].scoring_area, CARGO);
}

#[test]
fn a_three_way_tie_runs_off_between_all_three() {
    let (mut g, _, o) = at_decision(5, 10, &[3, 3, 3, 1]);

    assert!(runoff(&g));
    let mut asked = Vec::new();
    for _ in 0..3 {
        asked.push(g.current_player());
        g.apply(Action::Bid(0)).unwrap();
    }
    assert_eq!(asked, vec![o[0], o[1], o[2]]);
    assert_eq!(
        cash(&g, o[3]),
        10,
        "the fourth bidder was returned their bid"
    );
    assert!(deciding(&g));
    assert_eq!(choices(&g).len(), 4, "three candidates and a buyout");
}

// Every combination of bids, against the rules written out independently

/// Every list with entries in `0..=limit` for each limit.
fn product(limits: &[u32]) -> Vec<Vec<u32>> {
    let mut all = vec![Vec::new()];
    for &limit in limits {
        all = all
            .into_iter()
            .flat_map(|prefix| {
                (0..=limit).map(move |amount| {
                    let mut next = prefix.clone();
                    next.push(amount);
                    next
                })
            })
            .collect();
    }
    all
}

/// The seats holding the highest total, in seat order, and that total.
fn leaders(totals: &[(PlayerId, u32)]) -> (Vec<PlayerId>, u32) {
    let top = totals.iter().map(|&(_, total)| total).max().unwrap();
    let mut seats: Vec<PlayerId> = totals
        .iter()
        .filter(|&&(_, total)| total == top)
        .map(|&(seat, _)| seat)
        .collect();
    seats.sort_unstable();
    (seats, top)
}

struct Table {
    me: PlayerId,
    opponents: Vec<PlayerId>,
    start: u32,
}

fn expect_decision(g: &Game, table: &Table, winners: &[PlayerId], top: u32, context: &str) {
    assert!(deciding(g), "{context}");
    assert_eq!(g.current_player(), table.me, "{context}");
    let mut expected: Vec<Action> = winners.iter().map(|&w| Action::Accept(w)).collect();
    if cash(g, table.me) >= top {
        expected.push(Action::BuyOut);
    }
    assert_eq!(choices(g), expected, "{context}");
    for &p in &table.opponents {
        let on_table = if winners.contains(&p) { top } else { 0 };
        assert_eq!(cash(g, p), table.start - on_table, "{context}: seat {p}");
    }
    assert_eq!(g.check_invariants(), Ok(()), "{context}");
}

fn check_every_outcome(players: usize, start: u32, deliverer_cash: u32) {
    let (mut base, me) = delivery_game(players, start);
    set_cash(&mut base, me, deliverer_cash);
    let opponents = arrive(&mut base);
    let table = Table {
        me,
        opponents: opponents.clone(),
        start,
    };

    for first in product(&vec![start; opponents.len()]) {
        let mut g = base.clone();
        bid_all(&mut g, &first);
        let totals: Vec<(PlayerId, u32)> = opponents
            .iter()
            .copied()
            .zip(first.iter().copied())
            .collect();
        let (tied, top) = leaders(&totals);
        let context = format!("{players} players, bids {first:?}");
        if tied.len() == 1 {
            expect_decision(&g, &table, &tied, top, &context);
            continue;
        }

        assert!(runoff(&g), "{context}");
        let runners: Vec<PlayerId> = opponents
            .iter()
            .copied()
            .filter(|p| tied.contains(p))
            .collect();
        for extra in product(&vec![start - top; runners.len()]) {
            let mut after = g.clone();
            bid_all(&mut after, &extra);
            let totals: Vec<(PlayerId, u32)> = runners
                .iter()
                .copied()
                .zip(extra.iter().map(|e| top + e))
                .collect();
            let (winners, new_top) = leaders(&totals);
            let context = format!("{context}, runoff {extra:?}");
            expect_decision(&after, &table, &winners, new_top, &context);
        }
    }
}

#[test]
fn every_combination_of_bids_settles_as_the_rules_say_with_two_opponents() {
    check_every_outcome(3, 3, 2);
    check_every_outcome(3, 3, 3);
}

#[test]
fn every_combination_of_bids_settles_as_the_rules_say_with_three_opponents() {
    check_every_outcome(4, 2, 1);
    check_every_outcome(4, 2, 2);
}

#[test]
fn every_combination_of_bids_settles_as_the_rules_say_with_four_opponents() {
    check_every_outcome(5, 1, 0);
    check_every_outcome(5, 1, 1);
}

// Invariants

fn corrupt(g: &mut Game, edit: impl FnOnce(&mut Bidding)) {
    let Phase::Bid(mut b) = g.phase else {
        panic!("not in a delivery auction: {:?}", g.phase);
    };
    edit(&mut b);
    g.phase = Phase::Bid(b);
}

fn flagged(g: &Game) -> String {
    g.check_invariants()
        .expect_err("the invariants should be broken")
}

#[test]
fn the_invariants_flag_a_bidder_who_is_not_bidding() {
    let (mut g, me) = delivery_game(3, 10);
    arrive(&mut g);

    corrupt(&mut g, |b| b.bidder = me);

    assert!(flagged(&g).contains("not a contender"));
}

#[test]
fn the_invariants_flag_cash_on_the_table_from_someone_who_is_not_bidding() {
    let (mut g, me) = delivery_game(3, 10);
    arrive(&mut g);

    g.state.players[me].cash -= 3;
    corrupt(&mut g, |b| b.bids[me] = 3);

    assert!(flagged(&g).contains("not bidding"));
}

#[test]
fn the_invariants_flag_less_on_the_table_than_the_round_began_with() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    g.apply(Action::Bid(5)).unwrap();

    corrupt(&mut g, |b| b.base = 6);

    assert!(flagged(&g).contains("less than"));
}

// Whole games

#[test]
fn random_play_with_many_deliveries_keeps_every_invariant() {
    let mut deliveries = 0;
    for seed in 0..6 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            let mut rng = Rng::new(seed + 90);
            for _ in 0..1500 {
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
                if pick == Action::Sail(ShipLocation::Island) {
                    deliveries += 1;
                }
                g.apply(pick).unwrap();
                assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
            }
        }
    }
    assert!(deliveries > 0, "the playouts never delivered anything");
}
