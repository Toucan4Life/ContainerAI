"""PSRO: a population grown by approximate best responses to the solution of its own empirical meta-game.

    python -m training.psro init  --strategies run2,magnate,control,defense,exploit_run2,merchant \
        --cells reports/crossplay/deployed_dev/payoffs.json
    python -m training.psro step                       # one iteration; resumes an interrupted one with the settings it began with
    python -m training.psro run --iterations 5 --no-start-after 20:30     # touch reports/psro/STOP to end it after the current iteration
    python -m training.psro final --seeds 150                             # closing audit on the holdout block
    python -m training.psro solve --cells reports/crossplay/deployed_dev/payoffs.json --strategies run2,magnate --out reports/psro/x
    python -m training.psro report

One iteration, for the population P of m strategies (definitions in docs/equilibrium_spec.md):

 1. The cross-play cells of P give, for each player count n, the symmetric n-player meta-game on P. Every
    solver of `training.metagame` runs on it and the chosen solution sigma_n is checked by the independent
    verifier. A table size is solved on its own because the nets see it, so a deviation may depend on it.
 2. A fresh PPO best response trains against opponents drawn iid from sigma_n, smoothed towards the uniform
    mixture over P (`gamma`) so that the response does not fixate on one narrow support (`training.br`).
 3. A paired deviation study on seeds the training never saw measures the candidate's gain over sigma_n
    itself (no smoothing), per player count and averaged, together with every pure strategy of P. The
    weights are played in each of `modes` (a best response need not randomise); the mode with the larger
    mean gain represents the candidate, and the study records the gain of every mode and the largest gain
    of a single seat.
 4. The candidate joins P, in that mode, if the lower 95% bound of its mean gain is above `min_gain`;
    otherwise the oracle found no meaningful improvement within its budget and the iteration records that.
    A member is never removed, whatever its ELO.
 5. The candidate is cross-played against P (only the new compositions), what was measured about it is
    written to the results sidecar of the population (`training.population`), and the next iteration
    solves again.

A candidate's gain is a lower bound on the true deviation gain of sigma_n, because the oracle is only an
approximate best response; the iteration reports say so next to every number.
"""

from __future__ import annotations

import argparse
import datetime as dt
import itertools
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

import numpy as np

from training import br, br_report, crossplay
from training import evalcore as E
from training import metagame as MG
from training.env import LENGTH_NAMES, STANDARD
from training.population import (
    Member,
    Population,
    record_br,
    record_crossplay,
    record_trained,
    sha256_file,
)

ROOT = E.ROOT
POP_DIR = ROOT / "populations/psro"
OUT = ROOT / "reports/psro"
RUNS = "runs/psro"
CONFIG = OUT / "config.json"

DEFAULTS = dict(
    population=str((POP_DIR / "manifest.json").relative_to(ROOT)), cells=[], players=[3, 4, 5], length=STANDARD,
    minutes=25.0, utility="canonical", gamma=0.1, lr=3e-4, lr_decay="none", entropy=0.01, envs=256,
    eval_minutes=8.0, eval_seeds=10, study_seeds=60, study_offset=1000, cell_seeds=16, min_gain=0.0,
    seed=100, boot_reps=200, solver_seed=0, modes="sample,guarded",
)
TUNABLE = ("minutes", "min_gain", "study_seeds", "gamma", "utility", "seed", "cell_seeds", "envs", "eval_minutes",
           "eval_seeds", "modes", "lr", "lr_decay", "entropy")  # settable on the command line
EXACT_CHECK_LIMIT = 2_000_000  # m ** (n - 1) up to which the independent regret check is an exact enumeration


def rel(path: Path | str) -> str:
    path = Path(path)
    return str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)


def load_config() -> dict:
    if not CONFIG.exists():
        raise SystemExit(f"{rel(CONFIG)} does not exist; run `psro init` first")
    return DEFAULTS | json.loads(CONFIG.read_text())


def write_atomic(path: Path, text: str) -> None:
    """A run killed while writing leaves the old file, never a truncated one that every rerun would choke on."""
    tmp = path.with_name(path.name + ".tmp")
    tmp.write_text(text)
    os.replace(tmp, path)


def pin_config(d: Path, cfg: dict) -> dict:
    """The settings an iteration started with stay its settings when it is resumed, whatever flags the rerun has."""
    pinned = d / "cfg.json"
    if not pinned.exists():
        write_atomic(pinned, json.dumps(cfg, indent=1) + "\n")
        return cfg
    old = json.loads(pinned.read_text())
    changed = {k: (old.get(k), v) for k, v in cfg.items() if old.get(k) != v}
    if changed:
        print(f"resuming {rel(d)} with the settings it started with; ignored now: {changed} "
              f"(delete {rel(pinned)} to start the iteration again with the new settings)", flush=True)
    return DEFAULTS | old


def fresh_study_offset(cfg: dict, it: int) -> int:
    """First dev seed of the study of iteration `it`: past every seed an earlier study used, so that neither a
    retry nor a change of `study_seeds` can repeat a seed."""
    end = int(cfg["study_offset"])
    for k in range(1, it):
        path = OUT / f"iter_{k:02d}/study.json"
        if path.exists():
            study = json.loads(path.read_text())
            end = max(end, int(study["seed_offset"]) + int(study["seeds"]))
    return end


# ------------------------------------------------------------------------------------------ cell store


