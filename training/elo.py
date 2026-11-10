"""Fits ELO ratings to arena results.

Every game at a mixed table gives pairwise results between the bots that sat there (higher score
wins, equal scores count half). A Bradley-Terry model is fitted jointly over all pairs, so the
ratings are consistent across tables of any size. One rating point follows the ELO scale: a gap of
400 means the stronger bot wins 10 of 11 pairwise duels.

    python -m training.elo games.jsonl --anchor random [--bootstrap 200]
"""

from __future__ import annotations

import argparse
import json
import math
from collections import defaultdict

import numpy as np

SCALE = 400 / math.log(10)


def load(path: str) -> list[dict]:
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def pair_counts(games: list[dict], names: list[str]) -> tuple[np.ndarray, np.ndarray]:
    """Returns (wins, matches): wins[a, b] counts a's wins over b, ties as one half."""
    index = {n: i for i, n in enumerate(names)}
    k = len(names)
    wins = np.zeros((k, k))
    matches = np.zeros((k, k))
    for g in games:
        seats, scores = g["seats"], g["scores"]
        for i in range(len(seats)):
            for j in range(i + 1, len(seats)):
                a, b = index[seats[i]], index[seats[j]]
                if a == b:
                    continue
                matches[a, b] += 1
                matches[b, a] += 1
                if scores[i] > scores[j]:
                    wins[a, b] += 1
                elif scores[i] < scores[j]:
                    wins[b, a] += 1
                else:
                    wins[a, b] += 0.5
                    wins[b, a] += 0.5
    return wins, matches


def fit(wins: np.ndarray, matches: np.ndarray, iterations: int = 5000) -> np.ndarray:
    """Bradley-Terry strengths by minorize-maximize, as log-strengths centred on zero."""
    k = len(wins)
    p = np.ones(k)
    total = wins.sum(axis=1) + 0.5  # a half win keeps a bot that never wins finite
    for _ in range(iterations):
        denom = matches / (p[:, None] + p[None, :] + 1e-12)
        new = total / (denom.sum(axis=1) + 1e-12)
        new /= np.exp(np.log(new).mean())
        done = np.abs(np.log(new) - np.log(p)).max() < 1e-9
        p = new
        if done:
            break
    return np.log(p)


def ratings(
    games: list[dict], anchor: str = "random", anchor_value: float = 0.0
) -> dict[str, float]:
    names = sorted({s for g in games for s in g["seats"]})
    wins, matches = pair_counts(games, names)
    r = fit(wins, matches) * SCALE
    base = r[names.index(anchor)] if anchor in names else r.min()
    return {n: float(r[i] - base + anchor_value) for i, n in enumerate(names)}


def rate_against(
    anchors: dict[str, float], results: list[tuple[str, float]], prior: float = 0.5
) -> float:
    """Rating of one bot from its duels against bots of known rating.

    `results` holds (opponent, outcome) with outcome 1 for a win, 0.5 for a tie, 0 for a loss. A
    half win and half loss against each anchor's mean rating keep the answer finite.
    """
    opp = np.array([anchors[o] for o, _ in results] + [float(np.mean(list(anchors.values())))] * 2)
    out = np.array([x for _, x in results] + [prior, 1 - prior])
    lo, hi = -4000.0, 4000.0
    for _ in range(80):
        mid = (lo + hi) / 2
        p = 1 / (1 + np.exp(-(mid - opp) / SCALE))
        if (out - p).sum() > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def bootstrap(
    games: list[dict], anchor: str, rounds: int, seed: int = 0
) -> dict[str, float]:
    """Standard error of each rating, from resampling the games."""
    rng = np.random.default_rng(seed)
    samples = defaultdict(list)
    for _ in range(rounds):
        pick = rng.integers(0, len(games), len(games))
        for n, v in ratings([games[i] for i in pick], anchor).items():
            samples[n].append(v)
    return {n: float(np.std(v)) for n, v in samples.items()}


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("games")
    ap.add_argument("--anchor", default="random")
    ap.add_argument("--bootstrap", type=int, default=0)
    args = ap.parse_args()
    games = load(args.games)
    r = ratings(games, args.anchor)
    err = bootstrap(games, args.anchor, args.bootstrap) if args.bootstrap else {}
    for n, v in sorted(r.items(), key=lambda kv: -kv[1]):
        pm = f" ± {err[n]:.0f}" if n in err else ""
        print(f"{n:<20}{v:8.0f}{pm}")


if __name__ == "__main__":
    main()
