//! Whole games played through the public API only, the way a UI, a bot or a trainer would play them.

use std::collections::{BTreeSet, HashSet};

use container_engine::action::{Action, ActionError, Building, District};
use container_engine::color::{Color, ColorCounts};
use container_engine::config::{GameConfig, GameLength, MAX_PLAYERS, MIN_PLAYERS};
use container_engine::game::Game;
use container_engine::loans::MAX_LOANS;
use container_engine::observation::Observation;
use container_engine::phase::Phase;
use container_engine::rng::Rng;
use container_engine::state::{AuctionKind, HARBOR_LOTS, ShipLocation};

/// How a seat chooses: from what it can see and what it may do, and nothing else.
type Policy = fn(&Observation, &[Action], &mut Rng) -> Action;

fn uniform(_: &Observation, legal: &[Action], rng: &mut Rng) -> Action {
    legal[rng.below(legal.len() as u32) as usize]
}

fn first(_: &Observation, legal: &[Action], _: &mut Rng) -> Action {
    legal[0]
}

fn last(_: &Observation, legal: &[Action], _: &mut Rng) -> Action {
    legal[legal.len() - 1]
}

/// Keeps the economy going: produces, sells cheap, buys, delivers and bids small, but is still a
/// gambler, since every legal action keeps some chance.
fn sensible(obs: &Observation, legal: &[Action], rng: &mut Rng) -> Action {
    let weights: Vec<u32> = legal.iter().map(|&a| weight(obs, a)).collect();
    let mut pick = rng.below(weights.iter().sum());
    for (&action, &weight) in legal.iter().zip(&weights) {
        if pick < weight {
            return action;
        }
        pick -= weight;
    }
    unreachable!("the pick is below the sum of the weights")
}

fn weight(obs: &Observation, action: Action) -> u32 {
    let broke = obs.players[obs.viewer].cash.is_some_and(|cash| cash < 3);
    let cheap = |n: u32| 1 + 10u32.saturating_sub(n);
    match action {
        Action::Produce => 12,
        Action::ProduceColor(_) => 1,
        Action::Build(_) => 2,
        Action::FactoryPurchase(_) => 6,
        Action::Buy { lot, .. } => 1 + 6 / (lot as u32 + 1),
        Action::DoneBuying => 2,
        Action::Sail(ShipLocation::Island) => 12,
        Action::Sail(ShipLocation::Ocean) => 6,
        Action::Sail(_) => 3,
        Action::HarborPurchase => 10,
        Action::Load(_) => 6,
        Action::DoneLoading => 2,
        Action::Reprice(_) => 2,
        Action::Place(lot) => (6 - lot.min(5) as u32).pow(2),
        Action::Bid(amount) | Action::BidCash(amount) => cheap(amount),
        Action::Accept(_) => 8,
        Action::BuyOut => 2,
        Action::RequestLoan if broke => 6,
        Action::RequestLoan | Action::RepayLoan => 1,
        Action::Seize { .. } | Action::Deal(_) => 1,
        Action::StartAuction { .. } | Action::Outbid(_) => 1,
        Action::BidContainer { .. } => 3,
        Action::DoneBidding => 2,
    }
}

struct Playout {
    game: Game,
    trace: Vec<Action>,
}

/// Plays until the game is over or `limit` actions were taken. The seats take their decisions from one
/// random stream that is seeded by the game's own seed, so a game is a function of its configuration
/// and of who sits where. `watch` sees every state, and the action chosen in it, before it is applied.
fn play(
    config: GameConfig,
    seats: &[Policy],
    limit: usize,
    mut watch: impl FnMut(&Game, Action),
) -> Playout {
    let mut game = Game::new(config).expect("a valid configuration");
    let mut rng = Rng::new(config.seed ^ 0x5EED);
    let mut trace = Vec::new();
    while !game.is_terminal() && trace.len() < limit {
        let legal = game.legal_actions();
        assert!(!legal.is_empty(), "stuck in {:?}", game.phase);
        let viewer = game.current_player();
        let obs = game
            .observation(viewer)
            .expect("the player to decide sits at the table");
        let action = seats[viewer % seats.len()](&obs, &legal, &mut rng);
        watch(&game, action);
        game.apply(action)
            .unwrap_or_else(|e| panic!("{e:?} in {:?}", game.phase));
        game.check_invariants()
            .unwrap_or_else(|e| panic!("{e}, after {action:?}"));
        trace.push(action);
    }
    Playout { game, trace }
}

