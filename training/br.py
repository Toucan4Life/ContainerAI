"""Approximate best-response oracle: PPO against a fixed opponent distribution, and a paired deviation study.

A *profile* is a distribution over the strategies of a population (`training.population`). The oracle
trains a learner that takes one seat while every other seat plays a strategy drawn independently from the
profile. The study then measures how much a seat gains by deviating from the profile to a candidate, on
seeds the training never saw. Utilities are the canonical ones of `evalcore` (a winner takes 1/|W|, a
stalled game pays nothing) whatever objective the learner trains on.

    python -m training.br train --name m1 --init populations/baselines/run2.pt --profile run2,magnate,exploit_run2
    python -m training.br eval --candidates m1=runs/br/m1/final.pt --profile run2,magnate,exploit_run2 --block holdout

The deviation gain of a candidate at n players is e = u(candidate; sigma^(n-1)) - u(sigma; sigma^(n-1)),
where sigma^(n-1) are the other seats drawn iid from the profile and u(sigma; .) is the profile's own
value, the mean of the pure values it mixes. Every focal strategy is played on the same seeds against the
same opponents (common random numbers), so differences are paired. Summed over the n seats, the gains are
the NashConv of the profile restricted to the candidates tried. A trained candidate is only an approximate
best response, so its gain is a *lower bound* on the true deviation gain: a small gain proves nothing about
equilibrium unless the oracle has been shown to find improvements (see the oracle validation report).
"""

from __future__ import annotations

import argparse
import json
import time
from pathlib import Path

import numpy as np
import torch

from training import evalcore as E
from training.env import DONE, LENGTH_NAMES, MAX_PLAYERS, STANDARD, BatchEnv
from training.model import load_checkpoint, save_checkpoint
from training.population import Population, sha256_file
from training.selfplay import LEARNER, Roller, advantages, reward
from training.train import Run, ppo_update, seed_of

DEFAULT_POPULATION = E.ROOT / "populations/baselines/manifest.json"
UTILITIES = ("canonical", "shaped")
BLOCKS = {"dev": E.DEV, "holdout": E.HOLDOUT, "audit": E.AUDIT}
LENGTHS = {name: k for k, name in LENGTH_NAMES.items()}


# --------------------------------------------------------------------------------------- profiles


def _normalised(weights) -> np.ndarray:
    w = np.asarray(weights, dtype=np.float64)
    if (w < 0).any() or w.sum() <= 0:
        raise SystemExit("profile weights must be non-negative and not all zero")
    return w / w.sum()


def parse_profile(text: str, pop: Population):
    """`run2,magnate:2` -> strategy names and normalised weights.

    `@file.json` reads {"support": [names], "weights": [..] or {"3": [..], "4": [..], "5": [..]}}; weights
    given per player count make the profile depend on the table size (the weights are then a dict by n).
    """
    if text.startswith("@"):
        spec = json.loads(Path(text[1:]).read_text())
        names, raw = list(spec["support"]), spec["weights"]
        unknown = [s for s in names if s not in pop.strategies]
        if unknown:
            raise SystemExit(f"unknown strategies {unknown}; the population has {list(pop.strategies)}")
        if isinstance(raw, dict):
            return names, {int(n): _normalised(w) for n, w in raw.items()}
        return names, _normalised(raw)
    names, weights = [], []
    for part in text.split(","):
        name, _, w = part.strip().partition(":")
        if name not in pop.strategies:
            raise SystemExit(f"unknown strategy {name!r}; the population has {list(pop.strategies)}")
        names.append(name)
        weights.append(float(w) if w else 1.0)
    return names, _normalised(weights)


def weights_at(weights, n: int) -> np.ndarray:
    """The profile's weights at a table of n players."""
    return weights[n] if isinstance(weights, dict) else weights


def weights_json(weights):
    if isinstance(weights, dict):
        return {str(n): [float(x) for x in w] for n, w in sorted(weights.items())}
    return [float(x) for x in weights]


def terminal_payoff(kind: str, status, players, scores, winners) -> np.ndarray:
    """The outcome per finished game and seat that the learner is trained on.

    `canonical` is the utility of the equilibrium spec (1/|W| for a winner); `shaped` is the trainer's
    blend of win fraction and score margin. A game that did not finish pays 0 under both.
    """
    out = np.zeros((len(status), MAX_PLAYERS))
    for g in np.nonzero(status == DONE)[0]:
        n = int(players[g])
        if kind == "shaped":
            out[g, :n] = reward(scores[g], n)
        else:
            won = (int(winners[g]) >> np.arange(n)) & 1
            out[g, :n] = won / max(int(won.sum()), 1)
    return out


