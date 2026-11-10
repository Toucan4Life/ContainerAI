"""Trains a neural opponent within a time budget.

    python -m training.train --name first --budget-minutes 30

Stage 1 copies the merchant (imitation with DAgger-style relabelling), stage 2 improves the net by
league self-play with PPO: tables mix the learner with itself, scripted bots and frozen earlier
checkpoints. Everything lands in `runs/<name>/`: `metrics.jsonl` (what the dashboard reads),
`status.json`, `ckpt_*.pt`, and at the end `final.pt` and `weights.bin` for the Rust bot.
"""

from __future__ import annotations

import argparse
import copy
import json
import os
import time
from pathlib import Path

os.environ.setdefault("PYTORCH_CUDA_ALLOC_CONF", "expandable_segments:True")  # rollouts vary in size

import numpy as np  # noqa: E402
import torch  # noqa: E402
from torch.nn import functional as F

from training import anchors as anchor_lib
from training import ladder
from training.env import DONE, MERCHANT, RANDOM, SIMPLE, STALLED, BatchEnv
from training.model import PolicyNet, export_weights, save_checkpoint
from training.selfplay import LEARNER, STALL_REWARD, Roller, advantages, reward, table_stats

SCRIPTED = [MERCHANT, SIMPLE, RANDOM]
KIND_NAME = {MERCHANT: "merchant", SIMPLE: "simple", RANDOM: "random"}


class Run:
    def __init__(self, root: Path, budget: float):
        self.dir = root
        self.dir.mkdir(parents=True, exist_ok=True)
        self.t0 = time.monotonic()
        self.budget = budget
        self.metrics = open(self.dir / "metrics.jsonl", "a", buffering=1)

    def elapsed(self) -> float:
        return time.monotonic() - self.t0

    def log(self, **row) -> None:
        row["t"] = round(self.elapsed(), 2)
        self.metrics.write(json.dumps(row) + "\n")

    def status(self, state: str, stage: str, **extra) -> None:
        body = dict(state=state, stage=stage, elapsed=round(self.elapsed(), 1), budget=self.budget, **extra)
        (self.dir / "status.json").write_text(json.dumps(body))


seed_of = anchor_lib.seed_of


def imitation_table(rng):
    players = int(rng.choice([3, 4, 5]))
    if rng.random() < 0.6:
        kinds = [LEARNER] * players
    else:
        kinds = [int(rng.choice(SCRIPTED)) for _ in range(players)]
        for s in rng.choice(players, size=int(rng.integers(1, 3)), replace=False):
            kinds[s] = LEARNER
    return players, kinds, seed_of(rng)


def league_table(rng, frozen: list[int]):
    players = int(rng.choice([3, 4, 5]))
    if rng.random() < 0.25:
        return players, [LEARNER] * players, seed_of(rng)
    learners = rng.choice(players, size=int(rng.integers(1, players)), replace=False)
    kinds = []
    for s in range(players):
        if s in learners:
            kinds.append(LEARNER)
        elif frozen and rng.random() < 0.45:
            kinds.append(int(rng.choice(frozen)))
        else:
            kinds.append(int(rng.choice(SCRIPTED, p=[0.6, 0.3, 0.1])))
    return players, kinds, seed_of(rng)


