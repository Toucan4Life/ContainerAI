//! Delivery bids laid out as cards, when the configuration lets bidders revise them.

use super::delivery::{arrive, choices, delivery_game_after};
use super::new_game;
use crate::action::{Action, ActionError};
use crate::cards::{BLUFF_CARDS, card_counts};
use crate::config::{GameLength, MAX_PLAYERS};
use crate::game::Game;
use crate::phase::{BidStep, Bidding, Decision, Phase, Stack};
use crate::state::PlayerId;

/// A delivery auction in which everyone has `start` cash and each bidder may revise their bid
/// `revisions` times, at the first opponent's turn. Also returns the opponents in bidding order.
pub(super) fn auction(players: usize, start: u32, revisions: u8) -> (Game, Vec<PlayerId>) {
    auction_after(players, start, revisions, 0)
}

/// As `auction`, but the deliverer is the seat `turns` turns on from the first player.
fn auction_after(players: usize, start: u32, revisions: u8, turns: usize) -> (Game, Vec<PlayerId>) {
    let (mut g, _) = delivery_game_after(players, start, turns);
    g.config.max_bid_revisions = revisions;
    let opponents = arrive(&mut g);
    (g, opponents)
}

pub(super) fn bidding(g: &Game) -> Bidding {
    match g.phase {
        Phase::Bid(bidding) => bidding,
        other => panic!("not in a delivery auction: {other:?}"),
    }
}

fn change(g: &mut Game, edit: impl FnOnce(&mut Bidding)) {
    let mut b = bidding(g);
    edit(&mut b);
    g.phase = Phase::Bid(b);
}

fn cash(g: &Game, seat: PlayerId) -> u32 {
    g.state.players[seat].cash
}

/// Plays the three steps of one bid, checking the invariants after each.
pub(super) fn lay(g: &mut Game, amount: u32, cards: u32, bluffs: u8) {
    for action in [
        Action::Bid(amount),
        Action::BidCards(cards),
        Action::BidBluffs(bluffs),
    ] {
        g.apply(action).unwrap();
        assert_eq!(g.check_invariants(), Ok(()), "after {action:?}");
    }
}

/// The action is refused, and nothing changes.
fn refused(g: &mut Game, action: Action) {
    let before = g.clone();
    assert_eq!(g.apply(action), Err(ActionError::Illegal(action)));
    assert_eq!(*g, before, "{action:?} was refused but changed the game");
    assert!(!g.legal_actions().contains(&action), "{action:?}");
}

fn flagged(g: &Game) -> String {
    g.check_invariants()
        .expect_err("the invariants should be broken")
}

/// Everyone has laid out a stack, given as (amount, cards, bluffs) in bidding order. The bidders then
/// look the stacks over, starting with the first.
pub(super) fn reviewing(
    players: usize,
    start: u32,
    revisions: u8,
    stacks: &[(u32, u32, u8)],
) -> (Game, Vec<PlayerId>) {
    let (mut g, opponents) = auction(players, start, revisions);
    assert_eq!(stacks.len(), opponents.len());
    for &(amount, cards, bluffs) in stacks {
        lay(&mut g, amount, cards, bluffs);
    }
    (g, opponents)
}

/// Who must act in the auction, and what they are doing.
fn reviewer(g: &Game) -> (PlayerId, BidStep) {
    let b = bidding(g);
    (b.bidder, b.step)
}

/// The contenders who are content with the stacks as they lie.
fn content(g: &Game) -> Vec<PlayerId> {
    let b = bidding(g);
    (0..MAX_PLAYERS).filter(|&seat| b.ready[seat]).collect()
}

/// The highest bid the deliverer faces, and who holds it.
fn decision(g: &Game) -> (u32, Vec<PlayerId>) {
    let Phase::Decide(Decision { top, tied, .. }) = g.phase else {
        panic!("no decision to take: {:?}", g.phase);
    };
    (top, (0..MAX_PLAYERS).filter(|&seat| tied[seat]).collect())
}

/// Everyone still looking the stacks over is content with them.
pub(super) fn all_content(g: &mut Game) {
    while matches!(g.phase, Phase::Bid(b) if b.step == BidStep::Review) {
        g.apply(Action::ReadyToReveal).unwrap();
    }
}

/// The bidders lay out the given amounts in bidding order, each as the fewest cards and with no bluffs,
/// and are all content: the round is settled by the amounts alone.
pub(super) fn first_round(g: &mut Game, bids: &[u32]) {
    for &amount in bids {
        lay(g, amount, card_counts(amount)[0], 0);
    }
    all_content(g);
}

// Without revisions

#[test]
fn without_revisions_a_bid_is_the_cash_alone() {
    let (mut g, opponents) = auction(3, 10, 0);

    g.apply(Action::Bid(5)).unwrap();

    let b = bidding(&g);
    assert_eq!((b.bidder, b.step), (opponents[1], BidStep::Cash));
    assert!(!b.stacks_down);
    assert_eq!(b.stacks, [Stack::default(); MAX_PLAYERS]);
    assert_eq!(choices(&g), (0..=10).map(Action::Bid).collect::<Vec<_>>());
}