/// Every player count and game length, with `seeds` games of each.
fn configs(seeds: u64) -> Vec<GameConfig> {
    let lengths = [
        GameLength::Short,
        GameLength::Standard,
        GameLength::Extended,
    ];
    let mut all = Vec::new();
    for players in MIN_PLAYERS..=MAX_PLAYERS {
        for length in lengths {
            for seed in 0..seeds {
                all.push(GameConfig::new(players).with_length(length).with_seed(seed));
            }
        }
    }
    all
}

/// Every action there is, with seats, lots and amounts well beyond anything a game allows.
fn universe(max_amount: u32) -> Vec<Action> {
    let seats: Vec<usize> = (0..MAX_PLAYERS + 2).chain([usize::MAX]).collect();
    let lots: Vec<usize> = (0..HARBOR_LOTS + 2).chain([usize::MAX]).collect();
    let districts = [District::Factory, District::Harbor];
    let mut all = vec![
        Action::Produce,
        Action::Build(Building::Warehouse),
        Action::DoneBuying,
        Action::HarborPurchase,
        Action::DoneLoading,
        Action::BuyOut,
        Action::RequestLoan,
        Action::RepayLoan,
        Action::DoneBidding,
        Action::Sail(ShipLocation::Ocean),
        Action::Sail(ShipLocation::Island),
        Action::Sail(ShipLocation::Bank),
    ];
    for color in Color::ALL {
        all.extend([
            Action::ProduceColor(color),
            Action::Build(Building::Factory(color)),
            Action::Load(color),
            Action::Deal(color),
        ]);
        for &lot in &lots {
            all.push(Action::Buy { lot, color });
            all.push(Action::Seize { lot, color });
            for district in districts {
                all.push(Action::BidContainer {
                    district,
                    lot,
                    color,
                });
            }
        }
    }
    for &seat in &seats {
        all.extend([
            Action::FactoryPurchase(seat),
            Action::Sail(ShipLocation::Harbor(seat)),
            Action::Accept(seat),
        ]);
    }
    all.extend(districts.map(Action::Reprice));
    for &lot in &lots {
        all.push(Action::Place(lot));
        all.extend(AuctionKind::ALL.map(|kind| Action::StartAuction { kind, lot }));
    }
    all.extend(AuctionKind::ALL.map(Action::Outbid));
    for amount in (0..=max_amount + 2).chain([u32::MAX]) {
        all.extend([Action::Bid(amount), Action::BidCash(amount)]);
    }
    all
}

/// The most a player could name in a bid: their cash.
fn richest(game: &Game) -> u32 {
    game.state
        .players()
        .iter()
        .map(|p| p.cash)
        .max()
        .expect("players")
}

/// Games that end by themselves must do so well within this many actions; the longest of the games
/// below take under two thousand.
const LIMIT: usize = 20_000;

/// Tables of players that gamble, always take the first or the last option, or mix: they get
/// everywhere but seldom to the end of a game, so their games are cut short.
const GAMBLERS: [&[Policy]; 6] = [
    &[uniform],
    &[first],
    &[last],
    &[sensible, uniform],
    &[uniform, sensible, first],
    &[last, sensible, uniform, first],
];

#[test]
fn the_legal_list_is_exactly_what_the_game_accepts() {
    let mut step = 0;
    let mut check = |game: &Game, _: Action| {
        let legal = game.legal_actions();
        let distinct: HashSet<&Action> = legal.iter().collect();
        assert_eq!(
            distinct.len(),
            legal.len(),
            "an action is listed twice in {:?}",
            game.phase
        );
        let everything = universe(richest(game));
        for action in &legal {
            assert!(
                everything.contains(action),
                "{action:?} is not in the universe"
            );
        }
        for &action in &everything {
            assert_eq!(
                game.is_legal(action),
                legal.contains(&action),
                "{action:?} in {:?}",
                game.phase
            );
        }
        // Refusing an action must leave the game as it was, so only a rotating sample is tried.
        for &action in everything.iter().skip(step % 50).step_by(50) {
            if !legal.contains(&action) {
                let mut copy = game.clone();
                assert_eq!(copy.apply(action), Err(ActionError::Illegal(action)));
                assert_eq!(copy, *game, "{action:?} was refused but changed the game");
            }
        }
        step += 1;
    };
    for config in configs(2) {
        play(config, &[sensible], LIMIT, &mut check);
    }
    for table in GAMBLERS {
        for config in configs(1) {
            play(config, table, 800, &mut check);
        }
    }
}

