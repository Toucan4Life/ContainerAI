"""Net anchors: trained nets kept as fixed, rated opponents next to the scripted bots.

A net that beats Merchant 97% of the time cannot be told apart from one that beats it 99%, so the
scripted ladder stops measuring it. Rated nets extend the ladder upward. Each lives in
`training/anchors/<name>.pt` and has its rating in `training/ladder.json`.

    python -m training.anchors add NAME runs/run1/final.pt [--games 3000]
    python -m training.anchors rate runs/run1/final.pt [--games 3000]    # rates without storing
    python -m training.anchors duel runs/x/final.pt run2 [--games 3000] [--sample-target]
    python -m training.anchors list
"""

from __future__ import annotations

import argparse
import json
import shutil
from pathlib import Path

import numpy as np
import torch

from training.elo import rate_against
from training.env import DONE, MERCHANT, RANDOM, SIMPLE, BatchEnv
from training.model import load_checkpoint
from training.selfplay import LEARNER, Roller

DIR = Path(__file__).with_name("anchors")
LADDER = Path(__file__).with_name("ladder.json")
SCRIPTED = {"random": RANDOM, "simple": SIMPLE, "merchant": MERCHANT}
NET_SLOT_BASE = 1000  # slots of anchor nets inside training runs, clear of the frozen snapshots


def load_ladder() -> dict[str, float]:
    return json.loads(LADDER.read_text())


def net_names(ladder: dict[str, float] | None = None) -> list[str]:
    """Anchor nets that have both a rating and a checkpoint, weakest first."""
    ladder = ladder or load_ladder()
    return sorted((n for n in ladder if n not in SCRIPTED and (DIR / f"{n}.pt").exists()), key=ladder.get)


def load_nets(device: str, ladder: dict[str, float] | None = None) -> dict[str, torch.nn.Module]:
    return {n: load_checkpoint(str(DIR / f"{n}.pt"), device).eval() for n in net_names(ladder)}


def seed_of(rng) -> int:
    return int(rng.integers(1, 1 << 40))


def evaluate(env: BatchEnv, net, device: str, rng, games: int, ladder: dict[str, float],
             nets: dict[str, torch.nn.Module] | None = None) -> dict:
    """Plays `net` greedily at tables drawn from the whole ladder and rates it against it.

    Every table seats the net with two to four opponents, each a scripted bot or an anchor net
    picked uniformly. Games that stall are dropped.
    """
    nets = nets if nets is not None else {}
    names = [n for n in ladder if n in SCRIPTED or n in nets]
    slot_of = {**SCRIPTED, **{n: i + 1 for i, n in enumerate(sorted(nets))}}
    policies = {0: net, **{slot_of[n]: nets[n] for n in nets}}
    kind_name = {v: k for k, v in slot_of.items()}

    def next_table():
        n = int(rng.integers(2, 5))
        # Merchant is always in the field, so the scripted comparison stays comparable between runs.
        field = ["merchant"] + [str(rng.choice(names)) for _ in range(n - 1)]
        kinds = [LEARNER] + [slot_of[f] for f in field]
        rng.shuffle(kinds)
        return len(kinds), kinds, seed_of(rng)

    roller = Roller(env, next_table, device, rng, limit=games, record=False)
    _, status, _, scores, tables = roller.collect(policies, mode="greedy")
    roller.close()
    results, per_opp = [], {}
    for i, (n, kinds, _) in enumerate(tables):
        if status[i] != DONE:
            continue
        me = kinds.index(LEARNER)
        for s in range(n):
            if s == me:
                continue
            a, b = scores[i][me], scores[i][s]
            out = 1.0 if a > b else 0.5 if a == b else 0.0
            name = kind_name[kinds[s]]
            results.append((name, out))
            per_opp.setdefault(name, []).append(out)
    elo = rate_against({k: v for k, v in ladder.items() if k in names}, results)
    return {"elo": round(elo, 1), "games": games,
            **{f"vs_{k}": round(float(np.mean(v)), 3) for k, v in per_opp.items()}}


def exploit_table(rng, target_slot: int):
    """A 3- to 5-seat table with one seat for the learner; every other seat plays `target_slot`."""
    players = int(rng.integers(3, 6))
    kinds = [target_slot] * players
    kinds[int(rng.integers(players))] = LEARNER
    return players, kinds, seed_of(rng)


def exploit_stats(tables, status, scores, target: str, rating: float, games: int) -> dict:
    """Summarises games of one learner seat against copies of `target`; see `evaluate_exploit`."""
    duels, wins, fair, margins = [], [], [], []
    for i, (n, kinds, _) in enumerate(tables):
        if status[i] != DONE:
            continue
        me = kinds.index(LEARNER)
        mine = scores[i][me]
        rest = [scores[i][s] for s in range(n) if s != me]
        duels += [1.0 if mine > o else 0.5 if mine == o else 0.0 for o in rest]
        wins.append(0.0 if mine < max(rest) else 1 / (1 + rest.count(mine)))
        fair.append(1 / n)
        margins.append(mine - float(np.mean(rest)))

    def mean(xs):
        return float(np.mean(xs)) if len(xs) else 0.0

    return {"elo": round(rate_against({target: rating}, [(target, d) for d in duels]), 1), "games": games,
            f"vs_{target}": round(mean(duels), 3), "win_rate": round(mean(wins), 3),
            "fair_rate": round(mean(fair), 3), "margin": round(mean(margins), 2)}


