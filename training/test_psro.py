"""Tests of the PSRO bookkeeping: the cell store, the meta-solution and the profiles the trainer reads."""

import argparse
import itertools
import json
import sys
from pathlib import Path

import numpy as np
import pytest

from training import br, fields, population, psro
from training import evalcore as E
from training.env import MERCHANT, RANDOM
from training.population import Member, Population, sha256_file


def rps(i: int, j: int) -> float:
    return [0.0, -1.0, 1.0][(i - j) % 3]


def tables(fn, names: list[str], n: int, seeds: int = 4, offset: int = 0, noise: float = 0.0, rng=None) -> dict:
    """A cross-play `payoffs.json` for a game given by fn(i, others), the way `crossplay.build_payoffs` writes it."""
    rng = rng or np.random.default_rng(0)
    cells = []
    for ms in itertools.combinations_with_replacement(range(len(names)), n):
        distinct = sorted(set(ms))
        base = [fn(d, tuple(sorted(list(ms[:ms.index(d)]) + list(ms[ms.index(d) + 1:])))) for d in distinct]
        rows = np.array(base) + noise * rng.normal(size=(seeds, len(distinct)))
        cells.append(dict(players=n, multiset=list(ms), distinct=distinct, payoff=rows.mean(axis=0).tolist(),
                          games=seeds * n, stalled=0, seeds=seeds, per_seed=rows.tolist()))
    per = {nm: dict(seat_games=seeds * n * len(cells), fair_share=1.0, win_rate=0.3, tie_rate=0.0, score_margin=0.0,
                    actions=900.0) for nm in names}
    field = [[fn(i, (j,) * (n - 1)) for j in range(len(names))] for i in range(len(names))]
    games = sum(c["games"] for c in cells)
    return dict(strategies=names, players=[n], cells=cells, secondary=dict(strategies=per),
                accounting=dict(games=games, completed=games, stalled=0, stall_rate=0.0, horizon=2500),
                by_players={str(n): dict(field_matrix=dict(mean=field))},
                meta=dict(length=1, block=E.DEV, seed_offset=offset, seeds=seeds, players=[n]))


def rps_sum(i, others):
    return sum(rps(i, o) for o in others)


def test_store_rebuilds_the_game_it_was_given():
    names = ["a", "b", "c"]
    store = psro.CellStore()
    store.add(tables(rps_sum, names, 3))
    game = store.game(names, 3)
    for i, others in itertools.product(range(3), itertools.combinations_with_replacement(range(3), 2)):
        assert game.payoff(i, others) == pytest.approx(rps_sum(i, others))
    swapped = store.game(["c", "a", "b"], 3)  # strategies are matched by name, not by position
    assert swapped.payoff(0, (1, 2)) == pytest.approx(rps_sum(2, (0, 1)))


def test_store_merges_new_seeds_and_refuses_repeated_ones():
    names = ["a", "b"]
    store = psro.CellStore()
    store.add(tables(lambda i, o: float(i), names, 3, seeds=3))
    store.add(tables(lambda i, o: float(i), names, 3, seeds=5, offset=3))
    assert all(len(c["per_seed"]) == 8 for c in store.cells.values())
    with pytest.raises(ValueError, match="already holds"):
        store.add(tables(lambda i, o: float(i), names, 3, seeds=2, offset=6))
    assert store.missing(names + ["c"], [3]) and not store.missing(names, [3])
    with pytest.raises(ValueError, match="no simulated table"):
        store.game(names + ["c"], 3)


def test_solving_rock_paper_scissors_gives_the_uniform_mixture_verified_independently():
    names = ["a", "b", "c"]
    store = psro.CellStore()
    store.add(tables(rps_sum, names, 3))
    meta = psro.solve_meta(store, names, [3], quick=True, boot_reps=20)["3"]
    assert np.allclose(meta["sigma"], 1 / 3, atol=1e-4) and meta["gain"] < 1e-6
    assert meta["is_nash"] and meta["concept"].startswith("Nash")
    assert meta["verified_gain"] == pytest.approx(meta["gain"], abs=1e-6)  # the verifier shares no code with the solvers
    assert meta["verify_method"] == "enumerate"
    assert meta["bootstrap"]["mean"] == pytest.approx(0, abs=1e-9)  # no noise in these tables: nothing to resample