#[test]
fn without_revisions_no_cards_can_be_laid_out() {
    let (mut g, _) = auction(3, 10, 0);
    for _ in 0..2 {
        for cards in 0..=12 {
            refused(&mut g, Action::BidCards(cards));
        }
        for bluffs in 0..=BLUFF_CARDS + 1 {
            refused(&mut g, Action::BidBluffs(bluffs));
        }
        g.apply(Action::Bid(5)).unwrap();
    }
}

// Laying out a bid

#[test]
fn a_stack_counts_its_bluff_cards_among_its_cards() {
    let stack = |cards, bluffs| Stack { cards, bluffs };
    assert_eq!(Stack::default().total(), 0);
    assert_eq!(stack(3, 0).total(), 3);
    assert_eq!(stack(0, 2).total(), 2);
    assert_eq!(stack(4, 2).total(), 6);
}

#[test]
fn with_revisions_a_bid_is_laid_out_as_cash_then_cards_then_bluffs() {
    let (mut g, opponents) = auction(3, 10, 1);
    let first = opponents[0];
    assert_eq!(g.current_player(), first);
    assert_eq!(choices(&g), (0..=10).map(Action::Bid).collect::<Vec<_>>());

    g.apply(Action::Bid(5)).unwrap();
    assert_eq!(
        (g.current_player(), bidding(&g).step),
        (first, BidStep::Cards)
    );
    assert_eq!(g.legal_actions(), [1, 3, 4, 5].map(Action::BidCards));

    g.apply(Action::BidCards(3)).unwrap();
    assert_eq!(
        (g.current_player(), bidding(&g).step),
        (first, BidStep::Bluffs)
    );
    assert_eq!(g.legal_actions(), [0, 1, 2].map(Action::BidBluffs));

    g.apply(Action::BidBluffs(1)).unwrap();
    let b = bidding(&g);
    assert_eq!((b.bidder, b.step), (opponents[1], BidStep::Cash));
    assert_eq!(
        b.stacks[first],
        Stack {
            cards: 3,
            bluffs: 1
        }
    );
    assert!(!b.stacks_down);
}

#[test]
fn the_cash_leaves_the_hand_for_the_table_as_soon_as_the_amount_is_chosen() {
    let (mut g, opponents) = auction(3, 10, 1);
    let first = opponents[0];

    g.apply(Action::Bid(6)).unwrap();
    assert_eq!((cash(&g, first), bidding(&g).bids[first]), (4, 6));
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::BidCards(3)).unwrap();
    g.apply(Action::BidBluffs(2)).unwrap();
    assert_eq!((cash(&g, first), bidding(&g).bids[first]), (4, 6));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_bid_of_nothing_is_no_cash_cards_and_still_up_to_two_bluffs() {
    for bluffs in 0..=BLUFF_CARDS {
        let (mut g, opponents) = auction(3, 10, 1);

        g.apply(Action::Bid(0)).unwrap();
        assert_eq!(g.legal_actions(), [Action::BidCards(0)]);
        g.apply(Action::BidCards(0)).unwrap();
        g.apply(Action::BidBluffs(bluffs)).unwrap();

        assert_eq!(bidding(&g).stacks[opponents[0]], Stack { cards: 0, bluffs });
        assert_eq!(cash(&g, opponents[0]), 10);
    }
}

#[test]
fn any_bid_can_be_shown_as_any_number_of_cards_that_makes_it() {
    for amount in 0..=25 {
        let (mut g, opponents) = auction(3, 25, 1);
        g.apply(Action::Bid(amount)).unwrap();
        let offered: Vec<_> = card_counts(amount)
            .into_iter()
            .map(Action::BidCards)
            .collect();
        assert_eq!(g.legal_actions(), offered, "${amount}");

        for cards in card_counts(amount) {
            let mut after = g.clone();
            after.apply(Action::BidCards(cards)).unwrap();
            assert_eq!(bidding(&after).step, BidStep::Bluffs);
            assert_eq!(bidding(&after).stacks[opponents[0]].cards, cards);
        }
    }
}

#[test]
fn a_number_of_cards_that_cannot_make_the_cash_is_refused() {
    let (mut g, _) = auction(3, 10, 1);
    g.apply(Action::Bid(5)).unwrap();
    for cards in [0, 2, 6, 7, 100, u32::MAX] {
        refused(&mut g, Action::BidCards(cards));
    }
}

#[test]
fn no_more_than_two_bluff_cards_go_with_a_bid() {
    let (mut g, _) = auction(3, 10, 1);
    g.apply(Action::Bid(5)).unwrap();
    g.apply(Action::BidCards(3)).unwrap();
    for bluffs in [3, 4, u8::MAX] {
        refused(&mut g, Action::BidBluffs(bluffs));
    }
}

