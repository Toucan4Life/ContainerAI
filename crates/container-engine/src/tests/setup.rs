use super::new_game;
use crate::color::{Color, ColorCounts};
use crate::config::{ConfigError, GameConfig, GameLength};
use crate::game::Game;
use crate::state::ShipLocation;

const ALL_LENGTHS: [GameLength; 3] = [
    GameLength::Short,
    GameLength::Standard,
    GameLength::Extended,
];

#[test]
fn rejects_invalid_player_counts() {
    for n in [0, 1, 2, 6, 7] {
        assert_eq!(
            Game::new(GameConfig::new(n)).err(),
            Some(ConfigError::InvalidPlayerCount(n))
        );
    }
}

#[test]
fn accepts_three_to_five_players() {
    for n in 3..=5 {
        assert!(Game::new(GameConfig::new(n)).is_ok());
    }
}

#[test]
fn container_supply_per_color_matches_rulebook_table() {
    use GameLength::*;
    let table = [
        (3, Short, 9),
        (4, Short, 11),
        (5, Short, 13),
        (3, Standard, 11),
        (4, Standard, 14),
        (5, Standard, 17),
        (3, Extended, 12),
        (4, Extended, 16),
        (5, Extended, 20),
    ];
    for (players, length, expected) in table {
        let cfg = GameConfig::new(players).with_length(length);
        assert_eq!(
            cfg.containers_per_color(),
            expected,
            "{players}p {length:?}"
        );
    }
}

#[test]
fn building_supply_and_auction_tokens_match_rulebook_table() {
    for (players, warehouses, factories, tokens) in [(3, 12, 2, 1), (4, 16, 3, 1), (5, 20, 4, 2)] {
        let cfg = GameConfig::new(players);
        assert_eq!(cfg.total_warehouses(), warehouses, "{players}p warehouses");
        assert_eq!(cfg.factories_per_color(), factories, "{players}p factories");
        assert_eq!(cfg.auction_tokens(), tokens, "{players}p tokens");
    }
}

#[test]
fn config_defaults_to_standard_length() {
    assert_eq!(GameConfig::new(4).length, GameLength::Standard);
}

#[test]
fn every_player_starts_with_the_rulebook_setup() {
    for players in 3..=5 {
        let game = new_game(players, GameLength::Standard, 17);
        assert_eq!(game.state.players().len(), players);
        for p in game.state.players() {
            assert_eq!(p.cash, 20);
            assert_eq!(p.loans, 0);
            assert_eq!(p.warehouses, 1);
            assert_eq!(p.factory_count(), 1);
            assert_eq!(p.factory_limit(), 2);
            assert_eq!(p.harbor_limit(), 1);
            assert_eq!(p.ship_location, ShipLocation::Ocean);
            assert!(p.ship.is_empty());
            assert!(p.holding.is_empty());
            assert!(p.scoring_area.is_empty());
            assert!(p.harbor_lots.iter().all(ColorCounts::is_empty));
        }
    }
}

#[test]
fn starting_factories_are_distinct_colors() {
    for players in 3..=5 {
        for seed in 0..20 {
            let game = new_game(players, GameLength::Standard, seed);
            let colors: Vec<Color> = game
                .state
                .players()
                .iter()
                .map(|p| p.factory_colors().next().expect("one factory"))
                .collect();
            for i in 0..colors.len() {
                for j in i + 1..colors.len() {
                    assert_ne!(colors[i], colors[j], "seed {seed}");
                }
            }
        }
    }
}

#[test]
fn starting_container_sits_in_the_two_dollar_lot_with_factory_color() {
    let game = new_game(4, GameLength::Standard, 3);
    for p in game.state.players() {
        let color = p.factory_colors().next().unwrap();
        let mut expected = ColorCounts::default();
        expected[color] = 1;
        assert_eq!(p.factory_lots[1], expected);
        for lot in [0, 2, 3] {
            assert!(p.factory_lots[lot].is_empty(), "lot index {lot}");
        }
    }
}

#[test]
fn scoring_cards_are_distinct() {
    for players in 3..=5 {
        for seed in 0..20 {
            let game = new_game(players, GameLength::Standard, seed);
            let cards: Vec<_> = game
                .state
                .players()
                .iter()
                .map(|p| p.scoring_card)
                .collect();
            for i in 0..cards.len() {
                for j in i + 1..cards.len() {
                    assert_ne!(cards[i], cards[j], "seed {seed}");
                }
            }
        }
    }
}

