use std::collections::HashSet;

use super::{
    finish_arrange, leave_in_supply, new_game, pass_turn, set_container_lot, set_factories,
    set_factory_stock, set_loans, set_lots, set_ship, set_warehouses,
};
use crate::action::{Action, ActionError, Building, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;
use crate::scoring::ScoreBreakdown;
use crate::state::{AuctionKind, BANK_LOTS, PlayerId, ShipLocation};
use AuctionKind::{CashLot, ContainerLot};
use Color::{Blue, Green, Red, White, Yellow};
use District::{Factory, Harbor};

fn counts(items: &[(Color, u8)]) -> ColorCounts {
    let mut tally = ColorCounts::default();
    for &(color, n) in items {
        tally[color] += n;
    }
    tally
}

fn plus(mut lot: ColorCounts, color: Color) -> ColorCounts {
    lot[color] += 1;
    lot
}

/// Gives the active player exactly these factories and this stock in their factory district, and leaves
/// exactly these numbers of containers in the supply. The other colors keep what they have.
fn arrange_production(
    g: &mut Game,
    factories: &[Color],
    stock: &[(Color, u8)],
    supply: &[(Color, u8)],
) -> PlayerId {
    let me = g.current_player();
    set_factories(g, me, factories);
    set_factory_stock(g, me, counts(stock));
    for &(color, left) in supply {
        leave_in_supply(g, color, left);
    }
    me
}

fn producing(
    factories: &[Color],
    stock: &[(Color, u8)],
    supply: &[(Color, u8)],
) -> (Game, PlayerId) {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = arrange_production(&mut g, factories, stock, supply);
    (g, me)
}

/// The active player owns only a blue factory and has no stock. The supply has no red left and one
/// blue container: producing it empties a second color.
fn on_the_brink() -> (Game, PlayerId) {
    producing(&[Blue], &[], &[(Red, 0), (Blue, 1)])
}

fn produce(g: &mut Game) {
    g.apply(Action::Produce).unwrap();
    finish_arrange(g);
}

/// A harmless action that only moves containers between lots.
fn pass_action(g: &mut Game) {
    g.apply(Action::Reprice(Factory)).unwrap();
    finish_arrange(g);
}

/// Calls the Bank with a bid of $1 on container lot I.
fn call_the_bank(g: &mut Game) {
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 0,
    })
    .unwrap();
    g.apply(Action::BidCash(1)).unwrap();
}

/// The active player bids this much cash on a container lot and passes.
fn bid_cash_and_pass(g: &mut Game, lot: usize, amount: u32) {
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot,
    })
    .unwrap();
    g.apply(Action::BidCash(amount)).unwrap();
    pass_action(g);
}

/// The active player owns a green factory holding `n` green containers (at most 2), bids all of them
/// on a cash lot and passes.
fn bid_containers_and_pass(g: &mut Game, lot: usize, n: u8) {
    let me = g.current_player();
    set_factories(g, me, &[Green]);
    set_lots(g, me, Factory, &[(0, Green, n)]);
    g.apply(Action::StartAuction { kind: CashLot, lot })
        .unwrap();
    let green = Action::BidContainer {
        district: Factory,
        lot: 0,
        color: Green,
    };
    for _ in 0..n {
        g.apply(green).unwrap();
    }
    g.apply(Action::DoneBidding).unwrap();
    pass_action(g);
}

/// The colors the player is offered while choosing what to produce.
fn offered(g: &Game) -> Vec<Color> {
    g.legal_actions()
        .into_iter()
        .map(|action| match action {
            Action::ProduceColor(color) => color,
            other => panic!("expected a color to produce, got {other:?}"),
        })
        .collect()
}

fn finished_game() -> Game {
    let (mut g, _) = on_the_brink();
    produce(&mut g);
    pass_action(&mut g);
    assert!(g.is_terminal());
    g
}

// The last turn