#[test]
fn each_step_of_a_bid_accepts_only_its_own_actions() {
    let (mut g, _) = auction(3, 10, 1);
    for action in [
        Action::BidCards(0),
        Action::BidCards(5),
        Action::BidBluffs(0),
        Action::Bid(11),
    ] {
        refused(&mut g, action);
    }

    g.apply(Action::Bid(5)).unwrap();
    for action in [
        Action::Bid(0),
        Action::Bid(5),
        Action::BidBluffs(0),
        Action::BidBluffs(2),
    ] {
        refused(&mut g, action);
    }

    g.apply(Action::BidCards(3)).unwrap();
    for action in [
        Action::Bid(0),
        Action::Bid(5),
        Action::BidCards(3),
        Action::BidCards(1),
    ] {
        refused(&mut g, action);
    }
}

#[test]
fn the_card_actions_are_refused_outside_a_delivery_auction() {
    let mut g = new_game(3, GameLength::Standard, 1);
    g.config.max_bid_revisions = 2;
    for action in [
        Action::BidCards(0),
        Action::BidCards(1),
        Action::BidBluffs(0),
        Action::BidBluffs(1),
    ] {
        refused(&mut g, action);
    }
}

#[test]
fn a_loan_can_be_taken_when_choosing_the_cash_but_not_while_laying_out_the_cards() {
    let (mut g, opponents) = auction(3, 10, 1);
    assert!(g.is_legal(Action::RequestLoan));

    g.apply(Action::Bid(5)).unwrap();
    refused(&mut g, Action::RequestLoan);
    g.apply(Action::BidCards(3)).unwrap();
    refused(&mut g, Action::RequestLoan);
    g.apply(Action::BidBluffs(0)).unwrap();

    assert!(g.is_legal(Action::RequestLoan));
    g.apply(Action::RequestLoan).unwrap();
    assert_eq!(cash(&g, opponents[1]), 20);
    assert!(g.is_legal(Action::Bid(20)));
    assert_eq!(g.check_invariants(), Ok(()));
}

// All the stacks

#[test]
fn the_next_bidder_lays_out_their_stack_once_the_one_before_is_down() {
    let (mut g, opponents) = auction(4, 10, 2);

    lay(&mut g, 5, 1, 0);
    let b = bidding(&g);
    assert_eq!(
        (b.bidder, b.step, b.stacks_down),
        (opponents[1], BidStep::Cash, false)
    );

    lay(&mut g, 2, 2, 2);
    let b = bidding(&g);
    assert_eq!(
        (b.bidder, b.step, b.stacks_down),
        (opponents[2], BidStep::Cash, false)
    );
    assert_eq!(
        b.stacks[opponents[0]],
        Stack {
            cards: 1,
            bluffs: 0
        }
    );
    assert_eq!(
        b.stacks[opponents[1]],
        Stack {
            cards: 2,
            bluffs: 2
        }
    );
}

#[test]
fn once_every_stack_is_down_the_bidders_look_them_over_starting_with_the_first() {
    let (mut g, opponents) = auction(4, 10, 1);

    lay(&mut g, 5, 1, 0);
    lay(&mut g, 2, 2, 2);
    lay(&mut g, 7, 4, 1);

    let b = bidding(&g);
    assert!(b.stacks_down);
    assert_eq!((b.bidder, b.step), (opponents[0], BidStep::Review));
    for (seat, bid) in [(opponents[0], 5), (opponents[1], 2), (opponents[2], 7)] {
        assert_eq!(b.bids[seat], bid, "player {seat}");
    }
    assert_eq!(b.bids[g.state.active_player], 0);
    assert_eq!(g.check_invariants(), Ok(()));
}

// Looking the stacks over

