"""One design played with two versions of the heuristics, compared seed by seed.

    python -m training.egta_compare --old reports/strategist/egta_h0 --new DIR [--out FILE]

Both directories are `training.egta run` outputs. The comparison refuses unless the runs have the same horizon and policies and played every
seed they share with the same games (table size, seed, composition and seat arrangement), and it uses only the shared seeds, so a run of 40
seeds is compared with the first 40 of one of 400. Per table size, never pooled, it reports for each run the stalled games, the completed
games in which some seat defaulted and, for each archetype, the share of its seats that end dead (cash 0, no leftovers and two unpaid loans: the
signature in `breakdown` of a seat that is out of the game but still seated) and the share that end stuck (the same with cash up to $1).
Both are lower bounds on the seats that stopped playing, and "stuck" is the better one: a seat that owes $2 interest a turn and holds $0 or
$1 is in the same state, and a neighbour's wage moves its cash between the two, so the cash-0 signature alone misses about 40% of such seats.
Completed games only: a stalled game has no breakdown.

Each quantity is a ratio of sums over the seeds. The difference new - old has a 95% percentile bootstrap interval that resamples the seeds,
the same ones for both runs (`training.egta.contrast`), so that a seed that is hard for one version and for the other cancels and the games
of one seed (all its rotations and compositions) are never treated as independent. A difference is called distinguishable from sampling
variation only when that interval excludes 0; the intervals are not adjusted for the number of rows. Where neither run has an event no
interval exists, and the row gives the loose bound that the seeds having none implies.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import numpy as np

from training import egta as G
from training import evalcore as E
from training import ledger
from training.env import MAX_PLAYERS

DEAD = (0, 0, 2 * ledger.LOAN_PAYOFF)  # cash, leftovers and unpaid loans (dollars) of a seat that is out of the game but still seated
STUCK_CASH = 1  # a seat with both loans unpaid, no leftovers and at most this much cash cannot pay a turn's interest ($2)
Run = tuple[E.Records, np.ndarray, np.ndarray]


def design(rec: E.Records, seeds: np.ndarray) -> np.ndarray:
    """The games on `seeds` as sorted rows of table size, seed, cell and the archetype of every seat, whatever order they were played in."""
    rows = np.nonzero(np.isin(rec.cluster, seeds))[0]
    seats = np.where(np.arange(MAX_PLAYERS)[None, :] < rec.players[rows, None], rec.seats[rows], -1)
    key = np.column_stack([rec.players[rows].astype(np.int64), rec.cluster[rows].astype(np.int64), rec.cell[rows].astype(np.int64), seats])
    return key[np.lexsort(key.T[::-1])]


def shared_seeds(old: E.Records, new: E.Records) -> np.ndarray:
    """The seeds both runs hold, once it is certain that they played them the same way; raises otherwise."""
    for field in ("horizon", "length", "block"):
        if old.meta.get(field) != new.meta.get(field):
            raise ValueError(f"the runs differ in {field}: {old.meta.get(field)} against {new.meta.get(field)}")
    who = [[(p["name"], p["scripted"]) for p in r.meta["policies"]] for r in (old, new)]
    if who[0] != who[1]:
        raise ValueError(f"the runs seat different policies: {who[0]} against {who[1]}")
    shared = np.intersect1d(old.cluster, new.cluster)
    if not len(shared):
        raise ValueError("the runs share no seed")
    a, b = design(old, shared), design(new, shared)
    if a.shape != b.shape or not np.array_equal(a, b):
        raise ValueError("the runs played the shared seeds with a different design (other games, or another seat arrangement)")
    return shared


def restrict(run: Run, seeds: np.ndarray) -> Run:
    rec, interaction, breakdown = run
    keep = np.isin(rec.cluster, seeds)
    return rec.subset(keep), interaction[keep], breakdown[keep]


def tally(run: Run) -> dict[int, dict[str, tuple[np.ndarray, np.ndarray]]]:
    """Per table size and quantity, the per-seed sums of its numerator and denominator."""
    rec, interaction, breakdown = run
    t = G.seat_table(rec, interaction, breakdown)
    done = ~rec.stalled
    defaulted = np.bincount(t.game, np.nan_to_num(t.raw["defaulted"]), len(rec)) > 0
    seated = np.arange(MAX_PLAYERS)[None, :] < rec.players[:, None]
    dead = seated & (breakdown[:, :, [0, 2, 3]] == DEAD).all(2)
    stuck = seated & (breakdown[:, :, 0] <= STUCK_CASH) & (breakdown[:, :, 2] == DEAD[1]) & (breakdown[:, :, 3] == DEAD[2])
    out = {}
    for n, seeds in sorted(t.seeds.items()):
        rows = rec.players == n

        def by_seed(v, rows=rows, seeds=seeds):
            return np.bincount(t.game_seed[rows], np.asarray(v, dtype=float)[rows], seeds)

        q = {"stalled games": (by_seed(rec.stalled), by_seed(np.ones(len(rec)))),
             "completed games with a default": (by_seed(defaulted & done), by_seed(done))}
        for a, name in enumerate(G.NAMES):
            mine = seated & (rec.seats == a) & done[:, None]
            q[f"{name} seats dead at the end"] = (by_seed((dead & mine).sum(1)), by_seed(mine.sum(1)))
            q[f"{name} seats stuck at the end"] = (by_seed((stuck & mine).sum(1)), by_seed(mine.sum(1)))
        out[n] = q
    return out


def compare(old: Run, new: Run) -> dict:
    shared = shared_seeds(old[0], new[0])
    runs = [restrict(r, shared) for r in (old, new)]
    a, b = (tally(r) for r in runs)
    if a.keys() != b.keys():
        raise ValueError("the runs hold different table sizes")
    rows = []
    for n in sorted(a):
        seeds = len(next(iter(a[n].values()))[0])
        for name in a[n]:
            (old_top, old_bottom), (new_top, new_bottom) = a[n][name], b[n][name]
            if not (old_bottom.sum() and new_bottom.sum()):
                continue
            diff, lo, hi = G.contrast(b[n][name], a[n][name])
            ratio = [float("nan")] * 3
            if old_top.sum():
                before, after = G.resampled(a[n][name]), G.resampled(b[n][name])
                quotient = np.divide(after, before, out=np.full(len(after), np.nan), where=before > 0)
                ratio = G.interval(G.point(b[n][name]) / G.point(a[n][name]), quotient)
            rows.append({
                "players": n, "quantity": name, "seeds": seeds,
                "old": {"events": int(old_top.sum()), "total": int(old_bottom.sum()), "seeds_with": int((old_top > 0).sum())},
                "new": {"events": int(new_top.sum()), "total": int(new_bottom.sum()), "seeds_with": int((new_top > 0).sum())},
                "diff": [diff, lo, hi], "ratio": ratio,
                "excludes_zero": bool(lo > 0 or hi < 0), "none": bool(old_top.sum() == 0 and new_top.sum() == 0),
                "bound": 3 / seeds})
    return {"seeds": {n: len(next(iter(a[n].values()))[0]) for n in sorted(a)}, "rows": rows,
            "games": {n: int(next(iter(a[n].values()))[1].sum()) for n in sorted(a)},
            "horizon": old[0].meta.get("horizon"), "resamples": G.REPS}


def cell(c: dict, seeds: int) -> str:
    return f"{c['events']:,} / {c['total']:,} = {100 * c['events'] / c['total']:.2f}% ({c['seeds_with']} of {seeds} seeds)"


def render(result: dict, old: str, new: str) -> str:
    shared = ", ".join(f"{s} at {n} seats" for n, s in result["seeds"].items())
    header = (f"Old: `{old}`. New: `{new}`. Same horizon ({result['horizon']} actions), same policies, same games on every shared seed "
              f"(checked); compared on the shared seeds only: {shared}. Intervals: 95%, {result['resamples']:,} resamples of the seeds, "
              "the same seeds for both runs, per table size.")
    lines = [header, ""]
    for n in result["seeds"]:
        s = result["seeds"][n]
        lines += [f"**{n} seats** ({s} seeds)", "",
                  "| quantity | old | new | new - old, points [95% interval] | new / old [95% interval] | reading |",
                  "|---|---|---|---|---|---|"]
        for r in (r for r in result["rows"] if r["players"] == n):
            d, lo, hi = r["diff"]
            ratio = "—" if np.isnan(r["ratio"][0]) else f"{r['ratio'][0]:.1f} [{r['ratio'][1]:.1f}, {r['ratio'][2]:.1f}]"
            if r["none"]:
                reading = f"none in either run: the chance that a seed has one is below {r['bound']:.1%} at 95% (a loose seed-level bound)"
            else:
                reading = "interval excludes 0" if r["excludes_zero"] else "interval includes 0"
            lines.append(f"| {r['quantity']} | {cell(r['old'], s)} | {cell(r['new'], s)} "
                         f"| {100 * d:+.2f} [{100 * lo:+.2f}, {100 * hi:+.2f}] | {ratio} | {reading} |")
        lines.append("")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--old", required=True)
    ap.add_argument("--new", required=True)
    ap.add_argument("--out", help="also write the table to this file (nothing is written by default)")
    args = ap.parse_args()
    paths = [Path(p) if Path(p).is_absolute() else E.ROOT / p for p in (args.old, args.new)]
    text = render(compare(G.load(paths[0]), G.load(paths[1])), args.old, args.new)
    print(text)
    if args.out:
        Path(args.out).write_text(text + "\n")


if __name__ == "__main__":
    main()
