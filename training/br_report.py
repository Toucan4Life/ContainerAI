"""Tables that validate the best-response oracle: what each training run learned, from its own logs and from
the paired deviation studies of `training.br`.

    python -m training.br_report runs --runs runs/br/a,runs/br/b --studies reports/br/validation/a.json,reports/br/validation/b.json \
        --out reports/br/validation/summary.md --title "Oracle validation"
    python -m training.br_report pairs --study reports/br/milestone/holdout.json --pairs m1/sample:m2/sample,m1/guarded:m2/guarded
    python -m training.br_report select --study reports/psro/final/holdout_sigma_last.json

A run is matched to the study that holds its candidates: a study label is `<run name>[@<minutes>m]/<mode>`,
where `@<minutes>m` marks weights saved part-way through the run (the final weights carry no suffix).
`recovery` is the final candidate's gain over the best pure strategy's gain in the same study, only reported
when that pure gain is at least `MIN_PURE`: it says how much of a deviation that needs no training the oracle
finds within its budget. `pairs` compares focals of one study seed by seed. `select` takes away the optimism of
quoting the best of several focals: it picks on one half of the seeds and measures on the other.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import numpy as np

from training import br

MIN_PURE = 0.02
MODES = ("sample", "guarded")


def ci(x, digits: int = 3) -> str:
    return f"{x[0]:+.{digits}f} [{x[1]:+.{digits}f}, {x[2]:+.{digits}f}]"


def profile_text(cfg: dict) -> str:
    sup, w = cfg["support"], cfg["weights"]
    if isinstance(w, dict):
        return "; ".join(f"{n}p: " + ", ".join(f"{s} {x:.2f}" for s, x in zip(sup, ws, strict=True)) for n, ws in w.items())
    return sup[0] if len(sup) == 1 else ", ".join(f"{s} {x:.2f}" for s, x in zip(sup, w, strict=True))


def parts(label: str) -> tuple[str, float | None, str]:
    """`run[@<minutes>m]/mode` -> run name, minutes of training (None for the final weights), mode."""
    head, mode = label.rsplit("/", 1)
    name, _, at = head.partition("@")
    return name, (float(at.rstrip("m")) if at else None), mode


def candidates(run: dict, studies: list[dict]) -> tuple[dict | None, dict[float, dict[str, str]]]:
    """The study that holds the run's candidates, and its labels as {minutes of training: {mode: label}}."""
    for study in studies:
        found: dict[float, dict[str, str]] = {}
        for lab in study["candidates"]:
            name, minutes, mode = parts(lab)
            if name == run["config"]["name"]:
                found.setdefault(run["minutes"] if minutes is None else minutes, {})[mode] = lab
        if found:
            return study, found
    return None, {}


def best_pure(study: dict) -> tuple[str, list[float]] | None:
    pure = {lab: g for lab, g in study["mean_over_players"]["gain"].items() if lab not in study["candidates"]}
    return max(pure.items(), key=lambda kv: kv[1][0], default=None)


def render_runs(runs: list[dict], studies: list[dict], title: str) -> str:
    """`runs` are the `result.json` files of the training runs, `studies` the `training.br eval` outputs."""
    lines = [f"# {title}", "",
             "One row per best-response training run. `curve` is the gain over the profile of the run's own periodic "
             "dev studies (minutes of training: mean gain; each with a 95% seed-clustered interval in the run's "
             "`result.json`). `study` is the paired deviation study of the final weights on dev seeds the training "
             "never used. A trained response is an approximate best response, so every gain here is a lower bound "
             "on the profile's true deviation gain. `best pure` is the best strategy of the population in the same "
             "study; `recovery` is the better mode's gain divided by it (shown when it is at least "
             f"{MIN_PURE}).", "",
             "| run | init | profile | objective | seed | min | games | curve | study (mean over player counts) "
             "| best pure | recovery |", "|---|---|---|---|---|---|---|---|---|---|---|"]
    over_time = []
    for run in runs:
        cfg = run["config"]
        study, found = candidates(run, studies)
        curve = ", ".join(f"{e['minutes']:.0f}: {e['gain'][0]:+.3f}" for e in run["evals"])
        modes, pure, recovery = "no study", "", ""
        if study is not None:
            gain = study["mean_over_players"]["gain"]
            final = found[max(found)]
            modes = "<br>".join(f"{m} {ci(gain[lab])}" for m, lab in final.items())
            best = best_pure(study)
            if best is not None:
                pure = f"{best[0]} {best[1][0]:+.3f}"
                top = max(gain[lab][0] for lab in final.values())
                recovery = f"{top / best[1][0]:.0%}" if best[1][0] >= MIN_PURE else ""
            if len(found) > 1:
                last = max(found)
                for minutes in sorted(found):
                    gains = [ci(gain[found[minutes][m]]) if m in found[minutes] else "" for m in MODES]
                    behind = [ci(br.paired_difference(study, found[last][m], found[minutes][m]))
                              if minutes != last and m in found[minutes] and m in found[last] else "" for m in MODES]
                    over_time.append(f"| {cfg['name']} | {minutes:.0f} | " + " | ".join(gains + behind) + " |")
        init = Path(cfg["init"]).stem
        lines.append(f"| {cfg['name']} | {init} | {profile_text(cfg)} | {cfg['utility']} | {cfg['seed']} | "
                     f"{run['minutes']:.0f} | {run['games'] / 1000:.0f}k | {curve} | {modes} | {pure} | {recovery} |")
    lines.append("")
    if over_time:
        lines += ["## Gain against training time", "",
                  "The weights saved during a run, each played as a candidate in the same paired study (same seeds "
                  "and opponents; each interval is a 95% seed-clustered interval). The last two columns are the "
                  "paired difference between the final weights and the saved ones: an interval above zero means that "
                  "the run was still improving after that point.", "",
                  "| run | minutes of training | sample | guarded | final minus this, sample | final minus this, guarded |",
                  "|---|---|---|---|---|---|", *over_time, ""]
    return "\n".join(lines)