def ppo_update(net, opt, data, adv, ret, epochs=4, minibatch=8192, clip=0.2, ent_coef=0.01, vf_coef=0.5):
    n = len(adv)
    adv_t = torch.from_numpy(adv).float().to(data["state"].device)
    ret_t = torch.from_numpy(ret).float().to(data["state"].device)
    adv_t = (adv_t - adv_t.mean()) / (adv_t.std() + 1e-8)
    stats = dict(loss_pi=0.0, loss_v=0.0, entropy=0.0, kl=0.0)
    steps = 0
    for _ in range(epochs):
        perm = torch.randperm(n, device=adv_t.device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            logits, value = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            logp_all = torch.log_softmax(logits, dim=1)
            logp = logp_all.gather(1, data["action"][ix, None]).squeeze(1)
            ratio = torch.exp(logp - data["logp"][ix])
            a = adv_t[ix]
            pi_loss = -torch.min(ratio * a, torch.clamp(ratio, 1 - clip, 1 + clip) * a).mean()
            v_loss = F.mse_loss(value, ret_t[ix])
            entropy = -(logp_all.exp() * logp_all).sum(dim=1).mean()
            loss = pi_loss + vf_coef * v_loss - ent_coef * entropy
            opt.zero_grad(set_to_none=True)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(net.parameters(), 0.5)
            opt.step()
            stats["loss_pi"] += pi_loss.item()
            stats["loss_v"] += v_loss.item()
            stats["entropy"] += entropy.item()
            stats["kl"] += (data["logp"][ix] - logp).mean().item()
            steps += 1
    return {k: round(v / steps, 5) for k, v in stats.items()}


def imitate(net, opt, data, epochs=2, minibatch=4096):
    n = len(data["state"])
    total, correct, count = 0.0, 0, 0
    for _ in range(epochs):
        perm = torch.randperm(n, device=data["state"].device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            logits, _ = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            loss = F.cross_entropy(logits, data["label"][ix])
            opt.zero_grad(set_to_none=True)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(net.parameters(), 1.0)
            opt.step()
            total += loss.item() * len(ix)
            correct += (logits.argmax(dim=1) == data["label"][ix]).sum().item()
            count += len(ix)
    return {"loss_ce": round(total / count, 5), "accuracy": round(correct / count, 4)}


def value_fit(net, data, ret, opt, epochs=1, minibatch=8192):
    """Teaches the value head from game outcomes while the policy is still being imitated."""
    ret_t = torch.from_numpy(ret).float().to(data["state"].device)
    n = len(ret_t)
    for _ in range(epochs):
        perm = torch.randperm(n, device=ret_t.device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            _, value = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            loss = F.mse_loss(value, ret_t[ix])
            opt.zero_grad(set_to_none=True)
            loss.backward()
            opt.step()


def outcome_returns(rec, status, players, scores):
    """Return of every learner decision: the seat's final reward (no bootstrapping)."""
    env_ids = np.concatenate(rec.env)
    seats = np.concatenate(rec.seat)
    keep = np.array([status[e] in (DONE, STALLED) for e in env_ids])
    ret = np.zeros(len(env_ids))
    cache = {}
    for k, (e, s) in enumerate(zip(env_ids, seats)):
        if keep[k]:
            if status[e] == STALLED:
                ret[k] = STALL_REWARD
                continue
            if e not in cache:
                cache[e] = reward(scores[e], int(players[e]))
            ret[k] = cache[e][s]
    return ret, keep


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--name", default=time.strftime("run-%Y%m%d-%H%M%S"))
    ap.add_argument("--budget-minutes", type=float, default=30.0)
    ap.add_argument("--hidden", type=int, default=256)
    ap.add_argument("--envs", type=int, default=256)
    ap.add_argument("--threads", type=int, default=12)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--init", help="checkpoint to start from (skips imitation if --no-imitation)")
    ap.add_argument("--no-imitation", action="store_true")
    ap.add_argument("--imitation-share", type=float, default=0.17)
    ap.add_argument("--eval-seconds", type=float, default=90.0)
    ap.add_argument("--runs", default="runs")
    ap.add_argument("--lr", type=float, default=3e-4)
    ap.add_argument("--exploit", metavar="ANCHOR",
                    help="train a best-response exploiter of this anchor net: one seat for the learner, "
                         "every other seat plays the anchor (starts from its weights, skips imitation)")
    ap.add_argument("--target-mode", choices=["greedy", "sample"], default="greedy",
                    help="how the exploited anchor plays in training; ladder ratings are greedy")
    args = ap.parse_args()
    if args.exploit:
        if args.exploit not in anchor_lib.net_names():
            raise SystemExit(f"--exploit needs a rated anchor net, one of {anchor_lib.net_names()}")
        args.no_imitation = True
        args.init = args.init or str(anchor_lib.DIR / f"{args.exploit}.pt")

    device = "cuda" if torch.cuda.is_available() else "cpu"
    torch.backends.cuda.matmul.allow_tf32 = True
    rng = np.random.default_rng(args.seed)
    torch.manual_seed(args.seed)
    budget = args.budget_minutes * 60
    run = Run(Path(args.runs) / args.name, budget)
    (run.dir / "config.json").write_text(json.dumps(vars(args) | {"device": device}, indent=2))
    anchors = ladder.load()
    anchor_nets = anchor_lib.load_nets(device, anchors)
    anchor_slots = {anchor_lib.NET_SLOT_BASE + i: n for i, n in enumerate(anchor_nets.values())}  # slot -> net
    if args.exploit:
        target_slot = anchor_lib.NET_SLOT_BASE + list(anchor_nets).index(args.exploit)
        anchor_slots = {target_slot: anchor_nets[args.exploit]}
    stage_name = "exploit" if args.exploit else "selfplay"
    print(f"ladder: {anchors}  (net anchors: {list(anchor_nets)})", flush=True)
    env = BatchEnv(args.envs, args.threads, pinned=device == "cuda")
    eval_env = BatchEnv(256, args.threads, pinned=device == "cuda")  # evaluation must not disturb the games carried over in `env`

    net = PolicyNet(env.state_dim, env.action_dim, args.hidden).to(device)
    if args.init:
        from training.model import load_checkpoint
        net = load_checkpoint(args.init, device)
    games = decisions = 0
    ckpts: list[tuple[str, float]] = []
    frozen: dict[int, torch.nn.Module] = {}
    last_eval = -1e9

    def checkpoint(stage: str, big: bool = False) -> dict:
        net.eval()
        n_games = 1024 if big else 256
        res = anchor_lib.evaluate(eval_env, net, device, rng, n_games, anchors, anchor_nets)
        if args.exploit:
            # `elo` stays the ladder rating, which is the strength that counts; the duel only says how
            # well the target is beaten when every other seat plays it.
            duel = anchor_lib.evaluate_exploit(eval_env, net, anchor_nets[args.exploit], device, rng, n_games,
                                               args.exploit, anchors[args.exploit])
            res |= {f"duel_{k}": v for k, v in duel.items() if k not in ("elo", "games")}
        net.train()
        name = f"ckpt_{len(ckpts):03d}.pt"
        save_checkpoint(net, str(run.dir / name))
        ckpts.append((name, res["elo"]))
        run.log(type="eval", stage=stage, ckpt=name, games_trained=games, decisions=decisions, **res)
        print(f"[{run.elapsed():6.0f}s] eval {stage}: {res}", flush=True)
        return res

    imitation_end = budget * args.imitation_share
    reserve = budget * 0.08
    it = 0

    if not args.no_imitation:
        opt = torch.optim.Adam(net.parameters(), lr=1e-3)
        roller = Roller(env, lambda: imitation_table(rng), device, rng)
        while run.elapsed() < imitation_end:
            run.status("running", "imitation", games=games, decisions=decisions)
            t = time.monotonic()
            rec = data = None
            rec, status, players, scores, _ = roller.collect({0: net}, target=env.n, mode="teacher", beta=0.3)
            ret, keep = outcome_returns(rec, status, players, scores)
            data, _, _ = rec.tensors(keep)
            stats = imitate(net, opt, data)
            value_fit(net, data, ret[keep], opt)
            games += int(sum(status == DONE))
            decisions += len(data["state"])
            run.log(type="train", stage="imitation", iter=it, games=games, decisions=decisions,
                    games_per_sec=round(len(status) / (time.monotonic() - t), 1), **stats)
            it += 1
            if run.elapsed() - last_eval > args.eval_seconds:
                checkpoint("imitation")
                last_eval = run.elapsed()
        checkpoint("imitation")
        last_eval = run.elapsed()
        roller.close()
        del roller

    opt = torch.optim.Adam(net.parameters(), lr=args.lr)
    ppo_end = budget - reserve
    snapshot_at = run.elapsed()
    if args.exploit:
        def next_table():
            return anchor_lib.exploit_table(rng, target_slot)
    else:
        def next_table():
            return league_table(rng, list(frozen) + list(anchor_slots))
    roller = Roller(env, next_table, device, rng)
    greedy = set(anchor_slots) if args.exploit and args.target_mode == "greedy" else set()
    while run.elapsed() < ppo_end:
        run.status("running", stage_name, games=games, decisions=decisions,
                   elo=ckpts[-1][1] if ckpts else None)
        t = time.monotonic()
        rec = data = None  # last iteration's tensors would otherwise stay on the GPU during the rollout
        policies = {0: net, **frozen, **anchor_slots}
        net.eval()
        rec, status, players, scores, tables = roller.collect(policies, target=env.n, mode="sample",
                                                              greedy_slots=greedy)
        net.train()
        env_ids, seats = np.concatenate(rec.env), np.concatenate(rec.seat)
        adv, ret, keep = advantages(rec, env_ids, seats, status, players, scores)
        data, _, _ = rec.tensors(keep)
        mean_reward, finished = table_stats(status, players, scores, tables)
        stats = ppo_update(net, opt, data, adv[keep], ret[keep])
        games += finished
        decisions += len(data["state"])
        run.log(type="train", stage=stage_name, iter=it, games=games, decisions=decisions,
                mean_reward=round(mean_reward, 4), frozen=len(frozen), stalled=int((status == STALLED).sum()),
                games_per_sec=round(len(status) / (time.monotonic() - t), 1), **stats)
        it += 1
        if run.elapsed() - last_eval > args.eval_seconds:
            res = checkpoint(stage_name)
            last_eval = run.elapsed()
            if not args.exploit:
                # Freeze the net as a future opponent; keep the most recent few.
                slot = max(frozen, default=0) + 1
                frozen[slot] = copy.deepcopy(net).eval()
                if len(frozen) > 5:
                    del frozen[min(frozen)]

    final = checkpoint("final", big=True)
    save_checkpoint(net, str(run.dir / "final.pt"))
    export_weights(net, str(run.dir / "weights.bin"))
    run.log(type="done", reason="budget", games_trained=games, decisions=decisions, **final)
    run.status("finished", "done", games=games, decisions=decisions, elo=final["elo"], reason="budget")
    print(f"finished after {run.elapsed():.0f}s: {final}")


if __name__ == "__main__":
    main()