#[test]
fn every_listed_action_applies_changes_the_game_and_leaves_a_way_on() {
    let check = |game: &Game, _: Action| {
        for action in game.legal_actions() {
            let mut next = game.clone();
            next.apply(action)
                .unwrap_or_else(|e| panic!("{e:?} was listed in {:?}", game.phase));
            assert_ne!(
                next, *game,
                "{action:?} changed nothing in {:?}",
                game.phase
            );
            next.check_invariants()
                .unwrap_or_else(|e| panic!("{e}, after {action:?} in {:?}", game.phase));
            assert_eq!(
                next.legal_actions().is_empty(),
                next.is_terminal(),
                "no way on, or a way on in a finished game, after {action:?} in {:?}",
                game.phase
            );
        }
    };
    for config in configs(1) {
        play(config, &[sensible], LIMIT, check);
    }
    for table in GAMBLERS {
        for config in configs(1) {
            play(config, table, 500, check);
        }
    }
}

#[test]
fn sensible_players_bring_every_game_to_its_end() {
    let games = configs(6);
    let mut bankrupted = 0;
    for &config in &games {
        let Playout { game, trace } = play(config, &[sensible], LIMIT, |_, _| {});
        let seed = (config.num_players, config.length, config.seed);
        assert!(
            game.is_terminal(),
            "{seed:?} was still going after {} actions",
            trace.len()
        );

        let out_of: Vec<Color> = Color::ALL
            .into_iter()
            .filter(|&c| game.state.supply.containers[c] == 0)
            .collect();
        if out_of.len() < 2 {
            // The only other way a game ends is a table that has gone bankrupt.
            assert!(
                game.is_deadlocked(),
                "{seed:?} ended with the supply out of {out_of:?}"
            );
            bankrupted += 1;
        }
        let results = game.results().expect("a finished game has results");
        assert_eq!(results.scores.len(), config.num_players);
        let best = results.scores.iter().map(|s| s.total).max().unwrap();
        assert!(!results.winners.is_empty());
        for &winner in &results.winners {
            assert_eq!(
                results.scores[winner].total, best,
                "{seed:?}: player {winner} wins"
            );
        }

        assert!(game.legal_actions().is_empty());
        for seat in 0..config.num_players {
            let view = game.observation(seat).expect("a seat at the table");
            assert!(
                view.players
                    .iter()
                    .all(|p| p.cash.is_some() && p.scoring_card.is_some())
            );
        }
    }
    // Gamblers do go bankrupt now and then, but a bot that keeps its economy going rarely does.
    assert!(
        bankrupted * 20 <= games.len(),
        "{bankrupted} of {} games ended in bankruptcy",
        games.len()
    );
}

/// The rules don't end a game in which everybody is broke, so the engine does: it can't go on.
#[test]
fn a_table_of_broke_players_ends_the_game_instead_of_getting_stuck() {
    let config = GameConfig::new(4)
        .with_length(GameLength::Short)
        .with_seed(3);
    let mut game = Game::new(config).expect("a valid configuration");
    assert!(!game.is_deadlocked());
    for p in game.state.players.iter_mut().take(config.num_players) {
        game.state.cash_burned += p.cash;
        p.cash = 0;
        p.loans = MAX_LOANS;
        for lot in p.factory_lots.iter_mut().chain(&mut p.harbor_lots) {
            game.state.supply.containers.add_all(lot);
            *lot = ColorCounts::default();
        }
    }
    game.check_invariants().expect("a consistent game");
    assert!(game.is_deadlocked());
    let mut rng = Rng::new(9);
    // Interest is forgiven for lack of anything to seize, and the first turn ends after two actions.
    for _ in 0..2 {
        assert!(!game.is_terminal());
        let legal = game.legal_actions();
        game.apply(legal[rng.below(legal.len() as u32) as usize])
            .expect("a listed action");
        game.check_invariants().expect("a consistent game");
    }
    assert!(game.is_terminal());
    let results = game.results().expect("a finished game has results");
    assert_eq!(results.scores.len(), config.num_players);
    assert!(game.legal_actions().is_empty());
}

