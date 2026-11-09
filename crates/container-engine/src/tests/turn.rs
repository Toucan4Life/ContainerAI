use std::collections::HashSet;

use super::{actions_while_playing, new_game};
use crate::action::{Action, ActionError, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;

fn game() -> Game {
    new_game(3, GameLength::Standard, 1)
}

fn in_arrange(g: &Game) -> bool {
    matches!(g.phase, Phase::Arrange(_))
}

/// Repricing the factory district and keeping the first arrangement offered.
fn reprice_pass(g: &mut Game) {
    g.apply(Action::Reprice(District::Factory)).unwrap();
    while in_arrange(g) {
        let a = g.legal_actions()[0];
        g.apply(a).unwrap();
    }
}

fn game_with_factory_hand(hand: ColorCounts) -> (Game, usize) {
    let mut g = game();
    let p = g.state.active_player;
    g.state.players[p].factory_lots = Default::default();
    g.state.players[p].factory_lots[0] = hand;
    g.apply(Action::Reprice(District::Factory)).unwrap();
    (g, p)
}

fn game_with_harbor_hand(hand: ColorCounts) -> (Game, usize) {
    let mut g = game();
    let p = g.state.active_player;
    g.state.players[p].harbor_lots[0] = hand;
    g.apply(Action::Reprice(District::Harbor)).unwrap();
    (g, p)
}

fn place_actions(lots: usize) -> Vec<Action> {
    (0..lots).map(Action::Place).collect()
}

#[test]
fn a_new_game_starts_at_the_main_node_for_the_first_player() {
    let g = game();
    assert_eq!(g.current_player(), g.state.first_player);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert!(!g.is_terminal());
}

#[test]
fn main_node_offers_factory_reprice_but_not_an_empty_harbor() {
    let g = game();
    let actions = g.legal_actions();
    assert!(actions.contains(&Action::Reprice(District::Factory)));
    assert!(!actions.contains(&Action::Reprice(District::Harbor)));
}

#[test]
fn harbor_reprice_is_offered_once_the_harbor_holds_a_container() {
    let mut g = game();
    let p = g.state.active_player;
    g.state.players[p].harbor_lots[0][Color::Red] = 1;
    assert!(
        g.legal_actions()
            .contains(&Action::Reprice(District::Harbor))
    );
}

#[test]
fn repricing_picks_up_the_district_and_places_containers_one_by_one() {
    let mut g = game();
    let p = g.state.active_player;
    g.apply(Action::Reprice(District::Factory)).unwrap();

    assert!(
        g.state.players[p]
            .factory_lots
            .iter()
            .all(ColorCounts::is_empty)
    );
    assert_eq!(g.check_invariants(), Ok(()), "the hand is still counted");
    assert_eq!(g.legal_actions(), place_actions(4));
    assert_eq!(g.current_player(), p);

    g.apply(Action::Place(3)).unwrap();
    assert_eq!(g.state.players[p].factory_lots[3].total(), 1);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.current_player(), p);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn repricing_an_empty_district_is_a_pass_that_uses_an_action() {
    let mut g = game();
    let p = g.state.active_player;
    g.state.players[p].factory_lots = Default::default();
    g.state.supply.containers[Color::Red] += 1;
    g.apply(Action::Reprice(District::Factory)).unwrap();
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn two_actions_end_the_turn_and_play_passes_clockwise() {
    let mut g = game();
    let first = g.state.first_player;
    reprice_pass(&mut g);
    assert_eq!(g.current_player(), first);
    assert_eq!(g.turn.actions_left, 1);
    reprice_pass(&mut g);
    assert_eq!(g.current_player(), (first + 1) % 3);
    assert_eq!(g.state.active_player, (first + 1) % 3);
    assert_eq!(g.turn.actions_left, 2);
}

#[test]
fn turn_order_wraps_around_the_table() {
    for players in 3..=5 {
        let mut g = new_game(players, GameLength::Standard, 2);
        let first = g.state.first_player;
        let mut order = Vec::new();
        for _ in 0..=players {
            order.push(g.current_player());
            reprice_pass(&mut g);
            reprice_pass(&mut g);
        }
        let expected: Vec<_> = (0..=players).map(|i| (first + i) % players).collect();
        assert_eq!(order, expected);
    }
}

#[test]
fn illegal_actions_are_rejected_without_changing_the_game() {
    let mut g = game();
    let before = g.clone();
    for action in [Action::Reprice(District::Harbor), Action::Place(0)] {
        assert_eq!(g.apply(action), Err(ActionError::Illegal(action)));
    }
    assert_eq!(g, before);
}

#[test]
fn main_actions_are_illegal_while_arranging() {
    let (mut g, _) = game_with_factory_hand(ColorCounts([1, 0, 0, 0, 0]));
    let before = g.clone();
    assert!(g.apply(Action::Reprice(District::Factory)).is_err());
    assert_eq!(g, before);
}

#[test]
fn colors_are_placed_in_cycle_order() {
    let (mut g, p) = game_with_factory_hand(ColorCounts([1, 1, 0, 0, 0]));
    g.apply(Action::Place(2)).unwrap();
    assert_eq!(g.state.players[p].factory_lots[2][Color::Red], 1);
    assert_eq!(g.state.players[p].factory_lots[2][Color::Blue], 0);
}

#[test]
fn containers_of_one_color_never_move_to_a_lower_lot_than_the_previous_one() {
    let (mut g, _) = game_with_factory_hand(ColorCounts([2, 0, 0, 0, 0]));
    g.apply(Action::Place(2)).unwrap();
    assert_eq!(g.legal_actions(), vec![Action::Place(2), Action::Place(3)]);
    assert!(g.apply(Action::Place(1)).is_err());
}

#[test]
fn each_new_color_may_use_any_lot_again() {
    let (mut g, _) = game_with_factory_hand(ColorCounts([1, 1, 0, 0, 0]));
    g.apply(Action::Place(3)).unwrap();
    assert_eq!(g.legal_actions(), place_actions(4));
}

fn all_arrangements(g: &Game, player: usize, district: District) -> Vec<Vec<ColorCounts>> {
    fn go(g: &Game, player: usize, district: District, out: &mut Vec<Vec<ColorCounts>>) {
        if !in_arrange(g) {
            out.push(g.state.players[player].lots(district).to_vec());
            return;
        }
        for a in g.legal_actions() {
            let mut next = g.clone();
            next.apply(a).unwrap();
            go(&next, player, district, out);
        }
    }
    let mut out = Vec::new();
    go(g, player, district, &mut out);
    out
}

fn choose(n: usize, k: usize) -> usize {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

/// Multisets of `n` same-colored containers over `lots` lots, multiplied across colors.
fn expected_arrangements(hand: &ColorCounts, lots: usize) -> usize {
    hand.0
        .iter()
        .map(|&n| choose(lots + usize::from(n) - 1, usize::from(n)))
        .product()
}

fn assert_each_arrangement_has_one_sequence(hand: ColorCounts, district: District) {
    let (g, p) = match district {
        District::Factory => game_with_factory_hand(hand),
        District::Harbor => game_with_harbor_hand(hand),
    };
    let all = all_arrangements(&g, p, district);
    let distinct: HashSet<_> = all.iter().cloned().collect();
    assert_eq!(
        distinct.len(),
        all.len(),
        "duplicate sequences for {hand:?}"
    );
    assert_eq!(
        all.len(),
        expected_arrangements(&hand, district.lot_count()),
        "{hand:?} in {district:?}"
    );
    for lots in &all {
        let total: u32 = lots.iter().map(ColorCounts::total).sum();
        assert_eq!(total, hand.total());
    }
}

#[test]
fn every_factory_arrangement_is_reachable_by_exactly_one_action_sequence() {
    for hand in [
        ColorCounts([2, 0, 0, 0, 0]),
        ColorCounts([3, 0, 0, 0, 0]),
        ColorCounts([2, 1, 0, 0, 0]),
        ColorCounts([1, 1, 1, 0, 0]),
        ColorCounts([2, 2, 0, 0, 0]),
    ] {
        assert_each_arrangement_has_one_sequence(hand, District::Factory);
    }
}

#[test]
fn every_harbor_arrangement_is_reachable_by_exactly_one_action_sequence() {
    for hand in [ColorCounts([1, 1, 0, 0, 0]), ColorCounts([0, 2, 0, 0, 1])] {
        assert_each_arrangement_has_one_sequence(hand, District::Harbor);
    }
}

#[test]
fn the_arrangement_count_formula_matches_known_values() {
    assert_eq!(expected_arrangements(&ColorCounts([2, 0, 0, 0, 0]), 4), 10);
    assert_eq!(expected_arrangements(&ColorCounts([1, 1, 0, 0, 0]), 5), 25);
    assert_eq!(expected_arrangements(&ColorCounts([1, 1, 1, 0, 0]), 4), 64);
}

#[test]
fn random_play_keeps_invariants_and_every_legal_action_applies() {
    for players in 3..=5 {
        let mut g = new_game(players, GameLength::Standard, 7);
        let mut rng = Rng::new(99);
        for _ in 0..400 {
            let Some(legal) = actions_while_playing(&g) else {
                break;
            };
            for &a in &legal {
                let mut next = g.clone();
                next.apply(a).unwrap();
                assert_eq!(next.check_invariants(), Ok(()), "after {a:?}");
            }
            let pick = legal[rng.below(legal.len() as u32) as usize];
            g.apply(pick).unwrap();
        }
    }
}