def render_pairs(study: dict, pairs: list[tuple[str, str]]) -> str:
    """Paired differences `a - b` of the gains of two focals of one study."""
    rows = ["| a | b | gain of a minus gain of b (mean over player counts) |", "|---|---|---|"]
    rows += [f"| {a} | {b} | {ci(br.paired_difference(study, a, b))} |" for a, b in pairs]
    return "\n".join(rows) + "\n"


def selection_check(study: dict, pool: list[str], reps: int = 2000, seed: int = 0) -> dict:
    """Gain of the focal that looks best, measured without the optimism of having picked it.

    The best of several focals on the seeds that also measure it looks better than it is (the winner's curse):
    with k focals measured to about +-0.04, the best of them stands about 0.05 above the truth even when all have
    the same gain. Here the seeds are split into even and odd positions; the focal that looks best on one half is
    measured on the other, in both directions, and the two measurements are pooled into one interval."""
    blocks = list(study["by_players"].values())
    gains = {f: [np.asarray(b["per_seed_gain"][f], dtype=float) for b in blocks] for f in pool}
    size = [len(g) for g in gains[pool[0]]]
    halves = [[np.arange(h, k, 2) for k in size] for h in (0, 1)]
    everything = [np.arange(k) for k in size]

    def score(f: str, ix: list[np.ndarray]) -> float:
        return float(np.mean([g[i].mean() for g, i in zip(gains[f], ix)]))

    picked, held = [], []
    for h in (0, 1):
        f = max(pool, key=lambda f: score(f, halves[h]))
        picked.append(f)
        held += [g[i] for g, i in zip(gains[f], halves[1 - h])]
    best = max(pool, key=lambda f: score(f, everything))
    return dict(best=best, optimistic=br._boot_mean(gains[best], reps=reps, seed=seed), picked=picked,
                honest=br._boot_mean(held, reps=reps, seed=seed))


def render_selection(study: dict, pools: dict[str, list[str]]) -> str:
    rows = ["| picked among | best on all seeds (optimistic) | picked on even seeds, measured on odd | "
            "picked on odd seeds, measured on even | gain of the pick, measured on the seeds it was not picked on |",
            "|---|---|---|---|---|"]
    for name, pool in pools.items():
        r = selection_check(study, pool)
        rows.append(f"| {name} ({len(pool)}) | {r['best']} {ci(r['optimistic'])} | {r['picked'][0]} | {r['picked'][1]} "
                    f"| {ci(r['honest'])} |")
    return "\n".join(rows) + "\n"


def pools_of(study: dict) -> dict[str, list[str]]:
    focals, cands = study["focals"], study["candidates"]
    pools = {"every focal": focals, "trained candidates": cands, "strategies of the population": [f for f in focals if f not in cands]}
    return {name: pool for name, pool in pools.items() if pool}


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    rn = sub.add_parser("runs", help="the table of training runs against their studies")
    rn.add_argument("--runs", required=True, help="training run directories, comma separated")
    rn.add_argument("--studies", required=True, help="deviation study json files, comma separated")
    rn.add_argument("--out", required=True)
    rn.add_argument("--title", default="Best-response oracle validation")
    pr = sub.add_parser("pairs", help="paired differences between focals of one study")
    pr.add_argument("--study", required=True)
    pr.add_argument("--pairs", required=True, help="a:b,c:d (focal labels, e.g. m1/sample:m2/sample)")
    sl = sub.add_parser("select", help="the gain of the best-looking focal, picked on one half of the seeds and measured on the other")
    sl.add_argument("--study", required=True)
    args = ap.parse_args()
    if args.cmd == "pairs":
        study = json.loads(Path(args.study).read_text())
        print(render_pairs(study, [tuple(p.split(":")) for p in args.pairs.split(",")]))
        return
    if args.cmd == "select":
        study = json.loads(Path(args.study).read_text())
        print(render_selection(study, pools_of(study)))
        return
    runs = [json.loads((Path(d) / "result.json").read_text()) for d in args.runs.split(",")]
    studies = [json.loads(Path(p).read_text()) for p in args.studies.split(",")]
    Path(args.out).write_text(render_runs(runs, studies, args.title))
    print(f"wrote {args.out}")


if __name__ == "__main__":
    main()