def evaluate_exploit(env: BatchEnv, net, target_net, device: str, rng, games: int, target: str,
                     rating: float, target_greedy: bool = True) -> dict:
    """Plays `net` greedily against copies of one anchor and rates it against that anchor alone.

    Every table seats `net` once and gives all other seats to the anchor, which plays its best action
    (the way the ladder rates it) unless `target_greedy` is off, when it samples from its policy.
    `elo` is the anchor's `rating` plus the gap the results imply, so it only means something next to
    that anchor. `win_rate` is the share of games `net` wins (a shared first place splits the win);
    equal players would win `fair_rate`. `margin` is the score points `net` finishes ahead of the
    average opponent. Games that stall are dropped.
    """
    roller = Roller(env, lambda: exploit_table(rng, 1), device, rng, limit=games, record=False)
    _, status, _, scores, tables = roller.collect({0: net, 1: target_net}, mode="sample",
                                                  greedy_slots={0, 1} if target_greedy else {0})
    roller.close()
    return exploit_stats(tables, status, scores, target, rating, games)


def rate(checkpoint: str, games: int, exclude: str | None = None, seed: int = 7, threads: int = 12) -> dict:
    """Rates a checkpoint against the current ladder, leaving out the anchor `exclude`. Stores nothing."""
    device = "cuda" if torch.cuda.is_available() else "cpu"
    ladder = load_ladder()
    ladder.pop(exclude, None)  # re-adding rates the new net against the others, not against its old self
    nets = load_nets(device, ladder)
    net = load_checkpoint(checkpoint, device).eval()
    env = BatchEnv(256, threads, pinned=device == "cuda")
    return evaluate(env, net, device, np.random.default_rng(seed), games, ladder, nets)


def duel(checkpoint: str, target: str, games: int, sample_target: bool = False, seed: int = 7,
         threads: int = 12) -> dict:
    """Plays a checkpoint alone against copies of the anchor `target`; see `evaluate_exploit`."""
    device = "cuda" if torch.cuda.is_available() else "cpu"
    ladder = load_ladder()
    if target not in net_names(ladder):
        raise SystemExit(f"{target!r} is not an anchor net; the anchors are {net_names(ladder)}")
    target_net = load_checkpoint(str(DIR / f"{target}.pt"), device).eval()
    net = load_checkpoint(checkpoint, device).eval()
    env = BatchEnv(256, threads, pinned=device == "cuda")
    return evaluate_exploit(env, net, target_net, device, np.random.default_rng(seed), games, target,
                            ladder[target], target_greedy=not sample_target)


def add(name: str, checkpoint: str, games: int, seed: int = 7, threads: int = 12) -> dict:
    """Rates a checkpoint against the current ladder and stores it as an anchor."""
    if name in SCRIPTED:
        raise SystemExit(f"{name!r} is a scripted bot")
    res = rate(checkpoint, games, exclude=name, seed=seed, threads=threads)
    ladder = load_ladder()
    DIR.mkdir(exist_ok=True)
    shutil.copyfile(checkpoint, DIR / f"{name}.pt")
    ladder[name] = res["elo"]
    LADDER.write_text(json.dumps(dict(sorted(ladder.items())), indent=2) + "\n")
    return res


def main() -> None:
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("add")
    a.add_argument("name")
    a.add_argument("checkpoint")
    a.add_argument("--games", type=int, default=3000)
    r = sub.add_parser("rate")
    r.add_argument("checkpoint")
    r.add_argument("--games", type=int, default=3000)
    r.add_argument("--seed", type=int, default=7)
    d = sub.add_parser("duel")
    d.add_argument("checkpoint")
    d.add_argument("target")
    d.add_argument("--games", type=int, default=3000)
    d.add_argument("--seed", type=int, default=7)
    d.add_argument("--sample-target", action="store_true", help="the anchor samples from its policy instead of playing its best action")
    sub.add_parser("list")
    args = ap.parse_args()
    if args.cmd == "add":
        print(add(args.name, args.checkpoint, args.games))
    elif args.cmd == "rate":
        print(rate(args.checkpoint, args.games, seed=args.seed))
    elif args.cmd == "duel":
        print(duel(args.checkpoint, args.target, args.games, args.sample_target, seed=args.seed))
    for n, v in sorted(load_ladder().items(), key=lambda kv: kv[1]):
        print(f"{v:8.1f}  {n}{'  (net)' if n in net_names() else ''}")


if __name__ == "__main__":
    main()
