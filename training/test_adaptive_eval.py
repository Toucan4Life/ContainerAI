from pathlib import Path

import pytest
import torch

from training import adaptive_eval as A
from training import evalcore as E

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")
PEDDLER = str(Path(__file__).parent / "league" / "peddler2.pt")


@pytest.mark.parametrize("experiment", A.EXPERIMENTS)
def test_a_seed_seats_the_focus_at_every_seat_in_turn_against_the_same_opponents_in_the_same_order(experiment):
    games = {}
    for spec in A.focus_specs(experiment, (3, 4), 2, E.DEV):
        assert len(spec.seats) == spec.players and spec.seats.count(0) == 1
        games.setdefault((spec.cell, spec.seed), []).append(spec)
    assert len(games) == 2 * sum(len(A.compositions(experiment, n)) for n in (3, 4))
    for (cell, _), group in games.items():
        n = group[0].players
        assert sorted(s.seats.index(0) for s in group) == list(range(n))
        orders = {tuple(k for k in s.seats if k) for s in group}
        assert len(orders) == 1 and sorted(orders.pop()) == sorted(A.compositions(experiment, n)[cell % 10_000])


@needs_cuda
def test_a_net_in_the_focus_seat_is_its_own_reference_game_for_game_and_the_report_is_written(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "CONTROL", PEDDLER)  # runs/ is not checked in
    out = tmp_path / "adaptive"
    A.run(out, PEDDLER, seeds=2, sizes=(3,), log=lambda *_: None, only="adaptive")
    assert sorted(p.name for p in out.glob("*.npz")) == ["heuristics_adaptive.npz", "nets_adaptive.npz"]
    A.run(out, PEDDLER, seeds=2, sizes=(3,), log=lambda *_: None)
    played = {e: {name: A.Played(out / f"{e}_{name}.npz") for name in A.foci(PEDDLER)} for e in A.EXPERIMENTS}
    for e, nets in played.items():
        for ref in ("peddler2", "control_s1"):
            for metric in ("win", "score"):
                assert A.paired(nets["adaptive"], nets[ref], 3, metric) == [0.0, 0.0, 0.0], (e, ref, metric)
    text = A.report(out).read_text()
    assert "### Against the references" in text and "| 3 | deliveries | " in text
    with pytest.raises(SystemExit):
        A.run(out, PEDDLER, seeds=3, sizes=(3,), log=lambda *_: None)
