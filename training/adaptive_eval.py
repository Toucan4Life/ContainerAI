"""One look at a net as a seat among the heuristics and among other nets: the evaluation of the adaptive generalist.

    python -m training.adaptive_eval run    --net runs/strategist/adaptive_s1/final.pt --out reports/strategist/adaptive --block holdout
    python -m training.adaptive_eval report --out reports/strategist/adaptive

`run --only adaptive|peddler2|control_s1` plays one focus net, so the three can run as three processes (all-net tables are slow and the
games use one core each); `run` without it plays them one after the other and writes the report.

The net under test is the focus seat (policy 0) of every table and sits at every seat in turn. The other seats are (1) every composition
of Producer, Showcase and Shipper and (2) identical copies of each of Peddler v2, Magnate and Consortium. The same tables, seeds and seat
rotations are played with two reference nets in the focus seat, Peddler v2 (where the generalist started) and the Phase 2 control
(`runs/strategist/control_s1`: 30 minutes of the old pool, which did seat Peddler v2 and Consortium), so every comparison is paired on
the seeds. Nets play as deployed (`guarded`) under the five table rules the generalist was trained with. Intervals are 95% bootstraps
over seeds (`training/egta.py`) and every table size is its own game. No pass mark is set; the report says what the net does.

Two limits. The focus seat's own defaults cannot be counted (a net uses the Bank, so the ledger's residual is not the unpaid
interest), only the heuristics' (they never call it). A good score against the heuristics is not general strength: the Shipper's dead
seats at 3 seats and their Bank-free play are weaknesses a net can exploit.
"""

from __future__ import annotations

import argparse
import itertools
import json
from pathlib import Path

import numpy as np

from training import egta
from training import evalcore as E
from training.audit_interaction import parse
from training.env import PRODUCER, SHIPPER, SHOWCASE, STANDARD
from training.strategy_metrics import NETS, configure_table

SIZES = (3, 4, 5)
HEURISTICS = {"producer": PRODUCER, "showcase": SHOWCASE, "shipper": SHIPPER}
SHORT = {1: "P", 2: "Sc", 3: "Sh"}
HELD_OUT = ("peddler2", "magnate", "consortium")
RULES = NETS["peddler2"].partition(":")[2]  # the five table rules the generalist is trained and shipped with
CONTROL = "runs/strategist/control_s1/final.pt"
EXPERIMENTS = ("heuristics", "nets")
FOCUS_METRICS = ("score", "margin", "produced", "sold", "bought", "deliveries", "factories_built", "warehouses_built", "loans_taken")
RESPONSE_METRICS = ("factories_built", "warehouses_built", "bought", "deliveries", "sold", "produced")


def foci(net: str) -> dict[str, str]:
    """Name -> `path:rules` of each net that takes the focus seat; the one under test first."""
    return {"adaptive": f"{net}:{RULES}", "peddler2": NETS["peddler2"], "control_s1": f"{CONTROL}:{RULES}"}


def opponents(experiment: str) -> list[tuple[E.Policy, dict]]:
    """The policies of indices 1, 2 and 3, with the table rules each plays under."""
    if experiment == "heuristics":
        return [(E.Policy(name, scripted=kind), {}) for name, kind in HEURISTICS.items()]
    out = []
    for name in HELD_OUT:
        _, path, rules = parse(f"{name}={NETS[name]}")
        out.append((E.Policy(name, path, "guarded"), rules))
    return out


def compositions(experiment: str, n: int) -> list[tuple[int, ...]]:
    """The opponents' policy indices at a table of `n`: every multiset of the heuristics, or `n - 1` copies of one net."""
    if experiment == "heuristics":
        return list(itertools.combinations_with_replacement((1, 2, 3), n - 1))
    return [(k,) * (n - 1) for k in (1, 2, 3)]


