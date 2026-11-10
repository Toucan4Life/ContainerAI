//! Tunes the Merchant's parameters by evolution strategy against a field of reference bots.
//!
//! `tune [--generations 60] [--games 300] [--threads N] [--seed 1]`
//!
//! A candidate plays next to Simple and Random bots at 3, 4 and 5 seats, and is scored by the share
//! of pairwise duels it wins. It starts from Simple's behaviour and prints the best parameters as a Rust
//! array to paste into `TUNED` in `container-bots/src/merchant.rs`.

use container_arena::play_limited;
use container_bots::merchant::{Merchant, PARAMS, defaults};
use container_bots::{Heuristic, Opponent, Random};
use container_engine::rng::Rng;

/// Actions after which a game is called stalled; a normal game is far shorter.
const STALL: usize = 4_000;

/// Table layouts: 0 is the candidate, 1 Simple and 2 Random.
const TABLES: [&[u8]; 4] = [&[0, 1, 0, 1], &[0, 0, 1, 2], &[0, 1, 2], &[0, 0, 1, 1, 2]];

/// Share of pairwise duels the candidate wins. A candidate that stalls any game scores zero: a bot
/// that cannot finish a game is useless, whatever it would have scored.
fn fitness(w: &[f64], games: u64, seed: u64) -> f64 {
    let (mut won, mut duels) = (0.0, 0.0);
    for g in 0..games {
        let s = seed.wrapping_add(g).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        let table = TABLES[(g % TABLES.len() as u64) as usize];
        let n = table.len();
        let rot = (g / TABLES.len() as u64) as usize % n;
        let kinds: Vec<u8> = (0..n).map(|seat| table[(seat + rot) % n]).collect();
        let mut bots: Vec<Box<dyn Opponent + Send>> = kinds
            .iter()
            .enumerate()
            .map(|(seat, kind)| -> Box<dyn Opponent + Send> {
                let seed = s + seat as u64;
                match kind {
                    0 => Box::new(Merchant::with_params(seed, w.to_vec())),
                    1 => Box::new(Heuristic::new(seed)),
                    _ => Box::new(Random::new(seed)),
                }
            })
            .collect();
        let Some((scores, _)) = play_limited(&mut bots, s, STALL) else {
            return 0.0;
        };
        for m in (0..n).filter(|&i| kinds[i] == 0) {
            for o in (0..n).filter(|&o| kinds[o] != 0) {
                duels += 1.0;
                won += match scores[m].cmp(&scores[o]) {
                    std::cmp::Ordering::Greater => 1.0,
                    std::cmp::Ordering::Equal => 0.5,
                    _ => 0.0,
                };
            }
        }
    }
    won / duels
}

fn gauss(rng: &mut Rng) -> f64 {
    let u = (rng.below(1 << 24) as f64 + 1.0) / ((1u64 << 24) as f64 + 1.0);
    let v = rng.below(1 << 24) as f64 / (1u64 << 24) as f64;
    (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
}

fn main() {
    let mut generations = 60;
    let mut games = 300u64;
    let mut threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut seed = 1u64;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let v: u64 = args.next().and_then(|v| v.parse().ok()).expect("a number");
        match flag.as_str() {
            "--generations" => generations = v,
            "--games" => games = v,
            "--threads" => threads = v as usize,
            "--seed" => seed = v,
            _ => panic!("unknown flag {flag}"),
        }
    }

    let mut best: Vec<f64> = defaults();
    let mut rng = Rng::new(seed);
    let lambda = threads.max(8) * 2;
    for round in 0..generations {
        let sigma = 0.20 - 0.15 * round as f64 / generations as f64;
        let eval_seed = seed.wrapping_mul(1_000_003).wrapping_add(round);
        let cands: Vec<Vec<f64>> = std::iter::once(best.clone())
            .chain((0..lambda).map(|_| {
                best.iter()
                    .zip(&PARAMS)
                    .map(|(&x, p)| {
                        // Only a few parameters move at a time, which makes steps more selective.
                        let moved = if rng.below(4) == 0 {
                            x + gauss(&mut rng) * sigma * (p.hi - p.lo)
                        } else {
                            x
                        };
                        moved.clamp(p.lo, p.hi)
                    })
                    .collect()
            }))
            .collect();
        let fits: Vec<f64> = std::thread::scope(|scope| {
            let handles: Vec<_> = cands
                .chunks(cands.len().div_ceil(threads))
                .map(|chunk| {
                    scope.spawn(move || {
                        chunk
                            .iter()
                            .map(|c| fitness(c, games, eval_seed))
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
        });
        // The best of many noisy scores is flattered by luck, so the top few play again on fresh seeds
        // against the incumbent, and only a clear winner replaces it.
        let mut order: Vec<usize> = (1..cands.len()).collect();
        order.sort_by(|&a, &b| fits[b].partial_cmp(&fits[a]).unwrap());
        let recheck_seed = eval_seed ^ 0xABCD_EF01;
        let incumbent = fitness(&best, games * 3, recheck_seed);
        let mut winner = None;
        let mut top = incumbent;
        for &i in order.iter().take(3) {
            let f = fitness(&cands[i], games * 3, recheck_seed);
            if f > top + 0.005 {
                top = f;
                winner = Some(i);
            }
        }
        eprintln!("round {round:3}  incumbent {incumbent:.3}  best {top:.3}");
        if let Some(i) = winner {
            best = cands[i].clone();
        }
    }
    println!("pub const TUNED: [f64; {}] = [", best.len());
    for (x, p) in best.iter().zip(&PARAMS) {
        println!("    {x:.3}, // {}", p.name);
    }
    println!("];");
}
