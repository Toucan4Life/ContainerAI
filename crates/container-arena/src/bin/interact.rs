//! `interact --focus magnate --pool magnate,dealer,consortium,merchant,discounter --players 4 --games 400 --seed 1 [--out r.json]`
//!
//! Plays mixed tables and reports, per bot id, how it takes part in the economy next to how well it
//! wins. With `--focus`, every table seats exactly one focus bot (at a random seat) and fills the other
//! seats from `--pool`, so the focus is always measured against others. Without it, every seat is drawn
//! from the pool. Metrics are per seat-game means, measured from the actions each bot actually plays
//! and from what its own seat could see when it played them.
//!
//! The same arguments always print the same JSON.

use std::collections::BTreeMap;

use container_bots::{Opponent, create};
use container_engine::action::{Action, District};
use container_engine::config::{GameConfig, STALL_HORIZON};
use container_engine::game::Game;
use container_engine::observation::{Observation, PhaseView};
use container_engine::phase::Phase;
use container_engine::rng::Rng;
use container_engine::state::ShipLocation;

/// A price at which a buyer cannot lose: a container left on a ship scores $3.
const FAIR_PRICE: u32 = 3;
/// A price a human would call a gift.
const CHEAP_PRICE: u32 = 2;

#[derive(Default, Clone)]
struct Tally {
    seat_games: f64,
    stalled_seat_games: f64,
    wins: f64,
    fair: f64,
    score: f64,
    // Where the final score comes from: cash, Island, leftovers (ship, holding, harbor), loans; `won_*` only in won games.
    cash: f64,
    island: f64,
    leftovers: f64,
    loans: f64,
    won_cash: f64,
    won_island: f64,
    won_leftovers: f64,
    won_loans: f64,
    // Buying from opponents.
    bought: f64,
    bought_factory: f64,
    bought_harbor: f64,
    bought_fair: f64,
    spent: f64,
    // Selling to opponents.
    sold: f64,
    revenue: f64,
    // Delivery auctions.
    bids: f64,
    bid_sum: f64,
    bids_ge3: f64,
    deliveries: f64,
    accepted: f64,
    bought_out: f64,
    accepted_money: f64,
    auctions_won: f64,
    // The Bank and idling.
    bank_actions: f64,
    harbor_sails: f64,
    reprices: f64,
    // What it offers: containers on cheap lots, sampled at each of its own main decisions.
    listed: f64,
    listed_fair: f64,
    // Responsiveness: decisions with a cheap offer in reach, and how often it took one.
    cheap_opportunities: f64,
    cheap_taken: f64,
}

impl Tally {
    fn merge(&mut self, o: &Tally) {
        macro_rules! add { ($($f:ident),*) => { $(self.$f += o.$f;)* } }
        add!(
            seat_games, stalled_seat_games, wins, fair, score, cash, island, leftovers, loans, won_cash, won_island, won_leftovers,
            won_loans, bought, bought_factory, bought_harbor,
            bought_fair, spent, sold, revenue, bids, bid_sum, bids_ge3, deliveries, accepted,
            bought_out, accepted_money, auctions_won, bank_actions, harbor_sails, reprices, listed,
            listed_fair, cheap_opportunities, cheap_taken
        );
    }
}

fn lots_cheap(obs: &Observation, seller: usize, district: District, max_price: u32) -> bool {
    let p = &obs.players[seller];
    let (lots, first): (Vec<_>, u32) = match district {
        District::Factory => (p.factory_lots.to_vec(), 1),
        District::Harbor => (p.harbor_lots.to_vec(), 2),
    };
    let cash = obs.players[obs.viewer].cash.unwrap_or(0);
    lots.iter().enumerate().any(|(lot, n)| {
        let price = first + lot as u32;
        !n.is_empty() && price <= max_price && price <= cash
    })
}

