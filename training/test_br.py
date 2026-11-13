"""Tests of the best-response oracle: payoffs, profiles, the paired design and the study on scripted bots."""

import json

import numpy as np
import pytest

from training import br, br_report
from training import evalcore as E
from training.env import DONE, MERCHANT, RANDOM, STALLED, STANDARD
from training.population import Member, Population
from training.selfplay import LEARNER, reward, table_welfare


def scripted_population(tmp_path) -> Population:
    pop = Population(tmp_path / "manifest.json")
    for name, kind in (("random", RANDOM), ("merchant", MERCHANT)):
        pop.add_member(Member(name=name, scripted=kind))
        pop.add_strategy(name, name)
    return pop


def test_terminal_payoff_is_canonical_or_shaped_and_stalls_pay_nothing():
    status = np.array([DONE, DONE, STALLED])
    players = np.array([3, 4, 3])
    winners = np.array([0b001, 0b0110, 0b111])
    scores = np.zeros((3, 5), dtype=np.int32)
    scores[0, :3] = [30, 20, 10]
    scores[1, :4] = [5, 40, 40, 5]
    canonical = br.terminal_payoff("canonical", status, players, scores, winners)
    assert canonical[0].tolist() == [1, 0, 0, 0, 0]
    assert canonical[1].tolist() == [0, 0.5, 0.5, 0, 0]
    assert not canonical[2].any()
    shaped = br.terminal_payoff("shaped", status, players, scores, winners)
    assert np.allclose(shaped[0, :3], reward(scores[0], 3)) and not shaped[0, 3:].any()
    assert not shaped[2].any()


def test_welfare_weight_moves_the_reward_to_the_tables_mean_score_and_zero_changes_nothing():
    poor, rich = np.array([30, 20, 10]), np.array([130, 120, 110])
    assert np.allclose(reward(poor, 3, 0.0), reward(poor, 3))
    # Every seat ranks the same in both tables, so only the welfare term tells them apart.
    assert np.allclose(reward(poor, 3), reward(rich, 3))
    assert (reward(rich, 3, 0.4) > reward(poor, 3, 0.4)).all()
    # The same table total, split differently: the welfare term is shared by every seat alike.
    evener = np.array([20, 20, 20])
    assert np.isclose(table_welfare(poor, 3), table_welfare(evener, 3))


def test_parse_profile_normalises_and_rejects_unknown_strategies(tmp_path):
    pop = scripted_population(tmp_path)
    names, w = br.parse_profile("random,merchant:3", pop)
    assert names == ["random", "merchant"] and np.allclose(w, [0.25, 0.75])
    with pytest.raises(SystemExit):
        br.parse_profile("random,nobody", pop)


def test_training_tables_seat_one_learner_and_draw_the_rest_from_the_profile():
    kinds_of = [RANDOM, MERCHANT, 1002]
    weights = np.array([0.2, 0.3, 0.5])
    nxt = br.table_sampler(np.random.default_rng(0), [3, 4, 5], [STANDARD], kinds_of, weights)
    seen = {k: 0 for k in kinds_of}
    seats = np.zeros(5)
    for _ in range(3000):
        n, kinds, seed, length = nxt()
        assert len(kinds) == n and kinds.count(LEARNER) == 1 and seed < 1 << 40 and length == STANDARD
        seats[kinds.index(LEARNER)] += 1
        for k in kinds:
            if k != LEARNER:
                seen[k] += 1
    share = np.array([seen[k] for k in kinds_of], dtype=float)
    assert np.allclose(share / share.sum(), weights, atol=0.02)
    assert seats[0] > 0 and seats[4] > 0  # the learner sits everywhere


def test_every_focal_meets_the_same_opponents():
    specs = br.study_specs([0, 1], np.array([0.3, 0.7]), 4, block=E.DEV, seeds=40, players=[3, 4], length=STANDARD)
    groups: dict = {}
    for sp in specs:
        f, p = divmod(sp.cell, 8)
        assert sp.seats[p] == f and sp.cluster == sp.seed
        others = sp.seats[:p] + sp.seats[p + 1:]
        assert groups.setdefault((sp.players, sp.seed, p), others) == others
    assert len(groups) == 40 * (3 + 4) and len(specs) == 4 * len(groups)
    draws = np.array([o for others in groups.values() for o in others])
    assert abs((draws == 1).mean() - 0.7) < 0.05