#[test]
fn a_finished_game_refuses_everything() {
    let config = GameConfig::new(4)
        .with_length(GameLength::Short)
        .with_seed(7);
    let Playout { mut game, .. } = play(config, &[sensible], LIMIT, |_, _| {});
    assert!(game.is_terminal());
    let before = game.clone();
    for action in universe(richest(&game)) {
        assert_eq!(game.apply(action), Err(ActionError::Illegal(action)));
    }
    assert_eq!(game, before);
}

#[test]
fn reckless_and_adversarial_tables_never_break_the_game() {
    for table in GAMBLERS {
        for config in configs(2) {
            // `play` checks the invariants and that there is always a way on, after every action.
            play(config, table, 3_000, |_, _| {});
        }
    }
}

#[test]
fn the_same_seed_gives_the_same_game_action_for_action() {
    for config in configs(2) {
        let table: &[Policy] = &[sensible, uniform];
        let one = play(config, table, LIMIT, |_, _| {});
        let two = play(config, table, LIMIT, |_, _| {});
        assert_eq!(one.trace, two.trace, "{config:?}");
        assert_eq!(one.game, two.game, "{config:?}");
    }
}

#[test]
fn replaying_the_actions_rebuilds_the_game_state_by_state() {
    for config in configs(2) {
        let mut states = Vec::new();
        let Playout { game, trace } = play(config, &[sensible], LIMIT, |game, _| {
            states.push(game.clone());
        });
        let mut replay = Game::new(config).expect("a valid configuration");
        for (step, (&action, state)) in trace.iter().zip(&states).enumerate() {
            assert_eq!(replay, *state, "{config:?} diverged before action {step}");
            replay.apply(action).expect("a recorded action");
        }
        assert_eq!(replay, game, "{config:?}");
    }
}

#[test]
fn different_seeds_deal_and_play_different_games() {
    let games: Vec<(Game, Vec<Action>)> = (0..8)
        .map(|seed| {
            let config = GameConfig::new(4)
                .with_length(GameLength::Short)
                .with_seed(seed);
            let start = Game::new(config).expect("a valid configuration");
            (start, play(config, &[sensible], LIMIT, |_, _| {}).trace)
        })
        .collect();
    for (a, (start, trace)) in games.iter().enumerate() {
        for (b, (other_start, other_trace)) in games.iter().enumerate().skip(a + 1) {
            assert_ne!(
                start.state, other_start.state,
                "seeds {a} and {b} deal alike"
            );
            assert_ne!(trace, other_trace, "seeds {a} and {b} play alike");
        }
    }
}

#[test]
fn a_clone_plays_on_from_where_it_was_taken_without_disturbing_the_original() {
    for config in configs(1) {
        let mut states = Vec::new();
        let Playout { game, trace } = play(config, &[sensible], LIMIT, |game, _| {
            states.push(game.clone());
        });
        for at in [0, trace.len() / 3, trace.len() / 2, trace.len() - 1] {
            let mut fork = states[at].clone();
            for &action in &trace[at..] {
                fork.apply(action).expect("a recorded action");
            }
            assert_eq!(fork, game, "{config:?}, forked before action {at}");
            assert_ne!(
                states[at], game,
                "{config:?}: the fork's moves changed its source"
            );
        }
    }
}

const DECISIONS: [&str; 10] = [
    "the main decision",
    "which colors to produce",
    "buying",
    "loading",
    "pricing",
    "a delivery bid",
    "settling a delivery",
    "seizing in a default",
    "dealing out the seized",
    "a Bank bid",
];

const SITUATIONS: [&str; 5] = [
    "a runoff between tied bids",
    "tied bids to settle",
    "buying from a factory district",
    "buying from a harbor district",
    "the last turn",
];

