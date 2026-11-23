use container_engine::action::Action;
use container_engine::config::{GameConfig, GameLength, MAX_PLAYERS, MIN_PLAYERS};
use container_engine::game::Game;

use crate::{OPPONENTS, consortium, create, magnate};

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
fn broker_never_opens_a_delivery_auction_under_its_minimum_bid() {
    use container_engine::observation::PhaseView;
    let mut opening_bids = 0;
    for seed in 0..6 {
        let mut game = Game::new(config(4, GameLength::Standard, seed)).unwrap();
        let mut seats: Vec<_> = (0..4).map(|s| create("broker", s).unwrap()).collect();
        while !game.is_terminal() {
            let viewer = game.current_player();
            let obs = game.observation(viewer).unwrap();
            let action = seats[viewer].choose(&obs, &game.legal_actions());
            if let (PhaseView::Bid(b), Action::Bid(amount)) = (obs.phase, action) {
                if !b.runoff {
                    opening_bids += 1;
                    assert!(amount >= 3.min(obs.players[viewer].cash.unwrap()), "bid {amount}");
                }
            }
            game.apply(action).unwrap();
        }
    }
    assert!(opening_bids > 0);
}

#[test]
fn dealer_never_opens_a_delivery_auction_under_its_minimum_bid() {
    use container_engine::observation::PhaseView;
    let mut opening_bids = 0;
    for seed in 0..6 {
        let mut game = Game::new(config(4, GameLength::Standard, seed)).unwrap();
        let mut seats: Vec<_> = (0..4).map(|s| create("dealer", s).unwrap()).collect();
        while !game.is_terminal() {
            let viewer = game.current_player();
            let obs = game.observation(viewer).unwrap();
            let action = seats[viewer].choose(&obs, &game.legal_actions());
            if let (PhaseView::Bid(b), Action::Bid(amount)) = (obs.phase, action) {
                if !b.runoff {
                    opening_bids += 1;
                    assert!(amount >= 3.min(obs.players[viewer].cash.unwrap()), "bid {amount}");
                }
            }
            game.apply(action).unwrap();
        }
    }
    assert!(opening_bids > 0);
}

/// Plays `id` in every seat of `config` and checks each move against the three table rules. Returns the
/// number of trips it made to a harbor and to the Bank.
fn table_rule_trips(id: &str, config: GameConfig) -> (u32, u32) {
    use container_engine::observation::PhaseView;
    use container_engine::phase::Phase;
    use container_engine::state::ShipLocation;
    let (mut harbor_trips, mut bank_trips) = (0, 0);
    let mut game = Game::new(config).unwrap();
    let mut seats: Vec<_> = (0..config.num_players).map(|s| create(id, s as u64).unwrap()).collect();
    let mut last: Option<(usize, container_engine::action::District)> = None;
    let mut steps = 0;
    while !game.is_terminal() && steps < LIMIT {
        steps += 1;
        let viewer = game.current_player();
        let obs = game.observation(viewer).unwrap();
        let action = seats[viewer].choose(&obs, &game.legal_actions());
        if obs.phase == PhaseView::Open(Phase::Main) {
            if let Action::Reprice(d) = action {
                assert_ne!(last, Some((viewer, d)), "{id}: reprice twice in a row");
                last = Some((viewer, d));
            } else {
                last = None;
            }
        }
        let mut after = game.clone();
        after.apply(action).unwrap();
        match action {
            Action::Sail(ShipLocation::Bank) => {
                bank_trips += 1;
                assert!(matches!(after.phase, Phase::Load(_)), "{id}: sailed to the Bank with nothing to load");
            }
            Action::Sail(ShipLocation::Harbor(_)) => {
                harbor_trips += 1;
                assert!(matches!(after.phase, Phase::Buy(_)), "{id}: sailed to a player and could not buy");
            }
            Action::DoneBuying => {
                if let Phase::Buy(b) = game.phase {
                    assert!(!(b.optional && b.bought.is_empty() && b.district == container_engine::action::District::Harbor));
                }
            }
            _ => {}
        }
        game = after;
    }
    (harbor_trips, bank_trips)
}