#[test]
fn the_game_goes_on_while_a_single_color_is_missing_from_the_supply() {
    let (mut g, me) = producing(&[Blue], &[], &[(Red, 0), (Blue, 2)]);
    assert!(!g.is_last_turn());

    produce(&mut g);

    assert_eq!(g.state.supply.containers[Red], 0);
    assert_eq!(g.state.supply.containers[Blue], 1);
    assert!(!g.is_last_turn());
    pass_action(&mut g);
    assert!(!g.is_terminal());
    assert_ne!(g.current_player(), me);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn taking_the_last_container_of_a_second_color_makes_it_the_last_turn() {
    let (mut g, me) = on_the_brink();
    assert!(!g.is_last_turn());

    produce(&mut g);

    assert_eq!(g.state.supply.containers[Blue], 0);
    assert!(g.is_last_turn());
    // The turn is not cut short: the player still has their second action.
    assert!(!g.is_terminal());
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), me);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_last_containers_of_several_colors_taken_together_also_make_it_the_last_turn() {
    let (mut g, _) = producing(
        &[Red, Blue, Yellow],
        &[],
        &[(Red, 1), (Blue, 1), (Yellow, 1)],
    );
    produce(&mut g);
    assert!(g.is_last_turn());
}

#[test]
fn the_game_ends_when_the_last_turn_does() {
    let (mut g, me) = on_the_brink();
    produce(&mut g);
    pass_action(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.phase, Phase::GameOver);
    // Nobody else's turn begins.
    assert_eq!(g.state.active_player, me);
    assert_eq!(g.current_player(), me);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_second_action_that_empties_the_supply_ends_the_game_once_it_is_arranged() {
    let (mut g, _) = on_the_brink();
    pass_action(&mut g);
    assert!(!g.is_last_turn());

    g.apply(Action::Produce).unwrap();
    assert!(matches!(g.phase, Phase::Arrange(_)));
    assert!(!g.is_terminal());
    finish_arrange(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_player_who_empties_the_supply_still_takes_their_second_action() {
    let (mut g, me) = on_the_brink();
    let warehouses = g.state.players[me].warehouses;
    produce(&mut g);
    assert!(g.is_legal(Action::Build(Building::Warehouse)));

    g.apply(Action::Build(Building::Warehouse)).unwrap();

    assert_eq!(g.state.players[me].warehouses, warehouses + 1);
    assert!(g.is_terminal());
}

#[test]
fn a_delivery_auction_is_played_out_in_the_last_turn_before_the_game_ends() {
    let (mut g, me) = on_the_brink();
    set_ship(&mut g, me, counts(&[(Green, 2)]));
    let (first, second) = ((me + 1) % 3, (me + 2) % 3);
    let cash = g.state.players[me].cash;
    produce(&mut g);
    assert!(g.is_last_turn());

    g.apply(Action::Sail(ShipLocation::Island)).unwrap();
    assert!(matches!(g.phase, Phase::Bid(_)));
    g.apply(Action::Bid(1)).unwrap();
    g.apply(Action::Bid(0)).unwrap();
    assert!(matches!(g.phase, Phase::Decide(_)));
    assert!(!g.is_terminal());
    g.apply(Action::Accept(first)).unwrap();

    assert!(g.is_terminal());
    assert_eq!(g.state.players[first].scoring_area, counts(&[(Green, 2)]));
    assert!(g.state.players[me].ship.is_empty());
    // The wages, then the buyer's bid and the equal subsidy from the supply.
    assert_eq!(g.state.players[me].cash, cash - 1 + 2);
    // The player on the right was paid the wages, and bid nothing.
    assert_eq!(g.state.players[second].cash, 20 + 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

/// The cash the player after the active one holds once the active player has produced and passed,
/// with this many blue containers in the supply to begin with. They owe two loans.
fn cash_after_the_turn_of_the_player_before(blue: u8) -> (u32, u32) {
    let (mut g, me) = producing(&[Blue], &[], &[(Red, 0), (Blue, blue)]);
    let next = (me + 1) % 3;
    set_loans(&mut g, next, 2);
    produce(&mut g);
    pass_action(&mut g);
    (
        g.state.players[next].cash,
        g.state.bank.cash_lots.iter().sum(),
    )
}

#[test]
fn no_interest_is_paid_once_the_game_is_over() {
    // The Bank starts with $6 and the player with $20.
    assert_eq!(cash_after_the_turn_of_the_player_before(2), (18, 8));
    assert_eq!(cash_after_the_turn_of_the_player_before(1), (20, 6));
}

// Calling the Bank in the last turn

#[test]
fn the_bank_cannot_be_called_once_the_supply_has_run_out() {
    let (mut last, _) = producing(&[Blue], &[], &[(Red, 0), (Blue, 1)]);
    let (mut control, _) = producing(&[Blue], &[], &[(Red, 0), (Blue, 2)]);
    produce(&mut last);
    produce(&mut control);
    assert!(last.is_last_turn());
    assert!(!control.is_last_turn());

    for kind in AuctionKind::ALL {
        for lot in 0..BANK_LOTS {
            let start = Action::StartAuction { kind, lot };
            assert!(control.is_legal(start), "{start:?} when the game goes on");
            assert!(!last.is_legal(start), "{start:?} in the last turn");
            assert_eq!(last.apply(start), Err(ActionError::Illegal(start)));
        }
    }
    let calls_the_bank = |a: &Action| matches!(a, Action::StartAuction { .. } | Action::Outbid(_));
    assert!(control.legal_actions().iter().any(calls_the_bank));
    assert!(!last.legal_actions().iter().any(calls_the_bank));
}

/// The first player has put $1 on container lot I and passed. The next player owns a lone blue factory
/// without stock, and the supply has no red left and this many blue containers.
fn facing_a_bid(blue: u8) -> Game {
    let mut g = new_game(3, GameLength::Standard, 1);
    call_the_bank(&mut g);
    pass_action(&mut g);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, blue)]);
    g
}

#[test]
fn an_opponents_bid_cannot_be_beaten_once_the_supply_has_run_out() {
    let mut last = facing_a_bid(1);
    let mut control = facing_a_bid(2);
    produce(&mut last);
    produce(&mut control);
    assert!(last.is_last_turn());
    assert!(!control.is_last_turn());

    let outbid = Action::Outbid(ContainerLot);
    assert!(control.is_legal(outbid));
    assert!(!last.is_legal(outbid));
    assert_eq!(last.apply(outbid), Err(ActionError::Illegal(outbid)));
}

// Producing in a turn that called the Bank

#[test]
fn a_produce_that_would_end_the_game_is_illegal_in_a_turn_that_called_the_bank() {
    let (mut g, _) = on_the_brink();
    assert!(g.is_legal(Action::Produce));
    call_the_bank(&mut g);

    assert_eq!(g.phase, Phase::Main);
    assert!(g.turn.bank_called);
    assert!(!g.is_legal(Action::Produce));
    assert!(!g.legal_actions().contains(&Action::Produce));
    assert_eq!(
        g.apply(Action::Produce),
        Err(ActionError::Illegal(Action::Produce))
    );
}

#[test]
fn a_produce_that_leaves_the_game_going_is_legal_in_a_turn_that_called_the_bank() {
    let (mut g, _) = producing(&[Blue], &[], &[(Red, 0), (Blue, 2)]);
    call_the_bank(&mut g);

    assert!(g.is_legal(Action::Produce));
    assert!(g.legal_actions().contains(&Action::Produce));
    produce(&mut g);
    assert!(!g.is_last_turn());
}

#[test]
fn a_produce_that_yields_nothing_is_legal_in_a_turn_that_called_the_bank() {
    // The factory district is full, so the last blue container stays in the supply.
    let (mut g, _) = producing(&[Blue], &[(Green, 2)], &[(Red, 0), (Blue, 1)]);
    call_the_bank(&mut g);

    assert!(g.is_legal(Action::Produce));
    produce(&mut g);
    assert_eq!(g.state.supply.containers[Blue], 1);
    assert!(!g.is_last_turn());
}

#[test]
fn a_turn_that_won_an_auction_may_still_end_the_game_by_producing() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let winner = g.current_player();
    call_the_bank(&mut g);
    pass_action(&mut g);
    pass_turn(&mut g);
    pass_turn(&mut g);
    assert_eq!(g.current_player(), winner);
    assert!(g.turn.won_auction);
    assert!(!g.turn.bank_called);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, 1)]);

    assert!(g.is_legal(Action::Produce));
    produce(&mut g);
    assert!(g.is_last_turn());
    pass_action(&mut g);
    assert!(g.is_terminal());
}