#[test]
fn bank_starts_with_two_colors_in_lot_one_and_a_third_in_lot_two() {
    for seed in 0..20 {
        let b = new_game(3, GameLength::Standard, seed).state.bank;
        assert_eq!(b.container_lots[0].total(), 2);
        assert_eq!(b.container_lots[1].total(), 1);
        assert!(b.container_lots[2].is_empty());
        let mut all = b.container_lots[0];
        all.add_all(&b.container_lots[1]);
        assert_eq!(all.max_count(), 1, "three distinct colors, seed {seed}");
        assert_eq!(b.cash_lots, [1, 2, 3]);
    }
}

#[test]
fn container_supply_accounts_for_bank_and_starting_containers() {
    for players in 3..=5 {
        for length in ALL_LENGTHS {
            let game = new_game(players, length, 9);
            let per_color = game.config.containers_per_color();
            let st = &game.state;
            for c in Color::ALL {
                let in_bank: u8 = st.bank.container_lots.iter().map(|l| l[c]).sum();
                let in_factories: u8 = st
                    .players()
                    .iter()
                    .flat_map(|p| p.factory_lots.iter())
                    .map(|l| l[c])
                    .sum();
                assert_eq!(
                    st.supply.containers[c] + in_bank + in_factories,
                    per_color,
                    "{players}p {length:?} {c:?}"
                );
            }
        }
    }
}

#[test]
fn factory_supply_excludes_starting_factories() {
    for players in 3..=5 {
        let game = new_game(players, GameLength::Standard, 4);
        let per_color = game.config.factories_per_color();
        for c in Color::ALL {
            let owned = game
                .state
                .players()
                .iter()
                .filter(|p| p.has_factory(c))
                .count() as u8;
            assert!(owned <= 1);
            assert_eq!(game.state.supply.factories[c], per_color - owned);
        }
    }
}

#[test]
fn warehouse_supply_excludes_starting_warehouses() {
    for players in 3..=5 {
        let game = new_game(players, GameLength::Standard, 4);
        assert_eq!(
            game.state.supply.warehouses as usize,
            game.config.total_warehouses() as usize - players
        );
    }
}

#[test]
fn first_player_is_valid_and_every_seat_can_start() {
    for players in 3..=5 {
        let mut seen = vec![false; players];
        for seed in 0..300 {
            let g = new_game(players, GameLength::Standard, seed);
            assert!(g.state.first_player < players);
            assert_eq!(g.state.active_player, g.state.first_player);
            seen[g.state.first_player] = true;
        }
        assert!(seen.iter().all(|&s| s), "{players} players: {seen:?}");
    }
}

#[test]
fn same_seed_same_game_and_different_seeds_differ() {
    let a = new_game(4, GameLength::Standard, 5);
    let b = new_game(4, GameLength::Standard, 5);
    assert_eq!(a.state, b.state);

    let states: Vec<_> = (0..20)
        .map(|s| new_game(4, GameLength::Standard, s).state)
        .collect();
    assert!(states.iter().any(|s| *s != states[0]));
}

#[test]
fn fresh_games_pass_the_invariant_check() {
    for players in 3..=5 {
        for length in ALL_LENGTHS {
            for seed in 0..10 {
                let game = new_game(players, length, seed);
                assert_eq!(
                    game.check_invariants(),
                    Ok(()),
                    "{players}p {length:?} {seed}"
                );
            }
        }
    }
}

#[test]
fn invariants_detect_a_duplicated_container() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.players[0].harbor_lots[0][Color::Red] += 1;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("container"), "{err}");
}

#[test]
fn invariants_detect_a_missing_factory() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.supply.factories[Color::Red] -= 1;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("factory"), "{err}");
}

#[test]
fn invariants_detect_a_missing_warehouse() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.supply.warehouses -= 1;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("warehouse"), "{err}");
}

#[test]
fn invariants_detect_cash_appearing_from_nowhere() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.players[0].cash += 1;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("cash"), "{err}");
}

#[test]
fn invariants_accept_cash_that_was_burned_or_minted_on_the_books() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.players[0].cash -= 6;
    game.state.cash_burned += 6;
    game.state.players[1].cash += 10;
    game.state.cash_minted += 10;
    assert_eq!(game.check_invariants(), Ok(()));
}

#[test]
fn invariants_detect_an_overfull_ship() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.supply.containers[Color::Red] -= 6;
    game.state.players[0].ship[Color::Red] = 6;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("ship"), "{err}");
}

#[test]
fn invariants_detect_overfull_districts() {
    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.supply.containers[Color::Red] -= 4;
    game.state.players[0].factory_lots[3][Color::Red] = 4;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("factory district"), "{err}");

    let mut game = new_game(3, GameLength::Standard, 1);
    game.state.supply.containers[Color::Red] -= 2;
    game.state.players[0].harbor_lots[4][Color::Red] = 2;
    let err = game.check_invariants().unwrap_err();
    assert!(err.contains("harbor"), "{err}");
}
