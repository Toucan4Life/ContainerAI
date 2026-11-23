from argparse import Namespace
from pathlib import Path

import numpy as np
import pytest

from training import anchors
from training.model import export_weights, load_checkpoint
from training.selfplay import LEARNER
from training.train import HEURISTICS, LEAGUE_PEDDLER, league_slots, league_table, missing_peddler_rules

LADDER_ORDER = {n: f"net:{n}" for n in ("consortium", "exploit_run2", "run1", "run2", "traders")}


def test_the_league_takes_the_named_anchors_and_keeps_each_ones_slot():
    base = anchors.NET_SLOT_BASE
    assert league_slots(LADDER_ORDER, "consortium") == {base: "net:consortium"}
    assert league_slots(LADDER_ORDER, " run2, traders") == {base + 3: "net:run2", base + 4: "net:traders"}
    assert league_slots(LADDER_ORDER, "all") == {base + i: n for i, n in enumerate(LADDER_ORDER.values())}
    assert league_slots(LADDER_ORDER, "none") == league_slots(LADDER_ORDER, "") == {}


def test_a_league_net_that_is_not_an_anchor_is_refused_with_the_choices():
    with pytest.raises(SystemExit, match=r"\['consortim'\].*\['consortium'"):
        league_slots(LADDER_ORDER, "consortim,run1")


def test_the_league_keeps_every_anchor_on_the_ladder_while_naming_one():
    rated = anchors.net_names()
    nets = {n: n for n in rated}
    assert "consortium" in rated and len(rated) > 1
    assert list(league_slots(nets, "consortium").values()) == ["consortium"]
    assert len(league_slots(nets, "all")) == len(rated)


def test_the_league_peddler_is_the_shipped_peddler_v2(tmp_path):
    shipped = Path(__file__).resolve().parents[1] / "crates/container-bots/weights/peddler2.bin"
    exported = tmp_path / "peddler2.bin"
    export_weights(load_checkpoint(str(LEAGUE_PEDDLER), "cpu"), str(exported))
    assert exported.read_bytes() == shipped.read_bytes()


def test_the_peddler_seat_names_the_rules_a_run_leaves_out():
    rules = dict(no_same_turn_repay=True, min_bid=3, buy_at_harbor=True, bank_needs_load=True, no_double_reprice=True)
    assert missing_peddler_rules(Namespace(**rules)) == []
    assert missing_peddler_rules(Namespace(**rules | dict(min_bid=2, bank_needs_load=False))) == ["--min-bid 3", "--bank-needs-load"]
    assert len(missing_peddler_rules(Namespace(no_same_turn_repay=False, min_bid=0, buy_at_harbor=False,
                                               bank_needs_load=False, no_double_reprice=False))) == 5


def test_the_league_seats_the_heuristics_only_when_asked_and_leaves_the_old_tables_alone():
    def tables(**extra):
        rng = np.random.default_rng(7)
        return [league_table(rng, [1, 2], **extra) for _ in range(300)]

    assert tables() == tables(heuristic_share=0.0)
    opponents = [k for _, kinds, _ in tables(heuristic_share=1.0) for k in kinds if k != LEARNER]
    assert opponents and set(opponents) == set(HEURISTICS)