def table_sampler(rng, players, lengths, kind_of, weights):
    """Tables of the training: the learner at a random seat, the others drawn iid from the profile."""

    def next_table():
        n = int(rng.choice(players))
        focal = int(rng.integers(n))
        draw = rng.choice(len(kind_of), size=n, p=weights_at(weights, n))
        kinds = [LEARNER if s == focal else kind_of[int(draw[s])] for s in range(n)]
        return n, kinds, seed_of(rng), int(rng.choice(lengths))

    return next_table


# --------------------------------------------------------------------------------------- the study


def _boot_mean(groups: list[np.ndarray], reps: int = 2000, alpha: float = 0.05, seed: int = 0):
    """Mean over groups of each group's mean, with a percentile interval from resampling within groups."""
    rng = np.random.default_rng(seed)
    est = np.zeros(reps)
    for g in groups:
        est += g[rng.integers(0, len(g), size=(reps, len(g)))].mean(axis=1)
    est /= len(groups)
    mean = float(np.mean([g.mean() for g in groups]))
    return [mean, float(np.quantile(est, alpha / 2)), float(np.quantile(est, 1 - alpha / 2))]


def study_specs(sup_ix: list[int], weights: np.ndarray, n_focal: int, *, block: int, seeds: int, players: list[int],
                length: int, seed_offset: int = 0) -> list[E.GameSpec]:
    """Games of a deviation study. Every focal plays every (seed, player count, seat) against the same draw
    of opponents, so the focals' values differ only by what the focal plays. `cell` is `focal * 8 + seat`."""
    specs = []
    for n in players:
        for s in range(seed_offset, seed_offset + seeds):
            gseed = E.game_seed(block, n, length, s)
            for p in range(n):
                rng = np.random.default_rng([block >> 50, n, s, p])
                others = [sup_ix[j] for j in rng.choice(len(sup_ix), size=n - 1, p=weights_at(weights, n))]
                for f in range(n_focal):
                    specs.append(E.GameSpec(n, length, gseed, tuple(others[:p] + [f] + others[p:]), f * 8 + p, gseed))
    return specs


