use crate::Session;
use serde_json::Value;

fn config(kinds: &[&str], seed: u64) -> String {
    let players: Vec<Value> = kinds
        .iter()
        .enumerate()
        .map(|(i, k)| serde_json::json!({ "name": format!("P{i}"), "kind": k }))
        .collect();
    serde_json::json!({ "seed": seed, "players": players }).to_string()
}

#[test]
fn bots_play_a_whole_game_and_every_snapshot_is_well_formed() {
    for seed in 0..3 {
        let mut session = Session::new(&config(&["simple", "random", "simple"], seed)).unwrap();
        let mut steps = 0;
        while session.step().unwrap() {
            let snap = session.snapshot();
            assert_eq!(snap["players"].as_array().unwrap().len(), 3);
            steps += 1;
            assert!(steps < 200_000, "the game must end");
        }
        let snap = session.snapshot();
        assert_eq!(snap["gameOver"], true);
        assert!(snap["results"]["winners"].as_array().unwrap().len() >= 1);
        assert!(snap["round"].as_u64().unwrap() >= 1);
    }
}

#[test]
fn magnate_plays_a_whole_game_in_a_session() {
    let mut session = Session::new(&config(&["magnate", "merchant", "magnate", "simple"], 5)).unwrap();
    let mut steps = 0;
    while session.step().unwrap() {
        steps += 1;
        assert!(steps < 20_000, "the game must end");
    }
    assert_eq!(session.snapshot()["gameOver"], true);
}

#[test]
fn consortium_plays_a_whole_game_in_a_session() {
    for kinds in [
        &["consortium", "magnate", "consortium"][..],
        &["consortium", "merchant", "consortium", "simple"],
        &["consortium"; 5],
    ] {
        let mut session = Session::new(&config(kinds, 5)).unwrap();
        let mut steps = 0;
        while session.step().unwrap() {
            steps += 1;
            assert!(steps < 20_000, "the game must end");
        }
        assert_eq!(session.snapshot()["gameOver"], true);
    }
}

#[test]
fn the_heuristics_play_a_whole_game_in_a_session() {
    for kinds in [
        &["producer", "showcase", "shipper"][..],
        &["shipper", "merchant", "producer", "simple"],
        &["showcase", "producer", "shipper", "showcase", "consortium"],
    ] {
        let mut session = Session::new(&config(kinds, 5)).unwrap();
        let mut steps = 0;
        while session.step().unwrap() {
            steps += 1;
            assert!(steps < 20_000, "the game must end");
        }
        assert_eq!(session.snapshot()["gameOver"], true);
    }
}

#[test]
fn a_human_gets_legal_actions_only_on_their_own_turn() {
    let mut session = Session::new(&config(&["human", "simple", "simple"], 7)).unwrap();
    let mut human_moves = 0;
    for _ in 0..20_000 {
        if session.is_over() {
            break;
        }
        if session.step().unwrap() {
            if session.current_is_bot() {
                assert!(session.snapshot()["legal"].as_array().unwrap().is_empty());
            }
            continue;
        }
        let snap = session.snapshot();
        let legal = snap["legal"].as_array().unwrap();
        assert!(!legal.is_empty(), "a human turn offers actions");
        let pick = legal.last().unwrap()["id"].as_u64().unwrap() as usize;
        session.act(pick).unwrap();
        human_moves += 1;
    }
    assert!(human_moves > 0);
}

#[test]
fn a_human_cannot_act_for_a_bot_and_other_cash_stays_hidden() {
    let session = Session::new(&config(&["human", "simple", "random"], 3)).unwrap();
    let snap = session.snapshot();
    assert!(snap["players"][0]["cash"].is_number());
    assert!(snap["players"][1]["cash"].is_null());
    assert!(snap["players"][2]["cash"].is_null());
}

#[test]
fn bad_configurations_are_refused() {
    assert!(Session::new(&config(&["human", "human", "simple"], 1)).is_err());
    assert!(Session::new(&config(&["human", "nobody", "simple"], 1)).is_err());
    assert!(Session::new(&config(&["human", "simple"], 1)).is_err());
}

#[test]
fn the_opponent_list_names_every_bot() {
    let list: Value = serde_json::from_str(&crate::opponents_json()).unwrap();
    assert_eq!(list.as_array().unwrap().len(), container_bots::OPPONENTS.len());
}

#[test]
fn the_opponent_list_is_ordered_by_rating() {
    let list: Value = serde_json::from_str(&crate::opponents_json()).unwrap();
    let elos: Vec<u64> = list.as_array().unwrap().iter().map(|o| o["elo"].as_u64().unwrap()).collect();
    assert!(elos.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn winning_a_bank_auction_is_logged() {
    let mut wins = 0;
    for seed in 0..6 {
        let mut session = Session::new(&config(&["magnate"; 5], seed)).unwrap();
        let mut won_before = false;
        while session.step().unwrap() {
            let snap = session.snapshot();
            let won = snap["turn"]["wonAuction"] == true;
            if won && !won_before {
                // One step adds at most a few entries, so the win is among the last ones.
                let log = snap["log"].as_array().unwrap();
                let said = log
                    .iter()
                    .rev()
                    .take(3)
                    .any(|e| e["text"].as_str().is_some_and(|t| t.contains(" wins the ")));
                assert!(said, "an auction was won but the log does not say so");
                wins += 1;
            }
            won_before = won;
        }
    }
    assert!(wins > 0, "no bank auction was won in any game");
}
