//! Plays bots against each other at mixed tables and records who finished where.
//!
//! A table seats 3 to 5 bots drawn from a pool, in a random order. Every game is reproducible from
//! the run seed and its index. A game that has not ended after the horizon (by default
//! [`STALL_HORIZON`], the one the batch environment uses) is a stall: it is recorded as such, with no
//! scores and no winners, never silently dropped, so a report can always account for every game.

use container_bots::{Opponent, create};
use container_engine::config::{GameConfig, MAX_PLAYERS, MIN_PLAYERS, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::rng::Rng;

/// What happened at one table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameRecord {
    pub index: u64,
    /// The bot at each seat.
    pub seats: Vec<String>,
    /// Final score of each seat; empty if the game stalled.
    pub scores: Vec<i32>,
    /// The winners by the engine's tie-break; empty if the game stalled.
    pub winners: Vec<usize>,
    /// Engine actions played, forced moves included.
    pub actions: usize,
    /// The game had not ended after the horizon.
    pub stalled: bool,
}

impl GameRecord {
    pub fn to_json(&self) -> String {
        let seats: Vec<String> = self.seats.iter().map(|s| format!("\"{s}\"")).collect();
        format!(
            "{{\"game\":{},\"seats\":[{}],\"scores\":{:?},\"winners\":{:?},\"actions\":{},\"stalled\":{}}}",
            self.index,
            seats.join(","),
            self.scores,
            self.winners,
            self.actions,
            self.stalled
        )
    }
}

/// How a game ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Finished {
        scores: Vec<i32>,
        winners: Vec<usize>,
        actions: usize,
    },
    /// Still going after `actions` actions.
    Stalled { actions: usize },
}

/// Plays one game with these bots, one per seat, and returns the final scores and winners, or `None`
/// if it stalled.
pub fn play(bots: &mut [Box<dyn Opponent + Send>], seed: u64) -> Option<(Vec<i32>, Vec<usize>)> {
    play_limited(bots, seed, STALL_HORIZON)
}

/// Like [`play`], but gives up after `max_actions` actions.
pub fn play_limited(
    bots: &mut [Box<dyn Opponent + Send>],
    seed: u64,
    max_actions: usize,
) -> Option<(Vec<i32>, Vec<usize>)> {
    match play_outcome(bots, seed, max_actions)? {
        Outcome::Finished {
            scores, winners, ..
        } => Some((scores, winners)),
        Outcome::Stalled { .. } => None,
    }
}

/// Plays one game and says how it ended, a stall included. `None` only if the table cannot be set up.
pub fn play_outcome(
    bots: &mut [Box<dyn Opponent + Send>],
    seed: u64,
    max_actions: usize,
) -> Option<Outcome> {
    let mut game = Game::new(GameConfig::new(bots.len()).with_seed(seed)).ok()?;
    for actions in 0..max_actions {
        if game.is_terminal() {
            let results = game.results()?;
            let scores = results.scores.iter().map(|s| s.total).collect();
            return Some(Outcome::Finished {
                scores,
                winners: results.winners,
                actions,
            });
        }
        let seat = game.current_player();
        let obs = game.observation(seat)?;
        let legal = game.legal_actions();
        let action = bots[seat].choose(&obs, &legal);
        for bot in bots.iter_mut() {
            bot.observe(seat, &obs, action);
        }
        game.apply(action).ok()?;
    }
    if game.is_terminal() {
        let results = game.results()?;
        let scores = results.scores.iter().map(|s| s.total).collect();
        return Some(Outcome::Finished {
            scores,
            winners: results.winners,
            actions: max_actions,
        });
    }
    Some(Outcome::Stalled {
        actions: max_actions,
    })
}

/// Plays game number `index` of a run: draws a table of `players` bots from `pool` (a bot may sit
/// twice), seats them in a random order and plays. `None` if a bot id is unknown.
pub fn play_table(
    pool: &[String],
    players: usize,
    run_seed: u64,
    index: u64,
) -> Option<GameRecord> {
    play_table_limited(pool, players, run_seed, index, STALL_HORIZON)
}

