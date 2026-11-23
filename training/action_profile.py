"""Moves per game, building and cargo sizes of several nets and the three heuristics, at one table.

    python -m training.action_profile > reports/strategist/adaptive/actions.md

Each net sits at every seat of a table with Producer, Showcase and Shipper (DEV seeds, never the holdout). The heuristics'
columns are their seats in the games of the first net, so every column comes from one kind of table. A table of 3 seats has no
Shipper and one of 5 has two Producers.
"""

from __future__ import annotations

import argparse
import sys
from typing import NamedTuple

import numpy as np

from training import env as V
from training import evalcore as E
from training.adaptive_eval import HEURISTICS, RULES, foci
from training.audit_interaction import parse
from training.move_table import GROUPS
from training.strategy_metrics import configure_table

# Name -> `path:rules`: the nets under test, then the references. The runs play under the five table rules Peddler v2 ships with.
DEFAULT_NETS = {
    "warm": f"runs/strategist/adaptive_s1/final.pt:{RULES}",
    "scratch_s1": f"runs/strategist/adaptive_scratch_s1/final.pt:{RULES}",
    "scratch_s2": f"runs/strategist/adaptive_scratch_s2/final.pt:{RULES}",
    "peddler2": foci("")["peddler2"],
    "control_s1": foci("")["control_s1"],
}
BAR = 28  # characters in the longest bar of a histogram


class Column(NamedTuple):
    """The finished games of one player: a row of env counters, the win share and the score for every game."""

    rows: np.ndarray
    win: np.ndarray
    score: np.ndarray

    def mean(self, column: int) -> float:
        return float(self.rows[:, column].mean())

    def ratio(self, top: int, bottom: int) -> float:
        return float(self.rows[:, top].sum() / max(self.rows[:, bottom].sum(), 1))


def play(spec: str, n: int, seeds: int) -> E.Records:
    """`seeds` DEV games at `n` seats: the net at every seat in turn, Producer, Showcase and Shipper at the others."""
    _, path, rules = parse(f"focus={spec}")
    policies = [E.Policy("focus", path, "guarded"), *(E.Policy(k, scripted=v) for k, v in HEURISTICS.items())]
    specs = []
    for s in range(seeds):
        seed = E.game_seed(E.DEV, n, V.STANDARD, s)
        for r in range(n):
            order = [1, 2, 3, 1, 2][: n - 1]
            specs.append(E.GameSpec(n, V.STANDARD, seed, tuple(order[:r] + [0] + order[r:]), s, seed))
    return E.play(specs, policies, log=None, configure=lambda env: configure_table(env, [(E.SLOT_BASE, rules)]))


def column(rec: E.Records, k: int) -> Column:
    mask = (rec.seats == k) & ~rec.stalled[:, None]
    return Column(rec.interaction[mask], E.winner_share(rec)[mask], rec.scores[mask])


def profile(nets: dict[str, str], n: int, seeds: int) -> tuple[dict[str, Column], dict[str, int]]:
    """Every net's column, then the heuristics' (from the first net's games), and the stalled games of each net's table."""
    cols: dict[str, Column] = {}
    heuristics: dict[str, Column] = {}
    stalled: dict[str, int] = {}
    for i, (name, spec) in enumerate(nets.items()):
        rec = play(spec, n, seeds)
        cols[name] = column(rec, 0)
        stalled[name] = int(rec.stalled.sum())
        if i == 0:
            heuristics = {h: column(rec, k) for k, h in enumerate(HEURISTICS, 1) if (rec.seats == k).any()}
        print(f"{name}: {len(rec):,} games in {rec.meta['seconds']} s, {stalled[name]} stalled", file=sys.stderr, flush=True)
    return cols | heuristics, stalled


def fmt(v: float, digits: int = 1) -> str:
    """Zero as 0, and anything that would round to zero as `<0.05`."""
    small = 0.5 * 10**-digits
    return "0" if v == 0 else f"<{small:g}" if v < small else f"{v:.{digits}f}"


def table(title: str, heads: list[str], rows: list[tuple[str, list[str]]]) -> list[str]:
    return [f"**{title}**", "", "| | " + " | ".join(heads) + " |", "|---|" + "---|" * len(heads),
            *(f"| {label} | " + " | ".join(cells) + " |" for label, cells in rows), ""]


def size_share(c: Column, k: int) -> str:
    """The share of the player's trips to the Island that carried `k + 1` containers."""
    t = c.rows[:, V.CARGO_SIZE:V.CARGO_SIZE + 5].sum(0)
    return f"{100 * t[k] / max(t.sum(), 1):.0f}%"


