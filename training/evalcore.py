"""Evaluation core: every game is accounted for and every protocol choice is explicit.

The conventions below are specified in `docs/equilibrium_spec.md`:

* **Utility.** A completed game pays each seat `1/|W|` if it is among the winners `W` (highest total,
  ties broken by containers left in the factory district, as the engine decides) and 0 otherwise. A
  game that has not ended after `HORIZON` engine actions is *stalled* and pays 0 to every seat: no
  seat gains by stalling, and a leader loses by it. Nothing is ever dropped from a report.
* **Policies** are `(net, mode)` pairs or scripted bots. Modes are never pooled: `sample` draws from
  the softmax, `greedy` plays the arg-max, `guarded` plays the arg-max with the repetition guard of the
  deployed bot (see `selfplay.Guard`).
* **Seeds** come from disjoint predeclared blocks (`DEV`, `HOLDOUT`, `AUDIT`), all above the 2**40
  range the trainer draws from. Development decisions use `DEV`; final claims use `HOLDOUT`.
* **Designs** rotate every table: one seed seats the same arrangement in all its `n` cyclic rotations,
  so each policy meets every seat's endowment and first-player position once.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import time
from collections.abc import Callable, Sequence
from dataclasses import dataclass, field
from itertools import combinations_with_replacement
from pathlib import Path
from typing import NamedTuple

import numpy as np
import torch

from training.env import DONE, MAX_PLAYERS, STALLED, STANDARD, BatchEnv
from training.model import PolicyNet, load_checkpoint
from training.selfplay import Roller

HORIZON = 2500  # engine actions after which a running game is stalled
SLOT_BASE = 1000  # slots of nets; scripted seats use the env's negative kinds
DEV, HOLDOUT, AUDIT = 1 << 50, 2 << 50, 3 << 50
MODES = ("sample", "greedy", "guarded")
ROOT = Path(__file__).resolve().parent.parent


@dataclass(frozen=True)
class Policy:
    """One entry of a population: a net played in a mode, or a scripted bot of the Rust engine.

    `adjust` is for diagnostics: a hook that may replace the net's pick (see `Roller.collect`). A strategy
    with a hook is not a `(net, mode)` pair and never enters a population."""

    name: str
    path: str | None = None
    mode: str = "sample"
    scripted: int | None = None
    adjust: Callable | None = None

    def __post_init__(self):
        if (self.path is None) == (self.scripted is None):
            raise ValueError(f"{self.name}: give either a checkpoint path or a scripted kind")
        if self.mode not in MODES:
            raise ValueError(f"{self.name}: unknown mode {self.mode}")
        if self.adjust is not None and self.path is None:
            raise ValueError(f"{self.name}: only a net can be adjusted")

    def with_mode(self, mode: str) -> Policy:
        return Policy(self.name, self.path, mode, self.scripted, self.adjust) if self.path else self


class GameSpec(NamedTuple):
    """One game to play: who sits where (policy indices) and how it is grouped for statistics."""

    players: int
    length: int
    seed: int
    seats: tuple[int, ...]
    cell: int = 0  # the design cell, e.g. the table composition
    cluster: int = 0  # games of one cluster share a seed and are resampled together


@dataclass
class Records:
    """Everything that happened in a batch of games, in spec order."""

    players: np.ndarray
    length: np.ndarray
    seed: np.ndarray
    seats: np.ndarray
    cell: np.ndarray
    cluster: np.ndarray
    status: np.ndarray
    scores: np.ndarray
    winners: np.ndarray
    actions: np.ndarray
    meta: dict = field(default_factory=dict)

    def __len__(self) -> int:
        return len(self.status)

    @property
    def stalled(self) -> np.ndarray:
        return self.status == STALLED

    def subset(self, keep: np.ndarray) -> Records:
        arrays = {k: getattr(self, k)[keep] for k in ARRAYS}
        return Records(**arrays, meta=self.meta)

    def save(self, path: str | Path) -> None:
        Path(path).parent.mkdir(parents=True, exist_ok=True)
        np.savez_compressed(path, **{k: getattr(self, k) for k in ARRAYS}, meta=json.dumps(self.meta))

    @classmethod
    def load(cls, path: str | Path) -> Records:
        with np.load(path, allow_pickle=False) as z:
            return cls(**{k: z[k] for k in ARRAYS}, meta=json.loads(str(z["meta"])))

    @classmethod
    def concat(cls, parts: Sequence[Records]) -> Records:
        arrays = {k: np.concatenate([getattr(p, k) for p in parts]) for k in ARRAYS}
        return cls(**arrays, meta=parts[0].meta)


ARRAYS = ("players", "length", "seed", "seats", "cell", "cluster", "status", "scores", "winners", "actions")


def sha256_file(path: str | Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def git_commit() -> str:
    """HEAD and, when the code differs from it, a short hash of that difference, so that a result can be tied
    to the code that produced it even before the work is committed."""

    def git(*args: str) -> bytes:
        return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, check=True).stdout

    try:
        head = git("rev-parse", "HEAD").decode().strip()
        diff = git("diff", "HEAD", "--", "training", "crates")
        untracked = sorted(git("ls-files", "--others", "--exclude-standard", "--", "training", "crates").decode().split())
        digest = hashlib.sha256(diff)
        for name in untracked:
            digest.update(name.encode())
            digest.update((ROOT / name).read_bytes())
    except (OSError, subprocess.CalledProcessError):
        return "unknown"
    return head + (f"+dirty-{digest.hexdigest()[:12]}" if diff or untracked else "")


def load_nets(policies: Sequence[Policy], device: str) -> dict[str, PolicyNet]:
    """One frozen net per distinct checkpoint, shared by every mode that plays it."""
    nets: dict[str, PolicyNet] = {}
    for p in policies:
        if p.path and p.path not in nets:
            nets[p.path] = load_checkpoint(str(ROOT / p.path) if not Path(p.path).is_absolute() else p.path,
                                           device).eval().requires_grad_(False)
    return nets


def seat_slots(policies: Sequence[Policy], nets: dict[str, PolicyNet]):
    """Env kind per policy index, the net per slot, and which slots play greedy / guarded."""
    kind_of, net_of, greedy, guarded = [], {}, set(), set()
    for k, p in enumerate(policies):
        if p.scripted is not None:
            kind_of.append(p.scripted)
            continue
        slot = SLOT_BASE + k
        kind_of.append(slot)
        net_of[slot] = nets[p.path]
        if p.mode == "greedy":
            greedy.add(slot)
        elif p.mode == "guarded":
            guarded.add(slot)
    return kind_of, net_of, greedy, guarded


def play(specs: Sequence[GameSpec], policies: Sequence[Policy], *, device: str | None = None, envs: int = 256,
         threads: int = 12, horizon: int = HORIZON, seed: int = 0, nets: dict | None = None,
         log: Callable[[str], None] | None = print, chunk: int = 2048,
         configure: Callable[[BatchEnv], None] | None = None) -> Records:
    """Plays every spec to its end (or the horizon) and returns one record per spec, in spec order."""
    device = device or ("cuda" if torch.cuda.is_available() else "cpu")
    torch.manual_seed(seed)
    nets = nets if nets is not None else load_nets(policies, device)
    kind_of, net_of, greedy, guarded = seat_slots(policies, nets)
    adjust = {SLOT_BASE + k: p.adjust for k, p in enumerate(policies) if p.adjust is not None}
    g_total = len(specs)
    env = BatchEnv(max(1, min(envs, g_total)), threads, pinned=device == "cuda")
    env.set_horizon(horizon)
    if configure:
        configure(env)  # e.g. the rules a net was trained under, for the slots `SLOT_BASE + k`
    queue = iter(specs)

    def next_table():
        sp = next(queue)
        return sp.players, [kind_of[k] for k in sp.seats], sp.seed, sp.length

    roller = Roller(env, next_table, device, np.random.default_rng(seed), limit=g_total, record=False)
    cols = {k: np.zeros(g_total, dtype=d) for k, d in (("status", np.int8), ("winners", np.int32), ("actions", np.int32))}
    scores = np.zeros((g_total, MAX_PLAYERS), dtype=np.int32)
    breakdown = np.zeros((g_total, MAX_PLAYERS, 4), dtype=np.int32)
    seen = np.zeros(g_total, dtype=bool)
    t0 = time.monotonic()
    try:
        while not seen.all():
            _, status, _, got, _ = roller.collect(net_of, target=chunk, mode="sample", greedy_slots=greedy,
                                                  guarded_slots=guarded, adjust=adjust)
            fin = roller.finished
            if not len(fin.gid):
                break
            seen[fin.gid] = True
            cols["status"][fin.gid] = status
            cols["winners"][fin.gid] = fin.winners
            cols["actions"][fin.gid] = fin.actions
            scores[fin.gid] = got
            breakdown[fin.gid] = fin.breakdown
            if log:
                done = int(seen.sum())
                rate = done / (time.monotonic() - t0)
                log(f"  {done}/{g_total} games, {rate:.0f}/s, stalled so far {int((cols['status'][seen] == STALLED).sum())}")
    finally:
        roller.close()
        env.close()
    if not seen.all():
        raise RuntimeError(f"{int((~seen).sum())} games were never accounted for")
    seats = np.full((g_total, MAX_PLAYERS), -1, dtype=np.int16)
    for g, sp in enumerate(specs):
        seats[g, : sp.players] = sp.seats
    meta = dict(
        policies=[dict(name=p.name, path=p.path, mode=p.mode, scripted=p.scripted,
                       sha256=sha256_file(p.path) if p.path else None) for p in policies],
        horizon=horizon, torch_seed=seed, git=git_commit(), seconds=round(time.monotonic() - t0, 1),
    )
    rec = Records(
        players=np.array([s.players for s in specs], dtype=np.int8),
        length=np.array([s.length for s in specs], dtype=np.int8),
        seed=np.array([s.seed for s in specs], dtype=np.uint64), seats=seats,
        cell=np.array([s.cell for s in specs], dtype=np.int32),
        cluster=np.array([s.cluster for s in specs], dtype=np.uint64),
        status=cols["status"], scores=scores, winners=cols["winners"], actions=cols["actions"], meta=meta,
    )
    rec.breakdown = breakdown  # cash, island, leftovers, loans per seat; a diagnostic, not saved
    return rec


# --------------------------------------------------------------------------------------- utilities


def _seat_mask(players: np.ndarray) -> np.ndarray:
    return np.arange(MAX_PLAYERS)[None, :] < players[:, None]


def winner_share(rec: Records) -> np.ndarray:
    """The canonical utility per game and seat: 1/|W| for each winner, 0 otherwise, 0 for all if stalled."""
    won = ((rec.winners[:, None] >> np.arange(MAX_PLAYERS)[None, :]) & 1).astype(np.float64)
    won[rec.status != DONE] = 0.0
    count = won.sum(axis=1, keepdims=True)
    return np.where(count > 0, won / np.maximum(count, 1), 0.0) * _seat_mask(rec.players)


def rank_share(rec: Records) -> np.ndarray:
    """Share of opponents beaten on total score, ties half (the trainer's `win_fraction`); 0 if stalled."""
    s = rec.scores.astype(np.float64)
    mask = _seat_mask(rec.players)
    beat = ((s[:, :, None] > s[:, None, :]) + 0.5 * (s[:, :, None] == s[:, None, :])) * mask[:, None, :]
    beat -= 0.5 * np.eye(MAX_PLAYERS)[None] * mask[:, :, None]  # a seat does not beat itself
    out = beat.sum(axis=2) / np.maximum(rec.players[:, None] - 1, 1)
    return np.where((rec.status == DONE)[:, None], out, 0.0) * mask


def score_margin(rec: Records) -> np.ndarray:
    """Own total minus the mean of the opponents' totals; NaN for stalled games (they have no scores)."""
    s = rec.scores.astype(np.float64)
    mask = _seat_mask(rec.players)
    total = (s * mask).sum(axis=1, keepdims=True)
    margin = s - (total - s) / np.maximum(rec.players[:, None] - 1, 1)
    margin[rec.status != DONE] = np.nan
    return np.where(mask, margin, np.nan)


def accounting(rec: Records) -> dict:
    """The counts every report must carry."""
    n = len(rec)
    stalled = int(rec.stalled.sum())
    return dict(games=n, completed=n - stalled, stalled=stalled, stall_rate=stalled / max(n, 1),
                horizon=rec.meta.get("horizon"))


def first_seat(players, length, seed) -> np.ndarray:
    """The seat that moves first, for arrays of (players, length, seed).

    The engine draws the first player from the game seed alone, so it is the same for every arrangement
    of one seed. It is read from the first pending decision of a freshly reset game, once per distinct
    triple. Turn position t of a game is the seat `(first + t) % players`, as the turn passes to the next
    seat; the seat index itself carries no turn-order meaning.
    """
    triples = np.stack([np.asarray(players, np.int64), np.asarray(length, np.int64),
                        np.asarray(seed).astype(np.int64)], axis=1)
    keys, inverse = np.unique(triples, axis=0, return_inverse=True)
    seat = np.zeros(len(keys), dtype=np.int64)
    for lo in range(0, len(keys), 256):
        chunk = keys[lo:lo + 256]
        env = BatchEnv(len(chunk), threads=2)
        try:
            for i, (n, ln, sd) in enumerate(chunk):
                env.reset(i, int(n), int(sd), [0] * int(n), int(ln))  # kind 0: a net seat, so every seat is pending
            env.advance()
            _, _, _, slot, got = env.observe()
            if (slot < 0).any():
                raise RuntimeError("a freshly reset game has no pending decision")
            seat[lo:lo + len(chunk)] = got
        finally:
            env.close()
    return seat[inverse.reshape(-1)]


def first_seats(rec: Records) -> np.ndarray:
    """The seat that moves first in every game of `rec`."""
    return first_seat(rec.players, rec.length, rec.seed)


# --------------------------------------------------------------------------------------- statistics


def mean_ci(values: np.ndarray, clusters: np.ndarray | None = None, reps: int = 2000, alpha: float = 0.05,
            seed: int = 0) -> tuple[float, float, float]:
    """Mean and a percentile bootstrap interval; whole clusters (shared seeds) are resampled together."""
    values = np.asarray(values, dtype=np.float64)
    ok = ~np.isnan(values)
    if not ok.any():
        return float("nan"), float("nan"), float("nan")
    values = values[ok]
    clusters = np.arange(len(values)) if clusters is None else np.asarray(clusters)[ok]
    _, inv = np.unique(clusters, return_inverse=True)
    k = inv.max() + 1
    sums = np.bincount(inv, weights=values, minlength=k)
    counts = np.bincount(inv, minlength=k).astype(np.float64)
    rng = np.random.default_rng(seed)
    draws = rng.integers(0, k, size=(reps, k))
    est = sums[draws].sum(axis=1) / counts[draws].sum(axis=1)
    return float(sums.sum() / counts.sum()), float(np.quantile(est, alpha / 2)), float(np.quantile(est, 1 - alpha / 2))


# --------------------------------------------------------------------------------------- designs


def game_seed(block: int, players: int, length: int, index: int) -> int:
    """The game seed of seed index `index`: shared by every table composition so results are paired."""
    return block + (length * 8 + players) * 1_000_000 + index


def log_holdout(entry: dict, path: str | Path | None = None) -> None:
    """Appends one look at the holdout block to the log (`reports/holdout_log.jsonl` by default)."""
    path = Path(path) if path else ROOT / "reports/holdout_log.jsonl"
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "a") as f:
        f.write(json.dumps(dict(time=time.strftime("%Y-%m-%d %H:%M:%S"), **entry)) + "\n")


