"""A population of frozen policies, and the record of where each one came from.

A *member* is a frozen checkpoint (or a scripted bot of the Rust engine) with its provenance. A *strategy*
is a member played in one mode (`evalcore.MODES`): the unit that cross-play, the meta-game and the
meta-solver reason about. The manifest is the single source of truth for identity: `verify` recomputes every
hash, so a population that was edited after it was frozen is detected, not silently re-evaluated.

What was *measured* about a strategy lives in `results.json` next to the manifest, so that adding a result
never changes the manifest that the payoff files cite by hash. Per strategy it records the game
configurations it was trained and evaluated on (`coverage`), its cross-play results (`crossplay`) and its
best-response results, both as a member of a profile that was attacked and as a trained response
(`best_response`). Every record names the report that holds the full tables.
"""

from __future__ import annotations

import argparse
import json
import shutil
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path

from training.env import LENGTH_NAMES, MERCHANT
from training.evalcore import (
    AUDIT,
    DEV,
    HOLDOUT,
    MODES,
    ROOT,
    Policy,
    git_commit,
    sha256_file,
)

FEATURES = ROOT / "crates/container-bots/src/features.rs"
RESULTS = "results.json"
BLOCK_NAMES = {DEV: "dev", HOLDOUT: "holdout", AUDIT: "audit"}


@dataclass
class Member:
    """A frozen net or scripted bot, with how it was made."""

    name: str
    checkpoint: str | None = None  # repo-relative path of the frozen copy
    scripted: int | None = None
    sha256: str | None = None
    parent: str | None = None
    objective: str = ""
    seed: int | None = None
    config: dict = field(default_factory=dict)
    notes: str = ""


@dataclass
class Strategy:
    """A member played in a mode."""

    name: str
    member: str
    mode: str = "sample"

    def __post_init__(self):
        if self.mode not in MODES:
            raise ValueError(f"{self.name}: unknown mode {self.mode}")


class Population:
    """Members and strategies, stored as one JSON manifest next to the frozen checkpoints."""

    def __init__(self, path: str | Path, header: dict | None = None):
        self.path = Path(path)
        self.header = header or {}
        self.members: dict[str, Member] = {}
        self.strategies: dict[str, Strategy] = {}
        self.results: dict[str, dict] = {}  # strategy -> measured results, the sidecar file

    @classmethod
    def load(cls, path: str | Path) -> Population:
        blob = json.loads(Path(path).read_text())
        pop = cls(path, blob.get("header", {}))
        pop.members = {m["name"]: Member(**m) for m in blob["members"]}
        pop.strategies = {s["name"]: Strategy(**s) for s in blob["strategies"]}
        sidecar = pop.path.with_name(RESULTS)
        if sidecar.exists():
            pop.results = json.loads(sidecar.read_text())
        return pop

    def save_results(self) -> None:
        self.path.with_name(RESULTS).write_text(json.dumps(self.results, indent=1, sort_keys=True) + "\n")

    def entry(self, strategy: str) -> dict:
        """The measured results recorded for `strategy`, created empty on first use."""
        if strategy not in self.strategies:
            raise KeyError(strategy)
        return self.results.setdefault(strategy, dict(
            coverage=dict(trained={}, evaluated={}), crossplay={}, best_response=dict(as_target={}, as_candidate={})))

    def save(self) -> None:
        blob = {"header": self.header, "members": [asdict(m) for m in self.members.values()],
                "strategies": [asdict(s) for s in self.strategies.values()]}
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.path.write_text(json.dumps(blob, indent=2) + "\n")

    def add_member(self, member: Member) -> Member:
        if member.name in self.members:
            raise ValueError(f"{member.name} is already in the population")
        self.members[member.name] = member
        return member

    def add_strategy(self, name: str, member: str, mode: str = "sample") -> Strategy:
        if member not in self.members:
            raise KeyError(member)
        self.strategies[name] = Strategy(name, member, mode)
        return self.strategies[name]

    def policy(self, strategy: str) -> Policy:
        s = self.strategies[strategy]
        m = self.members[s.member]
        if m.scripted is not None:
            return Policy(s.name, scripted=m.scripted)
        return Policy(s.name, str(self.path.parent / Path(m.checkpoint).name), s.mode)

    def policies(self, names: list[str] | None = None) -> list[Policy]:
        return [self.policy(n) for n in (names or list(self.strategies))]

    def verify(self) -> list[str]:
        """Problems found: missing files and hashes that no longer match the manifest."""
        problems = []
        for m in self.members.values():
            if m.checkpoint is None:
                continue
            path = self.path.parent / Path(m.checkpoint).name
            if not path.exists():
                problems.append(f"{m.name}: {path} is missing")
            elif sha256_file(path) != m.sha256:
                problems.append(f"{m.name}: {path} does not match its recorded sha256")
        return problems