class CellStore:
    """Simulated payoffs of tables, keyed by (players, sorted strategy names), gathered from cross-play files.

    Names rather than indices, so that cells survive when the population grows. A cell keeps the per-seed
    payoffs of its distinct strategies; the payoff is their mean and the bootstrap resamples the seeds.
    """

    def __init__(self) -> None:
        self.cells: dict[tuple[int, tuple[str, ...]], dict] = {}
        self.length: int | None = None

    @classmethod
    def from_files(cls, paths) -> CellStore:
        store = cls()
        for p in paths:
            store.add(json.loads(Path(p).read_text()), source=str(p))
        return store

    def add(self, payoffs: dict, source: str = "") -> int:
        """Adds the cells of a cross-play `payoffs.json`. Seeds already used by a cell must not repeat."""
        names, meta = payoffs["strategies"], payoffs["meta"]
        length = int(meta["length"])
        if self.length not in (None, length):
            raise ValueError(f"{source}: game length {length} differs from the store's {self.length}")
        self.length = length
        block, lo = int(meta["block"]), int(meta.get("seed_offset", 0))
        span = (block, lo, lo + int(meta["seeds"]))
        for c in payoffs["cells"]:
            key = (int(c["players"]), tuple(sorted(names[i] for i in c["multiset"])))
            distinct = [names[i] for i in c["distinct"]]
            rows = np.asarray(c["per_seed"], dtype=np.float64).reshape(len(c["per_seed"]), len(distinct))
            old = self.cells.get(key)
            if old is None:
                self.cells[key] = dict(distinct=distinct, per_seed=rows, games=int(c["games"]),
                                       stalled=int(c["stalled"]), spans=[span])
                continue
            if any(b == span[0] and lo < z and a < span[2] for b, a, z in old["spans"]):
                raise ValueError(f"{source}: cell {key} already holds some of seeds {span[1]}..{span[2]}")
            order = [distinct.index(d) for d in old["distinct"]]
            old["per_seed"] = np.vstack([old["per_seed"], rows[:, order]])
            old["games"] += int(c["games"])
            old["stalled"] += int(c["stalled"])
            old["spans"].append(span)
        return len(payoffs["cells"])

    def _keys(self, names: list[str], n: int):
        for ms in itertools.combinations_with_replacement(range(len(names)), n):
            yield ms, (n, tuple(sorted(names[i] for i in ms)))

    def missing(self, names: list[str], players) -> list[tuple[int, tuple[str, ...]]]:
        return [key for n in players for _, key in self._keys(names, n) if key not in self.cells]

    def payoffs_of(self, key, values: np.ndarray | None = None) -> dict[str, float]:
        cell = self.cells[key]
        mean = cell["per_seed"].mean(axis=0) if values is None else values
        return dict(zip(cell["distinct"], mean))

    def game(self, names: list[str], n: int, boot: dict | None = None, rep: int = 0) -> MG.SymmetricGame:
        """The n-player meta-game on `names`; with `boot` (see `bootstrap`), the resampled game number `rep`."""
        payoffs = {}
        for ms, key in self._keys(names, n):
            if key not in self.cells:
                raise ValueError(f"no simulated table for {key}; cross-play the population first")
            by = self.payoffs_of(key, None if boot is None else boot[key][rep])
            payoffs[ms] = [by[names[i]] for i in ms]
        return MG.SymmetricGame(n, len(names), payoffs, names)

    def bootstrap(self, names: list[str], n: int, reps: int, seed: int = 0) -> list[MG.SymmetricGame]:
        """`reps` games whose cells resample their own seeds with replacement (cells independently)."""
        rng = np.random.default_rng([seed, n])
        boot = {}
        for _, key in self._keys(names, n):
            rows = self.cells[key]["per_seed"]
            boot[key] = rows[rng.integers(0, len(rows), size=(reps, len(rows)))].mean(axis=1)
        return [self.game(names, n, boot, r) for r in range(reps)]

    def accounting(self, names: list[str], players) -> dict:
        games = stalled = 0
        for n in players:
            for _, key in self._keys(names, n):
                games += self.cells[key]["games"]
                stalled += self.cells[key]["stalled"]
        return dict(games=games, stalled=stalled)


# ------------------------------------------------------------------------------------------ meta-solver


def entropy(p: np.ndarray) -> float:
    p = p[p > 1e-12]
    return float(-(p * np.log(p)).sum())


def choose(solutions: list[MG.Solution]) -> tuple[MG.Solution, bool]:
    """The meta-strategy to train against. Among the outputs that are Nash equilibria (verified by their
    deviation gain) the one with the largest entropy, which keeps the opponents diverse; if no solver found
    one, the output with the smallest gain, flagged as not a Nash equilibrium."""
    nash = [s for s in solutions if s.concept.startswith("Nash")]
    if nash:
        return max(nash, key=lambda s: (round(entropy(s.sigma), 4), -s.gain)), True
    return min(solutions, key=lambda s: s.gain), False


def solve_meta(store: CellStore, names: list[str], players, *, seed: int = 0, quick: bool = False,
               boot_reps: int = 200) -> dict:
    """The meta-game of every player count, solved and independently verified. JSON-ready."""
    out = {}
    for n in players:
        game = store.game(names, n)
        solutions = MG.solve_all(game, n_players=n, seed=seed, quick=quick)
        best, is_nash = choose(solutions)
        # the check enumerates every ordered opponent tuple, which stays cheap far beyond the sizes of a real population;
        # its sampled fallback has a standard error, so the comparison with the solver is then only as tight as that
        exact = game.m ** (n - 1) <= EXACT_CHECK_LIMIT
        check = MG.verify_regret(game.raw(), game.m, best.sigma, method="enumerate" if exact else "sampled")
        tol = 1e-6 if check["stderr"] is None else 1e-6 + 5 * float(check["stderr"])
        if abs(check["gain"] - best.gain) > tol:
            raise RuntimeError(f"n={n}: solver gain {best.gain} and independent check {check['gain']} disagree")
        boot = MG.bootstrap_gain(best.sigma, store.bootstrap(names, n, boot_reps, seed)) if boot_reps else np.zeros(1)
        values = game.expected_payoffs(best.sigma)
        out[str(n)] = dict(
            sigma=[float(x) for x in best.sigma], solver=best.method, concept=best.concept, is_nash=bool(is_nash),
            gain=float(best.gain), nash_conv=float(n * best.gain), verified_gain=float(check["gain"]),
            verify_method=check["method"], entropy=entropy(best.sigma),
            bootstrap=dict(mean=float(boot.mean()), p95=float(np.quantile(boot, 0.95)), reps=int(boot_reps)),
            value=float(best.sigma @ values), values=dict(zip(names, (float(v) for v in values))),
            solvers=[dict(method=s.method, concept=s.concept, gain=float(s.gain),
                          nash_conv=float(s.extra.get("nash_conv", n * s.gain)),
                          support=[names[i] for i in np.nonzero(s.sigma > 1e-3)[0]], entropy=entropy(s.sigma))
                     for s in solutions],
            accounting=store.accounting(names, [n]),
        )
    return out


def explore(sigma, gamma: float) -> np.ndarray:
    """Training distribution: sigma smoothed towards the uniform mixture over the population."""
    sigma = np.asarray(sigma, dtype=np.float64)
    return (1.0 - gamma) * sigma + gamma / len(sigma)


def render_meta(meta: dict, names: list[str]) -> list[str]:
    lines = ["| n | solver | solution concept | support (weight) | restricted-game gain | independent check | "
             "bootstrap gain, mean / 95th pct | NashConv |", "|---|---|---|---|---|---|---|---|"]
    for n, mt in meta.items():
        sup = ", ".join(f"{nm} {w:.2f}" for nm, w in zip(names, mt["sigma"]) if w > 1e-3)
        lines.append(f"| {n} | {mt['solver']} | {mt['concept']} | {sup} | {mt['gain']:.1e} | "
                     f"{mt['verified_gain']:.1e} ({mt['verify_method']}) | "
                     f"{mt['bootstrap']['mean']:.3f} / {mt['bootstrap']['p95']:.3f} | {mt['nash_conv']:.1e} |")
    lines += ["", "Utility of every strategy against sigma_n (n-1 opponents drawn iid from sigma_n; 1/n is an equal share):",
              "", "| n | " + " | ".join(names) + " |", "|---|" + "---|" * len(names)]
    for n, mt in meta.items():
        lines.append(f"| {n} | " + " | ".join(f"{mt['values'][nm]:.3f}" for nm in names) + " |")
    lines += ["", "Every solver output (not only the chosen one):", "",
              "| n | solver | concept | gain | NashConv | support |", "|---|---|---|---|---|---|"]
    for n, mt in meta.items():
        for s in mt["solvers"]:
            lines.append(f"| {n} | {s['method']} | {s['concept']} | {s['gain']:.1e} | {s['nash_conv']:.1e} | "
                         f"{', '.join(s['support'])} |")
    return lines + [""]