/// The factories red, blue and yellow with room for two of their containers, and one container of red
/// and of blue left in the supply.
fn choosing() -> (Game, PlayerId) {
    producing(&[Red, Blue, Yellow], &[(Green, 4)], &[(Red, 1), (Blue, 1)])
}

#[test]
fn colors_that_would_end_the_game_are_not_offered_in_a_turn_that_called_the_bank() {
    let (mut g, _) = choosing();
    call_the_bank(&mut g);
    g.apply(Action::Produce).unwrap();
    assert_eq!(offered(&g), [Red, Blue]);

    g.apply(Action::ProduceColor(Red)).unwrap();

    // Blue would take the last container of a second color.
    assert_eq!(offered(&g), [Yellow]);
    g.apply(Action::ProduceColor(Yellow)).unwrap();
    finish_arrange(&mut g);
    assert!(!g.is_last_turn());
}

#[test]
fn every_color_that_fits_is_offered_in_a_turn_that_did_not_call_the_bank() {
    let (mut g, _) = choosing();
    g.apply(Action::Produce).unwrap();
    assert_eq!(offered(&g), [Red, Blue]);

    g.apply(Action::ProduceColor(Red)).unwrap();

    assert_eq!(offered(&g), [Blue, Yellow]);
    g.apply(Action::ProduceColor(Blue)).unwrap();
    finish_arrange(&mut g);
    assert!(g.is_last_turn());
}