def test_study_with_scripted_bots_measures_a_known_gain(tmp_path):
    pop = scripted_population(tmp_path)
    res = br.deviation_study(pop, ["random"], np.array([1.0]), [], block=E.DEV, seeds=12, players=[3], envs=64, log=None)
    blk = res["by_players"]["3"]
    assert res["accounting"]["games"] == 12 * 3 * 2 and res["accounting"]["stalled"] == 0
    assert blk["gain"]["random"][0] == 0  # sigma against itself
    assert blk["gain"]["merchant"][0] > 0.2 and blk["gain"]["merchant"][1] > 0.05  # the Merchant beats random seats
    assert blk["nashconv"]["merchant"][0] == pytest.approx(3 * blk["gain"]["merchant"][0])
    assert res["mean_over_players"]["gain"]["merchant"][0] == pytest.approx(blk["gain"]["merchant"][0])


def test_gains_by_turn_position_regroup_the_gains_by_seat(tmp_path):
    pop = scripted_population(tmp_path)
    res = br.deviation_study(pop, ["random"], np.array([1.0]), [], block=E.DEV, seeds=9, players=[3, 4], envs=64, log=None)
    for blk in res["by_players"].values():
        seat, turn = (np.mean([g[0] for g in blk[k]["merchant"]]) for k in ("by_seat", "by_turn"))
        assert turn == pytest.approx(seat) and turn == pytest.approx(blk["gain"]["merchant"][0])  # a regrouping loses nothing


def test_gains_over_the_profile_cancel_on_its_support(tmp_path):
    pop = scripted_population(tmp_path)
    w = np.array([0.4, 0.6])
    res = br.deviation_study(pop, ["random", "merchant"], w, [], block=E.DEV, seeds=6, players=[3, 4], envs=64, log=None)
    for blk in res["by_players"].values():
        total = sum(wi * np.array(blk["per_seed_gain"][s]) for wi, s in zip(w, ["random", "merchant"]))
        assert np.allclose(total, 0, atol=1e-5)  # per-seed gains are stored rounded to 6 digits


def test_profiles_may_depend_on_the_table_size(tmp_path):
    pop = scripted_population(tmp_path)
    path = tmp_path / "profile.json"
    path.write_text(json.dumps({"support": ["random", "merchant"], "weights": {"3": [1, 0], "4": [1, 3]}}))
    names, w = br.parse_profile(f"@{path}", pop)
    assert names == ["random", "merchant"] and np.allclose(w[3], [1, 0]) and np.allclose(w[4], [0.25, 0.75])
    nxt = br.table_sampler(np.random.default_rng(0), [3, 4], [STANDARD], [RANDOM, MERCHANT], w)
    for _ in range(300):
        n, kinds, *_ = nxt()
        assert n == 4 or MERCHANT not in kinds
    res = br.deviation_study(pop, names, w, [], block=E.DEV, seeds=6, players=[3, 4], envs=64, log=None)
    assert res["profile"]["weights"]["3"] == [1.0, 0.0]
    for n, blk in res["by_players"].items():
        total = sum(wi * np.array(blk["per_seed_gain"][s]) for wi, s in zip(w[int(n)], names))
        assert np.allclose(total, 0, atol=1e-5)


def test_reliability_halfwidth_shrinks_with_the_square_root_of_the_seeds():
    rng = np.random.default_rng(0)
    res = {"by_players": {n: {"per_seed_gain": {"c": rng.normal(0.1, 0.2, 400).tolist()}} for n in "345"}}
    rows = {r["seeds"]: r for r in br.reliability(res, "c")}
    assert rows[100]["halfwidth"] == pytest.approx(rows[25]["halfwidth"] / 2)
    assert rows[25]["spread"] == pytest.approx(rows[25]["halfwidth"] / 1.96, rel=0.25)
    assert abs(rows[25]["mean"] - 0.1) < 0.03


def test_learning_rate_is_constant_or_falls_linearly_to_zero_over_the_budget():
    assert [br.lr_at(3e-4, "none", t, 100) for t in (0, 50, 120)] == [3e-4] * 3
    assert [br.lr_at(1e-4, "linear", t, 100) for t in (0, 50, 100, 120)] == pytest.approx([1e-4, 5e-5, 0.0, 0.0])


def test_picking_the_best_focal_is_measured_on_seeds_that_did_not_pick_it():
    def study(truth: list[float], rng) -> dict:
        names = [f"f{i}" for i in range(len(truth))]
        by_players = {str(n): dict(per_seed_gain={f: rng.normal(g, 1.0, 100).tolist() for f, g in zip(names, truth)})
                      for n in (3, 4, 5)}
        return dict(focals=names, candidates=[], by_players=by_players)

    rng = np.random.default_rng(0)
    flat = [br_report.selection_check(study([0.0] * 12, rng), [f"f{i}" for i in range(12)], reps=50) for _ in range(60)]
    assert np.mean([r["optimistic"][0] for r in flat]) > 0.06  # the best of twelve equal focals looks better than it is
    assert abs(np.mean([r["honest"][0] for r in flat])) < 0.03  # and the check takes that away

    one = br_report.selection_check(study([0.0] * 5 + [0.3], rng), [f"f{i}" for i in range(6)], reps=200)
    assert one["picked"] == ["f5", "f5"] and one["best"] == "f5"
    assert one["honest"][0] == pytest.approx(0.3, abs=0.15) and one["honest"][1] < one["honest"][0] < one["honest"][2]
    assert set(br_report.pools_of(dict(focals=["a", "b"], candidates=["b"]))) == {
        "every focal", "trained candidates", "strategies of the population"}


