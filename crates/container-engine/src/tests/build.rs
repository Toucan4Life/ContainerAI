use super::new_game;
use crate::action::{Action, Building};
use crate::build::{factory_build_cost, warehouse_build_cost};
use crate::color::Color;
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;

fn game() -> Game {
    new_game(3, GameLength::Standard, 1)
}

fn unowned_color(g: &Game, player: usize) -> Color {
    Color::ALL
        .into_iter()
        .find(|&c| !g.state.players[player].has_factory(c) && g.state.supply.factories[c] > 0)
        .expect("a buildable color")
}

fn build_actions(g: &Game) -> Vec<Action> {
    g.legal_actions()
        .into_iter()
        .filter(|a| matches!(a, Action::Build(_)))
        .collect()
}

#[test]
fn factory_costs_follow_the_building_track() {
    assert_eq!(factory_build_cost(0), Some(0));
    assert_eq!(factory_build_cost(1), Some(6));
    assert_eq!(factory_build_cost(2), Some(9));
    assert_eq!(factory_build_cost(3), Some(12));
    assert_eq!(factory_build_cost(4), None);
}

#[test]
fn warehouse_costs_follow_the_building_track() {
    assert_eq!(warehouse_build_cost(0), Some(0));
    assert_eq!(warehouse_build_cost(1), Some(4));
    assert_eq!(warehouse_build_cost(2), Some(5));
    assert_eq!(warehouse_build_cost(3), Some(6));
    assert_eq!(warehouse_build_cost(4), Some(7));
    assert_eq!(warehouse_build_cost(5), None);
}

#[test]
fn building_a_factory_pays_the_supply_and_adds_the_factory() {
    let mut g = game();
    let p = g.current_player();
    let color = unowned_color(&g, p);
    let supply_before = g.state.supply.factories[color];

    g.apply(Action::Build(Building::Factory(color))).unwrap();

    let player = &g.state.players[p];
    assert!(player.has_factory(color));
    assert_eq!(player.factory_count(), 2);
    assert_eq!(player.factory_limit(), 4);
    assert_eq!(player.cash, 14);
    assert_eq!(g.state.supply.factories[color], supply_before - 1);
    assert_eq!(g.state.cash_burned, 6);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn building_a_warehouse_pays_the_supply_and_adds_the_warehouse() {
    let mut g = game();
    let p = g.current_player();
    let supply_before = g.state.supply.warehouses;

    g.apply(Action::Build(Building::Warehouse)).unwrap();

    let player = &g.state.players[p];
    assert_eq!(player.warehouses, 2);
    assert_eq!(player.harbor_limit(), 2);
    assert_eq!(player.cash, 16);
    assert_eq!(g.state.supply.warehouses, supply_before - 1);
    assert_eq!(g.state.cash_burned, 4);
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_factory_color_the_player_already_owns_cannot_be_built() {
    let g = game();
    let own = g.state.players[g.current_player()]
        .factory_colors()
        .next()
        .unwrap();
    assert!(!g.is_legal(Action::Build(Building::Factory(own))));
}

#[test]
fn a_factory_color_missing_from_the_supply_cannot_be_built() {
    let mut g = game();
    let color = unowned_color(&g, g.current_player());
    assert!(g.is_legal(Action::Build(Building::Factory(color))));
    g.state.supply.factories[color] = 0;
    assert!(!g.is_legal(Action::Build(Building::Factory(color))));
}

#[test]
fn building_needs_enough_cash() {
    let mut g = game();
    let p = g.current_player();
    let color = unowned_color(&g, p);
    let factory = Action::Build(Building::Factory(color));
    let warehouse = Action::Build(Building::Warehouse);

    g.state.players[p].cash = 5;
    assert!(!g.is_legal(factory));
    assert!(g.is_legal(warehouse));
    g.state.players[p].cash = 6;
    assert!(g.is_legal(factory));
    g.state.players[p].cash = 3;
    assert!(!g.is_legal(warehouse));
    g.state.players[p].cash = 4;
    assert!(g.is_legal(warehouse));
}

#[test]
fn a_player_never_has_more_than_four_factories() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].factories = [true, true, true, true, false];
    g.state.players[p].cash = 100;
    assert!(!g.is_legal(Action::Build(Building::Factory(Color::Green))));
}

#[test]
fn a_player_never_has_more_than_five_warehouses() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].warehouses = 5;
    g.state.players[p].cash = 100;
    assert!(!g.is_legal(Action::Build(Building::Warehouse)));
}

#[test]
fn an_exhausted_warehouse_supply_blocks_building() {
    let mut g = game();
    g.state.supply.warehouses = 0;
    assert!(!g.is_legal(Action::Build(Building::Warehouse)));
}

#[test]
fn the_main_node_lists_every_affordable_build() {
    let g = game();
    let p = g.current_player();
    let builds = build_actions(&g);
    assert_eq!(
        builds.len(),
        5,
        "four other colors and a warehouse: {builds:?}"
    );
    assert!(builds.contains(&Action::Build(Building::Warehouse)));
    for c in Color::ALL {
        let expected = !g.state.players[p].has_factory(c);
        assert_eq!(
            builds.contains(&Action::Build(Building::Factory(c))),
            expected,
            "{c:?}"
        );
    }
}

#[test]
fn a_player_can_build_twice_in_one_turn_then_the_turn_ends() {
    let mut g = game();
    let p = g.current_player();
    let color = unowned_color(&g, p);
    g.apply(Action::Build(Building::Factory(color))).unwrap();
    g.apply(Action::Build(Building::Warehouse)).unwrap();

    assert_eq!(g.state.players[p].cash, 20 - 6 - 4);
    assert_eq!(g.state.cash_burned, 10);
    assert_eq!(g.current_player(), (p + 1) % 3);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn later_buildings_cost_more() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].cash = 100;

    g.apply(Action::Build(Building::Warehouse)).unwrap();
    g.apply(Action::Build(Building::Warehouse)).unwrap();
    assert_eq!(g.state.players[p].cash, 100 - 4 - 5);
    assert_eq!(g.state.players[p].warehouses, 3);
}

#[test]
fn a_rejected_build_leaves_the_game_unchanged() {
    let mut g = game();
    let p = g.current_player();
    g.state.players[p].cash = 3;
    let before = g.clone();
    let action = Action::Build(Building::Warehouse);
    assert!(g.apply(action).is_err());
    assert_eq!(g, before);
}