fn record(tallies: &mut [Tally], obs: &Observation, action: Action) {
    let me = obs.viewer;
    let t = &mut tallies[me];
    match (obs.phase, action) {
        (PhaseView::Open(Phase::Main), _) => {
            let p = &obs.players[me];
            for (lot, n) in p.factory_lots.iter().enumerate() {
                t.listed += f64::from(n.total());
                if lot as u32 + 1 <= FAIR_PRICE {
                    t.listed_fair += f64::from(n.total());
                }
            }
            for (lot, n) in p.harbor_lots.iter().enumerate() {
                t.listed += f64::from(n.total());
                if lot as u32 + 2 <= FAIR_PRICE {
                    t.listed_fair += f64::from(n.total());
                }
            }
            let opponents = (0..obs.num_players).filter(|&s| s != me);
            let factory_cheap = |s: usize| lots_cheap(obs, s, District::Factory, CHEAP_PRICE);
            let docked_cheap = match p.ship_location {
                ShipLocation::Harbor(s) if s != me => {
                    lots_cheap(obs, s, District::Harbor, CHEAP_PRICE + 1)
                }
                _ => false,
            };
            let can_act = obs.turn.actions_left > 0;
            let opportunity = can_act && (opponents.clone().any(factory_cheap) || docked_cheap);
            if opportunity {
                t.cheap_opportunities += 1.0;
                let taken = match action {
                    Action::FactoryPurchase(s) => factory_cheap(s),
                    Action::HarborPurchase => docked_cheap,
                    _ => false,
                };
                if taken {
                    t.cheap_taken += 1.0;
                }
            }
            match action {
                Action::Sail(ShipLocation::Harbor(_)) => t.harbor_sails += 1.0,
                Action::Reprice(_) => t.reprices += 1.0,
                Action::StartAuction { .. } | Action::Outbid(_) => t.bank_actions += 1.0,
                _ => {}
            }
        }
        (PhaseView::Open(Phase::Buy(b)), Action::Buy { lot, .. }) => {
            let price = match b.district {
                District::Factory => lot as u32 + 1,
                District::Harbor => lot as u32 + 2,
            };
            t.bought += 1.0;
            t.spent += f64::from(price);
            if price <= FAIR_PRICE {
                t.bought_fair += 1.0;
            }
            match b.district {
                District::Factory => t.bought_factory += 1.0,
                District::Harbor => t.bought_harbor += 1.0,
            }
            let seller = &mut tallies[b.seller];
            seller.sold += 1.0;
            seller.revenue += f64::from(price);
        }
        (PhaseView::Bid(v), Action::Bid(n)) if !v.runoff => {
            t.bids += 1.0;
            t.bid_sum += f64::from(n);
            if n >= 3 {
                t.bids_ge3 += 1.0;
            }
        }
        (PhaseView::Open(Phase::Decide(d)), Action::Accept(winner)) => {
            t.deliveries += 1.0;
            t.accepted += 1.0;
            t.accepted_money += f64::from(d.top);
            tallies[winner].auctions_won += 1.0;
        }
        (PhaseView::Open(Phase::Decide(_)), Action::BuyOut) => {
            t.deliveries += 1.0;
            t.bought_out += 1.0;
        }
        (PhaseView::Open(Phase::BankBid(_)), _) => {}
        _ => {}
    }
}

/// One game with `seats`; per-seat tallies, or all-stalled ones.
fn play(seats: &[String], game_seed: u64, horizon: usize) -> Vec<Tally> {
    let n = seats.len();
    let mut bots: Vec<Box<dyn Opponent + Send>> = seats
        .iter()
        .enumerate()
        .map(|(seat, id)| create(id, game_seed.wrapping_add(seat as u64 + 1)).expect("known bot"))
        .collect();
    let mut tallies = vec![Tally { seat_games: 1.0, ..Tally::default() }; n];
    let mut game = Game::new(GameConfig::new(n).with_seed(game_seed)).expect("game");
    for _ in 0..horizon {
        if game.is_terminal() {
            break;
        }
        let seat = game.current_player();
        let obs = game.observation(seat).expect("seat");
        let legal = game.legal_actions();
        let action = bots[seat].choose(&obs, &legal);
        record(&mut tallies, &obs, action);
        for bot in bots.iter_mut() {
            bot.observe(seat, &obs, action);
        }
        game.apply(action).expect("legal");
    }
    match game.is_terminal().then(|| game.results()).flatten() {
        Some(r) => {
            for (seat, t) in tallies.iter_mut().enumerate() {
                t.score = f64::from(r.scores[seat].total);
                t.fair = 1.0 / n as f64;
                let b = &r.scores[seat];
                let (c, i, l, d) = (f64::from(b.cash), f64::from(b.island), f64::from(b.leftovers), f64::from(b.loans));
                (t.cash, t.island, t.leftovers, t.loans) = (c, i, l, d);
                if r.winners.contains(&seat) {
                    t.wins = 1.0 / r.winners.len() as f64;
                    (t.won_cash, t.won_island, t.won_leftovers, t.won_loans) = (c * t.wins, i * t.wins, l * t.wins, d * t.wins);
                }
            }
        }
        None => {
            for t in &mut tallies {
                *t = Tally { seat_games: 1.0, stalled_seat_games: 1.0, ..Tally::default() };
            }
        }
    }
    tallies
}

