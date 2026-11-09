use super::delivery::{arrive, at_decision, bid_all, choices, delivery_game};
use super::{
    actions_while_playing, in_arrange, new_game, pass_turn, set_cash, set_holding, set_loans,
    set_lots, set_scoring_area, set_ship, set_warehouses,
};
use crate::action::{Action, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;
use crate::state::PlayerId;

fn cash(g: &Game, p: PlayerId) -> u32 {
    g.state.players[p].cash
}

fn loans(g: &Game, p: PlayerId) -> u8 {
    g.state.players[p].loans
}

fn borrow(g: &mut Game) {
    g.apply(Action::RequestLoan).unwrap();
}

fn repay(g: &mut Game) {
    g.apply(Action::RepayLoan).unwrap();
}

/// A fresh three-player game in which the first player has `held` cash.
fn game_with_cash(held: u32) -> (Game, PlayerId) {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = g.current_player();
    set_cash(&mut g, me, held);
    (g, me)
}

fn next_player(g: &Game) -> PlayerId {
    (g.current_player() + 1) % g.state.num_players
}

// Requesting a loan

#[test]
fn a_loan_is_ten_dollars_from_the_supply() {
    let (mut g, me) = game_with_cash(20);
    let minted = g.state.cash_minted;

    borrow(&mut g);

    assert_eq!(cash(&g, me), 30);
    assert_eq!(loans(&g, me), 1);
    assert_eq!(g.state.cash_minted - minted, 10);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn requesting_a_loan_is_a_free_action() {
    let (mut g, me) = game_with_cash(20);

    borrow(&mut g);

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert_eq!(g.current_player(), me);
}

#[test]
fn two_loans_can_be_requested_in_the_same_turn() {
    let (mut g, me) = game_with_cash(0);

    borrow(&mut g);
    borrow(&mut g);

    assert_eq!((cash(&g, me), loans(&g, me)), (20, 2));
}

#[test]
fn nobody_can_hold_more_than_two_loans() {
    let (mut g, _) = game_with_cash(0);
    borrow(&mut g);
    borrow(&mut g);
    let before = g.clone();

    assert!(!g.is_legal(Action::RequestLoan));
    assert!(g.apply(Action::RequestLoan).is_err());
    assert_eq!(g, before);
}

#[test]
fn the_invariants_flag_a_player_owing_more_than_two_loans() {
    let (mut g, me) = game_with_cash(20);
    set_loans(&mut g, me, 2);
    assert_eq!(g.check_invariants(), Ok(()));

    set_loans(&mut g, me, 3);

    assert!(g.check_invariants().is_err());
}

#[test]
fn a_loan_can_be_taken_right_after_repaying_one() {
    let (mut g, me) = game_with_cash(100);
    set_loans(&mut g, me, 2);
    assert!(!g.is_legal(Action::RequestLoan));

    repay(&mut g);

    assert!(g.is_legal(Action::RequestLoan));
}

#[test]
fn a_third_request_in_one_turn_is_refused_even_after_repaying() {
    let (mut g, me) = game_with_cash(0);
    borrow(&mut g);
    repay(&mut g);
    borrow(&mut g);

    assert_eq!(loans(&g, me), 1, "there is room for another loan");
    assert!(!g.is_legal(Action::RequestLoan), "two requests a turn");
    assert!(g.is_legal(Action::RepayLoan));
}

// Repaying a loan

#[test]
fn repaying_returns_ten_dollars_to_the_supply_and_clears_the_loan() {
    let (mut g, me) = game_with_cash(20);
    set_loans(&mut g, me, 1);
    let burned = g.state.cash_burned;

    repay(&mut g);

    assert_eq!(cash(&g, me), 10);
    assert_eq!(loans(&g, me), 0);
    assert_eq!(g.state.cash_burned - burned, 10);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn repaying_is_a_free_action() {
    let (mut g, me) = game_with_cash(20);
    set_loans(&mut g, me, 1);

    repay(&mut g);

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert_eq!(g.current_player(), me);
}

#[test]
fn repaying_needs_an_outstanding_loan() {
    let (g, _) = game_with_cash(50);
    assert!(!g.is_legal(Action::RepayLoan));
}

#[test]
fn repaying_needs_ten_dollars_in_hand() {
    for (held, allowed) in [(9, false), (10, true)] {
        let (mut g, me) = game_with_cash(held);
        set_loans(&mut g, me, 1);
        assert_eq!(g.is_legal(Action::RepayLoan), allowed, "cash {held}");
    }
}

#[test]
fn the_full_ten_dollars_is_repaid_whatever_the_interest_paid() {
    let (mut g, me) = game_with_cash(20);
    set_loans(&mut g, me, 2);

    repay(&mut g);
    repay(&mut g);

    assert_eq!((cash(&g, me), loans(&g, me)), (0, 0));
}

#[test]
fn a_third_repayment_in_one_turn_is_refused() {
    let (mut g, me) = game_with_cash(100);
    set_loans(&mut g, me, 2);
    repay(&mut g);
    repay(&mut g);
    borrow(&mut g);

    assert_eq!(loans(&g, me), 1);
    assert!(!g.is_legal(Action::RepayLoan), "two repayments a turn");
    assert!(g.is_legal(Action::RequestLoan));
}

#[test]
fn the_allowances_start_afresh_every_turn() {
    let (mut g, me) = game_with_cash(100);
    set_loans(&mut g, me, 2);
    for action in [
        Action::RepayLoan,
        Action::RepayLoan,
        Action::RequestLoan,
        Action::RequestLoan,
    ] {
        g.apply(action).unwrap();
    }
    for _ in 0..3 {
        pass_turn(&mut g);
    }

    assert_eq!(g.current_player(), me);
    for action in [
        Action::RepayLoan,
        Action::RepayLoan,
        Action::RequestLoan,
        Action::RequestLoan,
    ] {
        g.apply(action).unwrap();
    }
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn loans_cannot_be_requested_or_repaid_in_the_middle_of_an_action() {
    let (mut g, me) = game_with_cash(50);
    set_loans(&mut g, me, 1);
    let other = next_player(&g);

    let mut reprice = g.clone();
    reprice.apply(Action::Reprice(District::Factory)).unwrap();
    assert!(in_arrange(&reprice));
    assert!(!reprice.is_legal(Action::RequestLoan));
    assert!(!reprice.is_legal(Action::RepayLoan));

    g.apply(Action::FactoryPurchase(other)).unwrap();
    assert!(matches!(g.phase, Phase::Buy(_)));
    assert!(!g.is_legal(Action::RequestLoan));
    assert!(!g.is_legal(Action::RepayLoan));
}

// Loans away from the main node

#[test]
fn a_bidder_may_take_a_loan_before_bidding() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();
    let (phase, turn) = (g.phase, g.turn);

    borrow(&mut g);

    assert_eq!(g.current_player(), bidder, "the bid is still to be placed");
    assert_eq!(g.phase, phase);
    assert_eq!(g.turn, turn, "the deliverer's allowances are not touched");
    assert_eq!((cash(&g, bidder), loans(&g, bidder)), (20, 1));
    let expected: Vec<Action> = (0..=20).map(Action::Bid).collect();
    assert_eq!(choices(&g), expected);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_bidder_can_take_two_loans_but_not_three() {
    let (mut g, _) = delivery_game(3, 10);
    arrive(&mut g);
    let bidder = g.current_player();

    borrow(&mut g);
    borrow(&mut g);

    assert_eq!((cash(&g, bidder), loans(&g, bidder)), (30, 2));
    assert!(!g.is_legal(Action::RequestLoan));
}

#[test]
fn a_bidders_loans_do_not_depend_on_the_loans_the_deliverer_took_that_turn() {
    let (mut g, me) = delivery_game(3, 10);
    borrow(&mut g);
    borrow(&mut g);
    let bidders = arrive(&mut g);
    assert_eq!(g.current_player(), bidders[0]);

    borrow(&mut g);

    assert_eq!((cash(&g, bidders[0]), loans(&g, bidders[0])), (20, 1));
    assert_eq!(loans(&g, me), 2);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_bidder_cannot_repay_a_loan() {
    let (mut g, _) = delivery_game(3, 20);
    arrive(&mut g);
    let bidder = g.current_player();
    set_loans(&mut g, bidder, 1);

    assert!(!g.is_legal(Action::RepayLoan));
}

#[test]
fn a_bidder_in_a_runoff_may_take_a_loan_too() {
    let (mut g, _, o) = at_decision(3, 10, &[5, 5]);
    assert_eq!(g.current_player(), o[0], "the runoff is under way");

    borrow(&mut g);

    assert_eq!(g.current_player(), o[0]);
    let expected: Vec<Action> = (0..=15).map(Action::Bid).collect();
    assert_eq!(choices(&g), expected);
}

#[test]
fn the_deliverer_may_take_a_loan_to_afford_a_buyout() {
    let (mut g, me, _) = at_decision(3, 10, &[4, 8]);
    set_cash(&mut g, me, 5);
    assert!(!g.is_legal(Action::BuyOut));

    borrow(&mut g);

    assert_eq!((cash(&g, me), loans(&g, me)), (15, 1));
    assert!(g.is_legal(Action::BuyOut));
    assert!(matches!(g.phase, Phase::Decide(_)));
    assert_eq!(g.current_player(), me);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_deliverer_cannot_repay_a_loan_before_deciding() {
    let (mut g, me, _) = at_decision(3, 10, &[4, 8]);
    set_loans(&mut g, me, 1);

    assert!(!g.is_legal(Action::RepayLoan));
}

#[test]
fn the_deliverers_requests_that_turn_also_count_at_the_decision() {
    let (mut g, me) = delivery_game(3, 10);
    set_cash(&mut g, me, 100);
    set_loans(&mut g, me, 1);
    for action in [
        Action::RepayLoan,
        Action::RequestLoan,
        Action::RepayLoan,
        Action::RequestLoan,
    ] {
        g.apply(action).unwrap();
    }
    arrive(&mut g);
    bid_all(&mut g, &[1, 2]);

    assert!(matches!(g.phase, Phase::Decide(_)));
    assert_eq!(loans(&g, me), 1, "there is room for another loan");
    assert!(!g.is_legal(Action::RequestLoan), "two requests a turn");
}

// Interest

#[test]
fn interest_is_a_dollar_per_loan_paid_to_the_bank_when_the_turn_begins() {
    for (owed, bank) in [(1, [2, 2, 3]), (2, [2, 3, 3])] {
        let mut g = new_game(3, GameLength::Standard, 1);
        let debtor = next_player(&g);
        set_loans(&mut g, debtor, owed);
        assert_eq!(g.state.bank.cash_lots, [1, 2, 3]);

        pass_turn(&mut g);

        assert_eq!(g.current_player(), debtor);
        assert_eq!(cash(&g, debtor), 20 - u32::from(owed), "{owed} loans");
        assert_eq!(g.state.bank.cash_lots, bank, "{owed} loans");
        assert_eq!(g.check_invariants(), Ok(()));
    }
}

#[test]
fn interest_is_settled_before_the_player_can_do_anything() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let debtor = next_player(&g);
    set_loans(&mut g, debtor, 2);

    pass_turn(&mut g);

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert_eq!(cash(&g, debtor), 18);
}

#[test]
fn a_player_without_loans_pays_no_interest() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let next = next_player(&g);

    pass_turn(&mut g);

    assert_eq!(cash(&g, next), 20);
    assert_eq!(g.state.bank.cash_lots, [1, 2, 3]);
}

#[test]
fn only_the_player_whose_turn_begins_pays() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = g.current_player();
    let (first, second) = (next_player(&g), (me + 2) % 3);
    set_loans(&mut g, first, 2);
    set_loans(&mut g, second, 2);

    pass_turn(&mut g);

    assert_eq!(cash(&g, first), 18);
    assert_eq!(cash(&g, second), 20, "their turn is not here yet");
    assert_eq!(cash(&g, me), 20);
}

#[test]
fn interest_falls_due_at_the_start_of_every_turn_and_always_starts_at_lot_one() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let me = g.current_player();
    set_loans(&mut g, me, 1);

    for round in 1..=3 {
        for _ in 0..3 {
            pass_turn(&mut g);
        }
        assert_eq!(g.current_player(), me);
        assert_eq!(cash(&g, me), 20 - round);
    }

    assert_eq!(g.state.bank.cash_lots, [4, 2, 3]);
}

#[test]
fn interest_is_also_paid_when_the_turn_before_ended_with_a_delivery() {
    let (mut g, me) = delivery_game(3, 10);
    let opponents = arrive(&mut g);
    let next = opponents[0];
    set_loans(&mut g, next, 1);
    bid_all(&mut g, &[0, 0]);
    bid_all(&mut g, &[0, 0]);

    g.apply(Action::Accept(next)).unwrap();

    assert_eq!(g.current_player(), next);
    assert_ne!(next, me);
    assert_eq!(cash(&g, next), 9);
    assert_eq!(g.state.bank.cash_lots, [2, 2, 3]);
    assert_eq!(g.check_invariants(), Ok(()));
}

// Interest that cannot be paid

#[test]
fn a_player_who_cannot_pay_interest_must_take_another_loan_to_pay_it() {
    let mut g = new_game(3, GameLength::Standard, 1);
    let debtor = next_player(&g);
    set_loans(&mut g, debtor, 1);
    set_cash(&mut g, debtor, 0);
    let minted = g.state.cash_minted;

    pass_turn(&mut g);

    assert_eq!(g.current_player(), debtor);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(loans(&g, debtor), 2);
    assert_eq!(cash(&g, debtor), 9, "$10 borrowed, $1 paid in interest");
    assert_eq!(g.state.cash_minted - minted, 10);
    assert_eq!(g.state.bank.cash_lots, [2, 2, 3]);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn exact_change_for_the_interest_needs_no_new_loan() {
    for owed in [1, 2] {
        let mut g = new_game(3, GameLength::Standard, 1);
        let debtor = next_player(&g);
        set_loans(&mut g, debtor, owed);
        set_cash(&mut g, debtor, u32::from(owed));

        pass_turn(&mut g);

        assert_eq!(loans(&g, debtor), owed, "{owed} loans");
        assert_eq!(cash(&g, debtor), 0, "{owed} loans");
        assert_eq!(g.phase, Phase::Main, "{owed} loans");
        assert_eq!(g.check_invariants(), Ok(()));
    }
}

// Whole games

/// Plays at random, taking out a loan whenever the dice say so, and checks every position on the way.
/// Returns the containers the Bank seized from defaulters.
fn play_borrowing_at_random(mut g: Game, seed: u64, steps: usize) -> u32 {
    let mut rng = Rng::new(seed);
    let mut seized = 0;
    for _ in 0..steps {
        let Some(legal) = actions_while_playing(&g) else {
            break;
        };
        let pick = if legal.contains(&Action::RequestLoan) && rng.below(3) == 0 {
            Action::RequestLoan
        } else {
            legal[rng.below(legal.len() as u32) as usize]
        };
        let before: u32 = g.state.bank.container_lots.iter().map(|l| l.total()).sum();
        g.apply(pick).unwrap();
        let after: u32 = g.state.bank.container_lots.iter().map(|l| l.total()).sum();
        seized += after.saturating_sub(before);
        assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
        for p in g.state.players() {
            assert!(p.loans <= 2);
        }
    }
    seized
}

#[test]
fn random_play_with_loans_keeps_every_invariant() {
    let mut total = 0;
    for seed in 0..6 {
        for players in 3..=5 {
            let g = new_game(players, GameLength::Short, seed);
            total += play_borrowing_at_random(g, seed + 30, 1500);
        }
    }
    assert!(total > 100, "only {total} containers were ever seized");
}

/// Owes two loans, has no cash, and has containers in every place the Bank can seize from.
fn make_broke(g: &mut Game, p: PlayerId) {
    let color = |k: usize| Color::from_index((p + k) % 5);
    let pair = |a: Color, b: Color| {
        let mut counts = ColorCounts::default();
        counts[a] += 1;
        counts[b] += 1;
        counts
    };
    set_cash(g, p, 0);
    set_loans(g, p, 2);
    set_scoring_area(g, p, pair(color(0), color(1)));
    set_ship(g, p, pair(color(1), color(2)));
    set_holding(g, p, pair(color(2), color(3)));
    set_warehouses(g, p, 2);
    set_lots(
        g,
        p,
        District::Harbor,
        &[(0, color(3), 1), (1, color(4), 1)],
    );
}

#[test]
fn random_play_by_broke_borrowers_keeps_every_invariant_through_many_defaults() {
    let mut total = 0;
    for seed in 0..6 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            for p in 0..players {
                make_broke(&mut g, p);
            }
            total += play_borrowing_at_random(g, seed + 70, 600);
        }
    }
    assert!(total > 300, "only {total} containers were ever seized");
}