#[test]
fn the_stacks_are_looked_over_one_bidder_at_a_time_who_may_be_content_revise_or_borrow() {
    let (g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    assert_eq!(reviewer(&g), (opponents[0], BidStep::Review));
    assert_eq!(g.current_player(), opponents[0]);

    let mut offered = vec![Action::ReadyToReveal];
    offered.extend((0..=10).map(Action::Bid));
    offered.push(Action::RequestLoan);
    assert_eq!(g.legal_actions(), offered, "$5 on the table and $5 in hand");
}

#[test]
fn a_bidder_who_is_content_hands_the_review_to_the_next() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    let before = bidding(&g);

    g.apply(Action::ReadyToReveal).unwrap();

    let mut expected = before;
    expected.bidder = opponents[1];
    expected.ready[opponents[0]] = true;
    assert_eq!(bidding(&g), expected);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_stacks_are_revealed_once_everyone_is_content() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    g.apply(Action::ReadyToReveal).unwrap();
    assert!(
        matches!(g.phase, Phase::Bid(_)),
        "one bidder is still to say"
    );
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(decision(&g), (5, vec![opponents[0]]));
    assert_eq!(
        cash(&g, opponents[0]),
        5,
        "the winning bid stays on the table"
    );
    assert_eq!(cash(&g, opponents[1]), 10, "the other bid is taken back");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_revision_takes_the_stack_back_and_lays_out_another() {
    let (mut g, opponents) = reviewing(3, 10, 2, &[(5, 3, 1), (2, 2, 0)]);
    let [first, second] = [opponents[0], opponents[1]];
    let before = bidding(&g);

    g.apply(Action::Bid(7)).unwrap();

    let mut expected = before;
    expected.step = BidStep::Cards;
    expected.bids[first] = 7;
    expected.stacks[first] = Stack::default();
    expected.revisions[first] = 1;
    assert_eq!(bidding(&g), expected);
    assert_eq!(cash(&g, first), 3, "the $5 came back and $7 went down");
    assert_eq!(g.check_invariants(), Ok(()));

    let offered: Vec<_> = card_counts(7).into_iter().map(Action::BidCards).collect();
    assert_eq!(g.legal_actions(), offered);
    g.apply(Action::BidCards(2)).unwrap();
    assert_eq!(reviewer(&g), (first, BidStep::Bluffs));
    assert_eq!(g.legal_actions(), [0, 1, 2].map(Action::BidBluffs));
    g.apply(Action::BidBluffs(1)).unwrap();

    assert_eq!(reviewer(&g), (second, BidStep::Review));
    assert_eq!(
        content(&g),
        [first],
        "whoever revised has nothing more to say"
    );
    let b = bidding(&g);
    assert_eq!(b.bids[first], 7);
    assert_eq!(
        b.stacks[first],
        Stack {
            cards: 2,
            bluffs: 1
        }
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_revision_may_spend_the_cash_of_the_stack_it_replaces() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    refused(&mut g, Action::Bid(11));
    assert!(g.is_legal(Action::Bid(10)));
    g.apply(Action::Bid(10)).unwrap();

    assert_eq!(cash(&g, opponents[0]), 0);
    assert_eq!(bidding(&g).bids[opponents[0]], 10);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_stack_may_be_laid_out_again_the_same_way_or_differently() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    g.apply(Action::Bid(5)).unwrap();
    assert_eq!(g.legal_actions(), [1, 3, 4, 5].map(Action::BidCards));
    g.apply(Action::BidCards(1)).unwrap();
    g.apply(Action::BidBluffs(2)).unwrap();

    let b = bidding(&g);
    assert_eq!(
        b.stacks[opponents[0]],
        Stack {
            cards: 1,
            bluffs: 2
        }
    );
    assert_eq!((b.bids[opponents[0]], cash(&g, opponents[0])), (5, 5));
}

#[test]
fn a_revision_makes_the_others_look_the_stacks_over_again() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    let [first, second] = [opponents[0], opponents[1]];

    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(content(&g), [first]);
    g.apply(Action::Bid(3)).unwrap();
    assert!(content(&g).is_empty(), "the first bidder must look again");
    g.apply(Action::BidCards(2)).unwrap();
    g.apply(Action::BidBluffs(0)).unwrap();

    assert_eq!(reviewer(&g), (first, BidStep::Review));
    assert_eq!(content(&g), [second]);
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(decision(&g), (5, vec![first]));
}

#[test]
fn a_bidder_with_no_revision_left_is_not_asked_again() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    g.apply(Action::Bid(4)).unwrap();
    g.apply(Action::BidCards(2)).unwrap();
    g.apply(Action::BidBluffs(0)).unwrap();
    assert_eq!(reviewer(&g), (opponents[1], BidStep::Review));
    g.apply(Action::Bid(3)).unwrap();
    g.apply(Action::BidCards(2)).unwrap();
    g.apply(Action::BidBluffs(1)).unwrap();

    assert_eq!(decision(&g), (4, vec![opponents[0]]));
}

#[test]
fn after_a_revision_the_others_look_again_in_turn_order() {
    let (mut g, opponents) = reviewing(4, 10, 1, &[(1, 1, 0), (2, 2, 0), (3, 3, 0)]);
    let [first, second, third] = [opponents[0], opponents[1], opponents[2]];

    g.apply(Action::ReadyToReveal).unwrap();
    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(reviewer(&g), (third, BidStep::Review));
    g.apply(Action::Bid(4)).unwrap();
    g.apply(Action::BidCards(2)).unwrap();
    g.apply(Action::BidBluffs(0)).unwrap();

    assert_eq!(reviewer(&g), (first, BidStep::Review));
    assert_eq!(content(&g), [third]);
    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(reviewer(&g), (second, BidStep::Review));
    let mut content_now = vec![first, third];
    content_now.sort_unstable();
    assert_eq!(content(&g), content_now);
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(decision(&g), (4, vec![third]));
}