def test_a_dominant_strategy_is_the_solution():
    names = ["weak", "strong"]
    store = psro.CellStore()
    store.add(tables(lambda i, o: 0.2 + 0.1 * i, names, 4))
    meta = psro.solve_meta(store, names, [4], quick=True, boot_reps=10)["4"]
    assert np.allclose(meta["sigma"], [0, 1], atol=1e-6) and meta["gain"] == 0 and meta["is_nash"]


def test_beyond_the_enumeration_limit_the_check_samples_and_compares_within_its_standard_error(monkeypatch):
    names = ["a", "b", "c"]
    store = psro.CellStore()
    store.add(tables(rps_sum, names, 3))
    monkeypatch.setattr(psro, "EXACT_CHECK_LIMIT", 1)  # as for a population too large to enumerate
    meta = psro.solve_meta(store, names, [3], quick=True, boot_reps=0)["3"]
    assert meta["verify_method"] == "sampled" and meta["gain"] < 1e-6 and np.allclose(meta["sigma"], 1 / 3, atol=1e-4)


def test_admit_replaces_a_checkpoint_left_read_only_by_a_run_that_died_before_saving(tmp_path):
    pop = Population(tmp_path / "pop/manifest.json")
    parent = tmp_path / "pop/a.pt"
    parent.parent.mkdir()
    parent.write_bytes(b"a")
    pop.add_member(Member("a", checkpoint=str(parent), sha256=sha256_file(parent)))
    pop.add_strategy("a", "a")
    pop.save()
    stale = tmp_path / "pop/psro01.pt"
    stale.write_bytes(b"left by the dead run")
    stale.chmod(0o444)
    final = tmp_path / "final.pt"
    final.write_bytes(b"the response")
    decision = dict(mode="guarded", gain=[0.1, 0.05, 0.15], gain_by_mode=dict(sample=[0.05, 0.0, 0.1], guarded=[0.1, 0.05, 0.15]))
    name = psro.admit(pop, 1, "a", final, dict(utility="canonical", gamma=0.1, seed=101), decision)
    assert name == "psro01" and stale.read_bytes() == b"the response" and pop.members[name].sha256 == sha256_file(final)
    assert Population.load(pop.path).verify() == []


def test_a_response_trained_outside_the_loop_needs_its_better_mode_confirmed_on_holdout_seeds():
    study = dict(candidates=["r/sample", "r/guarded"],
                 mean_over_players=dict(gain={"r/sample": [0.01, -0.02, 0.04], "r/guarded": [0.03, 0.005, 0.06]}))

    def holdout(gain):  # the mode the dev study did not pick must not matter
        return dict(block=E.HOLDOUT, seeds=300, seed_offset=1000,
                    mean_over_players=dict(gain={"r/guarded": gain, "r/sample": [0.9, 0.9, 0.9]}))

    ok = psro.response_decision(study, "r", 0.0, holdout([0.022, 0.008, 0.035]))
    assert ok["accepted"] and ok["mode"] == "guarded" and ok["confirmation"]["gain"][0] == 0.022
    assert not psro.response_decision(study, "r", 0.0, holdout([0.02, -0.003, 0.04]))["accepted"]
    assert not psro.response_decision(study, "r", 0.01)["accepted"]  # the lower bound on dev seeds is 0.005
    with pytest.raises(SystemExit):
        psro.response_decision(study, "other", 0.0)


def test_command_line_settings_are_saved_so_that_a_rerun_without_them_keeps_them(tmp_path, monkeypatch):
    monkeypatch.setattr(psro, "CONFIG", tmp_path / "config.json")
    (tmp_path / "config.json").write_text("{}")
    cfg = psro.with_overrides(argparse.Namespace(lr=1e-4, lr_decay="linear", minutes=None))
    assert cfg["lr"] == 1e-4 and cfg["lr_decay"] == "linear" and cfg["minutes"] == psro.DEFAULTS["minutes"]
    again = psro.with_overrides(argparse.Namespace())
    assert again["lr"] == 1e-4 and again["lr_decay"] == "linear"