#[test]
fn every_color_that_fits_is_offered_in_a_turn_that_won_an_auction() {
    let mut g = new_game(3, GameLength::Standard, 1);
    call_the_bank(&mut g);
    pass_action(&mut g);
    pass_turn(&mut g);
    pass_turn(&mut g);
    assert!(g.turn.won_auction);
    assert!(!g.turn.bank_called);
    let stock = [(Green, 4)];
    let supply = [(Red, 1), (Blue, 1)];
    arrange_production(&mut g, &[Red, Blue, Yellow], &stock, &supply);
    g.apply(Action::Produce).unwrap();
    assert_eq!(offered(&g), [Red, Blue]);

    g.apply(Action::ProduceColor(Red)).unwrap();

    assert_eq!(offered(&g), [Blue, Yellow]);
}

#[test]
fn a_first_color_whose_every_completion_ends_the_game_is_not_offered() {
    // Green is already gone, so emptying red ends the game, and only blue with yellow does not.
    let (mut g, _) = producing(
        &[Red, Blue, Yellow],
        &[(Green, 4)],
        &[(Red, 1), (Blue, 3), (Yellow, 3), (Green, 0)],
    );
    call_the_bank(&mut g);
    g.apply(Action::Produce).unwrap();

    assert_eq!(offered(&g), [Blue]);
    g.apply(Action::ProduceColor(Blue)).unwrap();
    assert_eq!(offered(&g), [Yellow]);
}

#[test]
fn a_first_color_whose_every_completion_ends_the_game_is_offered_without_a_bank_call() {
    let (mut g, _) = producing(
        &[Red, Blue, Yellow],
        &[(Green, 4)],
        &[(Red, 1), (Blue, 3), (Yellow, 3), (Green, 0)],
    );
    g.apply(Action::Produce).unwrap();

    assert_eq!(offered(&g), [Red, Blue]);
}

#[test]
fn produce_is_illegal_when_every_choice_ends_the_game_in_a_turn_that_called_the_bank() {
    let (mut g, _) = producing(
        &[Red, Blue, Yellow],
        &[(Green, 4)],
        &[(Red, 1), (Blue, 1), (Yellow, 1)],
    );
    assert!(g.is_legal(Action::Produce));

    call_the_bank(&mut g);

    assert!(!g.is_legal(Action::Produce));
    assert!(!g.legal_actions().contains(&Action::Produce));
}

/// Every hand of new containers that a sequence of color choices can produce, without the stock the
/// player already had.
fn produced_hands(g: &Game, stock: u8, out: &mut HashSet<ColorCounts>) {
    match g.phase {
        Phase::Produce(_) => {
            let legal = g.legal_actions();
            assert!(!legal.is_empty(), "dead end while choosing colors");
            for action in legal {
                let mut next = g.clone();
                next.apply(action).unwrap();
                produced_hands(&next, stock, out);
            }
        }
        Phase::Arrange(a) => {
            let mut produced = a.hand;
            produced[Green] -= stock;
            out.insert(produced);
        }
        other => panic!("unexpected phase {other:?}"),
    }
}

