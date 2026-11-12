//! `audit --net magnate|PATH.bin [--temperature 0] --games 60 --players 3,4,5 --seed 1 --threads 8 [--out audit.json]`
//!
//! Plays self-play tables of one net and measures what the net's action interface does to the game:
//! how many actions are legal at each kind of decision, how often the list is thinned to
//! `MAX_ACTIONS`, which actions the thinning drops, how much of the net's own probability sat on them,
//! and how often the repetition guard overrides the net's best action. The game is played by the
//! deployed bot (`Neural`), so a temperature of 0 audits Magnate as shipped and a temperature of 1
//! audits a net as the trainer samples it. Prints one JSON object.

use std::collections::BTreeMap;
use std::sync::Arc;

use container_bots::features::{
    ACTION_DIM, MAX_ACTIONS, STATE_DIM, encode_action, encode_state, thin,
};
use container_bots::{Net, Neural, Opponent, magnate};
use container_engine::action::Action;
use container_engine::config::{GameConfig, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::state::AuctionKind;

#[derive(Default)]
struct PhaseStats {
    legal: Vec<u32>,
    /// Cash of the seat to move, recorded where the legal list is a range of cash amounts.
    cash: Vec<u32>,
    over_cap: u64,
    request_loan_legal: u64,
    request_loan_chosen: u64,
    /// The net's best action over the thinned list was not the one played (the repetition guard).
    guard_overrides: u64,
}

#[derive(Default)]
struct Stats {
    games: u64,
    stalled: u64,
    actions: Vec<u32>,
    phases: BTreeMap<&'static str, PhaseStats>,
    dropped_kinds: BTreeMap<String, u64>,
    kept_kinds: BTreeMap<String, u64>,
    full_best_dropped: u64,
    dropped_mass: Vec<f64>,
    logit_gap: Vec<f64>,
}

impl Stats {
    fn merge(&mut self, other: Stats) {
        self.games += other.games;
        self.stalled += other.stalled;
        self.actions.extend(other.actions);
        for (name, p) in other.phases {
            let mine = self.phases.entry(name).or_default();
            mine.legal.extend(p.legal);
            mine.cash.extend(p.cash);
            mine.over_cap += p.over_cap;
            mine.request_loan_legal += p.request_loan_legal;
            mine.request_loan_chosen += p.request_loan_chosen;
            mine.guard_overrides += p.guard_overrides;
        }
        for (kind, n) in other.dropped_kinds {
            *self.dropped_kinds.entry(kind).or_default() += n;
        }
        for (kind, n) in other.kept_kinds {
            *self.kept_kinds.entry(kind).or_default() += n;
        }
        self.full_best_dropped += other.full_best_dropped;
        self.dropped_mass.extend(other.dropped_mass);
        self.logit_gap.extend(other.logit_gap);
    }
}

fn phase_name(obs: &Observation) -> &'static str {
    match &obs.phase {
        PhaseView::Bid(_) => "bid",
        PhaseView::Open(phase) => match phase {
            Phase::Main => "main",
            Phase::Produce(_) => "produce",
            Phase::Buy(_) => "buy",
            Phase::Load(_) => "load",
            Phase::Arrange(_) => "arrange",
            Phase::Bid(_) => "bid",
            Phase::Decide(_) => "decide",
            Phase::Seize(_) => "seize",
            Phase::Distribute(_) => "distribute",
            Phase::BankBid(b) => match b.kind {
                AuctionKind::ContainerLot => "bank_bid_cash",
                AuctionKind::CashLot => "bank_bid_containers",
            },
            Phase::GameOver => "game_over",
        },
    }
}

/// `Bid(12)` -> `Bid`.
fn kind_name(action: Action) -> String {
    let text = format!("{action:?}");
    text.split(['(', ' ', '{']).next().unwrap_or("").to_string()
}

fn argmax(logits: &[f32]) -> usize {
    logits
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map_or(0, |(k, _)| k)
}

fn action_rows(obs: &Observation, legal: &[Action], which: &[usize]) -> Vec<f32> {
    let mut rows = vec![0.0; which.len() * ACTION_DIM];
    for (k, &i) in which.iter().enumerate() {
        encode_action(
            obs,
            legal[i],
            &mut rows[k * ACTION_DIM..(k + 1) * ACTION_DIM],
        );
    }
    rows
}