def focus_specs(experiment: str, sizes, seeds: int, block: int) -> list[E.GameSpec]:
    """Every composition, `seeds` seeds each; one seed seats the focus at every seat in turn, the opponents in one fixed order."""
    specs = []
    for n in sizes:
        for c, opp in enumerate(compositions(experiment, n)):
            for s in range(seeds):
                order = [int(x) for x in np.random.default_rng([n, c, s]).permutation(opp)]
                seed = E.game_seed(block, n, STANDARD, s)
                for r in range(n):
                    specs.append(E.GameSpec(n, STANDARD, seed, tuple(order[:r] + [0] + order[r:]), n * 10_000 + c, seed))
    return specs


def play_focus(experiment: str, focus: str, specs: list[E.GameSpec]) -> E.Records:
    _, path, rules = parse(f"focus={focus}")
    others = opponents(experiment)
    policies = [E.Policy("focus", path, "guarded"), *(p for p, _ in others)]
    ruled = [(E.SLOT_BASE + k, r) for k, r in enumerate([rules] + [r for _, r in others]) if policies[k].path]
    return E.play(specs, policies, log=None, configure=lambda env: configure_table(env, ruled))


def run(out: Path, net: str, seeds: int, block: str = "dev", sizes=SIZES, log=print, only: str | None = None) -> None:
    """Plays both experiments for the three nets in the focus seat; every finished file is kept, so an interrupted run resumes.

    `only` plays a single focus net, so that the three can run as three processes (start the first one alone: it writes the design
    and, on the holdout block, the log entry). The report needs all three."""
    out.mkdir(parents=True, exist_ok=True)
    design = dict(net=net, seeds=seeds, block=block, sizes=list(sizes))
    pinned = out / "design.json"
    if pinned.exists() and json.loads(pinned.read_text()) != design:
        raise SystemExit(f"{out} was started with another design ({pinned.read_text().strip()}); use a fresh --out")
    first = not pinned.exists()
    pinned.write_text(json.dumps(design) + "\n")
    blocks = {"dev": E.DEV, "holdout": E.HOLDOUT}
    if block == "holdout" and first:
        E.log_holdout(dict(command="adaptive_eval run", net=net, sha256=E.sha256_file(net), seeds=seeds, sizes=list(sizes), out=str(out)))
    for experiment in EXPERIMENTS:
        specs = focus_specs(experiment, sizes, seeds, blocks[block])
        for name, focus in foci(net).items():
            path = out / f"{experiment}_{name}.npz"
            if path.exists() or only not in (None, name):
                continue
            rec = play_focus(experiment, focus, specs)
            np.savez_compressed(path, **{f: getattr(rec, f) for f in E.ARRAYS}, interaction=rec.interaction,
                                breakdown=rec.breakdown, meta=json.dumps(rec.meta))
            log(f"{experiment} / {name}: {len(specs):,} games in {rec.meta['seconds']} s, {int(rec.stalled.sum())} stalled")


# --------------------------------------------------------------------------------------- report