def deviation_study(pop: Population, support: list[str], weights: np.ndarray, candidates: list[E.Policy], *,
                    block: int, seeds: int, players: list[int], length: int = STANDARD, seed_offset: int = 0,
                    focals: list[str] | None = None, envs: int = 256, device: str | None = None, log=print,
                    torch_seed: int = 0, nets: dict | None = None) -> dict:
    """Plays every focal strategy at every seat against opponents drawn from the profile.

    Focals are the population's strategies (pure deviations, `focals` or all) and the candidates. For every
    seed, player count and seat, the opponents are one draw from the profile, shared by all focals (see
    `study_specs`). The result holds, per player count, the utility of every focal, the value of the
    profile and each focal's gain over it, with seed-clustered 95% intervals.
    """
    names = list(dict.fromkeys([*(focals if focals is not None else pop.strategies), *support]))
    policies = pop.policies(names) + list(candidates)
    labels = names + [c.name for c in candidates]
    sup_ix = [names.index(s) for s in support]
    n_focal = len(policies)
    specs = study_specs(sup_ix, weights, n_focal, block=block, seeds=seeds, players=players, length=length,
                        seed_offset=seed_offset)
    rec = E.play(specs, policies, envs=envs, device=device, seed=torch_seed, log=log, nets=nets)

    util = E.winner_share(rec)
    focal_of, seat_of = rec.cell // 8, rec.cell % 8
    by_players: dict = {}
    per_n: dict[int, np.ndarray] = {}
    for n in players:
        rows = np.nonzero(rec.players == n)[0]
        s_local = rec.seed[rows].astype(np.int64) - E.game_seed(block, n, length, seed_offset)
        u = np.full((n_focal, seeds, n), np.nan)
        u[focal_of[rows], s_local, seat_of[rows]] = util[rows, seat_of[rows]]
        stalled = np.zeros(n_focal, dtype=np.int64)
        np.add.at(stalled, focal_of[rows], rec.stalled[rows].astype(np.int64))
        sigma = sum(w * u[j] for j, w in zip(sup_ix, weights_at(weights, n)))
        gain = u - sigma[None]
        per_n[n] = gain
        first = E.first_seat(np.full(seeds, n), np.full(seeds, length), E.game_seed(block, n, length, seed_offset) + np.arange(seeds))
        seat_at = (first[:, None] + np.arange(n)[None, :]) % n  # engine seat holding each turn position, per seed
        turn_gain = np.take_along_axis(gain, seat_at[None], axis=2)
        by_players[str(n)] = dict(
            games=len(rows), stalled=dict(zip(labels, stalled.tolist())),
            sigma=_boot_mean([sigma.mean(axis=1)]),
            utility={lab: _boot_mean([u[f].mean(axis=1)]) for f, lab in enumerate(labels)},
            gain={lab: _boot_mean([gain[f].mean(axis=1)]) for f, lab in enumerate(labels)},
            nashconv={lab: _boot_mean([n * gain[f].mean(axis=1)]) for f, lab in enumerate(labels)},
            by_seat={lab: [_boot_mean([gain[f][:, p]]) for p in range(n)] for f, lab in enumerate(labels)},
            by_turn={lab: [_boot_mean([turn_gain[f][:, t]]) for t in range(n)] for f, lab in enumerate(labels)},
            per_seed_gain={lab: np.round(gain[f].mean(axis=1), 6).tolist() for f, lab in enumerate(labels)},
        )
    mean_over_players = dict(
        gain={lab: _boot_mean([per_n[n][f].mean(axis=1) for n in players]) for f, lab in enumerate(labels)},
        nashconv={lab: _boot_mean([n * per_n[n][f].mean(axis=1) for n in players]) for f, lab in enumerate(labels)},
    )
    return dict(
        profile=dict(support=support, weights=weights_json(weights)), block=int(block), seeds=seeds,
        seed_offset=seed_offset, players=list(players), length=length, focals=labels,
        candidates=[c.name for c in candidates], by_players=by_players, mean_over_players=mean_over_players,
        accounting=E.accounting(rec), policies=rec.meta["policies"], git=rec.meta["git"],
    )


def _ci(x: list[float], digits: int = 3) -> str:
    return f"{x[0]:+.{digits}f} [{x[1]:+.{digits}f}, {x[2]:+.{digits}f}]"


def render_study(res: dict, title: str = "Deviation study") -> str:
    """The study as markdown."""
    pw = res["profile"]["weights"]
    sup = res["profile"]["support"]
    if isinstance(pw, dict):
        prof = "; ".join(f"{n} players: " + ", ".join(f"{s} {w:.2f}" for s, w in zip(sup, ws)) for n, ws in pw.items())
    else:
        prof = ", ".join(f"{s} {w:.2f}" for s, w in zip(sup, pw))
    acc = res["accounting"]
    block = {v: k for k, v in BLOCKS.items()}.get(res["block"], str(res["block"]))
    lines = [f"# {title}", "",
             f"Profile sigma: {prof}. Every other seat draws its strategy iid from sigma. Seeds: {res['seeds']} per player "
             f"count from the **{block}** block (offset {res['seed_offset']}), {len(res['players'])} player counts, every "
             f"seat of every table, {LENGTH_NAMES.get(res['length'], res['length'])} length; {acc['games']:,} games, "
             f"{acc['stalled']} stalled (horizon {acc['horizon']}, stalled games pay 0). Code version {res['git']}.", "",
             "Utility is the canonical one (a winner takes 1/|W|). `gain` = utility of the focal strategy minus the value "
             "of sigma itself against the same opponents; intervals are 95% seed-clustered bootstrap intervals of the "
             "paired difference. A trained candidate is only an approximate best response: its gain is a lower bound on "
             "the true deviation gain.", ""]
    for n, blk in res["by_players"].items():
        lines += [f"## {n} players", "", f"Value of sigma: {_ci(blk['sigma'])}", "",
                  "| focal | utility | gain over sigma | stalled |", "|---|---|---|---|"]
        for lab in res["focals"]:
            mark = " *" if lab in res["candidates"] else ""
            lines.append(f"| {lab}{mark} | {_ci(blk['utility'][lab])} | {_ci(blk['gain'][lab])} | {blk['stalled'][lab]} |")
        lines.append("")
        if res["candidates"]:
            where = "by_turn" if "by_turn" in blk else "by_seat"
            lines += [("Gain by turn position (candidates; position 0 moves first): " if where == "by_turn" else
                       "Gain by engine seat index (candidates; the first player is drawn at random, so the index is not "
                       "turn order): ") + "; ".join(
                f"{lab}: " + ", ".join(f"{g[0]:+.3f}" for g in blk[where][lab]) for lab in res["candidates"]), "",
                "NashConv (sum of the seats' gains): " + "; ".join(
                    f"{lab}: {_ci(blk['nashconv'][lab])}" for lab in res["candidates"]), ""]
    lines += ["## Mean over player counts", "", "| focal | gain over sigma | NashConv |", "|---|---|---|"]
    for lab in res["focals"]:
        mark = " *" if lab in res["candidates"] else ""
        lines.append(f"| {lab}{mark} | {_ci(res['mean_over_players']['gain'][lab])} | "
                     f"{_ci(res['mean_over_players']['nashconv'][lab])} |")
    lines += ["", "`*` marks trained candidates. Pure strategies of the population are listed to show the best deviation "
              "that needs no training; picking the best of several rows is optimistic, confirm it on a fresh block.", ""]
    for lab in [c for c in res["candidates"] if "@" not in c] or res["candidates"]:  # `@` marks weights saved part-way
        sizes = sorted({k for k in (5, 10, 25, 50, 100, 200) if k < res["seeds"]} | {res["seeds"]})
        lines += [f"## How well the gain of {lab} is measured", "",
                  "Mean over player counts. The half-width is the 95% half-width that this study's per-seed variance "
                  "predicts for k seeds per player count; the spread is the standard deviation of the estimate over "
                  f"random subsets of k of the study's {res['seeds']} seeds (it understates near k = {res['seeds']}).", "",
                  "| seeds per player count | predicted 95% half-width | spread of the estimate |", "|---|---|---|"]
        lines += [f"| {r['seeds']} | ±{r['halfwidth']:.3f} | {r['spread']:.3f} |" for r in reliability(res, lab, sizes)]
        lines.append("")
    return "\n".join(lines)


