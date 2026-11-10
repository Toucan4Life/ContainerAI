use container_engine::action::Action;
use container_engine::config::{GameConfig, GameLength, MAX_PLAYERS, MIN_PLAYERS};
use container_engine::game::Game;

use crate::{OPPONENTS, create, magnate};

const LIMIT: usize = 20_000;

struct Played {
    game: Game,
    trace: Vec<Action>,
}

/// Plays a game to its end, or to `LIMIT` actions. Seat `s` is played by `ids[s % ids.len()]`.
/// The game's invariants are checked after every action, and every choice must be a legal one.
fn play(config: GameConfig, ids: &[&str]) -> Played {
    play_with(config, |seat| ids[seat % ids.len()])
}

/// As `play`, with seat `s` played by `id_of(s)`.
fn play_with<'a>(config: GameConfig, id_of: impl Fn(usize) -> &'a str) -> Played {
    let mut game = Game::new(config).expect("a valid configuration");
    let mut seats: Vec<_> = (0..config.num_players)
        .map(|seat| {
            let seed = config.seed * 10 + seat as u64;
            create(id_of(seat), seed).expect("a listed opponent")
        })
        .collect();
    let mut trace = Vec::new();
    while !game.is_terminal() && trace.len() < LIMIT {
        let legal = game.legal_actions();
        let viewer = game.current_player();
        let obs = game.observation(viewer).expect("a seat at the table");
        let action = seats[viewer].choose(&obs, &legal);
        assert!(legal.contains(&action), "{action:?} is not legal");
        game.apply(action).expect("a legal action");
        game.check_invariants()
            .unwrap_or_else(|e| panic!("{e}, after {action:?}"));
        trace.push(action);
    }
    Played { game, trace }
}

fn config(players: usize, length: GameLength, seed: u64) -> GameConfig {
    GameConfig::new(players).with_length(length).with_seed(seed)
}

fn every_table() -> Vec<GameConfig> {
    let lengths = [
        GameLength::Short,
        GameLength::Standard,
        GameLength::Extended,
    ];
    let mut all = Vec::new();
    for players in MIN_PLAYERS..=MAX_PLAYERS {
        for length in lengths {
            for seed in 0..4 {
                all.push(config(players, length, seed));
            }
        }
    }
    all
}

#[test]
fn the_opponents_are_listed_once_and_can_all_be_created() {
    let mut ids: Vec<_> = OPPONENTS.iter().map(|o| o.id).collect();
    for id in &ids {
        assert!(create(id, 0).is_some(), "{id}");
    }
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), OPPONENTS.len());
    assert!(create("no-such-opponent", 0).is_none());
}

#[test]
fn every_opponent_plays_every_kind_of_game_to_its_end() {
    for info in OPPONENTS {
        for config in every_table() {
            let Played { game, trace } = play(config, &[info.id]);
            assert!(
                game.is_terminal(),
                "{}: {config:?} was still going after {} actions",
                info.id,
                trace.len()
            );
            assert!(game.results().is_some());
        }
    }
}

#[test]
fn opponents_of_different_kinds_can_share_a_table() {
    let ids: Vec<_> = OPPONENTS.iter().map(|o| o.id).collect();
    for config in every_table() {
        let Played { game, trace } = play(config, &ids);
        assert!(
            game.is_terminal(),
            "{config:?} after {} actions",
            trace.len()
        );
    }
}

#[test]
fn the_same_seed_plays_the_same_game() {
    for info in OPPONENTS {
        let table = config(4, GameLength::Standard, 5);
        assert_eq!(play(table, &[info.id]).trace, play(table, &[info.id]).trace);
    }
}

#[test]
fn different_seeds_play_differently() {
    for info in OPPONENTS {
        let first = play(config(4, GameLength::Standard, 1), &[info.id]).trace;
        let second = play(config(4, GameLength::Standard, 2), &[info.id]).trace;
        assert_ne!(first, second, "{}", info.id);
    }
}

#[test]
fn the_simple_opponent_plays_better_than_the_random_one() {
    let (mut simple, mut random, mut random_seats) = (0i64, 0i64, 0i64);
    for players in MIN_PLAYERS..=MAX_PLAYERS {
        for seed in 0..40 {
            // Seat 0 is the simple opponent, everyone else plays at random.
            let table = config(players, GameLength::Short, seed);
            let Played { game, .. } =
                play_with(table, |seat| if seat == 0 { "simple" } else { "random" });
            let scores = game.results().expect("a finished game").scores;
            simple += i64::from(scores[0].total);
            random += scores[1..].iter().map(|s| i64::from(s.total)).sum::<i64>();
            random_seats += players as i64 - 1;
        }
    }
    let games = 3 * 40;
    let (simple, random) = (
        simple as f64 / games as f64,
        random as f64 / random_seats as f64,
    );
    assert!(
        simple > random + 5.0,
        "the simple opponent averaged {simple:.1}, the random ones {random:.1}"
    );
}

#[test]
fn magnate_beats_merchant_at_every_length() {
    for length in [GameLength::Short, GameLength::Standard, GameLength::Extended] {
        let (mut won, mut games) = (0, 0);
        for players in MIN_PLAYERS..=MAX_PLAYERS {
            for seed in 0..6 {
                // Seat 0 is Magnate, everyone else plays Merchant.
                let table = config(players, length, seed);
                let Played { game, .. } =
                    play_with(table, |seat| if seat == 0 { magnate::ID } else { "merchant" });
                let results = game.results().expect("a finished game");
                won += usize::from(results.winners == [0]);
                games += 1;
            }
        }
        assert!(
            won * 10 >= games * 9,
            "Magnate won {won} of {games} games at the {length:?} length"
        );
    }
}

#[test]
fn features_fill_their_vectors_in_every_phase() {
    use crate::features::{ACTION_DIM, STATE_DIM, encode_action, encode_state, thin};
    let mut seen = std::collections::HashSet::new();
    for players in 3..=5 {
        let mut game = Game::new(GameConfig::new(players).with_seed(5)).unwrap();
        let mut bots: Vec<_> = (0..players)
            .map(|s| create(["random", "simple", "merchant"][s % 3], s as u64).unwrap())
            .collect();
        let mut state = vec![0.0f32; STATE_DIM];
        let mut action = vec![0.0f32; ACTION_DIM];
        while !game.is_terminal() {
            let seat = game.current_player();
            let obs = game.observation(seat).unwrap();
            let legal = game.legal_actions();
            encode_state(&obs, &mut state);
            assert!(state.iter().all(|x| x.is_finite() && (-0.01..=12.0).contains(x)), "{obs:?}");
            for &a in &legal {
                encode_action(&obs, a, &mut action);
                assert!(action.iter().all(|x| x.is_finite()));
                seen.insert(std::mem::discriminant(&a));
            }
            assert!(thin(legal.len()).len() <= crate::features::MAX_ACTIONS);
            let a = bots[seat].choose(&obs, &legal);
            game.apply(a).unwrap();
        }
    }
    assert!(seen.len() >= 18, "only {} action kinds met", seen.len());
}
