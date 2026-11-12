from pathlib import Path

import numpy as np
import pytest
import torch

from training import evalcore as E
from training.env import DONE, MERCHANT, SIMPLE, STALLED, STANDARD, BatchEnv
from training.model import PolicyNet, save_checkpoint
from training.selfplay import Guard

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")
DEFENSE = Path(__file__).resolve().parent.parent / "runs/defense_run2_vs_exploiter_30m/final.pt"
PSRO = Path(__file__).resolve().parent.parent / "populations/psro"


def fake(status, scores, winners, players):
    n = len(status)
    full = np.zeros((n, 5), dtype=np.int32)
    for g, s in enumerate(scores):
        full[g, : len(s)] = s
    z = lambda dtype, v=0: np.full(n, v, dtype=dtype)
    return E.Records(
        players=np.array(players, dtype=np.int8), length=z(np.int8, 1), seed=z(np.uint64), seats=np.zeros((n, 5), np.int16),
        cell=z(np.int32), cluster=np.arange(n, dtype=np.uint64), status=np.array(status, dtype=np.int8), scores=full,
        winners=np.array(winners, dtype=np.int32), actions=z(np.int32),
    )


def test_canonical_utility_shares_ties_and_pays_nothing_for_stalling():
    rec = fake([DONE, DONE, STALLED], [[10, 3, 10], [1, 9, 2], [50, 1, 1]], [0b101, 0b010, 0], [3, 3, 3])
    u = E.winner_share(rec)
    assert u[0].tolist() == [0.5, 0.0, 0.5, 0.0, 0.0]
    assert u[1].tolist() == [0.0, 1.0, 0.0, 0.0, 0.0]
    assert u[2].tolist() == [0.0] * 5  # the leader of a stalled game gets nothing
    assert E.accounting(rec) == dict(games=3, completed=2, stalled=1, stall_rate=1 / 3, horizon=None)


def test_secondary_metrics_leave_stalled_games_out_of_margins_but_not_out_of_counts():
    rec = fake([DONE, STALLED], [[10, 10, 5], [9, 9, 9]], [0b011, 0], [3, 3])
    assert E.rank_share(rec)[0].tolist() == [0.75, 0.75, 0.0, 0.0, 0.0]
    assert E.rank_share(rec)[1].tolist() == [0.0] * 5
    margin = E.score_margin(rec)
    assert margin[0, :3].tolist() == [10 - 7.5, 10 - 7.5, 5 - 10]
    assert np.isnan(margin[1, :3]).all()


def test_cluster_bootstrap_resamples_whole_seeds():
    values = np.array([0.0, 0.0, 1.0, 1.0])
    mean, lo, hi = E.mean_ci(values, np.array([0, 0, 1, 1]), reps=500)
    assert mean == 0.5 and lo == 0.0 and hi == 1.0  # two seeds only: the interval is as wide as it should be
    mean, lo, hi = E.mean_ci(np.full(6, 0.25), np.array([0, 0, 1, 1, 2, 2]))
    assert mean == lo == hi == 0.25


def test_crossplay_design_rotates_tables_and_pairs_seeds():
    specs = E.crossplay_specs(3, [3, 4], seeds_per_cell=2)
    assert len(specs) == (10 * 3 + 15 * 4) * 2  # multisets x rotations x seeds
    by_cell: dict = {}
    for sp in specs:
        by_cell.setdefault(sp.cell, []).append(sp)
    members = E.cell_members(3, [3, 4])
    for cell, group in by_cell.items():
        players, ms = members[cell]
        assert all(tuple(sorted(sp.seats)) == ms for sp in group)
        rotations = {sp.seats for sp in group if sp.seed == group[0].seed}
        assert len(rotations) == len({tuple(np.roll(group[0].seats, -r)) for r in range(players)})
    seeds = {c: sorted({sp.seed for sp in g}) for c, g in by_cell.items() if members[c][0] == 3}
    assert len({tuple(v) for v in seeds.values()}) == 1  # every composition sees the same deals