def reliability(res: dict, label: str, sizes=(5, 10, 25, 50, 100, 200), reps: int = 500, seed: int = 0) -> list[dict]:
    """How well the mean gain of `label` (over player counts) is measured with fewer seeds.

    Per size k: `halfwidth` is the normal 95% half-width the pooled per-seed variance predicts, `spread` the
    standard deviation of the estimate over random subsets of k seeds (it underestimates for k near the
    number of seeds in the study, which is finite).
    """
    pools = [np.asarray(b["per_seed_gain"][label]) for b in res["by_players"].values()]
    rng = np.random.default_rng(seed)
    sd1 = float(np.sqrt(sum(p.var(ddof=1) for p in pools))) / len(pools)  # sd of the estimate from one seed per count
    rows = []
    for k in sizes:
        if k > min(len(p) for p in pools):
            continue
        est = [np.mean([p[rng.choice(len(p), k, replace=False)].mean() for p in pools]) for _ in range(reps)]
        rows.append(dict(seeds=k, halfwidth=1.96 * sd1 / np.sqrt(k), spread=float(np.std(est)), mean=float(np.mean(est))))
    return rows


def paired_difference(res: dict, a: str, b: str, reps: int = 2000, seed: int = 0) -> list[float]:
    """Gain of focal `a` minus gain of focal `b`, mean over player counts, with a 95% seed-clustered interval.

    Both were played on the same seeds against the same opponents, so the seeds' luck cancels in the difference."""
    groups = [np.asarray(blk["per_seed_gain"][a]) - np.asarray(blk["per_seed_gain"][b]) for blk in res["by_players"].values()]
    return _boot_mean(groups, reps=reps, seed=seed)


# --------------------------------------------------------------------------------------- training


def lr_at(base: float, decay: str, elapsed: float, budget: float) -> float:
    """Learning rate after `elapsed` of `budget` seconds: constant, or falling linearly to zero."""
    if decay == "linear":
        return base * max(0.0, 1.0 - elapsed / budget)
    return base


