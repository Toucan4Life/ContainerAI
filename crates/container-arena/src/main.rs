//! `arena --bots random,simple --games 1000 --players 3,4,5 --seed 1 --threads 8 [--horizon N] [--out games.jsonl]`
//!
//! Writes one JSON line per game (seats, scores, winners, actions, stalled) to `--out`, or to stdout.
//! A game still going after the horizon (default: the engine's `STALL_HORIZON`) is a line with
//! `"stalled":true` and empty scores and winners.

use std::io::Write;

use container_engine::config::STALL_HORIZON;

fn main() {
    let mut bots = vec!["random".to_string(), "simple".to_string()];
    let mut games = 100u64;
    let mut players = vec![4usize];
    let mut seed = 1u64;
    let mut horizon = STALL_HORIZON;
    let mut threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut out: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .unwrap_or_else(|| fail(&format!("{flag} needs a value")));
        match flag.as_str() {
            "--bots" => bots = value.split(',').map(str::to_string).collect(),
            "--games" => games = value.parse().unwrap_or_else(|_| fail("bad --games")),
            "--players" => {
                players = value
                    .split(',')
                    .map(|p| p.parse().unwrap_or_else(|_| fail("bad --players")))
                    .collect()
            }
            "--seed" => seed = value.parse().unwrap_or_else(|_| fail("bad --seed")),
            "--horizon" => horizon = value.parse().unwrap_or_else(|_| fail("bad --horizon")),
            "--threads" => threads = value.parse().unwrap_or_else(|_| fail("bad --threads")),
            "--out" => out = Some(value),
            _ => fail(&format!("unknown flag {flag}")),
        }
    }
    if players.iter().any(|p| !(3..=5).contains(p)) {
        fail("--players takes values from 3 to 5");
    }
    for id in &bots {
        if container_bots::create(id, 0).is_none() {
            fail(&format!("unknown bot {id}"));
        }
    }

    let records = container_arena::run_limited(&bots, &players, games, seed, threads, horizon);
    let stalled = records.iter().filter(|r| r.stalled).count();
    if stalled > 0 {
        eprintln!(
            "{stalled} of {} games stalled after {horizon} actions (recorded with \"stalled\":true)",
            records.len()
        );
    }
    let lines: String = records.iter().map(|r| r.to_json() + "\n").collect();
    match out {
        Some(path) => std::fs::write(path, lines).unwrap_or_else(|e| fail(&e.to_string())),
        None => std::io::stdout()
            .write_all(lines.as_bytes())
            .expect("stdout"),
    }
}

fn fail(message: &str) -> ! {
    eprintln!("arena: {message}");
    std::process::exit(2);
}