def test_crossplay_include_plays_only_the_compositions_with_the_new_policy():
    full = E.crossplay_specs(4, [3, 4], seeds_per_cell=2)
    new = E.crossplay_specs(4, [3, 4], seeds_per_cell=2, include=[3])
    members = E.cell_members(4, [3, 4])
    assert {sp.cell for sp in new} == {c for c, (_, ms) in members.items() if 3 in ms}
    assert len(new) == (10 * 3 + 20 * 4) * 2  # C(n+2, n-1) compositions seat policy 3 at n players
    assert new == [sp for sp in full if 3 in members[sp.cell][1]]  # same arrangements as in the full design


def test_payoff_cells_credit_each_policy_with_its_own_seats():
    specs = E.crossplay_specs(2, [3], seeds_per_cell=2)
    n = len(specs)
    rec = fake([DONE] * n, [[1, 1, 1]] * n, [0] * n, [3] * n)
    rec.seats[:, :3] = np.array([sp.seats for sp in specs])
    rec.cell[:] = [sp.cell for sp in specs]
    rec.cluster[:] = [sp.cluster for sp in specs]
    util = (rec.seats == 0).astype(float)  # every seat of policy 0 is paid 1, whatever the rotation
    cells = E.payoff_cells(rec, 2, utility=util)
    assert cells[(3, (0, 1, 1))]["distinct"] == [0, 1]
    assert cells[(3, (0, 1, 1))]["payoff"].tolist() == [1.0, 0.0]
    assert cells[(3, (0, 0, 1))]["payoff"].tolist() == [1.0, 0.0]
    assert cells[(3, (0, 0, 0))]["payoff"].tolist() == [1.0]
    assert cells[(3, (1, 1, 1))]["payoff"].tolist() == [0.0]
    assert cells[(3, (0, 1, 1))]["games"] == 6 and cells[(3, (0, 1, 1))]["per_seed"].shape == (2, 2)


def test_guard_visits_every_action_before_repeating_one():
    guard = Guard(2, "cpu")
    state, acts = torch.randn(2, 5), torch.randn(2, 48, 3)
    logits = torch.tensor([[3.0, 1.0, 2.0, 0.5, -1e9, -1e9, -1e9, -1e9]] * 2)
    mask = torch.zeros(2, 8, dtype=torch.bool)
    mask[:, :4] = True

    def pick(env=0, seat=0):
        _, p = guard.choose(np.array([env]), np.array([seat]), logits, mask, state, acts)
        return int(p[0])

    assert [pick() for _ in range(6)] == [0, 2, 1, 3, 0, 2]  # best first, then the next best, then afresh
    assert pick(seat=1) == 0  # a seat's memory is its own
    assert pick(env=1) == 0
    guard.reset(0)
    assert pick() == 0
    tie = torch.tensor([[2.0, 2.0, 1.0] + [-1e9] * 5] * 2)
    guard.reset(0)
    _, p = guard.choose(np.array([0]), np.array([0]), tie, mask, torch.randn(2, 5), acts)
    assert int(p[0]) == 1  # the last of equal bests, as Rust's `max_by` does


def test_the_first_seat_comes_from_the_seed_and_covers_every_seat():
    seeds = np.arange(E.game_seed(E.DEV, 4, STANDARD, 0), E.game_seed(E.DEV, 4, STANDARD, 80))
    first = E.first_seat(np.full(80, 4), np.full(80, STANDARD), seeds)
    assert set(first.tolist()) == {0, 1, 2, 3}
    again = E.first_seat(np.full(160, 4), np.full(160, STANDARD), np.concatenate([seeds, seeds[::-1]]))
    assert (again[:80] == first).all() and (again[80:] == first[::-1]).all()


def test_turn_positions_follow_the_first_seat():
    from training import crossplay

    n = 90
    rec = fake([DONE] * n, [[5, 1, 1]] * n, [0b001] * n, [3] * n)  # the player in seat 0 always wins
    rec.seed = (E.game_seed(E.DEV, 3, STANDARD, 0) + np.arange(n)).astype(np.uint64)
    held = (-E.first_seats(rec)) % 3  # the turn position of seat 0
    got = [ci[0] for ci in crossplay.turn_order(rec)["3"]]
    assert got == pytest.approx([3.0 * float((held == t).mean()) for t in range(3)])
    assert sum(got) == pytest.approx(3.0)