/// Like [`play_table`], with another horizon.
pub fn play_table_limited(
    pool: &[String],
    players: usize,
    run_seed: u64,
    index: u64,
    horizon: usize,
) -> Option<GameRecord> {
    assert!((MIN_PLAYERS..=MAX_PLAYERS).contains(&players));
    let mut rng = Rng::new(run_seed ^ index.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let seats: Vec<String> = (0..players)
        .map(|_| pool[rng.below(pool.len() as u32) as usize].clone())
        .collect();
    play_seated_limited(&seats, run_seed, index, horizon)
}

/// Plays game number `index` with these bots in these seats.
pub fn play_seated(seats: &[String], run_seed: u64, index: u64) -> Option<GameRecord> {
    play_seated_limited(seats, run_seed, index, STALL_HORIZON)
}

/// Like [`play_seated`], with another horizon.
pub fn play_seated_limited(
    seats: &[String],
    run_seed: u64,
    index: u64,
    horizon: usize,
) -> Option<GameRecord> {
    let game_seed = run_seed
        .wrapping_add(index)
        .wrapping_mul(0xD134_2543_DE82_EF95)
        | 1;
    let mut bots = seats
        .iter()
        .enumerate()
        .map(|(seat, id)| create(id, game_seed.wrapping_add(seat as u64 + 1)))
        .collect::<Option<Vec<_>>>()?;
    let (scores, winners, actions, stalled) = match play_outcome(&mut bots, game_seed, horizon)? {
        Outcome::Finished {
            scores,
            winners,
            actions,
        } => (scores, winners, actions, false),
        Outcome::Stalled { actions } => (Vec::new(), Vec::new(), actions, true),
    };
    Some(GameRecord {
        index,
        seats: seats.to_vec(),
        scores,
        winners,
        actions,
        stalled,
    })
}

/// Plays games `0..games` over `threads` threads and returns the records in game order.
pub fn run(
    pool: &[String],
    players: &[usize],
    games: u64,
    run_seed: u64,
    threads: usize,
) -> Vec<GameRecord> {
    run_limited(pool, players, games, run_seed, threads, STALL_HORIZON)
}

/// Like [`run`], with another horizon.
pub fn run_limited(
    pool: &[String],
    players: &[usize],
    games: u64,
    run_seed: u64,
    threads: usize,
    horizon: usize,
) -> Vec<GameRecord> {
    let threads = threads.max(1);
    let mut records: Vec<GameRecord> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads as u64)
            .map(|t| {
                scope.spawn(move || {
                    (t..games)
                        .step_by(threads)
                        .filter_map(|i| {
                            let n = players[i as usize % players.len()];
                            play_table_limited(pool, n, run_seed, i, horizon)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("arena thread"))
            .collect()
    });
    records.sort_by_key(|r| r.index);
    records
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool() -> Vec<String> {
        ["random", "simple"].map(String::from).to_vec()
    }

    #[test]
    fn a_table_plays_to_the_end() {
        for players in 3..=5 {
            let r = play_table(&pool(), players, 7, 0).expect("game ends");
            assert_eq!(r.seats.len(), players);
            assert_eq!(r.scores.len(), players);
            assert!(!r.winners.is_empty());
            assert!(!r.stalled && r.actions > 0 && r.actions < STALL_HORIZON);
        }
    }

    #[test]
    fn a_stalled_game_is_recorded_with_its_length_not_dropped() {
        let records = run_limited(&pool(), &[3, 4], 6, 11, 2, 25);
        assert_eq!(records.len(), 6);
        for r in &records {
            assert!(r.stalled && r.scores.is_empty() && r.winners.is_empty());
            assert_eq!(r.actions, 25);
            assert!(r.to_json().contains("\"stalled\":true"));
        }
        let finished = run(&pool(), &[3, 4], 6, 11, 2);
        assert!(finished.iter().all(|r| !r.stalled && !r.winners.is_empty()));
    }

    #[test]
    fn runs_are_reproducible_and_thread_independent() {
        let a = run(&pool(), &[3, 4], 8, 11, 1);
        let b = run(&pool(), &[3, 4], 8, 11, 4);
        assert_eq!(a, b);
        assert_eq!(a.len(), 8);
    }

    #[test]
    fn the_simple_bot_outscores_random_at_mixed_tables() {
        let mut mean = std::collections::HashMap::<String, (i64, i64)>::new();
        for r in run(&pool(), &[4], 300, 3, 4) {
            for (bot, score) in r.seats.iter().zip(&r.scores) {
                let e = mean.entry(bot.clone()).or_default();
                e.0 += *score as i64;
                e.1 += 1;
            }
        }
        let avg = |id: &str| mean[id].0 as f64 / mean[id].1 as f64;
        assert!(avg("simple") > avg("random") + 10.0);
    }
}
