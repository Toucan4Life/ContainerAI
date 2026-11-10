"""Times the parts of one self-play iteration.

    python -m training.profile_iter [--envs 256] [--frozen 3] [--reps 2]
"""

from __future__ import annotations

import argparse
import copy
import cProfile
import pstats
import time

import numpy as np
import torch

from training.env import BatchEnv
from training.model import PolicyNet
from training.selfplay import Roller, advantages, table_stats
from training.train import league_table, ppo_update


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--envs", type=int, default=256)
    ap.add_argument("--hidden", type=int, default=256)
    ap.add_argument("--frozen", type=int, default=3)
    ap.add_argument("--reps", type=int, default=2)
    ap.add_argument("--threads", type=int, default=12)
    ap.add_argument("--cprofile", action="store_true")
    args = ap.parse_args()
    dev = "cuda"
    torch.backends.cuda.matmul.allow_tf32 = True
    rng = np.random.default_rng(1)
    env = BatchEnv(args.envs, args.threads, pinned=True)
    net = PolicyNet(env.state_dim, env.action_dim, args.hidden).to(dev)
    opt = torch.optim.Adam(net.parameters(), lr=3e-4)
    frozen = {i + 1: copy.deepcopy(net).eval() for i in range(args.frozen)}

    roller = Roller(env, lambda: league_table(rng, list(frozen)), dev, rng)

    def one():
        t0 = time.perf_counter()
        net.eval()
        rec, status, players, scores, tables = roller.collect({0: net, **frozen}, target=env.n, mode="sample")
        net.train()
        torch.cuda.synchronize()
        t1 = time.perf_counter()
        ids, seats = np.concatenate(rec.env), np.concatenate(rec.seat)
        adv, ret, keep = advantages(rec, ids, seats, status, players, scores)
        t2 = time.perf_counter()
        data, _, _ = rec.tensors(keep)
        _, finished = table_stats(status, players, scores, tables)
        ppo_update(net, opt, data, adv[keep], ret[keep])
        torch.cuda.synchronize()
        t3 = time.perf_counter()
        total = t3 - t0
        print(f"finished {len(status)} (learner seats {finished})  decisions {len(ids)}  play {t1-t0:.2f}s  advantages {t2-t1:.2f}s  "
              f"update {t3-t2:.2f}s  total {total:.2f}s  => {len(status)/total:.1f} games/s")

    one()  # warm-up: also lets the in-flight games reach a steady mix
    one()
    if args.cprofile:
        pr = cProfile.Profile()
        pr.enable()
    for _ in range(args.reps):
        one()
    if args.cprofile:
        pr.disable()
        pstats.Stats(pr).sort_stats("tottime").print_stats(14)


if __name__ == "__main__":
    main()