def feature_header() -> dict:
    from training.env import BatchEnv

    env = BatchEnv(1, 1)
    header = {"state_dim": env.state_dim, "action_dim": env.action_dim, "max_actions": 48,
              "features_rs_sha256": sha256_file(FEATURES), "git": git_commit()}
    env.close()
    return header


def run_config(run: str) -> dict:
    cfg = ROOT / "runs" / run / "config.json"
    return json.loads(cfg.read_text()) if cfg.exists() else {}


# The baselines of the equilibrium study. `runs/<run>/final.pt` is frozen as it is; nothing here retrains.
BASELINES = [
    ("run1", "run1", None, "imitation of Merchant, then PPO league self-play (60 min)"),
    ("run2", "run2", "run1", "PPO league continuation of run1 against its past versions and scripted bots (30 min)"),
    ("exploit_run2", "exploit_run2", "run2",
     "PPO best-response exploiter of greedy run2: one learner seat, every other seat plays run2 (30 min)"),
    ("control_run2_30m", "control_run2_30m", "run2",
     "PPO league continuation of run2; field: run1, run2, scripted bots (30 min). Control for the defense run"),
    ("defense_run2_vs_exploiter_30m", "defense_run2_vs_exploiter_30m", "run2",
     "PPO league continuation of run2 with exploit_run2 added to the field (30 min). The net of Magnate"),
]


def freeze_baselines(out: str | Path = ROOT / "populations/baselines") -> Population:
    """Copies the baseline checkpoints into `out`, read-only, and writes the manifest."""
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    pop = Population(out / "manifest.json", feature_header())
    pop.add_member(Member("merchant", scripted=MERCHANT, objective="scripted heuristic of container-bots (merchant.rs)",
                          notes="tuned in self-play by crates/container-arena/src/bin/tune.rs"))
    for name, run, parent, objective in BASELINES:
        src = ROOT / "runs" / run / "final.pt"
        dst = out / f"{name}.pt"
        if dst.exists():
            dst.chmod(0o644)
        shutil.copyfile(src, dst)
        dst.chmod(0o444)
        cfg = run_config(run)
        pop.add_member(Member(name, checkpoint=f"{out.relative_to(ROOT)}/{name}.pt", sha256=sha256_file(dst), parent=parent,
                              objective=objective, seed=cfg.get("seed"), config=cfg,
                              notes=f"source: runs/{run}/final.pt"))
    magnate_bin = ROOT / "crates/container-bots/weights/magnate.bin"
    defense_bin = ROOT / "runs/defense_run2_vs_exploiter_30m/weights.bin"
    same = "equals" if sha256_file(defense_bin) == sha256_file(magnate_bin) else "DIFFERS FROM"
    pop.members["defense_run2_vs_exploiter_30m"].notes += (
        f"; weights.bin sha256 {sha256_file(defense_bin)[:16]} {same} the built-in Magnate "
        "(crates/container-bots/weights/magnate.bin)")
    # The study's strategies: the nets as they were trained (stochastic) and Magnate as it is deployed.
    pop.add_strategy("merchant", "merchant")
    for name, _, _, _ in BASELINES:
        short = {"control_run2_30m": "control", "defense_run2_vs_exploiter_30m": "defense"}.get(name, name)
        pop.add_strategy(short, name, "sample")
    pop.add_strategy("magnate", "defense_run2_vs_exploiter_30m", "guarded")
    pop.save()
    return pop


def baseline_coverage(pop: Population) -> None:
    """The configurations the baselines were made for: training/train.py draws 3, 4 or 5 players per table and
    never passes a game length (Standard); tune.rs tunes the Merchant at 3-5 seats with the default configuration."""
    for name, s in pop.strategies.items():
        scripted = pop.members[s.member].scripted is not None
        source = ("scripted; parameters tuned by crates/container-arena/src/bin/tune.rs at 3-5 seats, default game configuration"
                  if scripted else "training/train.py: 3, 4 or 5 players per table, game length not varied (Standard)")
        record_trained(pop, name, [3, 4, 5], ["standard"], source)


def record_trained(pop: Population, strategy: str, players: list[int], lengths: list[str], source: str) -> None:
    """The game configurations a strategy was made for."""
    pop.entry(strategy)["coverage"]["trained"] = dict(players=sorted(players), lengths=list(lengths), source=source)