def test_noise_in_a_tied_game_shows_up_in_the_bootstrap_although_the_empirical_gain_is_zero():
    names = ["x", "y"]
    store = psro.CellStore()
    store.add(tables(lambda i, o: 0.25, names, 4, seeds=10, noise=0.2, rng=np.random.default_rng(3)))
    meta = psro.solve_meta(store, names, [4], quick=True, boot_reps=100)["4"]
    assert meta["verified_gain"] < 1e-6  # some mixture is always an equilibrium of the simulated game ...
    assert meta["bootstrap"]["mean"] > 0.005 and meta["bootstrap"]["p95"] > 0.02  # ... but resampling the seeds breaks it


def test_exploration_smooths_towards_uniform_and_profiles_round_trip(tmp_path):
    assert np.allclose(psro.explore([1, 0, 0, 0], 0.2), [0.85, 0.05, 0.05, 0.05])
    pop = Population(tmp_path / "manifest.json")
    for name, kind in (("random", RANDOM), ("merchant", MERCHANT)):
        pop.add_member(Member(name=name, scripted=kind))
        pop.add_strategy(name, name)
    path = tmp_path / "profile.json"
    path.write_text(json.dumps(dict(support=["random", "merchant"],
                                    weights={"3": psro.explore([1, 0], 0.1).tolist(), "5": [0.5, 0.5]})))
    names, weights = br.parse_profile(f"@{path}", pop)
    assert names == ["random", "merchant"] and np.allclose(weights[3], [0.95, 0.05]) and np.allclose(weights[5], 0.5)
    assert br.weights_json(weights)["3"] == pytest.approx([0.95, 0.05])


PAIRS = np.array([[0.30, 0.10, 0.50], [0.20, 0.25, 0.05], [0.10, 0.40, 0.15]])  # share of i against one opponent j


def test_a_mixture_is_read_against_a_field_as_the_weighted_mean_of_its_members(tmp_path):
    names = ["a", "b", "c"]
    store = psro.CellStore()
    store.add(tables(lambda i, others: float(np.mean([PAIRS[i, o] for o in others])), names, 4, seeds=6,
                     noise=0.05, rng=np.random.default_rng(1)))
    by_n = tmp_path / "sigma.json"
    by_n.write_text(json.dumps(dict(support=["c", "a"], weights={"4": [1, 3], "5": [1, 1]})))
    mixes = fields.labelled(["flat=a,b,c", "tilted=a:3,b", f"sigma=@{by_n}"], names)
    field_specs = fields.labelled(["uniform", "b", "heavy=a:1,c:3"], names)
    assert list(mixes) == ["flat", "tilted", "sigma"] and list(field_specs) == ["uniform", "b", "heavy"]
    assert np.allclose(mixes["sigma"](4), [0.75, 0, 0.25]) and np.allclose(mixes["sigma"](5), [0.5, 0, 0.5])

    exact = fields.compare(store, names, 4, mixes, field_specs, reps=0, seed=0)
    game = store.game(names, 4)
    for m, mix in mixes.items():
        for f, field in field_specs.items():
            assert exact["values"][m][f][0] == pytest.approx(mix(4) @ game.expected_payoffs(field(4)))
        assert exact["values"][m]["b"][0] == pytest.approx(mix(4) @ store.game(names, 4).expected_payoffs([0, 1, 0]))
    assert exact["gaps"]["flat"]["uniform"] == [0.0, 0.0, 0.0]  # the first mixture is the reference

    noisy = fields.compare(store, names, 4, mixes, field_specs, reps=50, seed=0)
    for m in mixes:
        for f in field_specs:
            point, lo, hi = noisy["values"][m][f]
            assert lo < point < hi and hi - lo < 0.2
    gap, value = noisy["gaps"]["tilted"]["uniform"], noisy["values"]["tilted"]["uniform"]
    assert gap[1] <= gap[0] <= gap[2] and value[0] == exact["values"]["tilted"]["uniform"][0]

    with pytest.raises(SystemExit, match="not one of the strategies"):
        fields.parse_spec("a,z", names)
    with pytest.raises(SystemExit, match="two specs are called"):
        fields.labelled(["a", "a"], names)