def write_meta(meta: dict, names: list[str], out: Path, title: str) -> list[str]:
    """Writes `out`.json (the solutions) and `out`.md (their report); returns the lines of the report."""
    acc = {n: m["accounting"] for n, m in meta.items()}
    lines = [f"# {title}", "",
             f"Strategies: {', '.join(names)}. Tables simulated: " +
             "; ".join(f"{n} players {a['games']:,} games, {a['stalled']} stalled" for n, a in acc.items()) + ". "
             "Utility: winner share (a stalled game pays 0). Every player count is its own symmetric game.", "",
             "Gains are exact in the empirical game. The bootstrap column resamples the simulated seeds of every table "
             "and re-measures the gain of the *fixed* solution: it is the noise floor under which a deviation gain "
             "cannot be told from sampling error. An equilibrium of the empirical game is not an equilibrium of the "
             "real game.", ""] + render_meta(meta, names)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.with_suffix(".json").write_text(json.dumps(meta, indent=1))
    out.with_suffix(".md").write_text("\n".join(lines))
    return lines


def cmd_solve(args) -> None:
    store = CellStore.from_files([ROOT / p for p in args.cells.split(",")] if args.cells else [])
    names = args.strategies.split(",")
    players = [int(p) for p in args.players.split(",")]
    meta = solve_meta(store, names, players, seed=args.seed, boot_reps=args.boot_reps)
    out = Path(args.out)
    print("\n".join(write_meta(meta, names, out, args.title)[:12]))
    profile = out.with_name(out.name + "_profile.json")  # the solution, as `br train` and `br eval` read a profile
    write_atomic(profile, json.dumps(dict(support=names, weights={n: m["sigma"] for n, m in meta.items()})))
    print(f"wrote {rel(profile)}")


# ------------------------------------------------------------------------------------------ the loop


def population() -> Population:
    cfg = load_config()
    pop = Population.load(ROOT / cfg["population"])
    problems = pop.verify()
    if problems:
        raise SystemExit("population does not verify:\n" + "\n".join(problems))
    return pop


def iteration_dirs() -> list[Path]:
    return sorted(OUT.glob("iter_*"))


def store_from_disk(cfg: dict) -> CellStore:
    files = [ROOT / p for p in cfg["cells"]] + sorted(OUT.glob("iter_*/crossplay/payoffs.json"))
    return CellStore.from_files(files)


def sh(cmd: list[str], log: Path) -> None:
    log.parent.mkdir(parents=True, exist_ok=True)
    with open(log, "a") as f:
        print("$ " + " ".join(cmd), file=f, flush=True)
        subprocess.run(cmd, check=True, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT)


def best_pure_init(meta: dict, pop: Population, names: list[str]) -> str:
    """The net strategy with the highest mean utility against the sigma_n: the best response in the population."""
    nets = [nm for nm in names if pop.members[pop.strategies[nm].member].scripted is None]
    return max(nets, key=lambda nm: np.mean([m["values"][nm] for m in meta.values()]))