/// Matches are exhaustive on purpose: a new decision has to be named here, and so has to be played
/// by the tables below.
fn decision_name(phase: Phase) -> &'static str {
    DECISIONS[match phase {
        Phase::Main => 0,
        Phase::Produce(_) => 1,
        Phase::Buy(_) => 2,
        Phase::Load(_) => 3,
        Phase::Arrange(_) => 4,
        Phase::Bid(_) => 5,
        Phase::Decide(_) => 6,
        Phase::Seize(_) => 7,
        Phase::Distribute(_) => 8,
        Phase::BankBid(_) => 9,
        Phase::GameOver => unreachable!("nothing is played in a finished game"),
    }]
}

/// Every kind of action, by what it does rather than by what it is called.
fn action_name(action: Action) -> &'static str {
    match action {
        Action::Produce => "produce",
        Action::ProduceColor(_) => "produce a color",
        Action::Build(Building::Factory(_)) => "build a factory",
        Action::Build(Building::Warehouse) => "build a warehouse",
        Action::FactoryPurchase(_) => "start a factory purchase",
        Action::Buy { .. } => "buy a container",
        Action::DoneBuying => "stop buying",
        Action::Sail(ShipLocation::Ocean) => "sail to the ocean",
        Action::Sail(ShipLocation::Island) => "sail to the island",
        Action::Sail(ShipLocation::Bank) => "sail to the Bank",
        Action::Sail(ShipLocation::Harbor(_)) => "sail to a harbor",
        Action::HarborPurchase => "harbor purchase",
        Action::Load(_) => "load a container",
        Action::DoneLoading => "stop loading",
        Action::Reprice(District::Factory) => "reprice the factory district",
        Action::Reprice(District::Harbor) => "reprice the harbor district",
        Action::Place(_) => "place a container",
        Action::Bid(_) => "bid in a delivery",
        Action::Accept(_) => "accept a bid",
        Action::BuyOut => "buy out",
        Action::RequestLoan => "request a loan",
        Action::RepayLoan => "repay a loan",
        Action::Seize { .. } => "seize a container",
        Action::Deal(_) => "deal a container",
        Action::StartAuction {
            kind: AuctionKind::ContainerLot,
            ..
        } => "start a container lot auction",
        Action::StartAuction {
            kind: AuctionKind::CashLot,
            ..
        } => "start a cash lot auction",
        Action::Outbid(AuctionKind::ContainerLot) => "outbid on a container lot",
        Action::Outbid(AuctionKind::CashLot) => "outbid on a cash lot",
        Action::BidCash(_) => "bid cash",
        Action::BidContainer {
            district: District::Factory,
            ..
        } => "bid a factory container",
        Action::BidContainer {
            district: District::Harbor,
            ..
        } => "bid a harbor container",
        Action::DoneBidding => "stop bidding",
    }
}

fn situations(game: &Game) -> Vec<&'static str> {
    let mut found = Vec::new();
    match game.phase {
        Phase::Bid(bidding) if bidding.runoff => found.push(SITUATIONS[0]),
        Phase::Decide(decision) if decision.tied.iter().filter(|&&tied| tied).count() > 1 => {
            found.push(SITUATIONS[1]);
        }
        Phase::Buy(buying) => found.push(match buying.district {
            District::Factory => SITUATIONS[2],
            District::Harbor => SITUATIONS[3],
        }),
        _ => {}
    }
    if game.is_last_turn() {
        found.push(SITUATIONS[4]);
    }
    found
}

#[test]
fn the_playouts_come_across_every_action_decision_and_situation() {
    let mut seen = BTreeSet::new();
    let tables: [&[Policy]; 3] = [
        &[sensible],
        &[sensible, uniform],
        &[uniform, sensible, sensible],
    ];
    for table in tables {
        for config in configs(6) {
            play(config, table, 5_000, |game, action| {
                seen.insert(decision_name(game.phase));
                seen.insert(action_name(action));
                seen.extend(situations(game));
            });
        }
    }
    let required: BTreeSet<&str> = DECISIONS
        .into_iter()
        .chain(SITUATIONS)
        .chain(universe(0).into_iter().map(action_name))
        .collect();
    let missing: Vec<&&str> = required.difference(&seen).collect();
    assert!(missing.is_empty(), "never come across: {missing:?}");
}