/// What producing can yield, worked out by trying every set of colors. The player owns factories of
/// red, blue, yellow and white, and the supply holds this many containers of each.
fn expected_hands(supply: [u8; 4], room: usize, bank_called: bool) -> HashSet<ColorCounts> {
    let colors = [Red, Blue, Yellow, White];
    let candidates = supply.iter().filter(|&&left| left > 0).count();
    let to_pick = room.min(candidates);
    let already_empty = supply.iter().filter(|&&left| left == 0).count();
    let mut hands = HashSet::new();
    for mask in 0u32..16 {
        let chosen: Vec<usize> = (0..4).filter(|&i| mask & (1 << i) != 0).collect();
        if chosen.len() != to_pick || chosen.iter().any(|&i| supply[i] == 0) {
            continue;
        }
        let emptied = chosen.iter().filter(|&&i| supply[i] == 1).count();
        if bank_called && already_empty + emptied >= 2 {
            continue;
        }
        let mut hand = ColorCounts::default();
        for &i in &chosen {
            hand[colors[i]] = 1;
        }
        hands.insert(hand);
    }
    hands
}

#[test]
fn the_producible_hands_match_a_search_of_every_set_of_colors() {
    let colors = [Red, Blue, Yellow, White];
    let levels = [0u8, 1, 3];
    for code in 0..81usize {
        let supply = [
            levels[code % 3],
            levels[code / 3 % 3],
            levels[code / 9 % 3],
            levels[code / 27],
        ];
        // Two colors short at the start of a turn would have ended the game already.
        if supply.iter().filter(|&&left| left == 0).count() >= 2 {
            continue;
        }
        for room in 0..=4u8 {
            for bank_called in [false, true] {
                let stock = 8 - room;
                let left: Vec<(Color, u8)> = colors.into_iter().zip(supply).collect();
                let (mut g, _) = producing(&colors, &[(Green, stock)], &left);
                g.turn.bank_called = bank_called;
                let expected = expected_hands(supply, usize::from(room), bank_called);
                let context = format!("supply {supply:?}, room {room}, bank called {bank_called}");

                assert_eq!(
                    g.is_legal(Action::Produce),
                    !expected.is_empty(),
                    "{context}"
                );
                if expected.is_empty() {
                    continue;
                }
                g.apply(Action::Produce).unwrap();
                let mut hands = HashSet::new();
                produced_hands(&g, stock, &mut hands);
                assert_eq!(hands, expected, "{context}");
            }
        }
    }
}

// Settling the Bank