#[test]
fn a_net_under_the_table_rules_keeps_them() {
    let (mut harbor_trips, mut bank_trips) = (0, 0);
    for seed in 0..4 {
        let (harbor, bank) = table_rule_trips("netr:weights/dealer.bin", config(3, GameLength::Standard, seed));
        harbor_trips += harbor;
        bank_trips += bank;
    }
    assert!(harbor_trips > 0 && bank_trips > 0, "the net should still sail: {harbor_trips} {bank_trips}");
}

#[test]
fn the_peddlers_keep_the_table_rules_at_every_table() {
    for id in ["peddler", "peddler2", "adaptive"] {
        let (mut harbor_trips, mut bank_trips) = (0, 0);
        for config in every_table() {
            let (harbor, bank) = table_rule_trips(id, config);
            harbor_trips += harbor;
            bank_trips += bank;
        }
        assert!(harbor_trips > 0 && bank_trips > 0, "{id} should still sail: {harbor_trips} {bank_trips}");
    }
}

#[test]
fn solitaire_calls_the_bank_for_a_container_lot_and_bids_cash() {
    let (mut starts, mut bids) = (0, 0);
    for seed in 0..6 {
        let mut game = Game::new(config(4, GameLength::Standard, seed)).unwrap();
        let mut seats: Vec<_> = (0..4).map(|s| create("solitaire", s).unwrap()).collect();
        let mut turns = 0;
        while !game.is_terminal() && turns < LIMIT * 4 {
            turns += 1;
            let viewer = game.current_player();
            let obs = game.observation(viewer).unwrap();
            let legal = game.legal_actions();
            let action = seats[viewer].choose(&obs, &legal);
            match action {
                Action::StartAuction { kind, .. } => {
                    starts += 1;
                    assert_eq!(kind, container_engine::state::AuctionKind::ContainerLot);
                }
                Action::BidCash(_) => bids += 1,
                Action::BidContainer { .. } => panic!("it never bids containers"),
                _ => {}
            }
            game.apply(action).unwrap();
        }
    }
    assert!(starts > 0 && bids >= starts, "{starts} calls, {bids} cash bids");
}

