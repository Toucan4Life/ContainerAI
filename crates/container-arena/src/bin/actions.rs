//! Which actions each bot takes, per game, when they share tables: `actions --bots haggler,dealer,magnate,consortium
//! --games 300 --seed 1 --out FILE`. Every game seats all the bots (4 seats, rotated); counts are averaged per seat-game.

use std::collections::BTreeMap;

use container_bots::create;
use container_engine::action::Action;
use container_engine::config::{GameConfig, STALL_HORIZON};
use container_engine::game::Game;

fn kind(a: Action) -> String {
    if let Action::Build(b) = a {
        return if matches!(b, container_engine::action::Building::Warehouse) { "BuildWarehouse" } else { "BuildFactory" }.into();
    }
    let s = format!("{a:?}");
    s.split(|c: char| !c.is_alphanumeric()).next().unwrap_or("").to_string()
}

fn main() {
    let mut bots = vec!["haggler".to_string(), "dealer".into(), "magnate".into(), "consortium".into()];
    let (mut games, mut seed, mut out) = (300u64, 1u64, String::new());
    let mut args = std::env::args().skip(1);
    while let Some(f) = args.next() {
        let v = args.next().expect("value");
        match f.as_str() {
            "--bots" => bots = v.split(',').map(String::from).collect(),
            "--games" => games = v.parse().expect("n"),
            "--seed" => seed = v.parse().expect("n"),
            "--out" => out = v,
            _ => panic!("unknown flag {f}"),
        }
    }
    let n = bots.len();
    let threads = 10u64;
    let rows: Vec<(String, BTreeMap<String, f64>)> = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..threads)
            .map(|th| {
                let bots = &bots;
                sc.spawn(move || {
                    let mut rows = Vec::new();
                    for g in (th..games).step_by(threads as usize) {
                        let seats: Vec<&String> = (0..n).map(|s| &bots[(s + g as usize) % n]).collect();
                        let gs = seed * 1_000_003 + g;
                        let mut ps: Vec<_> = seats.iter().enumerate().map(|(i, id)| create(id, gs + i as u64 + 1).expect("bot")).collect();
                        let mut counts = vec![BTreeMap::<String, f64>::new(); n];
                        let mut game = Game::new(GameConfig::new(n).with_seed(gs)).expect("game");
                        for _ in 0..STALL_HORIZON {
                            if game.is_terminal() {
                                break;
                            }
                            let seat = game.current_player();
                            let obs = game.observation(seat).expect("seat");
                            let legal = game.legal_actions();
                            let a = ps[seat].choose(&obs, &legal);
                            // Forced moves (one legal action) say nothing about the bot's taste.
                            if legal.len() > 1 {
                                *counts[seat].entry(kind(a)).or_default() += 1.0;
                            }
                            game.apply(a).expect("legal");
                        }
                        if game.is_terminal() {
                            for (id, c) in seats.iter().zip(counts) {
                                rows.push(((*id).clone(), c));
                            }
                        }
                    }
                    rows
                })
            })
            .collect();
        hs.into_iter().flat_map(|h| h.join().expect("thread")).collect()
    });
    let mut total: BTreeMap<String, (f64, BTreeMap<String, f64>)> = BTreeMap::new();
    for (id, c) in rows {
        let e = total.entry(id).or_default();
        e.0 += 1.0;
        for (k, v) in c {
            *e.1.entry(k).or_default() += v;
        }
    }
    let body: Vec<String> = total
        .iter()
        .map(|(id, (g, c))| {
            let per: Vec<String> = c.iter().map(|(k, v)| format!("\"{k}\":{:.2}", v / g)).collect();
            format!("\"{id}\":{{\"seat_games\":{g},{}}}", per.join(","))
        })
        .collect();
    let text = format!("{{{}}}\n", body.join(","));
    if out.is_empty() { print!("{text}") } else { std::fs::write(out, text).expect("write") }
}
