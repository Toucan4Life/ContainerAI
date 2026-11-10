mod auction;
mod bank;
mod build;
mod container_bids;
mod delivery;
mod game_end;
mod loans;
mod observation;
mod produce;
mod purchase;
mod sail;
mod scoring;
mod seizure;
mod setup;
mod turn;

use crate::action::{Action, District};
use crate::color::{Color, ColorCounts};
use crate::config::{GameConfig, GameLength};
use crate::game::Game;
use crate::phase::Phase;
use crate::state::PlayerId;

pub(crate) fn new_game(players: usize, length: GameLength, seed: u64) -> Game {
    Game::new(GameConfig::new(players).with_length(length).with_seed(seed)).expect("valid config")
}

pub(crate) fn in_arrange(g: &Game) -> bool {
    matches!(g.phase, Phase::Arrange(_))
}

/// Finishes an arrangement by always taking the first placement offered.
pub(crate) fn finish_arrange(g: &mut Game) {
    while in_arrange(g) {
        let action = g.legal_actions()[0];
        g.apply(action).unwrap();
    }
}

/// The legal actions while the game is going, or `None` once it is over. A game never gets stuck,
/// so there are no legal actions exactly when it has ended.
pub(crate) fn actions_while_playing(g: &Game) -> Option<Vec<Action>> {
    let legal = g.legal_actions();
    assert_eq!(
        legal.is_empty(),
        g.is_terminal(),
        "no way on, or a way on in a finished game, in {:?}",
        g.phase
    );
    (!legal.is_empty()).then_some(legal)
}

/// Gives a player exactly these factories, keeping the factory supply consistent.
pub(crate) fn set_factories(g: &mut Game, player: PlayerId, colors: &[Color]) {
    for c in Color::ALL {
        if g.state.players[player].has_factory(c) {
            g.state.supply.factories[c] += 1;
        }
    }
    g.state.players[player].factories = [false; 5];
    for &c in colors {
        g.state.players[player].factories[c.index()] = true;
        g.state.supply.factories[c] -= 1;
    }
}

/// Puts exactly this stock in a player's factory district ($1 lot), keeping the supply consistent.
pub(crate) fn set_factory_stock(g: &mut Game, player: PlayerId, stock: ColorCounts) {
    for lot in &mut g.state.players[player].factory_lots {
        g.state.supply.containers.add_all(lot);
        *lot = ColorCounts::default();
    }
    g.state.supply.containers.sub_all(&stock);
    g.state.players[player].factory_lots[0] = stock;
}

/// Leaves exactly `n` containers of a color in the supply, moving the rest to Bank lot III.
pub(crate) fn leave_in_supply(g: &mut Game, color: Color, n: u8) {
    let surplus = g.state.supply.containers[color] - n;
    g.state.supply.containers[color] = n;
    g.state.bank.container_lots[2][color] += surplus;
}

/// Moves the whole supply of a color to Bank lot III so that the supply is empty of it.
pub(crate) fn drain_supply(g: &mut Game, color: Color) {
    leave_in_supply(g, color, 0);
}

/// Replaces a district's containers with `(lot, color, count)` entries, keeping the supply consistent.
pub(crate) fn set_lots(
    g: &mut Game,
    player: PlayerId,
    district: District,
    lots: &[(usize, Color, u8)],
) {
    for lot in g.state.players[player].lots_mut(district) {
        g.state.supply.containers.add_all(lot);
        *lot = ColorCounts::default();
    }
    for &(lot, color, n) in lots {
        g.state.supply.containers[color] -= n;
        g.state.players[player].lots_mut(district)[lot][color] += n;
    }
}

/// Replaces a player's factory district with `(lot, color, count)` entries.
pub(crate) fn set_factory_lots(g: &mut Game, player: PlayerId, lots: &[(usize, Color, u8)]) {
    set_lots(g, player, District::Factory, lots);
}

/// Sets a player's cash, booking the difference as created or destroyed so cash conservation still holds.
pub(crate) fn set_cash(g: &mut Game, player: PlayerId, cash: u32) {
    let before = g.state.players[player].cash;
    if cash >= before {
        g.state.cash_minted += cash - before;
    } else {
        g.state.cash_burned += before - cash;
    }
    g.state.players[player].cash = cash;
}

/// Gives a player this many warehouses, keeping the warehouse supply consistent.
pub(crate) fn set_warehouses(g: &mut Game, player: PlayerId, count: u8) {
    let owned = g.state.players[player].warehouses;
    g.state.supply.warehouses = g.state.supply.warehouses + owned - count;
    g.state.players[player].warehouses = count;
}

/// Puts exactly these containers on a player's ship, keeping the supply consistent.
pub(crate) fn set_ship(g: &mut Game, player: PlayerId, cargo: ColorCounts) {
    let aboard = g.state.players[player].ship;
    g.state.supply.containers.add_all(&aboard);
    g.state.supply.containers.sub_all(&cargo);
    g.state.players[player].ship = cargo;
}

/// Puts exactly these containers in a player's holding area, keeping the supply consistent.
pub(crate) fn set_holding(g: &mut Game, player: PlayerId, holding: ColorCounts) {
    let waiting = g.state.players[player].holding;
    g.state.supply.containers.add_all(&waiting);
    g.state.supply.containers.sub_all(&holding);
    g.state.players[player].holding = holding;
}

/// Puts exactly these containers in a player's scoring area, keeping the supply consistent.
pub(crate) fn set_scoring_area(g: &mut Game, player: PlayerId, scored: ColorCounts) {
    let before = g.state.players[player].scoring_area;
    g.state.supply.containers.add_all(&before);
    g.state.supply.containers.sub_all(&scored);
    g.state.players[player].scoring_area = scored;
}

/// Sets how many loans a player owes. The cash they once got is not tracked.
pub(crate) fn set_loans(g: &mut Game, player: PlayerId, loans: u8) {
    g.state.players[player].loans = loans;
}

/// Sets what a Bank container lot holds, keeping the supply consistent.
pub(crate) fn set_container_lot(g: &mut Game, lot: usize, held: ColorCounts) {
    let before = g.state.bank.container_lots[lot];
    g.state.supply.containers.add_all(&before);
    g.state.supply.containers.sub_all(&held);
    g.state.bank.container_lots[lot] = held;
}

/// Sets what a Bank cash lot holds, booking the difference as created or destroyed cash.
pub(crate) fn set_cash_lot(g: &mut Game, lot: usize, amount: u32) {
    let before = g.state.bank.cash_lots[lot];
    if amount >= before {
        g.state.cash_minted += amount - before;
    } else {
        g.state.cash_burned += before - amount;
    }
    g.state.bank.cash_lots[lot] = amount;
}

/// Returns every container a player has, wherever it is, to the supply.
pub(crate) fn clear_containers(g: &mut Game, player: PlayerId) {
    let p = &mut g.state.players[player];
    let mut gone = ColorCounts::default();
    for lot in p.factory_lots.iter_mut().chain(p.harbor_lots.iter_mut()) {
        gone.add_all(lot);
        *lot = ColorCounts::default();
    }
    for held in [&mut p.ship, &mut p.holding, &mut p.scoring_area] {
        gone.add_all(held);
        *held = ColorCounts::default();
    }
    g.state.supply.containers.add_all(&gone);
}

/// The active player takes two harmless actions, so that the next player's turn begins.
pub(crate) fn pass_turn(g: &mut Game) {
    for _ in 0..2 {
        g.apply(Action::Reprice(District::Factory)).unwrap();
        finish_arrange(g);
    }
}
