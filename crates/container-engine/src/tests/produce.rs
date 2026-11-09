use std::collections::HashSet;

use super::{
    actions_while_playing, drain_supply, finish_arrange, new_game, set_factories, set_factory_stock,
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

/// The active player owns exactly these factories and holds no containers.
fn game_with(factories: &[Color]) -> (Game, PlayerId) {
    let mut g = game();
    let p = g.current_player();
    set_factories(&mut g, p, factories);
    set_factory_stock(&mut g, p, ColorCounts::default());
    (g, p)
}

fn factory_tally(g: &Game, p: PlayerId) -> ColorCounts {
    let mut tally = ColorCounts::default();
    for lot in &g.state.players[p].factory_lots {
        tally.add_all(lot);
    }
    tally
}

fn hand(g: &Game) -> ColorCounts {
    match g.phase {
        Phase::Arrange(a) => a.hand,
        other => panic!("expected an arrangement, got {other:?}"),
    }
}

fn end_turn_by_repricing(g: &mut Game) {
    let p = g.current_player();
    while g.current_player() == p {
        g.apply(Action::Reprice(District::Factory)).unwrap();
        finish_arrange(g);
    }
}

/// The hand each complete sequence of color choices ends in. Every selection node must offer a choice.
fn collect_hands(g: &Game, out: &mut Vec<ColorCounts>) {
    match g.phase {
        Phase::Produce(_) => {
            let legal = g.legal_actions();
            assert!(!legal.is_empty(), "dead end while choosing colors");
            for a in legal {
                let mut next = g.clone();
                next.apply(a).unwrap();
                collect_hands(&next, out);
            }
        }
        Phase::Arrange(a) => out.push(a.hand),
        other => panic!("unexpected phase {other:?}"),
    }
}

#[test]
fn the_player_on_the_right_is_the_previous_seat() {
    for players in 3..=5 {
        let g = new_game(players, GameLength::Standard, 1);
        assert_eq!(g.state.player_on_right(0), players - 1);
        for seat in 1..players {
            assert_eq!(g.state.player_on_right(seat), seat - 1);
        }
    }
}

#[test]
fn produce_is_offered_at_the_main_node() {
    assert!(game().legal_actions().contains(&Action::Produce));
}

#[test]
fn the_union_wages_go_to_the_player_on_the_right() {
    for players in 3..=5 {
        for seat in 0..players {
            let mut g = new_game(players, GameLength::Standard, 3);
            g.state.active_player = seat;
            let right = g.state.player_on_right(seat);
            let before: Vec<u32> = g.state.players().iter().map(|p| p.cash).collect();

            g.apply(Action::Produce).unwrap();

            for (i, p) in g.state.players().iter().enumerate() {
                let expected = if i == seat {
                    before[i] - 1
                } else if i == right {
                    before[i] + 1
                } else {
                    before[i]
                };
                assert_eq!(
                    p.cash, expected,
                    "{players} players, seat {seat}, player {i}"
                );
            }
            assert_eq!(g.check_invariants(), Ok(()));
        }
    }
}

#[test]
fn wages_are_one_dollar_however_many_containers_are_produced() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue, Color::Yellow, Color::White]);
    let right = g.state.player_on_right(p);
    let (mine, theirs) = (g.state.players[p].cash, g.state.players[right].cash);

    g.apply(Action::Produce).unwrap();

    assert_eq!(g.state.players[p].cash, mine - 1);
    assert_eq!(g.state.players[right].cash, theirs + 1);
    assert_eq!(hand(&g).total(), 4);
}

#[test]
fn produce_needs_one_dollar_for_the_wages() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].cash = 0;
    assert!(!g.is_legal(Action::Produce));
    assert!(!g.legal_actions().contains(&Action::Produce));
    assert!(g.apply(Action::Produce).is_err());

    g.state.players[p].cash = 1;
    assert!(g.is_legal(Action::Produce));
}