def multisets(n_policies: int, players: int) -> list[tuple[int, ...]]:
    return list(combinations_with_replacement(range(n_policies), players))


def crossplay_specs(n_policies: int, player_counts: Sequence[int], seeds_per_cell: int, block: int = DEV,
                    length: int = STANDARD, seed_offset: int = 0, seed: int = 0,
                    include: Sequence[int] | None = None) -> list[GameSpec]:
    """Every multiset of policies at every player count, `seeds_per_cell` seeds each, all rotations.

    The arrangement around the table is a random permutation of the multiset per seed; the same seed
    index is used for every multiset (common random numbers). `cell` numbers the multisets of a player
    count in `multisets` order, offset by 10**4 per player count, and `cluster` identifies the seed.
    With `include`, only the multisets that seat at least one of those policies are played (the cell
    numbers stay those of the full design): the new cells when a policy joins a population.
    """
    specs: list[GameSpec] = []
    wanted = None if include is None else set(include)
    for n in player_counts:
        for c, ms in enumerate(multisets(n_policies, n)):
            if wanted is not None and not wanted.intersection(ms):
                continue
            for s in range(seed_offset, seed_offset + seeds_per_cell):
                rng = np.random.default_rng([seed, n, c, s])
                order = tuple(int(x) for x in rng.permutation(ms))
                gseed = game_seed(block, n, length, s)
                for r in range(n):
                    specs.append(GameSpec(n, length, gseed, order[r:] + order[:r], n * 10_000 + c, gseed))
    return specs