struct Args {
    focus: Option<String>,
    pool: Vec<String>,
    players: Vec<usize>,
    games: u64,
    seed: u64,
    threads: usize,
    horizon: usize,
    out: Option<String>,
}

fn fail(message: &str) -> ! {
    eprintln!("interact: {message}");
    std::process::exit(2);
}

fn parse() -> Args {
    let mut a = Args {
        focus: None,
        pool: vec!["magnate".into(), "dealer".into(), "consortium".into(), "merchant".into()],
        players: vec![4],
        games: 200,
        seed: 1,
        threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
        horizon: STALL_HORIZON,
        out: None,
    };
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().unwrap_or_else(|| fail(&format!("{flag} needs a value")));
        let num = |v: &str| v.parse::<u64>().unwrap_or_else(|_| fail(&format!("bad {flag}")));
        match flag.as_str() {
            "--focus" => a.focus = Some(value),
            "--pool" => a.pool = value.split(',').map(str::to_string).collect(),
            "--players" => a.players = value.split(',').map(|p| num(p) as usize).collect(),
            "--games" => a.games = num(&value),
            "--seed" => a.seed = num(&value),
            "--threads" => a.threads = num(&value) as usize,
            "--horizon" => a.horizon = num(&value) as usize,
            "--out" => a.out = Some(value),
            _ => fail(&format!("unknown flag {flag}")),
        }
    }
    if a.players.iter().any(|p| !(3..=5).contains(p)) {
        fail("--players takes values from 3 to 5");
    }
    for id in a.pool.iter().chain(a.focus.iter()) {
        if create(id, 0).is_none() {
            fail(&format!("unknown bot {id}"));
        }
    }
    a
}

