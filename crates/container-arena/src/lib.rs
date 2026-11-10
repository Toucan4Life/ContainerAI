//! Plays bots against each other at mixed tables and records who finished where.
//!
//! A table seats 3 to 5 bots drawn from a pool, in a random order. Every game is reproducible from
//! the run seed and its index.

use container_bots::{Opponent, create};
use container_engine::config::{GameConfig, MAX_PLAYERS, MIN_PLAYERS};
use container_engine::game::Game;
use container_engine::rng::Rng;

/// A game that has not ended after this many actions is abandoned and not recorded.
const MAX_ACTIONS: usize = 50_000;

/// What happened at one table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameRecord {
    pub index: u64,
    /// The bot at each seat.
    pub seats: Vec<String>,
    /// Final score of each seat.
    pub scores: Vec<i32>,
    pub winners: Vec<usize>,
}

impl GameRecord {
    pub fn to_json(&self) -> String {
        let seats: Vec<String> = self.seats.iter().map(|s| format!("\"{s}\"")).collect();
        format!(
            "{{\"game\":{},\"seats\":[{}],\"scores\":{:?},\"winners\":{:?}}}",
            self.index,
            seats.join(","),
            self.scores,
            self.winners
        )
    }
}

/// Plays one game with these bots, one per seat, and returns the final scores and winners.
pub fn play(
    bots: &mut [Box<dyn Opponent + Send>],
    seed: u64,
) -> Option<(Vec<i32>, Vec<usize>)> {
    play_limited(bots, seed, MAX_ACTIONS)
}

/// Like [`play`], but gives up after `max_actions` actions.
pub fn play_limited(
    bots: &mut [Box<dyn Opponent + Send>],
    seed: u64,
    max_actions: usize,
) -> Option<(Vec<i32>, Vec<usize>)> {
    let mut game = Game::new(GameConfig::new(bots.len()).with_seed(seed)).ok()?;
    for _ in 0..max_actions {
        if game.is_terminal() {
            let results = game.results()?;
            let scores = results.scores.iter().map(|s| s.total).collect();
            return Some((scores, results.winners));
        }
        let seat = game.current_player();
        let obs = game.observation(seat)?;
        let legal = game.legal_actions();
        let action = bots[seat].choose(&obs, &legal);
        game.apply(action).ok()?;
    }
    None
}

/// Plays game number `index` of a run: draws a table of `players` bots from `pool` (a bot may sit
/// twice), seats them in a random order and plays. `None` if a bot id is unknown or the game stalled.
pub fn play_table(pool: &[String], players: usize, run_seed: u64, index: u64) -> Option<GameRecord> {
    assert!((MIN_PLAYERS..=MAX_PLAYERS).contains(&players));
    let mut rng = Rng::new(run_seed ^ index.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let seats: Vec<String> = (0..players)
        .map(|_| pool[rng.below(pool.len() as u32) as usize].clone())
        .collect();
    play_seated(&seats, run_seed, index)
}

/// Plays game number `index` with these bots in these seats.
pub fn play_seated(seats: &[String], run_seed: u64, index: u64) -> Option<GameRecord> {
    let game_seed = run_seed.wrapping_add(index).wrapping_mul(0xD134_2543_DE82_EF95) | 1;
    let mut bots = seats
        .iter()
        .enumerate()
        .map(|(seat, id)| create(id, game_seed.wrapping_add(seat as u64 + 1)))
        .collect::<Option<Vec<_>>>()?;
    let (scores, winners) = play(&mut bots, game_seed)?;
    Some(GameRecord {
        index,
        seats: seats.to_vec(),
        scores,
        winners,
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
    let threads = threads.max(1);
    let mut records: Vec<GameRecord> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads as u64)
            .map(|t| {
                scope.spawn(move || {
                    (t..games)
                        .step_by(threads)
                        .filter_map(|i| {
                            let n = players[i as usize % players.len()];
                            play_table(pool, n, run_seed, i)
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
        }
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