def test_the_fields_command_writes_its_tables(tmp_path, monkeypatch):
    names = ["a", "b", "c"]
    cells = tmp_path / "cells.json"
    cells.write_text(json.dumps(tables(lambda i, others: float(np.mean([PAIRS[i, o] for o in others])), names, 3, seeds=4,
                                       noise=0.05, rng=np.random.default_rng(2))))
    monkeypatch.setattr(fields, "ROOT", tmp_path)
    monkeypatch.setattr(sys, "argv", ["fields", "--cells", "cells.json", "--strategies", "a,b,c", "--players", "3",
                                      "--mix", "flat=a,b,c", "--mix", "b", "--field", "uniform", "--field", "a",
                                      "--boot-reps", "20", "--out", str(tmp_path / "out")])
    fields.main()
    report = (tmp_path / "out.md").read_text()
    assert "## 3 players" in report and "Gap to `flat` (paired)" in report and "| b |" in report
    stored = json.loads((tmp_path / "out.json").read_text())["players"]["3"]
    assert set(stored["values"]) == {"flat", "b"} and set(stored["fields"]) == {"uniform", "a"}


SKILL = [0.0, 1.0, 2.0, 4.0]  # the fourth strategy of the synthetic game, the response, beats the others


def skill_game(i, others):
    return 0.3 + 0.05 * (SKILL[i] - sum(SKILL[o] for o in others) / len(others))


def psro_world(tmp_path, monkeypatch):
    """A population of three strategies, the cross-play of the synthetic game and a configuration, all in `tmp_path`."""
    root = tmp_path / "psro"
    monkeypatch.setattr(psro, "OUT", root / "reports")
    monkeypatch.setattr(psro, "CONFIG", root / "reports/config.json")
    monkeypatch.setattr(psro, "POP_DIR", root / "pop")
    monkeypatch.setattr(psro, "RUNS", str(root / "runs"))
    pop = Population(root / "pop/manifest.json")
    pop.add_member(Member("bot", scripted=MERCHANT))
    pop.add_strategy("bot", "bot")
    for name in ("a", "b"):
        ck = root / "pop" / f"{name}.pt"
        ck.parent.mkdir(parents=True, exist_ok=True)
        ck.write_bytes(name.encode())
        pop.add_member(Member(name, checkpoint=str(ck), sha256=sha256_file(ck)))
        pop.add_strategy(name, name)
    pop.save()
    cells = root / "cells.json"
    cells.write_text(json.dumps(tables(skill_game, ["bot", "a", "b"], 3)))
    cfg = psro.DEFAULTS | dict(population=str(pop.path), cells=[str(cells)], players=[3], boot_reps=5)
    psro.CONFIG.parent.mkdir(parents=True)
    psro.CONFIG.write_text(json.dumps(cfg))
    return root, cfg, pop