#[test]
fn classic_never_calls_the_bank() {
    let mut turns = 0;
    for seed in 0..4 {
        let mut game = Game::new(config(4, GameLength::Standard, seed)).unwrap();
        let mut seats: Vec<_> = (0..4).map(|s| create("classic", s).unwrap()).collect();
        while !game.is_terminal() && turns < LIMIT * 4 {
            turns += 1;
            let viewer = game.current_player();
            let obs = game.observation(viewer).unwrap();
            let legal = game.legal_actions();
            let action = seats[viewer].choose(&obs, &legal);
            let calls = matches!(action, Action::StartAuction { .. } | Action::Outbid(_));
            let only_call = legal.iter().all(|a| matches!(a, Action::StartAuction { .. } | Action::Outbid(_)));
            assert!(!calls || only_call, "{action:?}");
            game.apply(action).unwrap();
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
fn consortium_beats_merchant_at_every_length() {
    for length in [GameLength::Short, GameLength::Standard, GameLength::Extended] {
        let (mut won, mut games) = (0, 0);
        for players in MIN_PLAYERS..=MAX_PLAYERS {
            for seed in 0..6 {
                // Seat 0 is Consortium, everyone else plays Merchant.
                let table = config(players, length, seed);
                let Played { game, .. } = play_with(table, |seat| {
                    if seat == 0 { consortium::ID } else { "merchant" }
                });
                let results = game.results().expect("a finished game");
                won += usize::from(results.winners == [0]);
                games += 1;
            }
        }
        assert!(
            won * 10 >= games * 9,
            "Consortium won {won} of {games} games at the {length:?} length"
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

#[test]
fn the_cheap_seller_archetypes_play_to_the_end_and_price_low() {
    use container_engine::action::District;
    use crate::ARCHETYPES;
    for id in ARCHETYPES {
        for config in every_table() {
            let Played { game, trace } = play(config, &[id, "merchant"]);
            assert!(game.is_terminal(), "{id}: {config:?} stalled after {} actions", trace.len());
        }
    }
    // A discounter's containers sit in the cheapest lot of its factory district.
    let mut game = Game::new(config(3, GameLength::Short, 2)).expect("game");
    let mut seats: Vec<_> = (0..3).map(|s| create("discounter", s as u64).expect("bot")).collect();
    let (mut priced, mut cheapest) = (0u32, 0u32);
    while !game.is_terminal() && priced < 200 {
        let viewer = game.current_player();
        let obs = game.observation(viewer).expect("seat");
        let action = seats[viewer].choose(&obs, &game.legal_actions());
        if let Action::Place(lot) = action {
            if matches!(obs.phase, container_engine::observation::PhaseView::Open(
                container_engine::phase::Phase::Arrange(a)) if a.district == District::Factory) {
                priced += 1;
                cheapest += u32::from(lot == 0);
            }
        }
        game.apply(action).expect("legal");
    }
    assert!(priced > 0 && cheapest * 10 >= priced * 9, "{cheapest} of {priced} at $1");
}

#[test]
fn the_heuristics_are_listed_as_opponents_and_can_be_created() {
    use crate::HEURISTICS;
    for id in HEURISTICS {
        assert!(create(id, 0).is_some(), "{id}");
        assert!(
            OPPONENTS.iter().any(|o| o.id == id),
            "{id} is not listed for the UI"
        );
    }
}

#[test]
fn the_price_variants_change_only_the_lots_and_the_null_ones_are_the_base() {
    use crate::archetype::{Archetype, VARIANTS, variant};
    assert_eq!(format!("{:?}", variant(0, 7).unwrap()), format!("{:?}", Archetype::producer(7)));
    assert_eq!(format!("{:?}", variant(5, 7).unwrap()), format!("{:?}", Archetype::showcase(7)));
    assert!(variant(VARIANTS.len(), 7).is_none());
    let shifted = format!("{:?}", variant(1, 7).unwrap());
    let base = format!("{:?}", Archetype::producer(7));
    assert_ne!(shifted, base);
    assert_eq!(shifted.replace("factory_lot: 1", "factory_lot: 0"), base);
}

#[test]
fn the_heuristics_play_every_kind_of_game_to_its_end_alone_and_among_merchants() {
    use crate::HEURISTICS;
    for id in HEURISTICS {
        for config in every_table() {
            for ids in [&[id][..], &[id, "merchant"][..]] {
                let Played { game, trace } = play(config, ids);
                assert!(game.is_terminal(), "{ids:?}: {config:?} stalled after {} actions", trace.len());
            }
        }
    }
}

#[test]
fn each_heuristic_does_what_it_is_for_with_the_same_chances() {
    use crate::HEURISTICS;
    use container_engine::observation::PhaseView;
    use container_engine::phase::Phase;
    // Per heuristic: produce actions, purchases from factories, deliveries made (sold or bought out).
    let mut made = [[0u32; 3]; 3];
    for seed in 0..40 {
        for shift in 0..3 {
            let kind = |seat: usize| (seat + shift) % 3;
            let mut game = Game::new(config(3, GameLength::Standard, seed)).expect("game");
            let mut seats: Vec<_> = (0..3)
                .map(|s| create(HEURISTICS[kind(s)], seed * 10 + s as u64).expect("bot"))
                .collect();
            // The horizon of the EGTA runs: a table that stalls is cut off rather than played forever.
            for _ in 0..2500 {
                if game.is_terminal() {
                    break;
                }
                let viewer = game.current_player();
                let obs = game.observation(viewer).expect("seat");
                let action = seats[viewer].choose(&obs, &game.legal_actions());
                match (obs.phase, action) {
                    (PhaseView::Open(Phase::Main), Action::Produce) => made[kind(viewer)][0] += 1,
                    (PhaseView::Open(Phase::Main), Action::FactoryPurchase(_)) => made[kind(viewer)][1] += 1,
                    (PhaseView::Open(Phase::Decide(_)), Action::Accept(_) | Action::BuyOut) => {
                        made[kind(viewer)][2] += 1
                    }
                    _ => {}
                }
                game.apply(action).expect("legal");
            }
        }
    }
    let [producer, showcase, shipper] = made;
    assert!(producer[0] > showcase[0] && producer[0] > shipper[0], "producing: {made:?}");
    assert!(showcase[1] > producer[1] && showcase[1] > shipper[1], "restocking: {made:?}");
    assert!(shipper[2] > producer[2] && shipper[2] > showcase[2], "delivering: {made:?}");
}