@needs_cuda
def test_games_past_the_horizon_are_counted_not_dropped():
    pol = [E.Policy("merchant", scripted=MERCHANT), E.Policy("simple", scripted=SIMPLE)]
    specs = E.crossplay_specs(2, [3], seeds_per_cell=2)
    rec = E.play(specs, pol, envs=16, log=None, horizon=15)
    assert len(rec) == len(specs)
    assert rec.stalled.all() and (rec.actions == 15).all()
    assert E.winner_share(rec).sum() == 0
    rec = E.play(specs, pol, envs=16, log=None)
    assert not rec.stalled.any()
    assert np.allclose(E.winner_share(rec).sum(axis=1), 1.0)


@needs_cuda
def test_play_is_reproducible_and_pairs_policies_with_modes(tmp_path):
    env = BatchEnv(1, 1)
    torch.manual_seed(0)
    path = str(tmp_path / "net.pt")
    save_checkpoint(PolicyNet(env.state_dim, env.action_dim, 32), path)
    pol = [E.Policy("net", path, "sample"), E.Policy("net-guarded", path, "guarded"), E.Policy("merchant", scripted=MERCHANT)]
    specs = E.crossplay_specs(3, [3], seeds_per_cell=1, length=STANDARD)[:48]
    a = E.play(specs, pol, envs=16, log=None, seed=3)
    b = E.play(specs, pol, envs=16, log=None, seed=3)
    for key in ("status", "scores", "winners", "actions"):
        assert np.array_equal(getattr(a, key), getattr(b, key))
    assert a.meta["policies"][1]["mode"] == "guarded"
    assert [p["sha256"] for p in a.meta["policies"]] == [E.sha256_file(path), E.sha256_file(path), None]


@needs_cuda
@pytest.mark.skipif(not DEFENSE.exists(), reason="the Magnate net is not checked in")
def test_guarded_greedy_reproduces_rust_magnate_game_for_game():
    # Scores and action counts of `arena --bots magnate --players 3,4,5 --seed 5` games 0-5, the built-in Rust Magnate
    expect = [[61, 53, 65], [61, 76, 89, 77], [107, 107, 105, 103, 104], [88, 68, 45], [75, 91, 75, 81],
              [104, 85, 75, 111, 150]]
    lengths = [533, 845, 1113, 417, 785, 1101]
    mask = (1 << 64) - 1
    specs = []
    for i in range(len(expect)):
        n = (3, 4, 5)[i % 3]
        specs.append(E.GameSpec(n, STANDARD, (((5 + i) * 0xD1342543DE82EF95) & mask) | 1, (0,) * n))
    rec = E.play(specs, [E.Policy("magnate", str(DEFENSE), "guarded")], envs=8, log=None)
    for g, scores in enumerate(expect):
        assert rec.scores[g, : len(scores)].tolist() == scores
        assert rec.actions[g] == lengths[g]
    assert [int(w) for w in rec.winners] == [1 << w for w in (2, 2, 1, 0, 1, 4)]


@needs_cuda
@pytest.mark.skipif(not (PSRO / "psro08.pt").exists(), reason="the PSRO nets are not checked in")
def test_guarded_greedy_reproduces_rust_consortium_game_for_game():
    # `arena --bots consortium --players 3,4,5 --seed 5` games 0-5: the built-in Rust Consortium plays psro08
    # (`crates/container-bots/src/consortium.rs`), embedded as exported weights
    expect = [[97, -21, 84], [95, 100, 45, 90], [98, 97, 93, 106, 110], [73, 58, 74], [74, 76, 72, 95],
              [113, 82, 101, 92, 106]]
    lengths = [1418, 915, 1307, 418, 990, 1231]
    mask = (1 << 64) - 1
    specs = []
    for i in range(len(expect)):
        n = (3, 4, 5)[i % 3]
        specs.append(E.GameSpec(n, STANDARD, (((5 + i) * 0xD1342543DE82EF95) & mask) | 1, (0,) * n))
    rec = E.play(specs, [E.Policy("psro08", str(PSRO / "psro08.pt"), "guarded")], envs=8, log=None)
    for g, scores in enumerate(expect):
        assert rec.scores[g, : len(scores)].tolist() == scores
        assert rec.actions[g] == lengths[g]
    assert [int(w) for w in rec.winners] == [1 << w for w in (0, 1, 4, 2, 3, 0)]