#[test]
fn each_factory_yields_one_container_of_its_color() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue]);
    let supply = g.state.supply.containers;

    g.apply(Action::Produce).unwrap();

    let Phase::Arrange(a) = g.phase else {
        panic!("expected an arrangement, got {:?}", g.phase);
    };
    assert_eq!(a.district, District::Factory);
    assert_eq!(a.hand, ColorCounts([1, 1, 0, 0, 0]));
    assert_eq!(
        g.state.supply.containers[Color::Red],
        supply[Color::Red] - 1
    );
    assert_eq!(
        g.state.supply.containers[Color::Blue],
        supply[Color::Blue] - 1
    );
    assert_eq!(
        g.state.supply.containers[Color::Green],
        supply[Color::Green]
    );
    assert_eq!(
        g.check_invariants(),
        Ok(()),
        "containers in hand still count"
    );

    finish_arrange(&mut g);
    assert_eq!(factory_tally(&g, p), ColorCounts([1, 1, 0, 0, 0]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_new_containers_are_priced_together_with_the_old_ones() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 1, 0, 0]));

    g.apply(Action::Produce).unwrap();
    assert_eq!(hand(&g), ColorCounts([1, 1, 1, 0, 0]));

    g.apply(Action::Place(0)).unwrap();
    g.apply(Action::Place(1)).unwrap();
    g.apply(Action::Place(3)).unwrap();

    let lots = g.state.players[p].factory_lots;
    assert_eq!(lots[0], ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(lots[1], ColorCounts([0, 1, 0, 0, 0]));
    assert_eq!(lots[2], ColorCounts::default());
    assert_eq!(lots[3], ColorCounts([0, 0, 1, 0, 0]));
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_color_missing_from_the_supply_yields_nothing() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue]);
    drain_supply(&mut g, Color::Red);

    g.apply(Action::Produce).unwrap();

    assert_eq!(hand(&g), ColorCounts([0, 1, 0, 0, 0]));
    assert_eq!(g.state.supply.containers[Color::Red], 0);
    assert_eq!(g.check_invariants(), Ok(()));
    finish_arrange(&mut g);
    assert_eq!(factory_tally(&g, p), ColorCounts([0, 1, 0, 0, 0]));
}

#[test]
fn the_last_container_of_a_color_can_be_produced() {
    let (mut g, p) = game_with(&[Color::Red]);
    let left = g.state.supply.containers[Color::Red] - 1;
    g.state.supply.containers[Color::Red] = 1;
    g.state.bank.container_lots[2][Color::Red] += left;

    g.apply(Action::Produce).unwrap();

    assert_eq!(hand(&g), ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(g.state.supply.containers[Color::Red], 0);
    finish_arrange(&mut g);
    assert_eq!(factory_tally(&g, p), ColorCounts([1, 0, 0, 0, 0]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn producing_nothing_is_legal_and_still_costs_the_wages_and_an_action() {
    let (mut g, p) = game_with(&[Color::Red]);
    drain_supply(&mut g, Color::Red);
    let right = g.state.player_on_right(p);
    let (mine, theirs) = (g.state.players[p].cash, g.state.players[right].cash);

    assert!(g.is_legal(Action::Produce));
    g.apply(Action::Produce).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.state.players[p].cash, mine - 1);
    assert_eq!(g.state.players[right].cash, theirs + 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_full_factory_district_produces_nothing_but_the_containers_are_repriced() {
    let (mut g, p) = game_with(&[Color::Red]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 1, 1]));
    let supply = g.state.supply.containers;

    g.apply(Action::Produce).unwrap();

    assert_eq!(hand(&g), ColorCounts([0, 0, 0, 1, 1]));
    assert_eq!(g.state.supply.containers, supply);
    finish_arrange(&mut g);
    assert_eq!(factory_tally(&g, p), ColorCounts([0, 0, 0, 1, 1]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn everything_is_produced_when_it_fits_exactly() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 1, 1]));

    g.apply(Action::Produce).unwrap();

    assert_eq!(
        hand(&g),
        ColorCounts([1, 1, 0, 1, 1]),
        "no choice is needed when every container fits"
    );
}

#[test]
fn over_the_storage_limit_the_player_chooses_which_colors_to_produce() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue, Color::Yellow]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 3, 2]));
    let supply = g.state.supply.containers;

    g.apply(Action::Produce).unwrap();

    assert!(matches!(g.phase, Phase::Produce(_)));
    assert_eq!(
        g.legal_actions(),
        vec![
            Action::ProduceColor(Color::Red),
            Action::ProduceColor(Color::Blue),
            Action::ProduceColor(Color::Yellow),
        ]
    );
    assert_eq!(
        g.state.supply.containers, supply,
        "nothing leaves the supply before the choice is made"
    );
    assert_eq!(g.turn.actions_left, 2);
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::ProduceColor(Color::Blue)).unwrap();

    assert_eq!(hand(&g), ColorCounts([0, 1, 0, 3, 2]));
    assert_eq!(
        g.state.supply.containers[Color::Blue],
        supply[Color::Blue] - 1
    );
    assert_eq!(g.state.supply.containers[Color::Red], supply[Color::Red]);
    assert_eq!(
        g.state.supply.containers[Color::Yellow],
        supply[Color::Yellow]
    );
    finish_arrange(&mut g);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn every_choice_of_colors_is_offered_exactly_once() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue, Color::Yellow, Color::White]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 0, 6]));
    g.apply(Action::Produce).unwrap();

    let mut hands = Vec::new();
    collect_hands(&g, &mut hands);

    assert_eq!(hands.len(), 6, "choosing 2 of 4 colors: {hands:?}");
    let distinct: HashSet<ColorCounts> = hands.iter().copied().collect();
    assert_eq!(distinct.len(), 6, "a choice was offered twice: {hands:?}");
    for h in &hands {
        assert_eq!(h.total(), 8);
        assert_eq!(h[Color::Green], 6);
    }
}

