"""Probe of the loan-while-bidding surface: does a net gain by taking a loan where it never does?

`RequestLoan` is legal in the `bid` and `decide` phases about 41,000 times per net in 300 self-play games
and no net ever picks it (`reports/audit/summary.md`, `docs/equilibrium_spec.md` sections 3.2 and 11). The
nets were never trained on it, so a rule that forces it is more likely to cost the $1 interest per turn
than to win a lot; this measures which.

Each rule replaces the pick of a deployed (`guarded`) net by `RequestLoan` in some of the states of one
phase. Every rule is compared with the same net unmodified, on the same seeds, seats and opponents
(`br.deviation_study`), against a profile of the population. Nothing is trained, the rules are fixed before
the study is run, and a rule is only worth keeping if the lower end of its paired gain is above zero.

    python -m training.loanprobe --profile @reports/psro/refined_metagame_profile.json \\
        --population populations/psro/manifest.json --nets psro03,psro05 --seeds 100 --seed-offset 1000 \\
        --out reports/loanprobe/dev

A rule is `PHASE:WHEN[:P]`: PHASE is `bid` (a bid is to be made) or `decide` (the deliverer decides on their
auction); WHEN is `allin` (the net's pick is the bid of all its cash) or `any`; P is the chance that the rule
fires where it applies (default 1).
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import numpy as np
import torch

from training import evalcore as E
from training.br import DEFAULT_POPULATION, _boot_mean, _ci, deviation_study, paired_difference, parse_profile
from training.population import Population

# Columns of an action's feature vector (`crates/container-bots/src/features.rs`): the action kinds
# are one-hot at the start, and the last three numbers are amount / 30, amount / cash and "has an amount".
BID, ACCEPT, LOAN = 13, 14, 16
AMOUNT_OVER_CASH = -2


class Rule:
    """Replaces a net's pick by `RequestLoan` where `phase` applies. Counts the decisions it saw and fired on."""

    def __init__(self, spec: str, seed: int = 0):
        phase, when, *p = spec.split(":")
        if phase not in ("bid", "decide") or when not in ("allin", "any") or len(p) > 1:
            raise ValueError(f"bad loan rule {spec!r}: expected bid|decide:allin|any[:P]")
        if phase == "decide" and when == "allin":
            raise ValueError("an all-in pick only exists in the bid phase")
        self.spec, self.phase, self.allin = spec, phase, when == "allin"
        self.p = float(p[0]) if p else 1.0
        self.seed = seed
        self.seen = self.fired = 0
        self._gen: torch.Generator | None = None

    def __call__(self, action: torch.Tensor, rows: torch.Tensor, mask: torch.Tensor, acts: torch.Tensor) -> torch.Tensor:
        a, valid = acts[rows, : mask.shape[1]], mask[rows]
        here = (valid & (a[..., BID if self.phase == "bid" else ACCEPT] > 0.5)).any(dim=1)
        loan = valid & (a[..., LOAN] > 0.5)
        applies = here & loan.any(dim=1)
        self.seen += int(applies.sum())
        fire = applies
        if self.allin:
            picked = a[torch.arange(len(rows), device=a.device), action[rows]]
            fire = fire & (picked[:, BID] > 0.5) & (picked[:, AMOUNT_OVER_CASH] >= 0.999)
        if self.p < 1.0:
            if self._gen is None:
                self._gen = torch.Generator(device=a.device).manual_seed(self.seed)
            fire = fire & (torch.rand(len(rows), generator=self._gen, device=a.device) < self.p)
        action[rows[fire]] = loan[fire].to(torch.int64).argmax(dim=1)
        self.fired += int(fire.sum())
        return action


