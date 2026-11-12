"""Mixtures of the population against opponents the solver never assumed.

An empirical equilibrium is the best answer to itself. The opponents of the deployed bot are Magnate, other nets and
people, none of whom play sigma. The utility of a mixture w against a field f (the other n-1 seats drawn iid from f) is
sum_i w_i u_i(f), exact arithmetic on the simulated cells: no game is played here. The bootstrap resamples the seeds of
every cell and re-measures, so each number carries the noise of the cells it comes from and a difference between two
mixtures is paired (both are read from the same resampled game).

    python -m training.fields --cells a.json,b.json --strategies x,y,z \
        --mix old=@reports/psro/final/profile_final.json --mix new=@reports/psro/refined_metagame_profile.json \
        --mix psro03 --field sigma=@reports/psro/refined_metagame_profile.json --field uniform --field magnate \
        --out reports/psro/refined_fields

A spec is `uniform`, `a,b:2` (weights default to 1) or `@file.json`, a profile like `profile_final.json` whose weights
may depend on the number of players. `label=spec` names it; a bare spec is its own label.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import numpy as np

from training.psro import ROOT, CellStore, write_atomic


def parse_spec(text: str, names: list[str]):
    """The distribution over `names` that `text` stands for, as a function of the number of players."""

    def spread(support, weights) -> np.ndarray:
        out = np.zeros(len(names))
        for s, w in zip(support, weights, strict=True):
            if s not in names:
                raise SystemExit(f"{s!r} is not one of the strategies {names}")
            out[names.index(s)] += float(w)
        if out.sum() <= 0:
            raise SystemExit(f"{text!r} puts no weight on any strategy")
        return out / out.sum()

    if text == "uniform":
        fixed = spread(names, [1.0] * len(names))
        return lambda n: fixed
    if text.startswith("@"):
        spec = json.loads(Path(text[1:]).read_text())
        if isinstance(spec["weights"], dict):
            by_n = {int(n): spread(spec["support"], w) for n, w in spec["weights"].items()}
            return lambda n: by_n[n]
        fixed = spread(spec["support"], spec["weights"])
        return lambda n: fixed
    parts = [p.partition(":") for p in text.split(",")]
    fixed = spread([p[0].strip() for p in parts], [p[2] or 1.0 for p in parts])
    return lambda n: fixed


def labelled(items: list[str], names: list[str]) -> dict:
    out = {}
    for item in items:
        label, eq, spec = item.partition("=")
        if not eq or label.startswith("@") or "," in label:
            label, spec = item, item
        if label in out:
            raise SystemExit(f"two specs are called {label!r}")
        out[label] = parse_spec(spec, names)
    return out


def interval(draws: np.ndarray) -> list[float]:
    """Point estimate (first row: the simulated cells as they are) and the 95% bootstrap interval (the other rows)."""
    boot = draws[1:]
    lo, hi = np.quantile(boot, [0.025, 0.975], axis=0) if len(boot) else (draws[0], draws[0])
    return [float(draws[0]), float(lo), float(hi)]


def compare(store: CellStore, names: list[str], n: int, mixes: dict, fields: dict, reps: int, seed: int) -> dict:
    """Utility of each mixture against each field at n players, and its gap to the first mixture, both with intervals."""
    games = [store.game(names, n)] + (store.bootstrap(names, n, reps, seed) if reps else [])
    # u[g, f, i]: strategy i against field f in game g (g = 0 is the simulated game, the rest are resamples)
    u = np.array([[g.expected_payoffs(f(n)) for f in fields.values()] for g in games])
    reference = next(iter(mixes.values()))(n)
    values, gaps = {}, {}
    for label, mix in mixes.items():
        w = mix(n)
        values[label] = {f: interval(u[:, k] @ w) for k, f in enumerate(fields)}
        gaps[label] = {f: interval(u[:, k] @ (w - reference)) for k, f in enumerate(fields)}
    return dict(values=values, gaps=gaps, weights={label: [float(x) for x in mix(n)] for label, mix in mixes.items()},
                fields={label: [float(x) for x in f(n)] for label, f in fields.items()})


def cell(x: list[float], signed: bool = False) -> str:
    f = "+.3f" if signed else ".3f"
    return f"{x[0]:{f}} [{x[1]:{f}}, {x[2]:{f}}]"


def render(out: dict, names: list[str], title: str, reps: int) -> list[str]:
    lines = [f"# {title}", "",
             "Utility (winner share; 1/n is an equal share) of a seat that plays the mixture while the other seats are "
             f"drawn iid from the field, read exactly from the simulated cells. Brackets: 95% bootstrap interval over "
             f"{reps} resamples of the seeds of every cell, so a difference of the bracketed numbers is not a test; "
             "the gap tables are paired and are.", ""]
    for n, r in out.items():
        fields = list(r["fields"])
        lines += [f"## {n} players", "", "Mixtures (weight per strategy, in this order: " + ", ".join(names) + ")", "",
                  "| mixture | " + " | ".join(names) + " |", "|---|" + "---|" * len(names)]
        lines += [f"| {m} | " + " | ".join(f"{x:.3f}" for x in w) + " |" for m, w in r["weights"].items()]
        lines += ["", "Fields (the same):", "", "| field | " + " | ".join(names) + " |", "|---|" + "---|" * len(names)]
        lines += [f"| {f} | " + " | ".join(f"{x:.3f}" for x in w) + " |" for f, w in r["fields"].items()]
        reference = next(iter(r["values"]))
        lines += ["", "Utility of each mixture against each field:", "",
                  "| mixture | " + " | ".join(fields) + " |", "|---|" + "---|" * len(fields)]
        lines += [f"| {m} | " + " | ".join(cell(v[f]) for f in fields) + " |" for m, v in r["values"].items()]
        lines += ["", f"Gap to `{reference}` (paired):", "", "| mixture | " + " | ".join(fields) + " |",
                  "|---|" + "---|" * len(fields)]
        lines += [f"| {m} | " + " | ".join(cell(v[f], signed=True) for f in fields) + " |"
                  for m, v in list(r["gaps"].items())[1:]]
        lines.append("")
    return lines


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cells", required=True, help="comma-separated cross-play payoffs.json files")
    ap.add_argument("--strategies", required=True)
    ap.add_argument("--players", default="3,4,5")
    ap.add_argument("--mix", action="append", required=True, help="a mixture to evaluate; the first is the reference")
    ap.add_argument("--field", action="append", required=True, help="a field of opponents")
    ap.add_argument("--boot-reps", type=int, default=200)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--out", required=True, help="path without extension")
    ap.add_argument("--title", default="Mixtures against fields of opponents")
    args = ap.parse_args()
    names = args.strategies.split(",")
    store = CellStore.from_files([ROOT / p for p in args.cells.split(",")])
    mixes, fields = labelled(args.mix, names), labelled(args.field, names)
    out = {str(n): compare(store, names, n, mixes, fields, args.boot_reps, args.seed)
           for n in map(int, args.players.split(","))}
    target = Path(args.out)
    target.parent.mkdir(parents=True, exist_ok=True)
    write_atomic(target.with_suffix(".json"), json.dumps(dict(strategies=names, players=out), indent=1))
    lines = render(out, names, args.title, args.boot_reps)
    write_atomic(target.with_suffix(".md"), "\n".join(lines))
    print("\n".join(lines))


if __name__ == "__main__":
    main()