#[test]
fn choices_are_made_in_color_order_so_no_dead_end_is_reachable() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue, Color::Yellow, Color::White]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 0, 6]));
    g.apply(Action::Produce).unwrap();

    // White could not be followed by a second pick, so it is not offered first.
    assert_eq!(
        g.legal_actions(),
        vec![
            Action::ProduceColor(Color::Red),
            Action::ProduceColor(Color::Blue),
            Action::ProduceColor(Color::Yellow),
        ]
    );
    g.apply(Action::ProduceColor(Color::Blue)).unwrap();
    assert_eq!(
        g.legal_actions(),
        vec![
            Action::ProduceColor(Color::Yellow),
            Action::ProduceColor(Color::White)
        ]
    );
}

#[test]
fn colors_missing_from_the_supply_are_not_offered() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue, Color::Yellow, Color::White]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 0, 6]));
    drain_supply(&mut g, Color::Yellow);
    g.apply(Action::Produce).unwrap();

    let mut hands = Vec::new();
    collect_hands(&g, &mut hands);

    assert_eq!(hands.len(), 3, "choosing 2 of 3 colors: {hands:?}");
    assert!(hands.iter().all(|h| h[Color::Yellow] == 0));
}

#[test]
fn color_choices_are_illegal_outside_a_selection() {
    let g = game();
    for c in Color::ALL {
        assert!(!g.is_legal(Action::ProduceColor(c)));
    }
}

#[test]
fn other_actions_are_illegal_while_choosing_colors() {
    let (mut g, p) = game_with(&[Color::Red, Color::Blue]);
    set_factory_stock(&mut g, p, ColorCounts([0, 0, 0, 1, 2]));
    g.apply(Action::Produce).unwrap();
    assert!(matches!(g.phase, Phase::Produce(_)));

    for a in [
        Action::Produce,
        Action::Reprice(District::Factory),
        Action::Build(Building::Warehouse),
        Action::Place(0),
        Action::ProduceColor(Color::Green),
    ] {
        assert!(!g.is_legal(a), "{a:?}");
        assert!(g.apply(a).is_err(), "{a:?}");
    }
}

#[test]
fn produce_is_limited_to_once_per_turn() {
    let mut g = game();
    let p = g.current_player();
    g.apply(Action::Produce).unwrap();
    finish_arrange(&mut g);

    assert_eq!(g.current_player(), p);
    assert_eq!(g.turn.actions_left, 1);
    assert!(!g.is_legal(Action::Produce));
    assert!(!g.legal_actions().contains(&Action::Produce));
    assert!(g.apply(Action::Produce).is_err());
}

#[test]
fn produce_is_available_again_on_the_next_turn() {
    let mut g = game();
    let p = g.current_player();
    g.apply(Action::Produce).unwrap();
    finish_arrange(&mut g);
    end_turn_by_repricing(&mut g);
    while g.current_player() != p {
        end_turn_by_repricing(&mut g);
    }

    assert_eq!(g.turn.actions_left, 2);
    assert!(g.is_legal(Action::Produce));
}

#[test]
fn producing_is_one_action_and_only_counts_once_the_prices_are_set() {
    let mut g = game();
    g.apply(Action::Produce).unwrap();
    assert_eq!(g.turn.actions_left, 2, "the action is still being resolved");
    finish_arrange(&mut g);
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn produce_can_be_followed_by_another_action_in_the_same_turn() {
    let mut g = game();
    let p = g.current_player();
    g.apply(Action::Produce).unwrap();
    finish_arrange(&mut g);
    g.apply(Action::Build(Building::Warehouse)).unwrap();

    assert_ne!(g.current_player(), p, "two actions end the turn");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_rejected_produce_leaves_the_game_unchanged() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].cash = 0;
    let before = g.clone();
    assert!(g.apply(Action::Produce).is_err());
    assert_eq!(g, before);
}

#[test]
fn random_play_with_a_dry_supply_keeps_every_invariant() {
    for seed in 0..4 {
        let mut g = new_game(3, GameLength::Short, seed);
        drain_supply(&mut g, Color::Red);
        let mut rng = Rng::new(seed + 100);
        for _ in 0..500 {
            // Running out of a second color ends the game.
            let Some(legal) = actions_while_playing(&g) else {
                break;
            };
            // Produce whenever possible, so the produce paths dominate the playout.
            let pick = if legal.contains(&Action::Produce) {
                Action::Produce
            } else {
                legal[rng.below(legal.len() as u32) as usize]
            };
            g.apply(pick).unwrap();
            assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
            assert_eq!(g.state.supply.containers[Color::Red], 0);
        }
    }
}
