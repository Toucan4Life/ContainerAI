from types import SimpleNamespace

import numpy as np
import pytest
import torch

from training import evalcore as E
from training import gate
from training.env import PRODUCER, SHIPPER, SHOWCASE, STANDARD

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")
KEYS = ("produced", "sell_through", "cheap_shelf_stock", "competitive_share", "win", "rank", "sold", "deliveries",
        "sales_share", "avg_price", "cash", "score")


def specs_of(n):
    return [E.GameSpec(4, STANDARD, i, (0, 1, 2, 3), 0, i) for i in range(n)]


def table(n=200, level=None, **shift):
    """Per-game values of the focus seat, `level` apart from 10, `shift` SDs further (the same noise in every call: the games are paired)."""
    rng = np.random.default_rng(0)
    out = {k: rng.normal((level or {}).get(k, 10.0), 1.0, n) for k in KEYS}
    for k, d in shift.items():
        out[k] = out[k] + d
    out["ok"] = np.arange(n)
    return out


def producer_verdict(level=None, base_level=None, **shift):
    return gate.verdict("producer", table(level=level, **shift), table(level=base_level), specs_of(200), effect=0.2)


def test_the_producer_gate_asks_for_production_and_only_reports_sales_share():
    v = producer_verdict(produced=1.0, sell_through=1.0)
    assert v["primary"] == "produced" and v["diverges"] and v["economic_ok"] and v["passes"]
    v = producer_verdict(sales_share=1.0, sell_through=1.0)
    assert not v["primary_pass"] and not v["diverges"] and not v["passes"]
    assert "sales_share" in v["metrics"], "still reported as a diagnostic"


def test_production_that_is_not_sold_or_loses_the_score_does_not_pass():
    # `cheap_shelf_stock` is a stock, so a hoarder meets the corroboration of the identity; the economic checks catch it
    v = producer_verdict(level={"sell_through": 0.5}, produced=1.0, cheap_shelf_stock=1.0)
    assert v["diverges"] and v["pathology"] == ["production without sales"] and not v["economic_ok"] and not v["passes"]
    v = producer_verdict(produced=1.0, sell_through=1.0, score=-1.0)
    assert v["diverges"] and v["pathology"] == ["gives its value away"] and not v["passes"]


def test_a_producer_that_sells_what_it_makes_is_not_failed_for_selling_less_than_a_reseller():
    # Peddler v2 resells, so its sell-through is well above 1: 1.0 is a fair Producer, 0.4 per container made below it is not a hoard
    v = producer_verdict(level={"sell_through": 1.0}, base_level={"sell_through": 1.4}, produced=1.0, cheap_shelf_stock=1.0, score=-0.1)
    assert v["metrics"]["sell_through"]["effect"] < -0.2
    assert v["diverges"] and v["economic_ok"] and v["passes"]


def test_stalls_are_game_quality_and_are_judged_against_the_baseline():
    specs = specs_of(200)
    done = SimpleNamespace(stalled=np.zeros(200, dtype=bool))
    stuck = SimpleNamespace(stalled=np.arange(200) < 20)
    worse = gate.stalls(stuck, done, specs)
    assert not worse["game_ok"] and worse["stall_rate"] == 0.1 and worse["base_stall_rate"] == 0.0
    assert gate.stalls(stuck, stuck, specs)["game_ok"], "Peddler v2 stalling as much is no fault of the candidate"
    assert gate.stalls(done, stuck, specs)["game_ok"]


@needs_cuda
def test_the_metrics_of_played_games_hold_together():
    pol = [E.Policy("producer", scripted=PRODUCER), E.Policy("showcase", scripted=SHOWCASE), E.Policy("shipper", scripted=SHIPPER)]
    specs = [E.GameSpec(3, STANDARD, E.game_seed(E.DEV, 3, STANDARD, i), seats, 0, i)
             for i in range(12) for seats in ((0, 1, 2), (1, 0, 2), (2, 1, 0))]
    rec = E.play(specs, pol, envs=16, threads=4, log=None)
    m = gate.game_metrics(rec)
    n = len(m["ok"])
    assert n > 30 and all(len(m[k]) == n for k in KEYS)
    final = rec.breakdown[(rec.seats == 0) & ~rec.stalled[:, None]]
    assert np.array_equal(m["score"], final[:, :3].sum(1) - final[:, 3])
    assert np.array_equal(m["cash"], final[:, 0])
    assert (m["avg_price"] >= 0).all() and (m["produced"] > 0).any()