def cell_members(n_policies: int, player_counts: Sequence[int]) -> dict[int, tuple[int, tuple[int, ...]]]:
    """cell id -> (players, sorted multiset)."""
    return {n * 10_000 + c: (n, ms) for n in player_counts for c, ms in enumerate(multisets(n_policies, n))}


def payoff_cells(rec: Records, n_policies: int, utility: np.ndarray | None = None) -> dict:
    """Mean payoff per policy of every multiset, with the per-seed values for bootstrapping.

    Returns {(players, multiset): dict(payoff=[k], per_seed=[S, k], seeds=[S], stalled=int, games=int)}
    where k indexes the distinct policies of the multiset (ascending). A policy seated twice is
    credited with the mean of its seats.
    """
    util = winner_share(rec) if utility is None else utility
    out: dict = {}
    members = cell_members(n_policies, sorted({int(p) for p in rec.players}))
    order = np.argsort(rec.cell, kind="stable")
    cells, starts = np.unique(rec.cell[order], return_index=True)
    bounds = list(starts[1:]) + [len(order)]
    for cell, a, b in zip(cells, starts, bounds):
        idx = order[a:b]
        n, ms = members[int(cell)]
        distinct = sorted(set(ms))
        clusters = np.unique(rec.cluster[idx])
        per_seed = np.zeros((len(clusters), len(distinct)))
        slot = {p: j for j, p in enumerate(distinct)}
        pos = np.searchsorted(clusters, rec.cluster[idx])
        count = np.zeros(len(clusters))
        np.add.at(count, pos, 1.0)
        for g_local, g in enumerate(idx):
            seats = rec.seats[g, :n]
            for p in distinct:
                where = seats == p
                per_seed[pos[g_local], slot[p]] += util[g, :n][where].mean()
        per_seed /= count[:, None]
        out[(n, ms)] = dict(payoff=per_seed.mean(axis=0), per_seed=per_seed, seeds=clusters, distinct=distinct,
                            stalled=int(rec.stalled[idx].sum()), games=len(idx))
    return out