def record_crossplay(pop: Population, payoffs: dict, report: str) -> list[str]:
    """Attaches the headline numbers of a cross-play table (`payoffs.json`) to every strategy it simulated."""
    meta, per, acc = payoffs["meta"], payoffs["secondary"]["strategies"], payoffs["accounting"]
    length = LENGTH_NAMES[int(meta["length"])]
    touched = []
    for name in payoffs["strategies"]:
        if name not in pop.strategies or name not in per:
            continue
        entry, s = pop.entry(name), per[name]
        entry["crossplay"][report] = dict(
            population=payoffs["strategies"], players=payoffs["players"], length=length,
            block=BLOCK_NAMES.get(int(meta["block"]), str(meta["block"])), seeds=meta["seeds"],
            seed_offset=meta.get("seed_offset", 0), seat_games=s["seat_games"], fair_share=s["fair_share"],
            win_rate=s["win_rate"], tie_rate=s["tie_rate"], score_margin=s["score_margin"], mean_actions=s["actions"],
            table_games=acc["games"], table_stalled=acc["stalled"])
        seen = entry["coverage"]["evaluated"]
        seen[length] = sorted(set(seen.get(length, [])) | set(payoffs["players"]))
        touched.append(name)
    return touched


def record_br(pop: Population, study: dict, report: str) -> list[str]:
    """Attaches a deviation study (`training.br`) to the strategies it concerns.

    A strategy in the support of the attacked profile gets the study as `as_target`, with its weight in the
    profile; a trained candidate that is a strategy of the population gets it as `as_candidate`. Gains are the
    candidate's gain over the profile, a lower bound on the true deviation gain.
    """
    prof, mean = study["profile"], study["mean_over_players"]
    weights = prof["weights"]
    pure = {lab: g for lab, g in mean["gain"].items() if lab in study["focals"] and lab not in study["candidates"]}
    best = max(pure.items(), key=lambda kv: kv[1][0], default=None)
    responses = {lab: dict(gain=mean["gain"][lab], nashconv=mean["nashconv"][lab],
                           max_seat_gain=max(g[0] for blk in study["by_players"].values() for g in blk["by_seat"][lab]))
                 for lab in study["candidates"]}
    common = dict(
        report=report, block=BLOCK_NAMES.get(int(study["block"]), str(study["block"])), seeds=study["seeds"],
        seed_offset=study["seed_offset"], players=study["players"], length=LENGTH_NAMES[int(study["length"])],
        profile=dict(support=prof["support"], weights=weights), games=study["accounting"]["games"],
        stalled=study["accounting"]["stalled"],
        best_pure_deviation=None if best is None else dict(strategy=best[0], gain=best[1]))
    touched = []
    for i, name in enumerate(prof["support"]):
        if name in pop.strategies:
            w = {n: ws[i] for n, ws in weights.items()} if isinstance(weights, dict) else weights[i]
            pop.entry(name)["best_response"]["as_target"][report] = dict(common, weight=w, responses=responses)
            touched.append(name)
    for name in dict.fromkeys(lab.split("/")[0] for lab in study["candidates"]):
        if name in pop.strategies:  # one entry per report; the candidate may have been played in several modes
            own = {lab: r for lab, r in responses.items() if lab.split("/")[0] == name}
            pop.entry(name)["best_response"]["as_candidate"][report] = dict(common, responses=own)
            touched.append(name)
    return touched


def annotate(pop: Population, crossplay: list[str], studies: list[str]) -> None:
    """Records cross-play tables and deviation studies, given as file paths, in the population's results."""
    for path in crossplay:
        record_crossplay(pop, json.loads(Path(path).read_text()), _rel(path))
    for path in studies:
        record_br(pop, json.loads(Path(path).read_text()), _rel(path))
    pop.save_results()


def _rel(path: str | Path) -> str:
    path = Path(path).resolve()
    return str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description="Freeze, verify and annotate a population of frozen policies.")
    sub = ap.add_subparsers(dest="cmd")
    sub.add_parser("freeze", help="freeze the baselines into populations/baselines")
    ve = sub.add_parser("verify", help="recompute every hash of a manifest (the default command)")
    ve.add_argument("manifest", nargs="?", default=str(ROOT / "populations/baselines/manifest.json"))
    an = sub.add_parser("annotate", help="record results in the results.json next to a manifest")
    an.add_argument("manifest")
    an.add_argument("--crossplay", default="", help="payoffs.json files, comma separated")
    an.add_argument("--br", default="", help="best-response study .json files, comma separated")
    an.add_argument("--baselines", action="store_true", help="record what the baselines were trained and evaluated on")
    args = ap.parse_args(argv)
    if args.cmd == "freeze":
        p = freeze_baselines()
        print(f"froze {len(p.members)} members, {len(p.strategies)} strategies into {p.path}")
        return 0
    if args.cmd == "annotate":
        p = Population.load(args.manifest)
        if args.baselines:
            baseline_coverage(p)
        annotate(p, [x for x in args.crossplay.split(",") if x], [x for x in args.br.split(",") if x])
        print(f"{p.path.with_name(RESULTS)}: {len(p.results)} strategies")
        return 0
    p = Population.load(getattr(args, "manifest", str(ROOT / "populations/baselines/manifest.json")))
    issues = p.verify()
    print("\n".join(issues) if issues else f"{p.path}: {len(p.members)} members verified")
    return 1 if issues else 0


if __name__ == "__main__":
    sys.exit(main())
