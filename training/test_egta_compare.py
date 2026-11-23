import numpy as np
import pytest

from training import egta
from training import egta_compare as C
from training import env as V
from training import evalcore as E


def played(seeds, stalled=(), dead=None, order=(0, 1, 2)):
    """Three-seat games, three cyclic rotations of one composition per seed. In the first `dead[s]` games of seed s the Producer ends dead."""
    n, dead = 3, dead or {}
    games = [(s, r) for s in seeds for r in range(n)]
    g = len(games)
    seat = np.full((g, V.MAX_PLAYERS), -1, dtype=np.int16)
    interaction = np.zeros((g, V.MAX_PLAYERS, V.INTERACTION), dtype=np.int32)
    breakdown = np.zeros((g, V.MAX_PLAYERS, 4), dtype=np.int32)
    status = np.full(g, E.DONE, dtype=np.int8)
    for i, (s, r) in enumerate(games):
        seat[i, :n] = (order * 2)[r:r + n]
        if s in stalled:
            status[i] = E.STALLED
            continue  # a stalled game has no final breakdown
        breakdown[i, :n, 0] = 20
        if r < dead.get(s, 0):
            breakdown[i, list(seat[i, :n]).index(0)] = (0, 0, 0, 22)
    scores = breakdown[:, :, :3].sum(2) - breakdown[:, :, 3]
    cluster = np.array([1000 + s for s, _ in games], dtype=np.uint64)
    meta = {"horizon": 2500, "length": 0, "block": "dev", "policies": [{"name": name, "scripted": True} for name in egta.NAMES]}
    rec = E.Records(players=np.full(g, n, dtype=np.int8), length=np.zeros(g, dtype=np.int8), seed=cluster, seats=seat,
                    cell=np.full(g, egta.cell_of(n, (0, 1, 2)), dtype=np.int32), cluster=cluster, status=status, scores=scores,
                    winners=np.full(g, 1, dtype=np.int32), actions=np.zeros(g, dtype=np.int32), meta=meta)
    return rec, interaction, breakdown


def row(result, quantity):
    return next(r for r in result["rows"] if r["quantity"] == quantity)


def test_only_the_seeds_both_runs_hold_are_compared_and_only_if_they_were_played_the_same_way():
    old = played(range(12), stalled={10, 11})
    result = C.compare(old, played(range(10)))
    assert result["seeds"] == {3: 10} and result["games"] == {3: 30}
    stalls = row(result, "stalled games")
    assert stalls["old"]["events"] == 0, "the two seeds that only the old run holds, and where it stalled, are left out"
    assert stalls["none"] and stalls["bound"] == pytest.approx(0.3)
    with pytest.raises(ValueError, match="design"):
        C.compare(old, played(range(10), order=(0, 2, 1)))


def test_the_difference_is_paired_on_the_seeds_and_a_stuck_seat_is_not_only_a_dead_one():
    before = dict(enumerate(np.random.default_rng(0).integers(0, 3, 40)))
    after = {s: k + 1 for s, k in before.items()}
    result = C.compare(played(range(40), dead=before), played(range(40), dead=after))
    dead = row(result, "producer seats dead at the end")
    assert dead["diff"] == pytest.approx([1 / 3, 1 / 3, 1 / 3]), "the seeds differ widely, the shift of one dead Producer per seed does not"
    assert dead["excludes_zero"] and dead["old"]["events"] == sum(before.values())

    rec, interaction, breakdown = played(range(5))
    breakdown[0, 0] = (1, 0, 0, 22)  # $1 and both loans unpaid: it cannot pay the next turn's interest, yet it is not the dead signature
    rec.scores[:] = breakdown[:, :, :3].sum(2) - breakdown[:, :, 3]
    tally = C.tally((rec, interaction, breakdown))[3]
    assert tally["producer seats dead at the end"][0].sum() == 0 and tally["producer seats stuck at the end"][0].sum() == 1