def histogram(title: str, moves: list[tuple[str, str]], by_move: dict[str, dict[str, float]]) -> list[str]:
    """One bar chart per move, each on its own scale; moves that no column plays 0.05 times a game are named, not drawn."""
    width = max(map(len, by_move))
    out, unplayed = [f"**{title}**", "", "```"], []
    for key, label in moves:
        top = max(by_move[c][key] for c in by_move)
        if top < 0.05:
            unplayed.append(label)
            continue
        out.append(f"{label}  (bars to {top:.1f} a game)")
        for c in by_move:
            v = by_move[c][key]
            out.append(f"  {c:<{width}} {'█' * max(1, round(BAR * v / top)) if v >= 0.05 else '':<{BAR}} {fmt(v)}")
        out.append("")
    return out + ["```", *([f"Not played 0.05 times a game by any column: {', '.join(unplayed)}.", ""] if unplayed else [""])]


def report(cols: dict[str, Column], stalled: dict[str, int], n: int, seeds: int) -> str:
    names = V.move_names()
    idx = {m: V.MOVES + i for i, m in enumerate(names)}
    by_move = {c: dict(zip(names, col.rows[:, V.MOVES:].mean(0))) for c, col in cols.items()}
    heads = list(cols)
    summary = [
        (f"win share at this table (an even split is {1 / n:.2f})", lambda c: f"{c.win.mean():.2f}"),
        ("score", lambda c: f"{c.score.mean():.0f}"),
        ("containers produced", lambda c: fmt(c.mean(V.PRODUCED))),
        ("containers bought", lambda c: fmt(c.mean(V.BOUGHT))),
        ("containers sold", lambda c: fmt(c.mean(V.SOLD))),
        ("factories built", lambda c: fmt(c.mean(idx["build_factory"]), 2)),
        ("warehouses built (harbor capacity)", lambda c: fmt(c.mean(idx["build_warehouse"]), 2)),
        ("factories owned, mean over turns", lambda c: f"{c.ratio(V.FACTORIES_HELD, V.MAIN_DECISIONS):.2f}"),
        ("warehouses owned, mean over turns", lambda c: f"{c.ratio(V.WAREHOUSES_HELD, V.MAIN_DECISIONS):.2f}"),
        ("deliveries (sails to the Island)", lambda c: fmt(c.mean(V.DELIVERIES))),
        ("containers per delivery", lambda c: f"{c.ratio(V.SHIPPED, V.DELIVERIES):.2f}"),
    ]
    out = [f"# Actions of the adaptive nets, the references and the heuristics ({n} seats, {seeds} DEV seeds)", "",
           f"Each net sits at every seat in turn; the other seats are Producer, Showcase and Shipper (a table of 3 has no Shipper, "
           f"one of 5 has two Producers). The three heuristic columns are their seats in the games of `{heads[0]}`. Every number is "
           "per game and seat. Every player starts with 1 factory and 1 warehouse.", ""]
    out += table("Result, building and shipping, per game", heads, [(label, [f(c) for c in cols.values()]) for label, f in summary])
    out += table("Share of the trips to the Island by number of containers on the ship (a ship holds 5)", heads,
                 [(f"{k + 1}", [size_share(c, k) for c in cols.values()]) for k in range(5)])
    for title, moves in GROUPS:
        out += table(f"Moves per game: {title.lower()}", heads, [(label, [fmt(by_move[c][key]) for c in cols]) for key, label in moves])
    out += ["## Histogram of each move", "", "One chart per move, so the bars compare the columns for that move only.", ""]
    for title, moves in GROUPS:
        out += histogram(title, moves, by_move)
    out += [f"Finished seat-games per column: {', '.join(f'{c} {len(col.rows)}' for c, col in cols.items())}. "
            f"Stalled games, left out: {', '.join(f'{c} {k}' for c, k in stalled.items())}."]
    return "\n".join(out)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--net", action="append", metavar="NAME=PATH[:RULES]",
                    help="a net to profile, repeatable (replaces the default list); the rules default to the five of Peddler v2")
    ap.add_argument("--seats", type=int, default=4)
    ap.add_argument("--seeds", type=int, default=100)
    args = ap.parse_args()
    nets = DEFAULT_NETS
    if args.net:
        nets = {}
        for given in args.net:
            name, _, spec = given.partition("=")
            nets[name] = spec if ":" in spec else f"{spec}:{RULES}"
    cols, stalled = profile(nets, args.seats, args.seeds)
    print(report(cols, stalled, args.seats, args.seeds))


if __name__ == "__main__":
    main()
