import json

import numpy as np
import pytest
import torch

from training import egta
from training import env as V
from training import evalcore as E

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")


def two_groups(seeds=40, seed=0):
    """Two groups of two seats per seed. Group `b` is group `a` shifted by a constant, on top of a large per-seed difference."""
    rng = np.random.default_rng(seed)
    per_seed = rng.normal(0, 10, seeds)
    a_rows = np.arange(2 * seeds)
    b_rows = a_rows + 2 * seeds
    seat_seed = np.tile(np.repeat(np.arange(seeds), 2), 2)
    value = np.concatenate([np.repeat(per_seed, 2) + 1.0, np.repeat(per_seed, 2)])
    t = egta.Seats(size=np.full(len(value), 3), arche=np.zeros(len(value), dtype=int), game=np.arange(len(value)), seed=seat_seed,
                   game_seed=seat_seed, seeds={3: seeds}, raw={"x": value, "one": np.ones(len(value))}, by={})
    return t, a_rows, b_rows


def test_a_rate_is_a_ratio_of_totals_and_a_difference_is_paired_on_the_seeds(monkeypatch):
    t, a_rows, b_rows = two_groups()
    monkeypatch.setitem(egta.METRICS, "x", ("x", None))
    one, lo, hi = egta.stat(t, a_rows, "x")
    assert hi - lo > 3, "a group on its own is as uncertain as the seeds differ"
    d, dlo, dhi = egta.contrast(egta.seed_sums(t, a_rows, "x"), egta.seed_sums(t, b_rows, "x"))
    assert (d, dlo, dhi) == pytest.approx((1.0, 1.0, 1.0)), "the same seeds in both groups cancel, only the shift is left"

    # a seat that sold nothing has no price, but the seats that did count by their weight, not by one each
    raw = {"num": np.array([1.0, 9.0, np.nan]), "den": np.array([1.0, 3.0, 5.0]), "one": np.ones(3)}
    seed = np.array([0, 1, 1])
    u = egta.Seats(size=np.full(3, 3), arche=np.zeros(3, dtype=int), game=np.arange(3), seed=seed, game_seed=seed, seeds={3: 2}, raw=raw,
                   by={})
    monkeypatch.setitem(egta.METRICS, "rate", ("num", "den"))
    assert egta.stat(u, np.arange(3), "rate")[0] == pytest.approx(10 / 4), "10 over 4, not the mean of 1 and 3, and the NaN row is out of both"


def test_the_producers_lead_is_measured_within_each_game():
    """A table of one Producer and two Showcases that differ in every game: the lead is per game, whoever sits where."""
    n, seeds = 3, 6
    cell = egta.cell_of(n, (0, 1, 1))
    games = []
    for s in range(seeds):
        for r in range(n):
            games.append((s, ((0, 1, 1) * 2)[r:r + n]))
    G = len(games)
    seat = np.full((G, V.MAX_PLAYERS), -1, dtype=np.int16)
    interaction = np.zeros((G, V.MAX_PLAYERS, V.INTERACTION), dtype=np.int32)
    breakdown = np.zeros((G, V.MAX_PLAYERS, 4), dtype=np.int32)
    for g, (s, order) in enumerate(games):
        seat[g, :n] = order
        for k, who in enumerate(order):
            # the Producer makes 10 + s, a Showcase 4 + 2 * (its seat), so no two seats of a game are the same
            interaction[g, k, V.PRODUCED] = 10 + s if who == 0 else 4 + 2 * k
            breakdown[g, k, 0] = 30 + k
    scores = np.zeros((G, V.MAX_PLAYERS), dtype=np.int32)
    scores[:, :n] = breakdown[:, :n, 0]
    cluster = np.array([1000 + s for s, _ in games], dtype=np.uint64)
    rec = E.Records(players=np.full(G, n, dtype=np.int8), length=np.zeros(G, dtype=np.int8), seed=cluster, seats=seat,
                    cell=np.full(G, cell, dtype=np.int32), cluster=cluster, status=np.full(G, E.DONE, dtype=np.int8), scores=scores,
                    winners=np.full(G, 1, dtype=np.int32), actions=np.zeros(G, dtype=np.int32))
    t = egta.seat_table(rec, interaction, breakdown)
    (row,) = egta.producer_leads(t, rec)
    lead = []
    for s, order in games:
        mine = 10 + s
        others = [4 + 2 * k for k, who in enumerate(order) if who == 1]
        lead.append(mine - np.mean(others))
    assert row["composition"] == "P Sc Sc"
    assert row["minus_showcase"][0] == pytest.approx(np.mean(lead))
    assert row["produced"]["producer"] == pytest.approx(np.mean([10 + s for s, _ in games]))
    assert "minus_shipper" not in row, "the table seats no Shipper"


@needs_cuda
def test_the_three_heuristics_play_the_design_and_the_report_is_written(tmp_path):
    out = tmp_path / "egta"
    rec = egta.run(out, seeds=3, sizes=[3], log=lambda *_: None)
    assert len(rec) == 10 * 3 * 3, "10 compositions of 3 archetypes at 3 seats, 3 seeds, 3 rotations"
    assert not list(out.glob("chunk_*")), "the chunks are folded into records.npz and counters.npz"
    with pytest.raises(SystemExit):
        egta.run(out, seeds=4, sizes=[3], log=lambda *_: None)
    rec, interaction, breakdown = egta.load(out)
    t = egta.seat_table(rec, interaction, breakdown)  # raises unless cash + island + leftovers - loans is the engine's score
    led = egta.ledger_check(t, rec, interaction)
    assert sum(led["bank_moves"].values()) == 0, "the heuristics never go to the Bank"
    assert led["games_with_open_market"] == 0, "what a seat buys another seat sold, at the same price"
    assert led["residual_negative"] == 0 and led["games_defaulted_by_ledger"] == led["games_with_seize_or_deal"]
    done = ~np.isnan(t.raw["unsold"])
    assert (t.raw["unsold"][done] >= 0).all()
    egta.report(out)
    report = (out / "report.md").read_text()
    assert "## 3 seats: the empirical payoff table" in report and "4 seats: the empirical payoff table" not in report
    assert json.loads((out / "design.json").read_text())["seeds"] == 3
    assert "Peddler" in report and "not in it" in report
