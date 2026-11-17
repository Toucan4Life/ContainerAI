//! `probe --net PATH.bin [--rules dealer|none] --pool a,b --positions 150 --rollouts 24 --seed 1 [--out p.json]`
//!
//! Why does a net leave cheap offers alone? At the focus net's own main decisions where an opponent's
//! factory sells a container at $2 or less that the seat can pay for, this probe asks three questions:
//!   1. policy: what probability and rank does the net give the purchase, and what does it play instead?
//!   2. value: does the net's value head rate the position after the purchase above the one before?
//!   3. truth: from the same position, force the purchase of every cheap container, or play the net's
//!      own choice, then play both out with the same net (temperature 0.3) and the same fillers on
//!      the same rollout seeds. Which wins more, which scores more?
//! A net that ignores a bargain for good reason shows a truth gain of zero or less; a blind policy shows
//! a positive truth gap with a low probability; a blind value head shows a positive truth gap and no
//! value gain.

use std::sync::Arc;

use container_bots::features::{ACTION_DIM, MAX_ACTIONS, STATE_DIM, encode_action, encode_state, thin};
use container_bots::{Net, Neural, Opponent, create};
use container_engine::action::{Action, District};
use container_engine::config::{GameConfig, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::observation::PhaseView;
use container_engine::phase::Phase;
use container_engine::rng::Rng;

const CHEAP: u32 = 2;

struct Cfg {
    net: Arc<Net>,
    dealer_rules: bool,
    pool: Vec<String>,
    seed: u64,
}

fn focus_bot(cfg: &Cfg, seed: u64, temperature: f32) -> Neural {
    let bot = Neural::new(cfg.net.clone(), seed, temperature);
    if cfg.dealer_rules { bot.without_same_turn_repay().with_min_bid(3) } else { bot }
}

fn forward(cfg: &Cfg, game: &Game, seat: usize, legal: &[Action]) -> (Vec<f32>, f32, Vec<usize>) {
    let obs = game.observation(seat).expect("seat");
    let kept = thin(legal.len());
    let mut state = vec![0.0; STATE_DIM];
    encode_state(&obs, &mut state);
    let mut acts = vec![0.0; kept.len() * ACTION_DIM];
    for (k, &i) in kept.iter().enumerate() {
        encode_action(&obs, legal[i], &mut acts[k * ACTION_DIM..(k + 1) * ACTION_DIM]);
    }
    let (logits, value) = cfg.net.forward(&state, &acts);
    (logits, value, kept)
}

fn state_value(cfg: &Cfg, game: &Game, seat: usize) -> f32 {
    // The value does not depend on the actions; one dummy action row is enough.
    let obs = game.observation(seat).expect("seat");
    let mut state = vec![0.0; STATE_DIM];
    encode_state(&obs, &mut state);
    cfg.net.forward(&state, &vec![0.0; ACTION_DIM]).1
}

/// The cheap containers the seat could buy from `seller`'s factories now.
fn cheap_lots(game: &Game, seat: usize, seller: usize) -> u32 {
    let cash = game.observation(seat).and_then(|o| o.players[seat].cash).unwrap_or(0);
    let p = &game.state.players()[seller];
    let mut n = 0;
    let mut budget = cash;
    for (lot, c) in p.factory_lots.iter().enumerate() {
        let price = District::Factory.lot_price(lot);
        if price > CHEAP {
            break;
        }
        for _ in 0..c.total() {
            if budget >= price {
                budget -= price;
                n += 1;
            }
        }
    }
    n
}

fn buy_cheap(game: &mut Game) {
    while matches!(game.phase, Phase::Buy(_)) {
        let legal = game.legal_actions();
        let pick = legal.iter().copied().find(
            |a| matches!(a, Action::Buy { lot, .. } if District::Factory.lot_price(*lot) <= CHEAP),
        );
        match pick.or_else(|| legal.iter().copied().find(|a| *a == Action::DoneBuying)) {
            Some(a) => game.apply(a).expect("legal"),
            None => break,
        }
    }
}

/// Plays on from `game` with the focus net at `seat`, and returns (score, won) of that seat.
fn play_out(cfg: &Cfg, mut game: Game, seats: &[String], seat: usize, seed: u64) -> Option<(f64, f64)> {
    let mut bots: Vec<Box<dyn Opponent + Send>> = seats
        .iter()
        .enumerate()
        .map(|(s, id)| -> Box<dyn Opponent + Send> {
            if s == seat {
                Box::new(focus_bot(cfg, seed.wrapping_add(s as u64), 0.3))
            } else {
                create(id, seed.wrapping_add(s as u64)).expect("bot")
            }
        })
        .collect();
    for _ in 0..STALL_HORIZON {
        if game.is_terminal() {
            break;
        }
        let who = game.current_player();
        let obs = game.observation(who)?;
        let a = bots[who].choose(&obs, &game.legal_actions());
        game.apply(a).ok()?;
    }
    let r = game.results()?;
    Some((f64::from(r.scores[seat].total), if r.winners.contains(&seat) { 1.0 / r.winners.len() as f64 } else { 0.0 }))
}

struct Position {
    game: Game,
    seats: Vec<String>,
    seat: usize,
    seller: usize,
    chosen: Action,
    p_buy: f64,
    rank: usize,
    gap: f64,
    v_before: f32,
    v_after: f32,
    cheap: u32,
}

fn collect(cfg: &Cfg, want: usize) -> Vec<Position> {
    let mut found = Vec::new();
    let mut rng = Rng::new(cfg.seed);
    let mut index = 0u64;
    while found.len() < want && index < 4000 {
        index += 1;
        let n = 3 + rng.below(3) as usize;
        let mut seats: Vec<String> = (0..n).map(|_| cfg.pool[rng.below(cfg.pool.len() as u32) as usize].clone()).collect();
        let seat = rng.below(n as u32) as usize;
        seats[seat] = "focus".into();
        let game_seed = cfg.seed.wrapping_add(index).wrapping_mul(0xD134_2543_DE82_EF95) | 1;
        let mut bots: Vec<Box<dyn Opponent + Send>> = seats
            .iter()
            .enumerate()
            .map(|(s, id)| -> Box<dyn Opponent + Send> {
                if s == seat { Box::new(focus_bot(cfg, game_seed + s as u64, 0.0)) } else { create(id, game_seed + s as u64).expect("bot") }
            })
            .collect();
        let mut game = Game::new(GameConfig::new(n).with_seed(game_seed)).expect("game");
        let mut taken_this_game = 0;
        for _ in 0..STALL_HORIZON {
            if game.is_terminal() || taken_this_game >= 3 {
                break;
            }
            let who = game.current_player();
            let obs = game.observation(who).expect("seat");
            let legal = game.legal_actions();
            let action = bots[who].choose(&obs, &legal);
            if who == seat && obs.phase == PhaseView::Open(Phase::Main) && obs.turn.actions_left > 0 && rng.below(4) == 0 {
                let best = (0..n).filter(|&s| s != seat).map(|s| (cheap_lots(&game, seat, s), s)).max();
                if let Some((cheap, seller)) = best.filter(|b| b.0 > 0) {
                    if legal.contains(&Action::FactoryPurchase(seller)) && legal.len() <= MAX_ACTIONS {
                        let (logits, v_before, kept) = forward(cfg, &game, seat, &legal);
                        let k = kept.iter().position(|&i| legal[i] == Action::FactoryPurchase(seller)).expect("kept");
                        let top = logits.iter().cloned().fold(f32::MIN, f32::max);
                        let z: f64 = logits.iter().map(|l| f64::from(l - top).exp()).sum();
                        let mut after = game.clone();
                        after.apply(Action::FactoryPurchase(seller)).expect("legal");
                        buy_cheap(&mut after);
                        found.push(Position {
                            game: game.clone(),
                            seats: seats.clone(),
                            seat,
                            seller,
                            chosen: action,
                            p_buy: f64::from(logits[k] - top).exp() / z,
                            rank: logits.iter().filter(|&&l| l > logits[k]).count(),
                            gap: f64::from(top - logits[k]),
                            v_before,
                            v_after: state_value(cfg, &after, seat),
                            cheap,
                        });
                        taken_this_game += 1;
                    }
                }
            }
            game.apply(action).expect("legal");
        }
    }
    found
}

fn mean_ci(x: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let m = x.iter().sum::<f64>() / n;
    let v = x.iter().map(|y| (y - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0);
    (m, 1.96 * (v / n).sqrt())
}

fn main() {
    let mut net_path = String::new();
    let mut rules = "dealer".to_string();
    let mut pool = "discounter,bargain,trader".to_string();
    let (mut positions, mut rollouts, mut seed, mut threads) = (150usize, 24u64, 1u64, 12usize);
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(f) = args.next() {
        let v = args.next().expect("value");
        match f.as_str() {
            "--net" => net_path = v,
            "--rules" => rules = v,
            "--pool" => pool = v,
            "--positions" => positions = v.parse().unwrap(),
            "--rollouts" => rollouts = v.parse().unwrap(),
            "--seed" => seed = v.parse().unwrap(),
            "--threads" => threads = v.parse().unwrap(),
            "--out" => out = Some(v),
            _ => panic!("unknown flag {f}"),
        }
    }
    let net = Arc::new(Net::from_bytes(&std::fs::read(&net_path).expect("net file")).expect("net"));
    let cfg = Cfg { net, dealer_rules: rules == "dealer", pool: pool.split(',').map(str::to_string).collect(), seed };
    let found = collect(&cfg, positions);
    let (cfg, found) = (&cfg, &found);
    // For each position, paired rollouts: (forced purchase) - (the net's own choice).
    let rows: Vec<(usize, Vec<(f64, f64, f64, f64)>)> = std::thread::scope(|scope| {
        let hs: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    (t..found.len()).step_by(threads).map(|i| {
                        let p = &found[i];
                        let mut pairs = Vec::new();
                        for r in 0..rollouts {
                            let rs = seed * 1_000_003 + i as u64 * 977 + r;
                            let mut a = p.game.clone();
                            a.apply(Action::FactoryPurchase(p.seller)).expect("legal");
                            buy_cheap(&mut a);
                            let mut b = p.game.clone();
                            b.apply(p.chosen).expect("legal");
                            if let (Some(x), Some(y)) = (play_out(cfg, a, &p.seats, p.seat, rs), play_out(cfg, b, &p.seats, p.seat, rs)) {
                                pairs.push((x.0, y.0, x.1, y.1));
                            }
                        }
                        (i, pairs)
                    }).collect::<Vec<_>>()
                })
            })
            .collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    let (mut ds, mut dw, mut pb, mut va, mut gaps) = (vec![], vec![], vec![], vec![], vec![]);
    let (mut rank0, mut played_buy) = (0.0, 0.0);
    let mut per_pos_ds = vec![];
    for (i, pairs) in &rows {
        let p = &found[*i];
        if pairs.is_empty() {
            continue;
        }
        let n = pairs.len() as f64;
        let d_score = pairs.iter().map(|q| q.0 - q.1).sum::<f64>() / n;
        let d_win = pairs.iter().map(|q| q.2 - q.3).sum::<f64>() / n;
        ds.push(d_score);
        dw.push(d_win);
        per_pos_ds.push((d_score, f64::from(p.v_after - p.v_before)));
        pb.push(p.p_buy);
        va.push(f64::from(p.v_after - p.v_before));
        gaps.push(p.gap);
        rank0 += f64::from(p.rank == 0);
        played_buy += f64::from(matches!(p.chosen, Action::FactoryPurchase(_)));
    }
    let n = ds.len() as f64;
    let corr = {
        let (mx, my) = (per_pos_ds.iter().map(|p| p.0).sum::<f64>() / n, per_pos_ds.iter().map(|p| p.1).sum::<f64>() / n);
        let c: f64 = per_pos_ds.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
        let sx: f64 = per_pos_ds.iter().map(|p| (p.0 - mx).powi(2)).sum::<f64>().sqrt();
        let sy: f64 = per_pos_ds.iter().map(|p| (p.1 - my).powi(2)).sum::<f64>().sqrt();
        c / (sx * sy).max(1e-12)
    };
    let (s, sc) = mean_ci(&ds);
    let (w, wc) = mean_ci(&dw);
    let (v, vc) = mean_ci(&va);
    let positive = ds.iter().filter(|&&d| d > 0.0).count() as f64 / n;
    let text = format!(
        "{{\"net\":\"{net_path}\",\"pool\":\"{pool}\",\"positions\":{},\"rollouts\":{rollouts},\"mean_cheap_offered\":{:.2},\
\"p_buy_mean\":{:.4},\"buy_is_top\":{:.3},\"buy_was_played\":{:.3},\"logit_gap_mean\":{:.3},\
\"value_gain_after_buy\":{:.4},\"value_gain_ci\":{:.4},\
\"truth_score_gain\":{:.2},\"truth_score_ci\":{:.2},\"truth_win_gain\":{:.4},\"truth_win_ci\":{:.4},\
\"share_positions_buy_better\":{:.3},\"corr_value_gain_vs_truth\":{:.3}}}\n",
        ds.len(),
        found.iter().map(|p| f64::from(p.cheap)).sum::<f64>() / found.len() as f64,
        pb.iter().sum::<f64>() / n,
        rank0 / n,
        played_buy / n,
        gaps.iter().sum::<f64>() / n,
        v, vc, s, sc, w, wc, positive, corr
    );
    match out {
        Some(p) => std::fs::write(p, text).unwrap(),
        None => print!("{text}"),
    }
}