#[test]
fn a_bidder_can_revise_only_as_often_as_the_cap_allows() {
    let (mut g, opponents) = reviewing(3, 10, 2, &[(5, 3, 1), (2, 2, 0)]);

    change(&mut g, |b| b.revisions[opponents[0]] = 1);
    assert!(g.is_legal(Action::Bid(0)), "one revision is left");

    change(&mut g, |b| b.revisions[opponents[0]] = 2);
    for amount in [0, 5, 10] {
        refused(&mut g, Action::Bid(amount));
    }
}

#[test]
fn the_invariants_hold_through_revisions_that_use_up_the_cap() {
    let (mut g, _) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    lay(&mut g, 4, 2, 0);
    lay(&mut g, 3, 2, 1);

    assert!(matches!(g.phase, Phase::Decide(_)));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_revised_stacks_decide_like_bids_placed_once() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    g.apply(Action::Bid(1)).unwrap();
    g.apply(Action::BidCards(1)).unwrap();
    g.apply(Action::BidBluffs(2)).unwrap();
    g.apply(Action::Bid(6)).unwrap();
    g.apply(Action::BidCards(2)).unwrap();
    g.apply(Action::BidBluffs(0)).unwrap();

    let (mut twin, _) = auction(3, 10, 0);
    twin.apply(Action::Bid(1)).unwrap();
    twin.apply(Action::Bid(6)).unwrap();

    assert_eq!(decision(&g), (6, vec![opponents[1]]));
    assert_eq!(g.phase, twin.phase);
    assert_eq!(g.state, twin.state);
}

#[test]
fn being_content_is_only_possible_while_the_stacks_are_looked_over() {
    let (mut g, _) = auction(3, 10, 1);
    refused(&mut g, Action::ReadyToReveal);
    g.apply(Action::Bid(5)).unwrap();
    refused(&mut g, Action::ReadyToReveal);
    g.apply(Action::BidCards(3)).unwrap();
    refused(&mut g, Action::ReadyToReveal);

    let (mut g, _) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    g.apply(Action::Bid(5)).unwrap();
    refused(&mut g, Action::ReadyToReveal);
    g.apply(Action::BidCards(3)).unwrap();
    refused(&mut g, Action::ReadyToReveal);

    let (mut g, _) = auction(3, 10, 0);
    refused(&mut g, Action::ReadyToReveal);

    let mut g = new_game(3, GameLength::Standard, 1);
    g.config.max_bid_revisions = 2;
    refused(&mut g, Action::ReadyToReveal);
}

#[test]
fn a_loan_can_be_taken_while_looking_the_stacks_over() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    g.apply(Action::ReadyToReveal).unwrap();
    let before = bidding(&g);
    let second = opponents[1];
    assert_eq!(cash(&g, second), 8);

    assert!(g.is_legal(Action::RequestLoan));
    g.apply(Action::RequestLoan).unwrap();

    assert_eq!(cash(&g, second), 18);
    assert_eq!(bidding(&g), before, "no one is made to look again");
    assert!(
        g.is_legal(Action::Bid(20)),
        "$18 in hand and $2 on the table"
    );
    refused(&mut g, Action::Bid(21));
    assert_eq!(g.check_invariants(), Ok(()));
}

// The runoff