/// Records one decision of the net: `chosen` is what the deployed bot played.
fn audit(net: &Net, obs: &Observation, legal: &[Action], chosen: Action, stats: &mut Stats) {
    let n = legal.len();
    let kept = thin(n);
    let phase = stats.phases.entry(phase_name(obs)).or_default();
    phase.legal.push(n as u32);
    if matches!(phase_name(obs), "bid" | "bank_bid_cash" | "decide") {
        phase.cash.push(obs.players[obs.viewer].cash.unwrap_or(0));
    }
    phase.request_loan_legal += legal.contains(&Action::RequestLoan) as u64;
    phase.request_loan_chosen += (chosen == Action::RequestLoan) as u64;

    let mut state = vec![0.0; STATE_DIM];
    encode_state(obs, &mut state);
    let (kept_logits, _) = net.forward(&state, &action_rows(obs, legal, &kept));
    let best = argmax(&kept_logits);
    phase.guard_overrides += (legal[kept[best]] != chosen) as u64;

    if n <= MAX_ACTIONS {
        return;
    }
    phase.over_cap += 1;
    let all: Vec<usize> = (0..n).collect();
    let (full_logits, _) = net.forward(&state, &action_rows(obs, legal, &all));
    let mut is_kept = vec![false; n];
    kept.iter().for_each(|&i| is_kept[i] = true);
    let top = full_logits.iter().cloned().fold(f32::MIN, f32::max);
    let weights: Vec<f64> = full_logits
        .iter()
        .map(|&l| f64::from(l - top).exp())
        .collect();
    let total: f64 = weights.iter().sum();
    let dropped_weight: f64 = (0..n).filter(|&i| !is_kept[i]).map(|i| weights[i]).sum();
    let full_best = argmax(&full_logits);
    stats.full_best_dropped += !is_kept[full_best] as u64;
    stats.dropped_mass.push(dropped_weight / total);
    stats
        .logit_gap
        .push(f64::from(full_logits[full_best] - kept_logits[best]));
    for (i, &action) in legal.iter().enumerate() {
        let map = if is_kept[i] {
            &mut stats.kept_kinds
        } else {
            &mut stats.dropped_kinds
        };
        *map.entry(kind_name(action)).or_default() += 1;
    }
}

fn play(net: &Arc<Net>, temperature: f32, players: usize, seed: u64, stats: &mut Stats) {
    let Ok(mut game) = Game::new(GameConfig::new(players).with_seed(seed)) else {
        return;
    };
    let mut bots: Vec<Neural> = (0..players)
        .map(|s| Neural::new(net.clone(), seed ^ (s as u64 + 1), temperature))
        .collect();
    let mut actions = 0;
    while !game.is_terminal() && actions < STALL_HORIZON {
        let seat = game.current_player();
        let obs = game.observation(seat).expect("seat");
        let legal = game.legal_actions();
        let action = bots[seat].choose(&obs, &legal);
        if legal.len() > 1 {
            audit(net, &obs, &legal, action, stats);
        }
        game.apply(action).expect("a bot plays a legal action");
        actions += 1;
    }
    stats.games += 1;
    stats.stalled += !game.is_terminal() as u64;
    stats.actions.push(actions as u32);
}

fn quantiles<T: Copy + PartialOrd + Into<f64>>(values: &[T]) -> String {
    if values.is_empty() {
        return "null".to_string();
    }
    let mut v: Vec<f64> = values.iter().map(|&x| x.into()).collect();
    v.sort_by(f64::total_cmp);
    let at = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    format!(
        "{{\"n\":{},\"mean\":{mean:.3},\"p50\":{},\"p90\":{},\"p99\":{},\"max\":{}}}",
        v.len(),
        at(0.5),
        at(0.9),
        at(0.99),
        v[v.len() - 1]
    )
}

fn counts(map: &BTreeMap<String, u64>) -> String {
    let items: Vec<String> = map.iter().map(|(k, n)| format!("\"{k}\":{n}")).collect();
    format!("{{{}}}", items.join(","))
}