/// The seats of game `index`, its seed and the focus seat, if any.
fn table(a: &Args, index: u64) -> (Vec<String>, u64, Option<usize>) {
    let n = a.players[index as usize % a.players.len()];
    let mut rng = Rng::new(a.seed ^ index.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut seats: Vec<String> = (0..n)
        .map(|_| a.pool[rng.below(a.pool.len() as u32) as usize].clone())
        .collect();
    let mut focus_seat = None;
    if let Some(focus) = &a.focus {
        let seat = rng.below(n as u32) as usize;
        seats[seat] = focus.clone();
        focus_seat = Some(seat);
    }
    let game_seed = a.seed.wrapping_add(index).wrapping_mul(0xD134_2543_DE82_EF95) | 1;
    (seats, game_seed, focus_seat)
}

fn json(id: &str, t: &Tally) -> String {
    let done = (t.seat_games - t.stalled_seat_games).max(1.0);
    let per = |x: f64| x / done;
    let ratio = |x: f64, y: f64| if y > 0.0 { x / y } else { 0.0 };
    let share = ratio(t.wins, done);
    let ci = 1.96 * (share * (1.0 - share) / done).sqrt();
    format!(
        concat!(
            "\"{id}\":{{\"seat_games\":{sg},\"stall_rate\":{stall:.4},",
            "\"win_share\":{ws:.4},\"win_ci95\":{ci:.4},\"fair_share\":{fair:.4},\"score\":{score:.2},",
            "\"cash\":{cash:.2},\"island\":{island:.2},\"leftovers\":{left:.2},\"loans\":{loans:.2},",
            "\"won_cash\":{wc:.2},\"won_island\":{wi:.2},\"won_leftovers\":{wl:.2},\"won_loans\":{wn:.2},",
            "\"bought\":{bought:.2},\"bought_factory\":{bf:.2},\"bought_harbor\":{bh:.2},",
            "\"mean_price_paid\":{pp:.3},\"bought_fair_share\":{bfs:.3},",
            "\"sold\":{sold:.2},\"mean_price_received\":{pr:.3},",
            "\"bids\":{bids:.2},\"mean_bid\":{mb:.3},\"bids_ge3_share\":{b3:.3},",
            "\"deliveries\":{del:.2},\"accepted\":{acc:.2},\"bought_out\":{bo:.2},\"mean_accepted_bid\":{mab:.3},\"auctions_won\":{aw:.2},",
            "\"bank_actions\":{bank:.2},\"harbor_sails\":{hs:.2},\"reprices\":{rp:.2},",
            "\"listed_fair_share\":{lf:.3},",
            "\"cheap_opportunities\":{co:.2},\"cheap_take_rate\":{ct:.3}}}"
        ),
        id = id,
        sg = t.seat_games,
        stall = ratio(t.stalled_seat_games, t.seat_games),
        ws = share,
        ci = ci,
        fair = ratio(t.fair, done),
        score = per(t.score),
        cash = per(t.cash),
        island = per(t.island),
        left = per(t.leftovers),
        loans = per(t.loans),
        wc = ratio(t.won_cash, t.wins),
        wi = ratio(t.won_island, t.wins),
        wl = ratio(t.won_leftovers, t.wins),
        wn = ratio(t.won_loans, t.wins),
        bought = per(t.bought),
        bf = per(t.bought_factory),
        bh = per(t.bought_harbor),
        pp = ratio(t.spent, t.bought),
        bfs = ratio(t.bought_fair, t.bought),
        sold = per(t.sold),
        pr = ratio(t.revenue, t.sold),
        bids = per(t.bids),
        mb = ratio(t.bid_sum, t.bids),
        b3 = ratio(t.bids_ge3, t.bids),
        del = per(t.deliveries),
        acc = per(t.accepted),
        bo = per(t.bought_out),
        mab = ratio(t.accepted_money, t.accepted),
        aw = per(t.auctions_won),
        bank = per(t.bank_actions),
        hs = per(t.harbor_sails),
        rp = per(t.reprices),
        lf = ratio(t.listed_fair, t.listed),
        co = per(t.cheap_opportunities),
        ct = ratio(t.cheap_taken, t.cheap_opportunities),
    )
}

fn main() {
    let a = parse();
    let a = &a;
    let mut results: Vec<(u64, Vec<String>, Option<usize>, Vec<Tally>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..a.threads.max(1) as u64)
            .map(|th| {
                scope.spawn(move || {
                    (th..a.games)
                        .step_by(a.threads.max(1))
                        .map(|i| {
                            let (seats, seed, focus) = table(a, i);
                            let tallies = play(&seats, seed, a.horizon);
                            (i, seats, focus, tallies)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().expect("thread")).collect()
    });
    results.sort_by_key(|r| r.0);
    let mut by_bot: BTreeMap<String, Tally> = BTreeMap::new();
    // The focus seat is reported as "focus", apart from fillers that are the same bot.
    for (_, seats, focus, tallies) in &results {
        for (seat, (id, t)) in seats.iter().zip(tallies).enumerate() {
            let key = if *focus == Some(seat) { "focus" } else { id };
            by_bot.entry(key.to_string()).or_default().merge(t);
        }
    }
    let bots: Vec<String> = by_bot.iter().map(|(id, t)| json(id, t)).collect();
    let text = format!(
        "{{\"focus\":{},\"pool\":{:?},\"players\":{:?},\"games\":{},\"seed\":{},\"horizon\":{},\"bots\":{{{}}}}}\n",
        a.focus.as_ref().map_or("null".to_string(), |f| format!("\"{f}\"")),
        a.pool,
        a.players,
        a.games,
        a.seed,
        a.horizon,
        bots.join(",")
    );
    match &a.out {
        Some(p) => std::fs::write(p, text).unwrap_or_else(|e| fail(&e.to_string())),
        None => print!("{text}"),
    }
}