def psro_fakes(gain):
    """The trainer and the studies of `training.br` and the cross-play of a new member, replaced by their output files.
    `gain` is the mean gain of a response in each mode."""

    def fake_sh(cmd, log):
        flags = dict(zip(cmd[4::2], cmd[5::2]))  # a flag without a value (`--all-focals`) comes last
        if cmd[3] == "train":
            run = Path(flags["--runs"]) / flags["--name"]
            run.mkdir(parents=True, exist_ok=True)
            (run / "final.pt").write_bytes(b"response seed " + flags["--seed"].encode())
            rows = [dict(type="train", iter=k, utility=0.25 + 0.01 * k) for k in range(5)]
            (run / "metrics.jsonl").write_text("\n".join(json.dumps(r) for r in rows) + "\n")
            (run / "result.json").write_text(json.dumps(dict(
                minutes=0.1, games=100, decisions=5000, evals=[dict(minutes=0.05, gain=[0.01, -0.02, 0.04])])))
            return
        paths = dict(c.partition("=")[::2] for c in flags["--candidates"].split(",")) if "--candidates" in flags else {}
        labels = [f"{c}/{m}" for c in paths for m in flags["--modes"].split(",")]
        cis = {lab: [gain[lab.split("/")[1]] + w for w in (0.0, -0.03, 0.03)] for lab in labels}
        profile = json.loads(Path(flags["--profile"][1:]).read_text())
        seeds = int(flags["--seeds"])
        every = Population.load(flags["--population"]).strategies if "--all-focals" in cmd else ["bot", "a", "b"]
        focals = [*every, *labels]
        pure = {"a": [0.0, -0.01, 0.01]}
        study = dict(profile=profile, focals=focals, candidates=labels, block=br.BLOCKS[flags["--block"]],
                     policies=[dict(name=f"{c}/{m}", sha256=sha256_file(p)) for c, p in paths.items()
                               for m in flags["--modes"].split(",")],
                     seeds=seeds, seed_offset=int(flags["--seed-offset"]), players=[3], length=1,
                     mean_over_players=dict(gain=cis | pure,
                                            nashconv={lab: [3 * x for x in ci] for lab, ci in (cis | pure).items()}),
                     by_players={"3": dict(gain=cis | pure | {"b": [0.0, 0.0, 0.0]},
                                           nashconv={lab: [3 * x for x in ci] for lab, ci in cis.items()},
                                           sigma=[0.33, 0.3, 0.36],
                                           by_seat={lab: [[x, x - 0.05, x + 0.05] for x in (0.05, 0.1, 0.15)]
                                                    for lab in labels},
                                           per_seed_gain={f: [cis[f][0] if f in cis else 0.0] * seeds for f in focals})},
                     accounting=dict(games=240, stalled=0, completed=240, stall_rate=0.0, horizon=2500))
        Path(flags["--out"]).with_suffix(".json").write_text(json.dumps(study))

    def fake_crossplay_new(pop, name, cfg, d):  # only the compositions that seat the new member are simulated
        out = d / "crossplay"
        if (out / "payoffs.json").exists():
            return
        names = list(pop.strategies)
        payoffs = tables(skill_game, names, 3)
        payoffs["cells"] = [c for c in payoffs["cells"] if names.index(name) in c["multiset"]]
        out.mkdir(parents=True, exist_ok=True)
        (out / "payoffs.json").write_text(json.dumps(payoffs))

    return fake_sh, fake_crossplay_new