def probe(pop: Population, support: list[str], weights, nets: list[str], rules: list[str], *, seeds: int,
          seed_offset: int, players: list[int], envs: int = 256, torch_seed: int = 0) -> dict:
    """Plays every net unmodified and with every rule against the profile; returns the study and the rule counts."""
    candidates, hooks = [], {}
    for net in nets:
        path = pop.policies([net])[0].path
        for spec in rules:
            label = f"{net}+{spec}"
            hooks[label] = Rule(spec, seed=torch_seed)
            candidates.append(E.Policy(label, path, "guarded", adjust=hooks[label]))
    res = deviation_study(pop, support, weights, candidates, block=E.DEV, seeds=seeds, players=players,
                          seed_offset=seed_offset, focals=nets, envs=envs, torch_seed=torch_seed)
    rows = []
    for label, rule in hooks.items():
        net = label.partition("+")[0]
        rows.append(dict(
            label=label, net=net, rule=rule.spec, seen=rule.seen, fired=rule.fired,
            gain=res["mean_over_players"]["gain"][label],
            paired=paired_difference(res, label, net),
            by_players={n: _boot_mean([_diff(blk, label, net)]) for n, blk in res["by_players"].items()},
        ))
    res["probe"] = rows
    return res


def _diff(block: dict, a: str, b: str) -> np.ndarray:
    return np.asarray(block["per_seed_gain"][a]) - np.asarray(block["per_seed_gain"][b])


def render(res: dict, title: str) -> str:
    acc = res["accounting"]
    lines = [f"# {title}", "",
             f"Profile sigma: {json.dumps(res['profile']['weights'])} over {res['profile']['support']}. "
             f"{res['seeds']} dev seeds per player count (offset {res['seed_offset']}), every seat of every table, "
             f"{acc['games']:,} games, {acc['stalled']} stalled. Code version {res['git']}.", "",
             "Each row plays a net with a rule that replaces its pick by `RequestLoan`, against opponents drawn from sigma. "
             "`paired gain` is its utility minus the same net unmodified on the same seeds, seats and opponents (mean over "
             "player counts, 95% seed-clustered interval); `fired` counts the decisions the rule replaced of those where it "
             "applied (a loan was legal in that phase).", "",
             "| net + rule | applied | fired | paired gain | 3 players | 4 players | 5 players | gain over sigma |",
             "|---|---|---|---|---|---|---|---|"]
    for r in res["probe"]:
        per = " | ".join(_ci(r["by_players"][str(n)]) if str(n) in r["by_players"] else "-" for n in (3, 4, 5))
        lines.append(f"| {r['label']} | {r['seen']:,} | {r['fired']:,} | {_ci(r['paired'])} | {per} | {_ci(r['gain'])} |")
    base = ", ".join(f"{n}: {_ci(res['mean_over_players']['gain'][n])}" for n in res["focals"] if "+" not in n)
    lines += ["", f"Gain over sigma of the unmodified nets (mean over player counts): {base}.", ""]
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--population", default=str(DEFAULT_POPULATION))
    ap.add_argument("--profile", required=True)
    ap.add_argument("--nets", default="psro03,psro05", help="population strategies to modify")
    ap.add_argument("--rules", default="bid:allin,bid:any:0.1,decide:any:0.1")
    ap.add_argument("--players", default="3,4,5")
    ap.add_argument("--seeds", type=int, default=100)
    ap.add_argument("--seed-offset", type=int, default=1000)
    ap.add_argument("--envs", type=int, default=256)
    ap.add_argument("--out", required=True, help="path without extension; writes .json and .md")
    ap.add_argument("--title", default="Loan-while-bidding probe")
    args = ap.parse_args()
    pop = Population.load(args.population)
    support, weights = parse_profile(args.profile, pop)
    res = probe(pop, support, weights, args.nets.split(","), args.rules.split(","), seeds=args.seeds,
                seed_offset=args.seed_offset, players=[int(p) for p in args.players.split(",")], envs=args.envs)
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.with_suffix(".json").write_text(json.dumps(res, indent=1))
    out.with_suffix(".md").write_text(render(res, args.title))
    print(f"wrote {out.with_suffix('.json')} and {out.with_suffix('.md')}")


if __name__ == "__main__":
    main()