def train(args) -> dict:
    device = "cuda" if torch.cuda.is_available() else "cpu"
    pop = Population.load(args.population)
    support, weights = parse_profile(args.profile, pop)
    players = [int(p) for p in args.players.split(",")]
    lengths = [LENGTHS[x] for x in args.length.split(",")]
    rng = np.random.default_rng(args.seed)
    torch.manual_seed(args.seed)
    torch.backends.cuda.matmul.allow_tf32 = True

    policies = pop.policies(support)
    nets = E.load_nets(policies, device)
    kind_of, net_of, greedy, guarded = E.seat_slots(policies, nets)
    net = load_checkpoint(args.init, device)
    opt = torch.optim.Adam(net.parameters(), lr=args.lr)

    budget = args.budget_minutes * 60
    run = Run(Path(args.runs) / args.name, budget)
    config = vars(args) | dict(device=device, support=support, weights=weights_json(weights), git=E.git_commit(),
                               init_sha256=sha256_file(args.init), horizon=E.HORIZON)
    (run.dir / "config.json").write_text(json.dumps(config, indent=2))

    env = BatchEnv(args.envs, args.threads, pinned=device == "cuda")
    env.set_horizon(E.HORIZON)
    roller = Roller(env, table_sampler(rng, players, lengths, kind_of, weights), device, rng)
    games = decisions = it = 0
    evals: list[dict] = []
    last_eval = run.elapsed()
    n_ckpt = 0

    def checkpoint(final: bool = False) -> dict:
        nonlocal n_ckpt
        path = run.dir / ("final.pt" if final else f"ckpt_{n_ckpt:03d}.pt")
        save_checkpoint(net, str(path))
        n_ckpt += 1
        net.eval()
        label = f"{args.name}@{run.elapsed() / 60:.0f}m"
        rng_state = torch.get_rng_state(), (torch.cuda.get_rng_state() if device == "cuda" else None)
        res = deviation_study(pop, support, weights, [E.Policy(label, str(path.resolve()), "sample")], block=E.DEV,
                              seeds=args.eval_seeds, players=players, length=lengths[0], focals=support,
                              envs=args.eval_envs, device=device, log=None, nets=None, torch_seed=n_ckpt)
        torch.set_rng_state(rng_state[0])  # the study reseeds torch; training goes on with its own stream
        if rng_state[1] is not None:
            torch.cuda.set_rng_state(rng_state[1])
        net.train()
        gain = res["mean_over_players"]["gain"][label]
        row = dict(type="eval", ckpt=path.name, minutes=round(run.elapsed() / 60, 2), games_trained=games,
                   decisions=decisions, gain=gain, by_players={n: b["gain"][label] for n, b in res["by_players"].items()})
        run.log(**row)
        evals.append(row)
        print(f"[{run.elapsed():6.0f}s] {path.name}: gain over sigma {_ci(gain)} (dev, {args.eval_seeds} seeds)", flush=True)
        return res

    while run.elapsed() < budget:
        run.status("running", "best-response", games=games, decisions=decisions)
        t = time.monotonic()
        rec = data = None
        net.eval()
        rec, status, nplayers, scores, tables = roller.collect({LEARNER: net, **net_of}, target=env.n, mode="sample",
                                                               greedy_slots=greedy, guarded_slots=guarded)
        net.train()
        payoff = terminal_payoff(args.utility, status, nplayers, scores, roller.finished.winners)
        env_ids, seats = np.concatenate(rec.env), np.concatenate(rec.seat)
        adv, ret, keep = advantages(rec, env_ids, seats, status, nplayers, scores, payoff=payoff)
        data, _, _ = rec.tensors(keep)
        lr = lr_at(args.lr, args.lr_decay, run.elapsed(), budget)
        for group in opt.param_groups:
            group["lr"] = lr
        stats = ppo_update(net, opt, data, adv[keep], ret[keep], ent_coef=args.entropy)
        focal = np.array([kinds.index(LEARNER) for _, kinds, *_ in tables])
        done = status == DONE
        canonical = terminal_payoff("canonical", status, nplayers, scores, roller.finished.winners)
        utility = canonical[np.arange(len(status)), focal]
        games += int(done.sum())
        decisions += len(data["state"])
        run.log(type="train", iter=it, games=games, decisions=decisions, utility=round(float(utility.mean()), 4),
                stalled=int((~done).sum()), games_per_sec=round(len(status) / (time.monotonic() - t), 1), lr=lr, **stats)
        it += 1
        if run.elapsed() - last_eval > args.eval_minutes * 60:
            checkpoint()
            last_eval = run.elapsed()
    roller.close()
    env.close()
    final = checkpoint(final=True)
    summary = dict(config=config, games=games, decisions=decisions, minutes=round(run.elapsed() / 60, 2), evals=evals,
                   final_dev=final)
    (run.dir / "result.json").write_text(json.dumps(summary, indent=2))
    run.status("finished", "done", games=games, decisions=decisions)
    return summary