def test_an_iteration_admits_a_significant_response_rejects_a_marginal_one_and_resumes_after_a_crash(tmp_path, monkeypatch):
    root, cfg, pop = psro_world(tmp_path, monkeypatch)
    gain = {"sample": 0.1, "guarded": 0.15}  # mean gain of the response in each mode
    fake_sh, fake_crossplay_new = psro_fakes(gain)
    monkeypatch.setattr(psro, "sh", fake_sh)
    monkeypatch.setattr(psro, "crossplay_new", fake_crossplay_new)

    (root / "runs/iter_01").mkdir(parents=True)  # a trainer that was killed in an earlier attempt of this iteration
    (root / "runs/iter_01/metrics.jsonl").write_text(json.dumps(dict(type="train", iter=0, utility=9.9)) + "\n")

    first = psro.step(cfg)
    assert first["iteration"] == 1 and first["decision"]["accepted"] and first["new_member"] == "psro01"
    assert first["decision"]["mode"] == "guarded" and first["decision"]["gain"][0] == pytest.approx(0.15)  # the better mode
    assert first["decision"]["max_seat_gain"] == pytest.approx(0.15)
    assert first["training"]["utility_first"] < 1 and len(list((root / "runs/iter_01").glob("metrics.aborted-*.jsonl"))) == 1
    pop = Population.load(pop.path)
    assert list(pop.strategies) == ["bot", "a", "b", "psro01"] and pop.verify() == []
    member = pop.members["psro01"]
    assert member.parent == "b" and member.seed == 101 and member.sha256 == sha256_file(root / "pop/psro01.pt")
    assert "# PSRO iteration 1" in (psro.OUT / "iter_01/report.md").read_text()
    assert "## 5. Against each member alone" in (psro.OUT / "iter_01/report.md").read_text()
    entry = pop.results["psro01"]  # what the population records about the response
    assert entry["coverage"]["trained"]["players"] == [3] and entry["coverage"]["evaluated"] == {"standard": [3]}
    assert len(entry["crossplay"]) == 1 and len(entry["best_response"]["as_candidate"]) == 1
    (report, as_candidate), = entry["best_response"]["as_candidate"].items()
    assert pop.strategies["psro01"].mode == "guarded"
    assert as_candidate["responses"]["psro01/sample"]["gain"][0] == pytest.approx(0.1)
    assert as_candidate["responses"]["psro01/guarded"]["max_seat_gain"] == pytest.approx(0.15)
    assert report.endswith("iter_01/study.json") and as_candidate["profile"]["support"] == ["bot", "a", "b"]
    weights = [pop.results[nm]["best_response"]["as_target"][report]["weight"]["3"] for nm in ("bot", "a", "b")]
    assert sum(weights) == pytest.approx(1.0)  # the targets carry their weights in sigma_3

    (psro.OUT / "iter_01/record.json").unlink()  # the process died after admitting the response, before recording it
    again = psro.step(cfg | dict(lr=1.0, min_gain=0.5))  # flags of the rerun do not change an iteration in progress
    assert again["iteration"] == 1 and again["population"] == ["bot", "a", "b"] and again["new_member"] == "psro01"
    assert again["train_config"]["lr"] == cfg["lr"] and again["decision"]["accepted"]
    assert len(Population.load(pop.path).strategies) == 4

    gain.update(sample=0.01, guarded=0.0)  # the next response is not distinguishable from no improvement
    second = psro.step(cfg | dict(study_seeds=10))
    assert second["iteration"] == 2 and second["population"] == ["bot", "a", "b", "psro01"]
    assert not second["decision"]["accepted"] and second["new_member"] is None
    studies = [json.loads((psro.OUT / f"iter_0{k}/study.json").read_text()) for k in (1, 2)]
    assert studies[1]["seed_offset"] >= studies[0]["seed_offset"] + studies[0]["seeds"] and studies[1]["seeds"] == 10
    assert len(Population.load(pop.path).strategies) == 4
    summary = (psro.OUT / "summary.md").read_text()
    assert summary.count("added psro01") == 1 and summary.count("rejected") == 1


def test_annotate_reads_its_options_in_any_order(tmp_path):
    pop = Population(tmp_path / "manifest.json")
    pop.add_member(Member("bot", scripted=MERCHANT))
    pop.add_strategy("bot", "bot")
    pop.save()
    payoffs = tmp_path / "payoffs.json"
    payoffs.write_text(json.dumps(tables(lambda i, o: 0.3, ["bot"], 3)))
    assert population.main(["annotate", str(pop.path), "--baselines", "--crossplay", str(payoffs)]) == 0
    entry = Population.load(pop.path).results["bot"]
    assert entry["coverage"]["trained"]["players"] == [3, 4, 5] and len(entry["crossplay"]) == 1


def test_run_ends_after_the_current_iteration_when_asked_to(tmp_path, monkeypatch):
    monkeypatch.setattr(psro, "OUT", tmp_path)
    monkeypatch.setattr(psro, "CONFIG", tmp_path / "config.json")
    (tmp_path / "config.json").write_text("{}")
    calls = []
    monkeypatch.setattr(psro, "step", lambda cfg: calls.append(cfg) or (tmp_path / "STOP").touch())
    psro.cmd_run(argparse.Namespace(iterations=5, no_start_after=None))
    assert len(calls) == 1