def test_validation_table_matches_runs_to_their_study_and_reports_recovery_of_the_best_pure_gain():
    run = {"config": dict(name="r", init="populations/x/run1.pt", support=["run1"], weights=[1.0], utility="shaped", seed=3),
           "minutes": 12.0, "games": 96370, "evals": [dict(minutes=4.1, gain=[0.02, -0.05, 0.09])]}
    gain = {"run2": [0.2, 0.1, 0.3], "r@12m/sample": [0.1, 0.0, 0.2], "r@12m/guarded": [0.15, 0.05, 0.25]}
    study = dict(candidates=["r@12m/sample", "r@12m/guarded"], mean_over_players=dict(gain=gain))
    table = br_report.render_runs([run], [study], "t")
    row = next(x for x in table.splitlines() if x.startswith("| r |"))
    assert "run2 +0.200" in row and "75%" in row and "guarded +0.150" in row  # recovery uses the better mode
    assert "no study" in br_report.render_runs([run], [], "t")


def test_validation_table_lists_part_way_checkpoints_by_minutes_of_training():
    run = {"config": dict(name="r", init="x/run1.pt", support=["run1"], weights=[1.0], utility="shaped", seed=3),
           "minutes": 12.0, "games": 1000, "evals": []}
    labels = ["r@4m/sample", "r@4m/guarded", "r@8m/sample", "r/sample", "r/guarded", "other/sample"]
    gain = {lab: [0.1 * i, 0.0, 0.2] for i, lab in enumerate(labels)} | {"run2": [0.5, 0.4, 0.6]}
    per_seed = {lab: [0.1 * i] * 4 for i, lab in enumerate(labels)}
    study = dict(candidates=labels, mean_over_players=dict(gain=gain),
                 by_players={n: dict(per_seed_gain=per_seed) for n in "34"})
    table = br_report.render_runs([run], [study], "t")
    section = table.split("## Gain against training time")[1].splitlines()
    rows = [x.split(" | ")[1] for x in section if x.startswith("| r |")]
    assert rows == ["4", "8", "12"]
    assert "guarded +0.400" in next(x for x in table.splitlines() if x.startswith("| r | run1"))  # the final weights
    first = next(x for x in section if x.startswith("| r | 4 |"))
    assert "+0.300 [+0.300, +0.300]" in first  # final minus this: 0.3 - 0.0 in sample mode, the same on every seed
    assert "+0.300" in br_report.render_pairs(study, [("r/sample", "r@4m/sample")])


def test_trade_and_geomean_rewards_pay_what_they_name():
    from training.selfplay import reward

    scores = np.array([60, 40, 20, 0, 0])
    plain = reward(scores, 3, trade_weight=0.3, traded=np.array([0, 0, 0, 0, 0]))
    busy = reward(scores, 3, trade_weight=0.3, traded=np.array([80, 0, 0, 0, 0]))
    assert busy[0] > plain[0] and busy[1] == plain[1]
    assert reward(scores, 3, geomean_weight=0.7)[0] > reward(scores, 3, geomean_weight=0.7)[2]
    rich = reward(np.array([60, 60, 60]), 3, geomean_weight=0.7)
    poor = reward(np.array([10, 10, 10]), 3, geomean_weight=0.7)
    assert rich[0] > poor[0]


def test_min_trade_penalty_charges_the_shortfall_and_nothing_once_met():
    from training.selfplay import reward

    scores = np.array([60, 40, 20])
    short = reward(scores, 3, min_trade=40.0, traded=np.array([0, 20, 40]), penalty=0.5)
    assert np.allclose(short, [1.0 - 0.5, 0.5 - 0.25, 0.0])
    met = reward(scores, 3, min_trade=40.0, traded=np.array([40, 50, 60]), penalty=0.5)
    assert np.allclose(met, reward(scores, 3, win_only=True))


def test_the_island_reward_pays_island_points_next_to_the_win():
    scores = np.array([30, 20, 10, 0, 0])
    island = np.array([0, 80, 40, 0, 0])
    r = reward(scores, 3, island_weight=0.5, island=island)
    # seat 0 wins both opponents (1.0) but took nothing from the island; seat 1 took all of it
    assert np.allclose(r[:3], [0.5 * 1.0, 0.5 * 0.5 + 0.5 * 1.0, 0.0 + 0.5 * 0.5])