fn report(stats: &Stats, config: &str) -> String {
    let decisions: usize = stats.phases.values().map(|p| p.legal.len()).sum();
    let over_cap: u64 = stats.phases.values().map(|p| p.over_cap).sum();
    let phases: Vec<String> = stats
        .phases
        .iter()
        .map(|(name, p)| {
            let at_cap = |c: u32| p.cash.iter().filter(|&&x| x >= c).count();
            format!(
                "\"{name}\":{{\"decisions\":{},\"legal\":{},\"over_cap\":{},\"cash\":{},\"cash_at_least_47\":{},\
                 \"request_loan_legal\":{},\"request_loan_chosen\":{},\"guard_overrides\":{}}}",
                p.legal.len(),
                quantiles(&p.legal),
                p.over_cap,
                quantiles(&p.cash),
                at_cap(MAX_ACTIONS as u32 - 1),
                p.request_loan_legal,
                p.request_loan_chosen,
                p.guard_overrides
            )
        })
        .collect();
    let guard: u64 = stats.phases.values().map(|p| p.guard_overrides).sum();
    format!(
        "{{\"config\":{config},\"games\":{},\"stalled\":{},\"actions_per_game\":{},\"decisions\":{decisions},\
         \"over_cap\":{over_cap},\"guard_overrides\":{guard},\"phases\":{{{}}},\
         \"thinned\":{{\"decisions\":{over_cap},\"full_best_dropped\":{},\"dropped_mass\":{},\"logit_gap\":{},\
         \"dropped_kinds\":{},\"kept_kinds\":{}}}}}",
        stats.games,
        stats.stalled,
        quantiles(&stats.actions),
        phases.join(","),
        stats.full_best_dropped,
        quantiles(&stats.dropped_mass),
        quantiles(&stats.logit_gap),
        counts(&stats.dropped_kinds),
        counts(&stats.kept_kinds)
    )
}

fn main() {
    let mut net_arg = "magnate".to_string();
    let mut temperature = 0.0f32;
    let mut games = 60u64;
    let mut players = vec![3usize, 4, 5];
    let mut seed = 1u64;
    let mut threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut out: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .unwrap_or_else(|| fail(&format!("{flag} needs a value")));
        match flag.as_str() {
            "--net" => net_arg = value,
            "--temperature" => {
                temperature = value.parse().unwrap_or_else(|_| fail("bad --temperature"))
            }
            "--games" => games = value.parse().unwrap_or_else(|_| fail("bad --games")),
            "--players" => {
                players = value
                    .split(',')
                    .map(|p| p.parse().unwrap_or_else(|_| fail("bad --players")))
                    .collect()
            }
            "--seed" => seed = value.parse().unwrap_or_else(|_| fail("bad --seed")),
            "--threads" => threads = value.parse().unwrap_or_else(|_| fail("bad --threads")),
            "--out" => out = Some(value),
            _ => fail(&format!("unknown flag {flag}")),
        }
    }
    if players.iter().any(|p| !(3..=5).contains(p)) {
        fail("--players takes values from 3 to 5");
    }
    let net = if net_arg == "magnate" {
        magnate::net()
    } else {
        let bytes = std::fs::read(&net_arg).unwrap_or_else(|e| fail(&format!("{net_arg}: {e}")));
        Arc::new(Net::from_bytes(&bytes).unwrap_or_else(|e| fail(&format!("{net_arg}: {e:?}"))))
    };

    let threads = threads.max(1);
    let merged = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads as u64)
            .map(|t| {
                let (net, players) = (&net, &players);
                scope.spawn(move || {
                    let mut stats = Stats::default();
                    for i in (t..games).step_by(threads) {
                        let n = players[i as usize % players.len()];
                        play(net, temperature, n, seed.wrapping_add(i), &mut stats);
                    }
                    stats
                })
            })
            .collect();
        let mut merged = Stats::default();
        for handle in handles {
            merged.merge(handle.join().expect("audit thread"));
        }
        merged
    });

    let config = format!(
        "{{\"net\":\"{net_arg}\",\"temperature\":{temperature},\"games\":{games},\"players\":{players:?},\
         \"seed\":{seed},\"max_actions\":{MAX_ACTIONS},\"horizon\":{STALL_HORIZON}}}"
    );
    let json = report(&merged, &config);
    match out {
        Some(path) => std::fs::write(path, json + "\n").unwrap_or_else(|e| fail(&e.to_string())),
        None => println!("{json}"),
    }
}

fn fail(message: &str) -> ! {
    eprintln!("audit: {message}");
    std::process::exit(2);
}