def test_final_audit_attacks_the_last_sigma_and_the_solution_of_the_final_game_on_holdout_seeds(tmp_path, monkeypatch):
    root, cfg, pop = psro_world(tmp_path, monkeypatch)
    gain = {"sample": 0.1, "guarded": 0.15}
    fake_sh, fake_crossplay_new = psro_fakes(gain)
    commands = []

    def record(cmd, log):
        commands.append(cmd)
        fake_sh(cmd, log)

    monkeypatch.setattr(psro, "sh", record)
    monkeypatch.setattr(psro, "crossplay_new", fake_crossplay_new)
    psro.step(cfg)  # a response is admitted as psro01
    gain.update(sample=0.01, guarded=0.0)
    psro.step(cfg)  # the next one is rejected
    commands.clear()

    out = tmp_path / "final"

    def audit():
        monkeypatch.setattr(sys, "argv", ["psro", "final", "--seeds", "8", "--seed-offset", "200", "--out", str(out)])
        psro.main()

    audit()
    flags = [dict(zip(c[4::2], c[5::2])) for c in commands]
    assert [c[3] for c in commands] == ["eval", "eval"] and all(c[-1] == "--all-focals" for c in commands)
    assert all(f["--block"] == "holdout" and f["--seeds"] == "8" for f in flags)
    assert flags[0]["--profile"] == f"@{psro.OUT / 'iter_02/profile_eval.json'}"  # what the last response was trained against
    assert flags[0]["--candidates"] == f"iter02={root / 'runs/iter_02/final.pt'}"  # the admitted one is a strategy already
    assert flags[0]["--modes"] == "sample,guarded"
    assert flags[0]["--seed-offset"] == "200" and flags[1]["--seed-offset"] == "208"  # two looks that share no seed
    assert flags[1]["--profile"] == f"@{out / 'profile_final.json'}" and "--candidates" not in flags[1]
    solution = json.loads((out / "profile_final.json").read_text())
    assert solution["support"] == ["bot", "a", "b", "psro01"] and sum(solution["weights"]["3"]) == pytest.approx(1.0)
    report = (out / "report.md").read_text()
    assert "responses added: psro01; iterations whose response was rejected: 2" in report
    assert "| 1 | added | psro01/guarded |" in report and "| 2 | rejected | psro02/sample |" in report
    assert "iter02/guarded (trained, not in the population)" in report
    assert (out / "holdout_sigma_last_selection.md").exists() and (out / "holdout_sigma_final_selection.md").exists()

    commands.clear()
    audit()  # the studies are kept: a rerun does not look at the holdout block again
    assert commands == []

    target = out / "holdout_sigma_last.json"
    kept = json.loads(target.read_text())
    other_looks = dict(
        offset=dict(seed_offset=7), seeds=dict(seeds=3), block=dict(block=E.DEV), candidates=dict(candidates=[]),
        profile=dict(profile=dict(kept["profile"], weights={"3": [1.0, 0.0, 0.0, 0.0]})),
        checkpoint=dict(policies=[p | dict(sha256="0" * 64) for p in kept["policies"]]))
    for what, change in other_looks.items():  # a study of another look is not reported as this one
        target.write_text(json.dumps(kept | change))
        with pytest.raises(SystemExit, match="not the look this run asks for"):
            audit()
    target.write_text(json.dumps(kept))
    audit()
    assert commands == []

    (root / "runs/iter_02/final.pt").unlink()  # a rejected response that cannot be found must not be dropped silently
    with pytest.raises(SystemExit, match="is gone"):
        audit()

    monkeypatch.setattr(sys, "argv", ["psro", "solve", "--cells", str(root / "cells.json"), "--strategies", "bot,a,b",
                                      "--players", "3", "--boot-reps", "5", "--out", str(tmp_path / "solved")])
    psro.main()
    assert "bot, a, b" in (tmp_path / "solved.md").read_text()
    profile = json.loads((tmp_path / "solved_profile.json").read_text())
    assert profile["support"] == ["bot", "a", "b"] and sum(profile["weights"]["3"]) == pytest.approx(1.0)