class Played:
    """The games of one experiment with one net in the focus seat, as per-seat rows."""

    def __init__(self, path: Path):
        self.rec, interaction, breakdown = egta.read_chunk(path)
        self.t = egta.seat_table(self.rec, interaction, breakdown)

    def rows(self, n: int, arche: int = 0, cells=None) -> np.ndarray:
        parts = [r for (cell, a), r in self.t.by.items() if a == arche and cell // 10_000 == n and (cells is None or cell in cells)]
        return np.concatenate(parts) if parts else np.zeros(0, dtype=np.int64)

    def sums(self, n: int, metric: str, arche: int = 0, cells=None):
        return egta.seed_sums(self.t, self.rows(n, arche, cells), metric)

    def stat(self, n: int, metric: str, arche: int = 0, cells=None) -> list[float]:
        return egta.interval(egta.point(s := self.sums(n, metric, arche, cells)), egta.resampled(s))

    def stalled(self, n: int, cells=None) -> tuple[int, int]:
        here = self.rec.players == n
        if cells is not None:
            here &= np.isin(self.rec.cell, list(cells))
        return int(self.rec.stalled[here].sum()), int(here.sum())


def ci(s, digits: int = 3) -> str:
    return "n/a" if np.isnan(s[0]) else f"{s[0]:.{digits}f} [{s[1]:.{digits}f}, {s[2]:.{digits}f}]"


def diff(s, digits: int = 3) -> str:
    return "n/a" if np.isnan(s[0]) else f"{s[0]:+.{digits}f} [{s[1]:+.{digits}f}, {s[2]:+.{digits}f}]"


def paired(a: Played, b: Played, n: int, metric: str, cells=None) -> list[float]:
    return egta.contrast(a.sums(n, metric, cells=cells), b.sums(n, metric, cells=cells))


def label(experiment: str, composition: tuple[int, ...]) -> str:
    return " ".join(SHORT[k] for k in composition) if experiment == "heuristics" else f"{HELD_OUT[composition[0] - 1]} x{len(composition)}"


def among_heuristics(p: dict[str, Played], sizes) -> list[str]:
    lines = ["## 1. One net seat among the heuristics", "",
             "The net sits at every seat in turn; the other seats are every composition of Producer (P), Showcase (Sc) and Shipper (Sh), "
             "each composition with the same seeds for every net. A stalled game pays nobody (`win`, `margin` count it as 0, the rest leave it out).", ""]
    for n in sizes:
        lines += [f"### {n} seats (an even split is {1 / n:.3f})", "",
                  "| net | win share | score | margin | produced | sold | bought | deliveries | factories built | warehouses built | loans | "
                  "opponents defaulted | stalled |", "|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
        for name, played in p.items():
            stall = played.stalled(n)
            opp = np.concatenate([played.rows(n, k) for k in (1, 2, 3)])
            defaulted = egta.stat(played.t, opp, "defaulted")[0]
            row = [ci(played.stat(n, "win")), ci(played.stat(n, "score"), 1), ci(played.stat(n, "margin"), 1)]
            row += [f"{played.stat(n, m)[0]:.2f}" for m in FOCUS_METRICS[2:]]
            lines.append(f"| {name} | " + " | ".join(row) + f" | {defaulted:.1%} | {stall[0]} of {stall[1]:,} |")
        lines.append("")
        for ref in ("peddler2", "control_s1"):
            lines.append(f"Adaptive minus {ref}, paired on the seeds: win share {diff(paired(p['adaptive'], p[ref], n, 'win'))}, "
                         f"score {diff(paired(p['adaptive'], p[ref], n, 'score'), 1)}, margin {diff(paired(p['adaptive'], p[ref], n, 'margin'), 1)}.  ")
        lines += ["", "By composition of the opponents (win share of the net in the focus seat):", "",
                  "| opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 |", "|---|---|---|---|---|"]
        for c, comp in enumerate(compositions("heuristics", n)):
            cell = {n * 10_000 + c}
            lines.append(f"| {label('heuristics', comp)} | {ci(p['adaptive'].stat(n, 'win', cells=cell))} | "
                         f"{p['peddler2'].stat(n, 'win', cells=cell)[0]:.3f} | {p['control_s1'].stat(n, 'win', cells=cell)[0]:.3f} | "
                         f"{diff(paired(p['adaptive'], p['peddler2'], n, 'win', cell))} |")
        lines.append("")
    return lines


def among_nets(p: dict[str, Played], sizes) -> list[str]:
    lines = ["## 2. One net seat among copies of a held-out net", "",
             "The other seats are all the same net. The generalist never met Peddler v2, Magnate or Consortium in training "
             "(Peddler v2 is where it started; the control did train against Peddler v2 and Consortium). "
             "Peddler v2 against copies of itself is the check of the harness: an even split.", "",
             "| seats | opponents | adaptive | peddler2 | control_s1 | adaptive minus peddler2 | adaptive minus control_s1 | stalled (adaptive, peddler2, control_s1) |",
             "|---|---|---|---|---|---|---|---|"]
    for n in sizes:
        for c, comp in enumerate(compositions("nets", n)):
            cell = {n * 10_000 + c}
            stalls = ", ".join(str(p[k].stalled(n, cell)[0]) for k in p)
            lines.append(f"| {n} | {label('nets', comp)} | {ci(p['adaptive'].stat(n, 'win', cells=cell))} | "
                         f"{ci(p['peddler2'].stat(n, 'win', cells=cell))} | {ci(p['control_s1'].stat(n, 'win', cells=cell))} | "
                         f"{diff(paired(p['adaptive'], p['peddler2'], n, 'win', cell))} | "
                         f"{diff(paired(p['adaptive'], p['control_s1'], n, 'win', cell))} | {stalls} of {p['adaptive'].stalled(n, cell)[1]:,} |")
    return lines + [""]


def largest(played: Played, n: int, metric: str, field: dict) -> tuple[tuple[int, int], list[float], float]:
    """The pair of fields with the largest paired difference of `metric` in SDs of the seat's per-game value: (pair, [diff, lo, hi], SD)."""
    sums = {k: played.sums(n, metric, cells=c) for k, c in field.items()}
    found = []
    for j, k in itertools.combinations((1, 2, 3), 2):
        sd = egta.spread(played.t, [played.rows(n, 0, field[j]), played.rows(n, 0, field[k])], metric) or 1.0
        found.append(((j, k), egta.contrast(sums[j], sums[k]), sd))
    return max(found, key=lambda f: abs(f[1][0] / f[2]))


def responds(d, sd: float) -> str:
    return "yes" if abs(d[0] / sd) >= egta.EFFECT and (d[1] > 0 or d[2] < 0) else "no"


def double(a, b, c, d) -> list[float]:
    """[(a - b) - (c - d), lo, hi], paired on the seeds."""
    return egta.interval(egta.point(a) - egta.point(b) - egta.point(c) + egta.point(d),
                         egta.resampled(a) - egta.resampled(b) - egta.resampled(c) + egta.resampled(d))


def adaptation(p: dict[str, Played], sizes) -> list[str]:
    lines = ["## 3. Does the net's play depend on its opponents?", "",
             "The net's own counters per game when all the opponents are Producers, all Showcases or all Shippers, and the largest paired "
             "difference between two such fields in standard deviations of the seat's per-game value (the convention of `training/egta.py`: "
             "a response from 0.2 with the interval excluding 0). Then the smallest change of field: one opponent swapped from a Producer to a Shipper. "
             "Any net's counters move with the field, because the field changes what there is to buy, sell and deliver: "
             "that is why a reference net is read next to it (and why the flags below mostly say yes).", "",
             "| seats | net | counter | vs Producers | vs Showcases | vs Shippers | largest difference [95% CI] | in SDs | responds | one Producer swapped for a Shipper [95% CI] | in SDs | responds |",
             "|---|---|---|---|---|---|---|---|---|---|---|---|"]
    versus = ["### Against the references", "",
              "For the pair of fields where the generalist's own response is largest, the same difference for a reference net, and the difference "
              "between the two responses (a difference in differences, paired on the seeds) in the same SDs: did training against the heuristics "
              "change how the net answers its field? Peddler v2 is where the generalist started; `control_s1` is the same recipe on the "
              "older pool without the heuristics.", "",
              "| seats | counter | fields | adaptive [95% CI] | reference | reference [95% CI] | adaptive minus reference [95% CI] | in SDs | answers differently |",
              "|---|---|---|---|---|---|---|---|---|"]
    for n in sizes:
        comps = compositions("heuristics", n)
        field = {k: {n * 10_000 + comps.index((k,) * (n - 1))} for k in (1, 2, 3)}
        old, new = {n * 10_000 + comps.index((1,) * (n - 1))}, {n * 10_000 + comps.index((1,) * (n - 2) + (3,))}
        for name, played in p.items():
            for m in RESPONSE_METRICS:
                (j, k), d, sd = largest(played, n, m, field)
                swap = egta.contrast(played.sums(n, m, cells=new), played.sums(n, m, cells=old))
                swap_sd = egta.spread(played.t, [played.rows(n, 0, old), played.rows(n, 0, new)], m) or 1.0
                lines.append(
                    f"| {n} | {name} | {m} | " + " | ".join(f"{egta.point(played.sums(n, m, cells=field[f])):.2f}" for f in (1, 2, 3))
                    + f" | {SHORT[j]} minus {SHORT[k]}: {diff(d, 2)} | {d[0] / sd:+.2f} | {responds(d, sd)}"
                    + f" | {diff(swap, 2)} | {swap[0] / swap_sd:+.2f} | {responds(swap, swap_sd)} |")
        for m in RESPONSE_METRICS:
            (j, k), d, sd = largest(p["adaptive"], n, m, field)
            for ref in ("peddler2", "control_s1"):
                ours = [p["adaptive"].sums(n, m, cells=field[f]) for f in (j, k)]
                theirs = [p[ref].sums(n, m, cells=field[f]) for f in (j, k)]
                dd = double(*ours, *theirs)
                versus.append(f"| {n} | {m} | {SHORT[j]} minus {SHORT[k]} | {diff(d, 2)} | {ref} | {diff(egta.contrast(*theirs), 2)} | "
                              f"{diff(dd, 2)} | {dd[0] / sd:+.2f} | {responds(dd, sd)} |")
    return lines + [""] + versus + [""]


def training_note(net: str) -> list[str]:
    done = Path(net).with_name("metrics.jsonl")
    if not done.exists():
        return []
    last = [json.loads(line) for line in done.read_text().splitlines() if '"type": "done"' in line]
    if not last:
        return []
    row = last[-1]
    return [f"Training: {row['games_trained']:,} games in {row['t'] / 60:.0f} minutes, ladder rating of the final checkpoint {row['elo']} "
            f"(`{done.parent.name}`, evaluated by the trainer on its own seeds, not on this block).", ""]


def report(out: Path) -> Path:
    design = json.loads((out / "design.json").read_text())
    sizes = design["sizes"]
    played = {e: {name: Played(out / f"{e}_{name}.npz") for name in foci(design["net"])} for e in EXPERIMENTS}
    look = "one look at HOLDOUT" if design["block"] == "holdout" else f"development run on the {design['block'].upper()} seeds"
    header = [f"# Adaptive generalist: {look}", "",
              f"Net under test: `{design['net']}` (sha256 `{E.sha256_file(design['net'])[:16]}`); references: Peddler v2 and `{CONTROL}`. "
              f"{design['seeds']} seeds per composition and table size, the focus seat at every seat in turn, nets played `guarded` under the five table rules. "
              f"Code: `{played['heuristics']['adaptive'].rec.meta.get('git')}`.", "", *training_note(design["net"])]
    text = header + among_heuristics(played["heuristics"], sizes) + among_nets(played["nets"], sizes) + adaptation(played["heuristics"], sizes)
    path = out / "report.md"
    path.write_text("\n".join(text) + "\n")
    return path


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("action", choices=("run", "report"))
    ap.add_argument("--net", help="checkpoint under test (run)")
    ap.add_argument("--out", default="reports/strategist/adaptive")
    ap.add_argument("--seeds", type=int, default=400)
    ap.add_argument("--block", choices=("dev", "holdout"), default="dev")
    ap.add_argument("--sizes", type=int, nargs="+", default=list(SIZES))
    ap.add_argument("--only", choices=list(foci("")), help="play one focus net only (run); the report then waits for the others")
    args = ap.parse_args()
    out = Path(args.out)
    if args.action == "run":
        if not args.net:
            raise SystemExit("run needs --net")
        run(out, args.net, args.seeds, args.block, tuple(args.sizes), only=args.only)
        if args.only:
            return
    print(f"report: {report(out)}")


if __name__ == "__main__":
    main()
