"""Rates the scripted bots and keeps the result in `training/ladder.json`, the fixed anchors that
every neural checkpoint is rated against.

    python -m training.ladder [--games 6000]
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import numpy as np

from training.elo import ratings
from training.env import DONE, MERCHANT, RANDOM, SIMPLE, BatchEnv

LADDER = Path(__file__).with_name("ladder.json")
NAMES = {RANDOM: "random", SIMPLE: "simple", MERCHANT: "merchant"}


def play_tables(games: int, seed: int = 1) -> list[dict]:
    rng = np.random.default_rng(seed)
    out = []
    env = BatchEnv(512, threads=12)
    done = 0
    while done < games:
        tables = []
        for i in range(env.n):
            players = int(rng.choice([3, 4, 5]))
            kinds = [int(rng.choice([RANDOM, SIMPLE, MERCHANT])) for _ in range(players)]
            tables.append(kinds)
            env.reset(i, players, int(rng.integers(1 << 40)), kinds)
        env.advance()
        status, nplayers, scores = env.results()
        for i, kinds in enumerate(tables):
            if status[i] == DONE:
                n = len(kinds)
                out.append({"seats": [NAMES[k] for k in kinds], "scores": scores[i, :n].tolist()})
        done += env.n
    return out


def load() -> dict[str, float]:
    return json.loads(LADDER.read_text())


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--games", type=int, default=6000)
    args = ap.parse_args()
    r = ratings(play_tables(args.games), "random")
    LADDER.write_text(json.dumps({k: round(v, 1) for k, v in r.items()}, indent=2) + "\n")
    print(LADDER.read_text())


if __name__ == "__main__":
    main()