#[test]
fn a_cash_bid_on_a_container_lot_wins_the_lot_when_the_game_ends() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let bidder = g.current_player();
    set_container_lot(&mut g, 0, counts(&[(Yellow, 3)]));
    bid_cash_and_pass(&mut g, 0, 4);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, 1)]);
    produce(&mut g);
    let cash = g.state.players[bidder].cash;
    assert!(g.state.players[bidder].holding.is_empty());
    assert_eq!(g.state.bank.cash_lots, [1, 2, 3]);

    pass_action(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.state.players[bidder].holding, counts(&[(Yellow, 3)]));
    assert!(g.state.bank.container_lots[0].is_empty());
    assert_eq!(g.state.bank.auctions.active(), 0);
    // The bid is dealt out over the cash lots, a dollar at a time from lot I.
    assert_eq!(g.state.bank.cash_lots, [3, 3, 4]);
    assert_eq!(g.state.players[bidder].cash, cash);
    assert_eq!(g.results().unwrap().scores[bidder].leftovers, 9);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_container_bid_on_a_cash_lot_wins_the_cash_when_the_game_ends() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let bidder = g.current_player();
    bid_containers_and_pass(&mut g, 1, 2);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, 1)]);
    produce(&mut g);
    let cash = g.state.players[bidder].cash;
    let lots = g.state.bank.container_lots;
    assert_eq!(g.state.players[bidder].reserve_factory, 2);

    pass_action(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.state.players[bidder].cash, cash + 2);
    assert_eq!(g.state.bank.cash_lots, [1, 0, 3]);
    assert_eq!(g.state.players[bidder].reserve_factory, 0);
    assert_eq!(g.state.bank.auctions.active(), 0);
    // The containers of the bid go to the Bank, one lot at a time from lot I.
    assert_eq!(
        g.state.bank.container_lots,
        [plus(lots[0], Green), plus(lots[1], Green), lots[2]]
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn every_container_of_a_bid_goes_to_the_bank_whichever_district_it_came_from() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let bidder = g.current_player();
    set_factories(&mut g, bidder, &[Green]);
    set_lots(&mut g, bidder, Factory, &[(0, Green, 1)]);
    set_lots(&mut g, bidder, Harbor, &[(0, White, 1)]);
    g.apply(Action::StartAuction {
        kind: CashLot,
        lot: 1,
    })
    .unwrap();
    for (district, color) in [(Factory, Green), (Harbor, White)] {
        g.apply(Action::BidContainer {
            district,
            lot: 0,
            color,
        })
        .unwrap();
    }
    g.apply(Action::DoneBidding).unwrap();
    pass_action(&mut g);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, 1)]);
    produce(&mut g);
    let cash = g.state.players[bidder].cash;
    let lots = g.state.bank.container_lots;
    assert_eq!(g.state.players[bidder].reserve_factory, 1);
    assert_eq!(g.state.players[bidder].reserve_harbor, 1);

    pass_action(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.state.players[bidder].cash, cash + 2);
    assert_eq!(g.state.players[bidder].reserve_factory, 0);
    assert_eq!(g.state.players[bidder].reserve_harbor, 0);
    // The Bank deals them out in color order, one lot at a time from lot I.
    assert_eq!(
        g.state.bank.container_lots,
        [plus(lots[0], White), plus(lots[1], Green), lots[2]]
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn both_auctions_of_a_five_player_game_are_won_by_their_holders_when_the_game_ends() {
    let mut g = new_game(5, GameLength::Standard, 1);
    let buyer = g.current_player();
    let seller = (buyer + 1) % 5;
    set_container_lot(&mut g, 0, counts(&[(Yellow, 3)]));
    bid_cash_and_pass(&mut g, 0, 3);
    bid_containers_and_pass(&mut g, 2, 2);
    assert_eq!(g.state.bank.auctions.active(), 2);
    arrange_production(&mut g, &[Blue], &[], &[(Red, 0), (Blue, 1)]);
    produce(&mut g);
    let cash = g.state.players[seller].cash;
    let lots = g.state.bank.container_lots;

    pass_action(&mut g);

    assert!(g.is_terminal());
    assert_eq!(g.state.bank.auctions.active(), 0);
    assert_eq!(g.state.players[buyer].holding, counts(&[(Yellow, 3)]));
    assert_eq!(g.state.players[seller].cash, cash + 3);
    assert_eq!(g.state.players[seller].reserve_factory, 0);
    // Both bids are paid at the same moment: the cash lot the seller collects is the one they bid on
    // before the buyer's $3 reached the Bank, and the buyer's lot is the one they bid on before the
    // seller's containers reached the Bank.
    assert_eq!(g.state.bank.cash_lots, [3, 3, 0]);
    assert_eq!(
        g.state.bank.container_lots,
        [counts(&[(Green, 1)]), plus(lots[1], Green), lots[2]]
    );
    assert_eq!(g.results().unwrap().scores[buyer].leftovers, 9);
    assert_eq!(g.check_invariants(), Ok(()));
}

// After the game

/// One action of every kind, aimed at the places a player could want to act on.
fn every_kind_of_action() -> Vec<Action> {
    let mut actions = vec![
        Action::Produce,
        Action::HarborPurchase,
        Action::DoneBuying,
        Action::DoneLoading,
        Action::BuyOut,
        Action::RequestLoan,
        Action::RepayLoan,
        Action::DoneBidding,
        Action::Reprice(Factory),
        Action::Reprice(Harbor),
        Action::Build(Building::Warehouse),
        Action::Outbid(ContainerLot),
        Action::Outbid(CashLot),
        Action::Sail(ShipLocation::Ocean),
        Action::Sail(ShipLocation::Island),
        Action::Sail(ShipLocation::Bank),
    ];
    for color in Color::ALL {
        actions.extend([
            Action::ProduceColor(color),
            Action::Build(Building::Factory(color)),
            Action::Buy { lot: 0, color },
            Action::Load(color),
            Action::Seize { lot: 0, color },
            Action::Deal(color),
            Action::BidContainer {
                district: Factory,
                lot: 0,
                color,
            },
        ]);
    }
    for seat in 0..3 {
        actions.extend([
            Action::FactoryPurchase(seat),
            Action::Accept(seat),
            Action::Sail(ShipLocation::Harbor(seat)),
        ]);
    }
    for n in 0..4 {
        actions.extend([
            Action::Bid(n),
            Action::BidCash(n),
            Action::Place(n as usize),
        ]);
    }
    for kind in AuctionKind::ALL {
        for lot in 0..BANK_LOTS {
            actions.push(Action::StartAuction { kind, lot });
        }
    }
    actions
}

#[test]
fn nothing_can_be_done_once_the_game_is_over() {
    let g = finished_game();

    assert!(g.legal_actions().is_empty());
    for action in every_kind_of_action() {
        assert!(!g.is_legal(action), "{action:?}");
        let mut after = g.clone();
        assert_eq!(after.apply(action), Err(ActionError::Illegal(action)));
        assert_eq!(after, g, "{action:?} changed a finished game");
    }
}

#[test]
fn there_are_no_results_until_the_game_is_over() {
    let (mut g, _) = on_the_brink();
    assert_eq!(g.results(), None);
    produce(&mut g);
    assert!(g.is_last_turn());
    assert_eq!(g.results(), None);

    pass_action(&mut g);

    assert!(g.results().is_some());
}

#[test]
fn the_results_score_every_player_on_the_final_position() {
    let (mut g, me) = on_the_brink();
    let right = g.state.player_on_right(me);
    let left = (me + 1) % 3;
    set_loans(&mut g, me, 1);
    set_ship(&mut g, me, counts(&[(Green, 2)]));
    set_warehouses(&mut g, me, 3);
    set_lots(&mut g, me, Harbor, &[(0, Green, 3)]);

    produce(&mut g);
    pass_action(&mut g);

    let results = g.results().unwrap();
    let score = |cash, leftovers, loans, total| ScoreBreakdown {
        cash,
        island: 0,
        leftovers,
        loans,
        total,
        discarded: None,
    };
    // $3 for each of the two containers on the ship and $2 for each of the three in the harbor, and
    // $11 to pay off the loan.
    assert_eq!(results.scores[me], score(19, 12, 11, 20));
    assert_eq!(results.scores[right], score(21, 0, 0, 21));
    assert_eq!(results.scores[left], score(20, 0, 0, 20));
    assert_eq!(results.winners, [right]);
}

#[test]
fn random_play_on_a_short_supply_ends_with_every_invariant_kept() {
    let mut ended = 0;
    for players in 3..=5 {
        for seed in 0..8 {
            let mut g = new_game(players, GameLength::Short, seed);
            for color in Color::ALL {
                let left = g.state.supply.containers[color];
                leave_in_supply(&mut g, color, left.min(3));
            }
            let mut rng = Rng::new(seed + 100);
            for _ in 0..4000 {
                if g.is_terminal() {
                    break;
                }
                let legal = g.legal_actions();
                assert!(!legal.is_empty(), "stuck in {:?}", g.phase);
                let pick = if legal.contains(&Action::Produce) && rng.below(3) == 0 {
                    Action::Produce
                } else {
                    legal[rng.below(legal.len() as u32) as usize]
                };
                g.apply(pick).unwrap();
                assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
            }
            if g.is_terminal() {
                ended += 1;
                assert!(g.legal_actions().is_empty());
                assert!(g.results().is_some());
                assert_eq!(g.state.bank.auctions.active(), 0);
            } else {
                assert!(g.results().is_none());
            }
        }
    }
    assert!(ended >= 20, "only {ended} of 24 games ended");
}
