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
    assert_eq!(list.as_array().unwrap().len(), 2);
}