#[test]
fn a_tie_starts_a_runoff_that_begins_afresh_whatever_the_round_left_behind() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    let [first, second] = [opponents[0], opponents[1]];
    g.apply(Action::ReadyToReveal).unwrap();
    lay(&mut g, 5, 1, 2);
    g.apply(Action::ReadyToReveal).unwrap();

    let mut tied = [false; MAX_PLAYERS];
    tied[first] = true;
    tied[second] = true;
    assert_eq!(
        bidding(&g),
        Bidding::runoff(g.state.active_player, first, tied, 5)
    );
    assert_eq!((cash(&g, first), cash(&g, second)), (5, 5));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_runoff_lays_out_only_the_cash_added_to_the_bid_on_the_table() {
    let (mut g, opponents) = auction(3, 10, 1);
    lay(&mut g, 5, 1, 2);
    lay(&mut g, 5, 1, 2);
    all_content(&mut g);
    assert_eq!(reviewer(&g), (opponents[0], BidStep::Cash));

    let mut offered: Vec<_> = (0..=5).map(Action::Bid).collect();
    offered.push(Action::RequestLoan);
    assert_eq!(g.legal_actions(), offered, "$5 in hand");

    let mut nothing = g.clone();
    nothing.apply(Action::Bid(0)).unwrap();
    assert_eq!(nothing.legal_actions(), [Action::BidCards(0)]);

    g.apply(Action::Bid(2)).unwrap();
    let offered: Vec<_> = card_counts(2).into_iter().map(Action::BidCards).collect();
    assert_eq!(
        g.legal_actions(),
        offered,
        "the $5 down are not laid out again"
    );
    g.apply(Action::BidCards(2)).unwrap();
    assert_eq!(
        g.legal_actions(),
        [0, 1, 2].map(Action::BidBluffs),
        "the bluff cards are back"
    );
    g.apply(Action::BidBluffs(2)).unwrap();

    let b = bidding(&g);
    assert_eq!(b.bids[opponents[0]], 7);
    assert_eq!(
        b.stacks[opponents[0]],
        Stack {
            cards: 2,
            bluffs: 2
        }
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_runoff_stacks_are_looked_over_and_revealed_like_any_others() {
    let (mut g, opponents) = auction(3, 10, 1);
    first_round(&mut g, &[5, 5]);

    lay(&mut g, 3, 2, 0);
    lay(&mut g, 1, 1, 2);

    let b = bidding(&g);
    assert!(b.runoff && b.stacks_down);
    assert_eq!((b.bidder, b.step), (opponents[0], BidStep::Review));
    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(reviewer(&g), (opponents[1], BidStep::Review));
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(decision(&g), (8, vec![opponents[0]]));
    assert_eq!(
        cash(&g, opponents[0]),
        2,
        "the winning bid stays on the table"
    );
    assert_eq!(
        cash(&g, opponents[1]),
        10,
        "everything is taken back, the $5 too"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_runoff_revision_takes_back_only_the_cash_added_and_may_spend_it_again() {
    let (mut g, opponents) = auction(3, 10, 1);
    first_round(&mut g, &[5, 5]);
    let [first, second] = [opponents[0], opponents[1]];
    lay(&mut g, 3, 2, 0);
    lay(&mut g, 1, 1, 0);
    assert_eq!((cash(&g, first), bidding(&g).bids[first]), (2, 8));

    refused(&mut g, Action::Bid(6));
    assert!(
        g.is_legal(Action::Bid(5)),
        "the $2 in hand and the $3 in the stack"
    );
    g.apply(Action::Bid(5)).unwrap();

    let b = bidding(&g);
    assert_eq!(b.bids[first], 10, "the $5 of the first round stay down");
    assert_eq!(cash(&g, first), 0);
    assert_eq!(b.revisions[first], 1);
    assert_eq!(b.stacks[first], Stack::default());
    let offered: Vec<_> = card_counts(5).into_iter().map(Action::BidCards).collect();
    assert_eq!(g.legal_actions(), offered);
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::BidCards(1)).unwrap();
    g.apply(Action::BidBluffs(0)).unwrap();
    assert_eq!(reviewer(&g), (second, BidStep::Review));
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(decision(&g), (10, vec![first]));
    assert_eq!((cash(&g, first), cash(&g, second)), (0, 10));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_runoff_stack_can_be_taken_back_altogether_but_not_the_bid_beneath_it() {
    let (mut g, opponents) = auction(3, 10, 1);
    first_round(&mut g, &[5, 5]);
    lay(&mut g, 3, 2, 0);
    lay(&mut g, 1, 1, 0);

    g.apply(Action::Bid(0)).unwrap();

    assert_eq!(bidding(&g).bids[opponents[0]], 5);
    assert_eq!(cash(&g, opponents[0]), 5);
    assert_eq!(g.legal_actions(), [Action::BidCards(0)]);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_revisions_of_a_bidder_are_counted_afresh_in_the_runoff() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    g.apply(Action::ReadyToReveal).unwrap();
    lay(&mut g, 5, 1, 0);
    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(bidding(&g).revisions, [0; MAX_PLAYERS]);

    lay(&mut g, 1, 1, 0);
    lay(&mut g, 1, 1, 0);
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(reviewer(&g), (opponents[1], BidStep::Review));
    assert!(
        g.is_legal(Action::Bid(0)),
        "the one revision is theirs again"
    );
}

#[test]
fn the_bidders_outside_a_tie_sit_the_runoff_out_and_are_not_asked_to_look() {
    let (mut g, o) = auction(4, 10, 1);
    first_round(&mut g, &[5, 2, 5]);

    let b = bidding(&g);
    let mut expected = vec![o[0], o[2]];
    expected.sort_unstable();
    let contenders: Vec<_> = (0..MAX_PLAYERS)
        .filter(|&seat| b.contenders[seat])
        .collect();
    assert!(b.runoff);
    assert_eq!(contenders, expected);
    assert_eq!(cash(&g, o[1]), 10, "the lower bid was taken back");
    assert_eq!(g.check_invariants(), Ok(()));

    lay(&mut g, 1, 1, 0);
    assert_eq!(
        reviewer(&g),
        (o[2], BidStep::Cash),
        "the middle bidder sits out"
    );
    lay(&mut g, 2, 1, 0);
    assert_eq!(reviewer(&g), (o[0], BidStep::Review));
    g.apply(Action::ReadyToReveal).unwrap();
    assert_eq!(reviewer(&g), (o[2], BidStep::Review));
    lay(&mut g, 0, 0, 0);
    assert_eq!(reviewer(&g), (o[0], BidStep::Review));
    g.apply(Action::ReadyToReveal).unwrap();

    assert_eq!(decision(&g), (6, vec![o[0]]));
    assert_eq!(
        [cash(&g, o[0]), cash(&g, o[1]), cash(&g, o[2])],
        [4, 10, 10]
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_runoff_of_laid_out_stacks_decides_like_a_runoff_of_plain_bids() {
    let (mut g, opponents) = auction(3, 10, 2);
    first_round(&mut g, &[5, 5]);
    lay(&mut g, 4, 2, 1);
    lay(&mut g, 4, 3, 0);
    all_content(&mut g);

    let (mut twin, _) = auction(3, 10, 0);
    for amount in [5, 5, 4, 4] {
        twin.apply(Action::Bid(amount)).unwrap();
    }

    let mut tied = opponents.clone();
    tied.sort_unstable();
    assert_eq!(decision(&g), (9, tied), "there is only one runoff");
    assert_eq!(g.phase, twin.phase);
    assert_eq!(g.state, twin.state);
}

/// The stacks have just gone down. Each bidder looks them over in turn and is content, except the last,
/// who revises to `revised` and so makes the others look again. Returns who looked, in order.
fn review_order(g: &mut Game, opponents: &[PlayerId], revised: u32) -> Vec<PlayerId> {
    let mut looked = Vec::new();
    for _ in 1..opponents.len() {
        looked.push(bidding(g).bidder);
        g.apply(Action::ReadyToReveal).unwrap();
    }
    looked.push(bidding(g).bidder);
    lay(g, revised, card_counts(revised)[0], 0);
    while matches!(g.phase, Phase::Bid(b) if b.step == BidStep::Review) {
        looked.push(bidding(g).bidder);
        g.apply(Action::ReadyToReveal).unwrap();
    }
    looked
}

#[test]
fn the_stacks_are_looked_over_clockwise_from_the_deliverer_whoever_delivers() {
    for players in 3..=5 {
        for turns in 0..players {
            let context = format!("{players} players, the deliverer {turns} turns on");
            let (mut g, o) = auction_after(players, 10, 1, turns);
            let expected = [o.clone(), o[..o.len() - 1].to_vec()].concat();

            for _ in &o {
                lay(&mut g, 1, 1, 0);
            }
            assert_eq!(
                review_order(&mut g, &o, 1),
                expected,
                "{context}, round one"
            );
            assert!(
                matches!(g.phase, Phase::Bid(b) if b.runoff),
                "{context}: all tied"
            );

            for _ in &o {
                lay(&mut g, 0, 0, 0);
            }
            assert_eq!(review_order(&mut g, &o, 1), expected, "{context}, runoff");
            assert_eq!(decision(&g), (2, vec![o[o.len() - 1]]), "{context}");
            assert_eq!(g.check_invariants(), Ok(()), "{context}");
        }
    }
}

// Invariants

#[test]
fn the_invariants_flag_a_bidder_who_is_not_bidding() {
    let (mut g, _) = auction(3, 10, 1);
    let deliverer = g.state.active_player;

    change(&mut g, |b| b.bidder = deliverer);

    assert!(flagged(&g).contains("not a contender"));
}

#[test]
fn the_invariants_flag_cash_on_the_table_from_someone_who_is_not_bidding() {
    let (mut g, _) = auction(3, 10, 1);
    let deliverer = g.state.active_player;

    g.state.players[deliverer].cash -= 3;
    change(&mut g, |b| b.bids[deliverer] = 3);

    assert!(flagged(&g).contains("not bidding"));
}

#[test]
fn the_invariants_flag_cards_laid_out_by_someone_who_is_not_bidding() {
    let corruptions: [fn(&mut Stack); 2] = [|stack| stack.cards = 1, |stack| stack.bluffs = 1];
    for corrupt in corruptions {
        let (mut g, _) = auction(3, 10, 1);
        let deliverer = g.state.active_player;

        change(&mut g, |b| corrupt(&mut b.stacks[deliverer]));

        assert!(flagged(&g).contains("cards laid out"));
    }
}

#[test]
fn the_invariants_flag_less_on_the_table_than_the_round_began_with() {
    let (mut g, _) = auction(3, 10, 1);
    lay(&mut g, 5, 3, 1);

    change(&mut g, |b| b.base = 6);

    assert!(flagged(&g).contains("less than"));
}

#[test]
fn the_invariants_flag_cards_when_bids_cannot_be_revised() {
    let corruptions: [fn(&mut Bidding); 6] = [
        |b| b.step = BidStep::Cards,
        |b| b.step = BidStep::Review,
        |b| b.stacks_down = true,
        |b| b.stacks[b.bidder].bluffs = 1,
        |b| b.ready[b.bidder] = true,
        |b| b.revisions[b.bidder] = 1,
    ];
    for corrupt in corruptions {
        let (mut g, _) = auction(3, 10, 0);
        change(&mut g, corrupt);
        assert!(flagged(&g).contains("cannot be revised"));
    }
}

#[test]
fn the_invariants_flag_a_step_that_does_not_fit_the_stacks() {
    let (mut g, _) = auction(3, 10, 1);
    change(&mut g, |b| b.step = BidStep::Review);
    assert!(
        flagged(&g).contains("step"),
        "no one reviews before the stacks are down"
    );

    let (mut g, _) = auction(3, 10, 1);
    change(&mut g, |b| b.stacks_down = true);
    assert!(
        flagged(&g).contains("step"),
        "no one chooses cash once the stacks are down"
    );
}

#[test]
fn the_invariants_flag_a_stack_that_cannot_be_laid_out_as_the_cards_it_has() {
    let (mut g, opponents) = auction(3, 10, 1);
    lay(&mut g, 5, 3, 1);
    change(&mut g, |b| b.stacks[opponents[0]].cards = 2);
    assert!(
        flagged(&g).contains("cannot be laid out"),
        "a stack that is down"
    );

    let (mut g, opponents) = auction(3, 10, 1);
    g.apply(Action::Bid(5)).unwrap();
    g.apply(Action::BidCards(3)).unwrap();
    change(&mut g, |b| b.stacks[opponents[0]].cards = 2);
    assert!(
        flagged(&g).contains("cannot be laid out"),
        "a stack being laid out"
    );

    let (mut g, opponents) = auction(3, 10, 1);
    lay(&mut g, 5, 3, 1);
    g.apply(Action::Bid(2)).unwrap();
    change(&mut g, |b| b.stacks[opponents[0]].cards = 2);
    assert!(
        flagged(&g).contains("cannot be laid out"),
        "a stack that is down, while another is being laid out"
    );
}

#[test]
fn the_invariants_flag_more_bluff_cards_than_a_player_has() {
    let (mut g, opponents) = auction(3, 10, 1);
    lay(&mut g, 5, 3, 1);

    change(&mut g, |b| b.stacks[opponents[0]].bluffs = BLUFF_CARDS + 1);

    assert!(flagged(&g).contains("bluff"));
}

#[test]
fn the_invariants_flag_a_bidder_who_is_not_bidding_but_is_content_or_has_revised() {
    let corruptions: [fn(&mut Bidding, PlayerId); 2] = [
        |b, seat| b.ready[seat] = true,
        |b, seat| b.revisions[seat] = 1,
    ];
    for corrupt in corruptions {
        let (mut g, _) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
        let deliverer = g.state.active_player;

        change(&mut g, |b| corrupt(b, deliverer));

        assert!(flagged(&g).contains("not bidding but is content or has revised"));
    }
}

#[test]
fn the_invariants_flag_more_revisions_than_the_cap_allows() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);

    change(&mut g, |b| b.revisions[opponents[1]] = 1);
    assert_eq!(
        g.check_invariants(),
        Ok(()),
        "one revision is within the cap"
    );

    change(&mut g, |b| b.revisions[opponents[1]] = 2);
    assert!(flagged(&g).contains("times, at most 1"));
}

#[test]
fn the_invariants_flag_revisions_before_every_stack_is_down() {
    let (mut g, opponents) = auction(3, 10, 2);
    lay(&mut g, 5, 3, 1);
    change(&mut g, |b| b.revisions[opponents[0]] = 1);
    assert!(flagged(&g).contains("revised before"));
}

#[test]
fn the_invariants_flag_a_bidder_who_is_content_when_the_stacks_are_not_being_looked_over() {
    for who in 0..2 {
        let (mut g, opponents) = auction(3, 10, 1);
        lay(&mut g, 5, 3, 1);
        change(&mut g, |b| b.ready[opponents[who]] = true);
        assert!(
            flagged(&g).contains("content although"),
            "choosing the cash"
        );
        g.apply(Action::Bid(2)).unwrap();
        assert!(
            flagged(&g).contains("content although"),
            "choosing the cards"
        );
        g.apply(Action::BidCards(2)).unwrap();
        assert!(
            flagged(&g).contains("content although"),
            "choosing the bluffs"
        );

        let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
        g.apply(Action::Bid(5)).unwrap();
        change(&mut g, |b| b.ready[opponents[who]] = true);
        assert!(
            flagged(&g).contains("content although"),
            "choosing the new cards"
        );
        g.apply(Action::BidCards(3)).unwrap();
        assert!(
            flagged(&g).contains("content although"),
            "choosing the new bluffs"
        );
    }
}

#[test]
fn the_invariants_flag_a_reviewer_who_has_nothing_to_decide() {
    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    change(&mut g, |b| b.ready[opponents[0]] = true);
    assert!(flagged(&g).contains("reviewer"), "content already");

    let (mut g, opponents) = reviewing(3, 10, 1, &[(5, 3, 1), (2, 2, 0)]);
    change(&mut g, |b| b.revisions[opponents[0]] = 1);
    assert!(flagged(&g).contains("reviewer"), "no revision left");
}