def train_summary(run_dir: Path) -> dict:
    result = json.loads((run_dir / "result.json").read_text())
    lines = [json.loads(x) for x in (run_dir / "metrics.jsonl").read_text().splitlines() if x.strip()]
    train = [x for x in lines if x.get("type") == "train"]
    tail = train[-max(1, len(train) // 5):]
    return dict(minutes=result["minutes"], games=result["games"], decisions=result["decisions"],
                utility_first=float(np.mean([x["utility"] for x in train[:max(1, len(train) // 5)]])),
                utility_last=float(np.mean([x["utility"] for x in tail])),
                curve=[dict(minutes=e["minutes"], gain=e["gain"]) for e in result["evals"]])


def admit(pop: Population, it: int, init: str, ckpt: Path, train_cfg: dict, decision: dict,
          objective: str | None = None) -> str:
    name = f"psro{it:02d}"
    if name in pop.strategies:
        return name
    dst = pop.path.parent / f"{name}.pt"
    if not (dst.exists() and sha256_file(dst) == sha256_file(ckpt)):  # else: a run that died between the copy and the save
        if dst.exists():
            dst.chmod(0o644)
        shutil.copyfile(ckpt, dst)
    dst.chmod(0o444)
    notes = (f"admitted in {decision['mode']} mode; mean gain over sigma on dev seeds {decision['gain'][0]:+.3f} "
             f"[{decision['gain'][1]:+.3f}, {decision['gain'][2]:+.3f}]; by mode: "
             + ", ".join(f"{m} {g[0]:+.3f}" for m, g in decision["gain_by_mode"].items()))
    if "confirmation" in decision:
        c = decision["confirmation"]
        notes += (f"; on {c['seeds']} holdout seeds per player count (offset {c['seed_offset']}) "
                  f"{c['gain'][0]:+.3f} [{c['gain'][1]:+.3f}, {c['gain'][2]:+.3f}]")
    pop.add_member(Member(
        name, checkpoint=rel(dst), sha256=sha256_file(dst), parent=pop.strategies[init].member,
        objective=objective or (f"PPO best response to the meta-game solution of iteration {it} "
                                f"(utility {train_cfg['utility']}, exploration gamma {train_cfg['gamma']})"),
        seed=train_cfg["seed"], config=train_cfg, notes=notes))
    pop.add_strategy(name, name, decision["mode"])
    pop.save()
    return name


def response_decision(study: dict, cand: str, min_gain: float, confirm: dict | None = None) -> dict:
    """The admission rule of `step` applied to a deviation study of the response `cand`, which is played in several
    modes: the mode with the larger mean gain represents it and its lower 95% bound must be above `min_gain`. A
    response trained outside the loop also has to hold up in `confirm`, a study on holdout seeds."""
    modes = [lab.split("/", 1)[1] for lab in study["candidates"] if lab.split("/", 1)[0] == cand]
    if not modes:
        raise SystemExit(f"{cand!r} is not a candidate of the study: {list(study['candidates'])}")
    by_mode = {m: study["mean_over_players"]["gain"][f"{cand}/{m}"] for m in modes}
    mode = max(modes, key=lambda m: by_mode[m][0])  # a best response is not obliged to randomise
    decision = dict(accepted=bool(by_mode[mode][1] > min_gain), mode=mode, gain=by_mode[mode], gain_by_mode=by_mode)
    if confirm is not None:
        gain = confirm["mean_over_players"]["gain"][f"{cand}/{mode}"]
        decision["confirmation"] = dict(gain=gain, block=confirm["block"], seeds=confirm["seeds"],
                                        seed_offset=confirm["seed_offset"])
        decision["accepted"] = decision["accepted"] and bool(gain[1] > min_gain)
    return decision


def crossplay_new(pop: Population, name: str, cfg: dict, d: Path) -> None:
    out = d / "crossplay"
    if (out / "payoffs.json").exists():
        return
    names = list(pop.strategies)
    rec = crossplay.run_crossplay(pop.policies(names), cfg["players"], cfg["length"], cfg["cell_seeds"], E.DEV, out,
                                  include=[names.index(name)], envs=cfg["envs"])
    payoffs = crossplay.build_payoffs(rec, names)
    payoffs["population"] = dict(manifest=rel(pop.path), manifest_sha256_16=crossplay.manifest_digest(pop.path),
                                 new_member=name)
    write_atomic(out / "payoffs.json", json.dumps(payoffs, indent=1) + "\n")
    print(f"cross-play of {name}: {len(rec):,} games, {E.accounting(rec)}", flush=True)


def step(cfg: dict) -> dict:
    """One PSRO iteration. Every phase writes its result next to the others, so a rerun resumes."""
    pop = population()
    done = [x for x in iteration_dirs() if (x / "record.json").exists()]
    it = len(done) + 1
    pending = f"psro{it:02d}"  # admitted by an interrupted run of this very iteration
    names = [nm for nm in pop.strategies if nm != pending]
    d = OUT / f"iter_{it:02d}"
    d.mkdir(parents=True, exist_ok=True)
    cfg = pin_config(d, cfg)
    log = d / "log.txt"
    players = cfg["players"]
    store = store_from_disk(cfg)
    gaps = store.missing(names, players)
    if gaps:
        raise SystemExit(f"{len(gaps)} compositions of the population have no simulated table, e.g. {gaps[0]}")

    if (d / "meta.json").exists():
        meta = json.loads((d / "meta.json").read_text())
    else:
        t0 = time.monotonic()
        meta = solve_meta(store, names, players, seed=cfg["solver_seed"], boot_reps=cfg["boot_reps"])
        write_atomic(d / "meta.json", json.dumps(meta, indent=1))
        print(f"[iter {it}] meta-game solved in {time.monotonic() - t0:.0f}s: "
              + "; ".join(f"n={n} {m['solver']} gain {m['gain']:.1e}" for n, m in meta.items()), flush=True)
    sigma = {n: np.array(m["sigma"]) for n, m in meta.items()}
    (d / "profile_train.json").write_text(json.dumps(
        dict(support=names, gamma=cfg["gamma"], weights={n: explore(s, cfg["gamma"]).tolist() for n, s in sigma.items()})))
    (d / "profile_eval.json").write_text(json.dumps(dict(support=names, weights={n: s.tolist() for n, s in sigma.items()})))

    init = best_pure_init(meta, pop, names)
    run_name = f"iter_{it:02d}"
    run_dir = ROOT / RUNS / run_name
    seed = cfg["seed"] + it
    length = LENGTH_NAMES[cfg["length"]]
    train_cfg = dict(utility=cfg["utility"], gamma=cfg["gamma"], seed=seed, minutes=cfg["minutes"], lr=cfg["lr"],
                     lr_decay=cfg["lr_decay"], entropy=cfg["entropy"], init=init, population=names, players=players,
                     length=length)
    if not (run_dir / "result.json").exists():
        aborted = run_dir / "metrics.jsonl"
        if aborted.exists():  # a trainer that was killed: its log must not be mixed with the one of the rerun
            aborted.rename(run_dir / f"metrics.aborted-{int(aborted.stat().st_mtime)}.jsonl")
        print(f"[iter {it}] training a response, init {init}, {cfg['minutes']} min", flush=True)
        sh([sys.executable, "-m", "training.br", "train", "--population", cfg["population"],
            "--profile", f"@{d / 'profile_train.json'}", "--players", ",".join(map(str, players)),
            "--length", length, "--name", run_name, "--init", str(pop.policy(init).path),
            "--budget-minutes", str(cfg["minutes"]),
            "--utility", cfg["utility"], "--seed", str(seed), "--lr", str(cfg["lr"]), "--lr-decay", cfg["lr_decay"],
            "--entropy", str(cfg["entropy"]),
            "--envs", str(cfg["envs"]), "--runs", RUNS, "--eval-minutes", str(cfg["eval_minutes"]),
            "--eval-seeds", str(cfg["eval_seeds"])], log)
    training = train_summary(run_dir)

    cand = pending  # the strategy name the response gets if it is admitted; the study labels it `psroNN/<mode>`
    modes = cfg["modes"].split(",")
    if not (d / "study.json").exists():
        print(f"[iter {it}] deviation study: {cfg['study_seeds']} dev seeds per player count", flush=True)
        sh([sys.executable, "-m", "training.br", "eval", "--population", cfg["population"],
            "--profile", f"@{d / 'profile_eval.json'}", "--players", ",".join(map(str, players)),
            "--length", length, "--candidates", f"{cand}={run_dir / 'final.pt'}", "--modes", cfg["modes"],
            "--block", "dev", "--seeds", str(cfg["study_seeds"]),
            "--seed-offset", str(fresh_study_offset(cfg, it)),
            "--envs", str(cfg["envs"]), "--out", str(d / "study"),
            "--title", f"PSRO iteration {it}: deviation study of the new response"], log)
    study = json.loads((d / "study.json").read_text())
    by_mode = {m: study["mean_over_players"]["gain"][f"{cand}/{m}"] for m in modes}
    mode = max(modes, key=lambda m: by_mode[m][0])  # a best response is not obliged to randomise
    label, gain = f"{cand}/{mode}", by_mode[mode]
    decision = dict(
        accepted=bool(gain[1] > cfg["min_gain"]), mode=mode, gain_by_mode=by_mode, gain=gain,
        nash_conv=study["mean_over_players"]["nashconv"][label],
        max_seat_gain=max(g[0] for b in study["by_players"].values() for g in b["by_seat"][label]),
        per_n={n: dict(gain=b["gain"][label], nash_conv=b["nashconv"][label], sigma=b["sigma"],
                       max_seat_gain=max(g[0] for g in b["by_seat"][label]),
                       best_pure=max(((k, v[0]) for k, v in b["gain"].items() if k in names), key=lambda kv: kv[1]))
               for n, b in study["by_players"].items()},
        rule=f"accept when the lower 95% bound of the mean gain over player counts, in the better of the modes "
             f"{', '.join(modes)}, is above {cfg['min_gain']}")
    new = None
    generalisation = None
    if decision["accepted"]:
        new = admit(pop, it, init, run_dir / "final.pt", train_cfg, decision)
        crossplay_new(pop, new, cfg, d)
        payoffs_path = d / "crossplay/payoffs.json"
        payoffs = json.loads(payoffs_path.read_text())
        record_trained(pop, new, players, [length], f"training.br train, profile of iteration {it}")
        record_crossplay(pop, payoffs, rel(payoffs_path))
        generalisation = field_row(payoffs, new)
    record_br(pop, study, rel(d / "study.json"))
    pop.save_results()
    record = dict(iteration=it, population=names, init=init, training=training, train_config=train_cfg,
                  meta=meta, decision=decision, new_member=new, generalisation=generalisation,
                  accounting=study["accounting"], study=rel(d / "study.json"),
                  results=rel(pop.path.with_name("results.json")), git=E.git_commit(),
                  finished=time.strftime("%Y-%m-%d %H:%M:%S"))
    write_atomic(d / "record.json", json.dumps(record, indent=1))
    (d / "report.md").write_text(render_iteration(record))
    render_summary()
    print(f"[iter {it}] gain over sigma {gain[0]:+.3f} [{gain[1]:+.3f}, {gain[2]:+.3f}] -> "
          f"{'added ' + new if new else 'rejected'}", flush=True)
    return record


def field_row(payoffs: dict, name: str) -> dict:
    """Utility of `name` at a table whose other seats all play one pure strategy, per player count.

    This is how the response does against each member on its own, including those it never trained against
    in isolation. 1/n means an equal share.
    """
    names = payoffs["strategies"]
    i = names.index(name)
    return {n: dict(zip(names, blk["field_matrix"]["mean"][i], strict=True)) for n, blk in payoffs["by_players"].items()}


def ci(x) -> str:
    return f"{x[0]:+.3f} [{x[1]:+.3f}, {x[2]:+.3f}]"


def oracle_label(train_cfg: dict, minutes: float) -> str:
    decay = " falling to 0" if train_cfg.get("lr_decay", "none") == "linear" else ""
    return f"{minutes:.0f} min, lr {train_cfg['lr']:g}{decay}"


def render_iteration(rec: dict) -> str:
    names, it = rec["population"], rec["iteration"]
    tr, dec = rec["training"], rec["decision"]
    lines = [f"# PSRO iteration {it}", "",
             f"Population before the iteration ({len(names)} strategies): {', '.join(names)}. Code version {rec['git']}.", "",
             "## 1. Meta-game solution of the population", ""] + render_meta(rec["meta"], names) + [
             "## 2. Best response", "",
             f"Initialised from `{rec['init']}` (the population strategy with the highest utility against sigma). "
             f"PPO against opponents drawn from sigma smoothed with gamma = {rec['train_config']['gamma']} towards the "
             f"uniform mixture; objective `{rec['train_config']['utility']}`, seed {rec['train_config']['seed']}, "
             f"Adam learning rate {rec['train_config']['lr']:g}"
             f"{' falling linearly to 0 over the budget' if rec['train_config'].get('lr_decay', 'none') == 'linear' else ''}. "
             f"{tr['games']:,} games, {tr['decisions']:,} decisions in {tr['minutes']:.0f} min. Utility during training "
             f"(canonical, against the smoothed profile): {tr['utility_first']:.3f} at the start, "
             f"{tr['utility_last']:.3f} at the end. Periodic dev gain over the smoothed profile: "
             + ", ".join(f"{c['minutes']:.0f} min {ci(c['gain'])}" for c in tr["curve"]) + ".", "",
             "## 3. Deviation study (seeds unseen by the training)", "",
             f"{rec['accounting']['games']:,} games, {rec['accounting']['stalled']} stalled. Full tables: `{rec['study']}`.",
             "The candidate's gain over sigma is a lower bound on sigma's true deviation gain.", "",
             "| n | value of sigma | candidate gain over sigma | NashConv (n x gain) | largest single-seat gain | "
             "best pure deviation in the population |", "|---|---|---|---|---|---|"]
    for n, b in dec["per_n"].items():
        lines.append(f"| {n} | {ci(b['sigma'])} | {ci(b['gain'])} | {ci(b['nash_conv'])} | {b['max_seat_gain']:+.3f} | "
                     f"{b['best_pure'][0]} ({b['best_pure'][1]:+.3f}) |")
    lines += [f"| mean | | {ci(dec['gain'])} | {ci(dec['nash_conv'])} | {dec['max_seat_gain']:+.3f} | |", "",
              f"The table is for the response played in `{dec['mode']}` mode, the better of the modes evaluated "
              "(mean gain over player counts: " + ", ".join(f"{m} {ci(g)}" for m, g in dec["gain_by_mode"].items())
              + "). The largest single-seat gain is the maximum over the engine seats of the per-seat mean gain. The "
              "seats are exchangeable (the first player is drawn at random), so each seat estimates the same individual "
              "regret, which is the mean gain; their maximum is a noisy overestimate and is shown for completeness.", "",
              f"## 4. Decision: {'**accepted** as ' + rec['new_member'] if dec['accepted'] else '**rejected**'}", "",
              f"Rule: {dec['rule']}.", ""]
    gen = rec.get("generalisation")
    if gen:
        names = rec["population"]
        lines += ["## 5. Against each member alone", "",
                  f"Utility of `{rec['new_member']}` at a table whose other seats all play the member in the column "
                  "(dev seeds of the cross-play; 1/n is an equal share). The response was trained against a mixture, so "
                  "this shows whether it also beats the members one by one.", "",
                  "| n | " + " | ".join(names) + " |", "|---|" + "---|" * len(names)]
        for n, row in gen.items():
            lines.append(f"| {n} | " + " | ".join("—" if np.isnan(row[nm]) else f"{row[nm]:.3f}" for nm in names) + " |")
        lines.append("")
    return "\n".join(lines)


def render_summary() -> None:
    rows = []
    for d in iteration_dirs():
        if (d / "record.json").exists():
            rows.append(json.loads((d / "record.json").read_text()))
    lines = ["# PSRO progress", "",
             "One row per iteration. `gain` is the deviation gain of the newly trained response over the meta-strategy sigma "
             "it was trained against (mean over player counts, 95% interval, dev seeds unseen by the training); it is a "
             "lower bound on the exploitability of sigma, and NashConv = n x gain at table size n. `restricted gain` is "
             "the deviation gain of sigma inside the population itself (0 for a Nash equilibrium of the empirical game). "
             "`oracle curve` is the response's own periodic dev gain over the smoothed profile by minutes of training. "
             "Nothing here is a claim about the full game.", "",
             "| iter | strategies | sigma_3 / sigma_4 / sigma_5 support | restricted gain (max over n) | gain of the new response | "
             "NashConv mean over n | largest single-seat gain | stalled / games | oracle curve | decision | oracle |",
             "|---|---|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        names = r["population"]
        sup = " / ".join("+".join(f"{nm}" for nm, w in zip(names, m["sigma"]) if w > 0.05) for m in r["meta"].values())
        dec = r["decision"]
        acc = r["accounting"]
        curve = ", ".join(f"{c['minutes']:.0f}: {c['gain'][0]:+.3f}" for c in r["training"]["curve"])
        lines.append(f"| {r['iteration']} | {len(names)} | {sup} | {max(m['gain'] for m in r['meta'].values()):.1e} | "
                     f"{ci(dec['gain'])} ({dec['mode']}) | {ci(dec['nash_conv'])} | {dec['max_seat_gain']:+.3f} | "
                     f"{acc['stalled']} / {acc['games']:,} | {curve} | "
                     f"{'added ' + r['new_member'] if dec['accepted'] else 'rejected'} | "
                     f"{oracle_label(r['train_config'], r['training']['minutes'])} |")
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "summary.md").write_text("\n".join(lines) + "\n")


# ------------------------------------------------------------------------------------------ command line


def cmd_init(args) -> None:
    src = Population.load(args.source)
    names = args.strategies.split(",")
    pop = Population(POP_DIR / "manifest.json", src.header)
    for nm in names:
        s = src.strategies[nm]
        member = src.members[s.member]
        if member.name not in pop.members:
            copy = Member(**member.__dict__)
            if member.checkpoint is not None:
                dst = POP_DIR / Path(member.checkpoint).name
                dst.parent.mkdir(parents=True, exist_ok=True)
                if dst.exists():
                    dst.chmod(0o644)
                shutil.copyfile(src.path.parent / Path(member.checkpoint).name, dst)
                dst.chmod(0o444)
                copy.checkpoint = rel(dst)
            pop.add_member(copy)
        pop.add_strategy(nm, s.member, s.mode)
    pop.save()
    problems = pop.verify()
    if problems:
        raise SystemExit("\n".join(problems))
    cfg = dict(population=rel(pop.path), cells=args.cells.split(","), players=[int(p) for p in args.players.split(",")])
    for key in TUNABLE:
        if getattr(args, key, None) is not None:
            cfg[key] = getattr(args, key)
    OUT.mkdir(parents=True, exist_ok=True)
    CONFIG.write_text(json.dumps(DEFAULTS | cfg, indent=2) + "\n")
    store = store_from_disk(DEFAULTS | cfg)
    gaps = store.missing(names, cfg["players"])
    print(f"population {rel(pop.path)}: {len(pop.strategies)} strategies, {len(gaps)} compositions without a table")
    if gaps:
        raise SystemExit(f"cross-play the population first, e.g. missing {gaps[0]}")


def with_overrides(args) -> dict:
    """The saved configuration with the command-line settings applied. A setting that differs from the file is
    written back, so that `config.json` always says what the loop runs with and a rerun without the flag does
    not fall back to the old value."""
    cfg = load_config()
    changed = {}
    for key in TUNABLE:
        value = getattr(args, key, None)
        if value is not None and cfg[key] != value:
            changed[key] = (cfg[key], value)
            cfg[key] = value
    if changed:
        write_atomic(CONFIG, json.dumps(cfg, indent=2) + "\n")
        print(f"settings changed in {rel(CONFIG)} (old, new): {changed}", flush=True)
    return cfg


def cmd_step(args) -> None:
    step(with_overrides(args))


def profile_text(profile: dict) -> str:
    """The weights of a profile file (`support` and `weights` by player count), without the strategies it never plays."""
    return "; ".join(f"{n} players: " + ", ".join(f"{s} {w:.2f}" for s, w in zip(profile["support"], ws, strict=True) if w >= 0.005)
                     for n, ws in profile["weights"].items())


def top_gains(study: dict, top: int = 6) -> list[str]:
    """The focals that gain most over the attacked profile (mean over player counts), with the NashConv they imply."""
    gain, nash = study["mean_over_players"]["gain"], study["mean_over_players"]["nashconv"]
    rows = ["| focal | gain over the profile | NashConv (n x gain, mean over n) |", "|---|---|---|"]
    for f in sorted(gain, key=lambda f: -gain[f][0])[:top]:
        kind = " (trained, not in the population)" if f in study["candidates"] else ""
        rows.append(f"| {f}{kind} | {ci(gain[f])} | {ci(nash[f])} |")
    return rows


def beyond_best_member(records: list[dict]) -> list[str]:
    """Each response against the best member of the population it was measured with, on the same dev seeds."""
    rows = ["| iteration | decision | response | its gain over sigma | best member of the population | its gain | "
            "response minus member (paired) |", "|---|---|---|---|---|---|---|"]
    for r in records:
        it = r["iteration"]
        study = json.loads((OUT / f"iter_{it:02d}" / "study.json").read_text())
        label = f"psro{it:02d}/{r['decision']['mode']}"
        member, member_gain = br_report.best_pure(study)
        rows.append(f"| {it} | {'added' if r['decision']['accepted'] else 'rejected'} | {label} | "
                    f"{ci(study['mean_over_players']['gain'][label])} | {member} | {ci(member_gain)} | "
                    f"{ci(br.paired_difference(study, label, member))} |")
    return rows


def kept_study_differs(study: dict, *, pop: Population, profile: Path, candidates: list[str], modes: str,
                       **asked) -> str | None:
    """How a study kept on disk differs from the look that is asked for (None: it is that look).

    `asked` holds block, seeds, seed_offset, players and length; `candidates` are `label=checkpoint` items."""
    for key, value in asked.items():
        if study[key] != value:
            return f"{key} was {study[key]}, it is {value} now"
    support, weights = br.parse_profile(f"@{profile}", pop)
    now, kept = br.weights_json(weights), study["profile"]
    if kept["support"] != support or kept["weights"].keys() != now.keys() or not all(
            np.allclose(kept["weights"][n], now[n], atol=1e-9) for n in now):
        return "it attacked another profile"
    labels = {f"{c.partition('=')[0]}/{m}": sha256_file(c.partition("=")[2]) for c in candidates for m in modes.split(",")}
    played = {p["name"]: p["sha256"] for p in study["policies"] if p["name"] in labels}
    if set(study["focals"]) != {*pop.strategies, *labels} or set(study["candidates"]) != set(labels) or played != labels:
        return "it played other strategies or other checkpoints"
    return None


def cmd_final(args) -> None:
    """The closing audit of a run, on `HOLDOUT` seeds (each look is logged in reports/holdout_log.jsonl).

    1. The meta-strategy that the last response was trained against is attacked by every strategy of the
       population and by every response that was not admitted.
    2. The meta-game of the final population is solved; every strategy of the population deviates from that
       solution on seeds the solver never saw (no new training), which is its restricted-game regret re-measured.
    3. Each response is compared with the best member of the population it was measured against, on the same dev
       seeds."""
    cfg = load_config()
    pop = population()
    names = list(pop.strategies)
    records = [json.loads((d / "record.json").read_text()) for d in iteration_dirs() if (d / "record.json").exists()]
    if not records:
        raise SystemExit("no iteration has finished: there is nothing to audit")
    last = records[-1]["iteration"]
    players = cfg["players"]
    store = store_from_disk(cfg)
    gaps = store.missing(names, players)
    if gaps:
        raise SystemExit(f"{len(gaps)} compositions of the population have no simulated table, e.g. {gaps[0]}")
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    log = out / "log.txt"
    rejected = []  # an admitted response is a strategy of the population and plays as a focal in the mode it was admitted in
    for r in records:
        if r["decision"]["accepted"]:
            continue
        ckpt = ROOT / RUNS / f"iter_{r['iteration']:02d}" / "final.pt"
        if not ckpt.exists():
            raise SystemExit(f"iteration {r['iteration']} was rejected and {ckpt} is gone: the audit would leave it out")
        rejected.append(f"iter{r['iteration']:02d}={ckpt}")

    def attack(stem: str, profile: Path, offset: int, candidates: list[str], title: str) -> dict:
        target = out / f"{stem}.json"
        if target.exists():
            problem = kept_study_differs(json.loads(target.read_text()), pop=pop, profile=profile, candidates=candidates,
                                         modes=cfg["modes"], block=E.HOLDOUT, seeds=args.seeds, seed_offset=offset,
                                         players=players, length=cfg["length"])
            if problem:
                raise SystemExit(f"{rel(target)} is not the look this run asks for ({problem}); delete it to take that look")
            print(f"{rel(target)} is the look this run asks for and is reused; delete it to look at the holdout block again",
                  flush=True)
        else:
            cmd = [sys.executable, "-m", "training.br", "eval", "--population", cfg["population"], "--profile", f"@{profile}",
                   "--players", ",".join(map(str, players)), "--length", LENGTH_NAMES[cfg["length"]], "--block", "holdout",
                   "--seeds", str(args.seeds), "--seed-offset", str(offset), "--envs", str(cfg["envs"]),
                   "--out", str(out / stem), "--title", title]
            if candidates:
                cmd += ["--candidates", ",".join(candidates), "--modes", cfg["modes"]]
            sh(cmd + ["--all-focals"], log)
        return json.loads(target.read_text())

    print(f"[final] attacking the meta-strategy of iteration {last} with {len(names)} strategies and "
          f"{len(rejected)} responses that were not admitted, {args.seeds} holdout seeds per player count", flush=True)
    sigma_last = OUT / f"iter_{last:02d}" / "profile_eval.json"
    attacked = attack("holdout_sigma_last", sigma_last, args.seed_offset, rejected,
                      f"PSRO, holdout study of the meta-strategy that iteration {last} attacked")
    picked_last = br_report.render_selection(attacked, br_report.pools_of(attacked))
    write_atomic(out / "holdout_sigma_last_selection.md", picked_last)

    meta = solve_meta(store, names, players, seed=cfg["solver_seed"], boot_reps=cfg["boot_reps"])
    write_meta(meta, names, out / "final_metagame", "PSRO, restricted game of the final population")
    solution = out / "profile_final.json"
    write_atomic(solution, json.dumps(dict(support=names, weights={str(n): m["sigma"] for n, m in meta.items()})))
    print("[final] auditing the solution of the final restricted game", flush=True)
    audited = attack("holdout_sigma_final", solution, args.seed_offset + args.seeds, [],
                     "PSRO, holdout study of the solution of the final restricted game")
    picked_final = br_report.render_selection(audited, br_report.pools_of(audited))
    write_atomic(out / "holdout_sigma_final_selection.md", picked_final)

    added = [r["new_member"] for r in records if r["decision"]["accepted"]]
    refused = [str(r["iteration"]) for r in records if not r["decision"]["accepted"]]
    solver = "; ".join(f"{n} players: {m['solver']}, regret on the simulated cells {m['gain']:.1e}, noise floor of the cells "
                       f"(bootstrap) mean {m['bootstrap']['mean']:.3f}, 95th percentile {m['bootstrap']['p95']:.3f}"
                       for n, m in meta.items())
    unsolved = [n for n, m in meta.items() if not m["is_nash"]]
    by_construction = ("These gains are exact in the empirical game and about 0 by construction" if not unsolved else
                       "These gains are exact in the empirical game; they are about 0 by construction only where a solver "
                       f"reached a Nash equilibrium of it (not at {', '.join(unsolved)} players, where the solution is the "
                       "smallest-gain output of any solver)")
    lines = [
        "# PSRO, closing audit on the holdout block", "",
        f"Population ({len(names)}): {', '.join(names)}. {len(records)} iterations finished; responses added: "
        f"{', '.join(added) or 'none'}; iterations whose response was rejected: {', '.join(refused) or 'none'}. "
        f"Utility: winner share (a stalled game pays 0). Each study uses {args.seeds} holdout seeds per player count "
        f"({', '.join(map(str, players))} players, {LENGTH_NAMES[cfg['length']]} length) at seed offsets {args.seed_offset} and "
        f"{args.seed_offset + args.seeds}; every look at the holdout block is in `reports/holdout_log.jsonl`. "
        "A gain measured here is a lower bound on the exploitability of the profile that was attacked: the oracle is "
        "only an approximate best response and the deviations are limited to the strategies listed. Nothing here is a "
        "statement about the full game.", "",
        f"## 1. The meta-strategy that iteration {last} attacked", "",
        f"sigma: {profile_text(attacked['profile'])}.", "",
        *top_gains(attacked), "",
        "The focal that looks best is optimistic by having been picked; picked on one half of the seeds and measured "
        "on the other (`holdout_sigma_last_selection.md`):", "", picked_last,
        "Full study: `holdout_sigma_last.md`.", "",
        "## 2. The solution of the final restricted game", "",
        f"sigma: {profile_text(audited['profile'])}.", "",
        f"Solver, from the cross-play cells of the final population: {solver}. {by_construction}; the study below "
        "re-measures them on holdout seeds, so what it finds is the estimation error of the cells plus the real "
        "deviation gain inside the population.", "",
        *top_gains(audited), "",
        "Picked on one half of the seeds, measured on the other (`holdout_sigma_final_selection.md`):", "", picked_final,
        "Full study: `holdout_sigma_final.md`. The population only contains what was trained so far, so a small gain "
        "here says nothing about strategies outside it.", "",
        "## 3. Each response against the best member of the population it was measured against (dev seeds, paired)", "",
        *beyond_best_member(records), "",
        "A response that is no better than the best member has not been shown to be a new strategy; the admission "
        "rule of the loop compares it with sigma, which carries the estimation error of the cells.", ""]
    write_atomic(out / "report.md", "\n".join(lines))
    print(f"[final] wrote {rel(out / 'report.md')}", flush=True)


def cmd_admit(args) -> None:
    """Admits a response that `br train` made outside the loop, on the strength of its dev study and of a study on
    holdout seeds, by the rule of `step` (`response_decision`)."""
    cfg, pop = load_config(), population()
    study = json.loads(Path(args.study).read_text())
    confirm = json.loads(Path(args.confirm).read_text()) if args.confirm else None
    decision = response_decision(study, args.candidate, cfg["min_gain"], confirm)
    if not decision["accepted"]:
        raise SystemExit(f"not admitted: {json.dumps(decision)}")
    run_dir = Path(args.run).resolve()
    run = json.loads((run_dir / "config.json").read_text())
    profile = json.loads((ROOT / run["profile"].lstrip("@")).read_text())
    train_cfg = dict(utility=run["utility"], gamma=profile.get("gamma"), seed=run["seed"], minutes=run["budget_minutes"],
                     lr=run["lr"], lr_decay=run["lr_decay"], entropy=run["entropy"], init=args.init,
                     population=run["support"], players=[int(p) for p in run["players"].split(",")], length=run["length"])
    objective = (f"PPO best response to {run['profile'].lstrip('@')} (utility {run['utility']}, exploration gamma "
                 f"{profile.get('gamma')}), trained outside the loop")
    name = admit(pop, args.iteration, args.init, run_dir / "final.pt", train_cfg, decision, objective)
    record_trained(pop, name, train_cfg["players"], [train_cfg["length"]], f"training.br train, {rel(run_dir)}")
    for path, one in ((args.study, study), (args.confirm, confirm)):
        if one is not None:
            record_br(pop, one, rel(Path(path).resolve()))
    pop.save_results()
    print(f"admitted {name} in {decision['mode']} mode: gain over sigma {decision['gain'][0]:+.3f} "
          f"[{decision['gain'][1]:+.3f}, {decision['gain'][2]:+.3f}] on dev seeds"
          + (f", {decision['confirmation']['gain'][0]:+.3f} [{decision['confirmation']['gain'][1]:+.3f}, "
             f"{decision['confirmation']['gain'][2]:+.3f}] on holdout seeds" if confirm else ""))


def cmd_run(args) -> None:
    cfg = with_overrides(args)
    for _ in range(args.iterations):
        if (OUT / "STOP").exists():
            print(f"{rel(OUT / 'STOP')} exists: not starting another iteration", flush=True)
            break
        if args.no_start_after:
            hh, mm = map(int, args.no_start_after.split(":"))
            if dt.datetime.now(dt.UTC).time() >= dt.time(hh, mm):
                print(f"past {args.no_start_after} UTC: not starting another iteration", flush=True)
                break
        step(cfg)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    tuning = argparse.ArgumentParser(add_help=False)
    tuning.add_argument("--minutes", type=float, help="PPO budget of each response")
    tuning.add_argument("--lr", type=float, help="learning rate of the response")
    tuning.add_argument("--lr-decay", choices=("none", "linear"), help="linear: the learning rate falls to zero over the budget")
    tuning.add_argument("--entropy", type=float, help="entropy bonus of the response")
    tuning.add_argument("--min-gain", dest="min_gain", type=float)
    tuning.add_argument("--study-seeds", dest="study_seeds", type=int)
    tuning.add_argument("--gamma", type=float, help="exploration: share of the uniform mixture in the training opponents")
    tuning.add_argument("--utility", choices=["canonical", "shaped"])
    tuning.add_argument("--seed", type=int, help="base seed; iteration k trains with seed + k")
    tuning.add_argument("--cell-seeds", dest="cell_seeds", type=int, help="dev seeds of the cross-play of a new member")
    tuning.add_argument("--envs", type=int, help="parallel games of the trainer and of the simulations")
    tuning.add_argument("--eval-minutes", dest="eval_minutes", type=float, help="minutes between the trainer's dev studies")
    tuning.add_argument("--eval-seeds", dest="eval_seeds", type=int, help="dev seeds of those periodic studies")
    tuning.add_argument("--modes", help="modes in which the response is evaluated; the better one is admitted, "
                                        "e.g. sample,guarded")
    ini = sub.add_parser("init", parents=[tuning])
    ini.add_argument("--source", default=str(ROOT / "populations/baselines/manifest.json"))
    ini.add_argument("--strategies", required=True)
    ini.add_argument("--cells", required=True, help="comma separated cross-play payoffs.json files covering the strategies")
    ini.add_argument("--players", default="3,4,5")
    sub.add_parser("step", parents=[tuning])
    run = sub.add_parser("run", parents=[tuning])
    run.add_argument("--iterations", type=int, default=1)
    run.add_argument("--no-start-after", help="HH:MM UTC after which no new iteration starts")
    sub.add_parser("report")
    fin = sub.add_parser("final", help="closing audit of a run on the holdout block")
    fin.add_argument("--seeds", type=int, default=150, help="holdout seeds per player count in each study")
    fin.add_argument("--seed-offset", dest="seed_offset", type=int, default=200,
                     help="first holdout seed of the attack on the last meta-strategy (the audit of the final "
                          "solution starts `--seeds` later)")
    fin.add_argument("--out", default=str(OUT / "final"))
    adm = sub.add_parser("admit", help="admit a response that `br train` made outside the loop")
    adm.add_argument("--run", required=True, help="run directory of `br train` (config.json and final.pt)")
    adm.add_argument("--study", required=True, help="dev deviation study of the response (`br eval`)")
    adm.add_argument("--confirm", help="study on holdout seeds in which the response was played in the same modes")
    adm.add_argument("--candidate", required=True, help="label of the response in those studies")
    adm.add_argument("--init", required=True, help="strategy whose net the response started from")
    adm.add_argument("--iteration", type=int, required=True, help="the response becomes psro<NN> with this number")
    sol = sub.add_parser("solve")
    sol.add_argument("--cells", required=True)
    sol.add_argument("--strategies", required=True)
    sol.add_argument("--players", default="3,4,5")
    sol.add_argument("--seed", type=int, default=0)
    sol.add_argument("--boot-reps", type=int, default=200)
    sol.add_argument("--out", required=True, help="path without extension")
    sol.add_argument("--title", default="Empirical meta-game")
    args = ap.parse_args()
    {"init": cmd_init, "step": cmd_step, "run": cmd_run, "solve": cmd_solve, "final": cmd_final, "admit": cmd_admit,
     "report": lambda a: render_summary()}[args.cmd](args)


if __name__ == "__main__":
    main()