# --------------------------------------------------------------------------------------- command line


def cmd_eval(args) -> None:
    pop = Population.load(args.population)
    support, weights = parse_profile(args.profile, pop)
    players = [int(p) for p in args.players.split(",")]
    candidates = []
    for item in args.candidates.split(",") if args.candidates else []:
        label, _, path = item.partition("=")
        for mode in args.modes.split(","):
            candidates.append(E.Policy(f"{label}/{mode}", str(Path(path).resolve()), mode))
    block = BLOCKS[args.block]
    if block == E.HOLDOUT:  # every look at the holdout is recorded
        E.log_holdout(dict(command="br eval", profile=args.profile, seeds=args.seeds, offset=args.seed_offset,
                           candidates=[(c.name, sha256_file(c.path)) for c in candidates], out=args.out),
                      args.holdout_log)
    res = deviation_study(pop, support, weights, candidates, block=block, seeds=args.seeds, players=players,
                          length=LENGTHS[args.length], seed_offset=args.seed_offset,
                          focals=None if args.all_focals else support, envs=args.envs)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.with_suffix(".json").write_text(json.dumps(res, indent=1))
    out.with_suffix(".md").write_text(render_study(res, args.title))
    print(f"wrote {out.with_suffix('.json')} and {out.with_suffix('.md')}")


def main() -> None:
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    common = argparse.ArgumentParser(add_help=False)
    common.add_argument("--population", default=str(DEFAULT_POPULATION))
    common.add_argument("--profile", required=True, help="opponent strategies, e.g. run2,magnate,exploit_run2 or run2:2,magnate")
    common.add_argument("--players", default="3,4,5")
    common.add_argument("--length", default="standard", help="game length(s), comma separated for training")
    tr = sub.add_parser("train", parents=[common])
    tr.add_argument("--name", required=True)
    tr.add_argument("--init", required=True, help="checkpoint the learner starts from")
    tr.add_argument("--budget-minutes", type=float, default=20.0)
    tr.add_argument("--utility", choices=UTILITIES, default="canonical")
    tr.add_argument("--seed", type=int, default=1)
    tr.add_argument("--lr", type=float, default=3e-4)
    tr.add_argument("--lr-decay", choices=("none", "linear"), default="none",
                    help="linear: the learning rate falls from --lr to zero over the time budget")
    tr.add_argument("--entropy", type=float, default=0.01)
    tr.add_argument("--envs", type=int, default=256)
    tr.add_argument("--threads", type=int, default=12)
    tr.add_argument("--runs", default="runs/br")
    tr.add_argument("--eval-minutes", type=float, default=5.0)
    tr.add_argument("--eval-seeds", type=int, default=25, help="dev seeds per player count for the periodic study")
    tr.add_argument("--eval-envs", type=int, default=256)
    ev = sub.add_parser("eval", parents=[common])
    ev.add_argument("--candidates", default="", help="label=checkpoint[,label=checkpoint...]")
    ev.add_argument("--modes", default="sample,guarded", help="modes in which each candidate is played")
    ev.add_argument("--block", choices=list(BLOCKS), default="dev")
    ev.add_argument("--seeds", type=int, default=100)
    ev.add_argument("--seed-offset", type=int, default=0)
    ev.add_argument("--all-focals", action="store_true", help="also play every strategy of the population as a focal")
    ev.add_argument("--envs", type=int, default=256)
    ev.add_argument("--out", required=True, help="path without extension; writes .json and .md")
    ev.add_argument("--title", default="Deviation study")
    ev.add_argument("--holdout-log", default=str(E.ROOT / "reports/holdout_log.jsonl"))
    rd = sub.add_parser("render", help="rewrite the markdown of a finished study from its json")
    rd.add_argument("--study", required=True, help="path of the study's .json; the .md is written next to it")
    rd.add_argument("--title", default="Deviation study")
    args = ap.parse_args()
    if args.cmd == "train":
        summary = train(args)
        print(f"finished: {summary['games']:,} games in {summary['minutes']} min")
    elif args.cmd == "render":
        path = Path(args.study)
        path.with_suffix(".md").write_text(render_study(json.loads(path.read_text()), args.title))
    else:
        cmd_eval(args)


if __name__ == "__main__":
    main()
