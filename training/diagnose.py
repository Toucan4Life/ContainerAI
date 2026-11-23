"""Who wins a table, and what do the shelves look like? Evaluation only.

    python -m training.diagnose peddler2 --games 200 \
        --compose current=peddler2,consortium,magnate \
        --compose complementary=runs/strategist/producer_s2/final.pt,runs/strategist/showcase_fair_s2/final.pt,runs/strategist/control_s2/final.pt

Per composition, with the focus net seated at every seat in turn: win share by table entry and by seat position
(ties split), the mean score of each entry, and per-entry shelf and trade numbers. "Stock per decision" is containers
on the seat's own lots averaged over its Main decisions (a level, not a flow); "cheap" means priced $3 or less.
Prices the seats lay out ($1..$6) and the colours bought are pooled over the table's seats of each entry.
"""

from __future__ import annotations

import argparse

import numpy as np

from training import economics, gate
from training import env as V


def entries(rec, table_size: int, n_entries: int):
    ok = ~rec.stalled
    seats = rec.seats[ok][:, :table_size]
    won = ((rec.winners[ok][:, None] >> np.arange(table_size)[None, :]) & 1).astype(np.float64)
    won /= np.maximum(won.sum(1, keepdims=True), 1)
    return ok, seats, won, rec.scores[ok][:, :table_size].astype(np.float64), rec.interaction[ok][:, :table_size].astype(np.float64)


def report(focus: str, name: str, table: tuple[str, ...], games: int) -> None:
    label, path, rules = economics.resolve(focus)
    rec, _ = gate.play_focus(path, rules, games, table=table)
    size = len(table) + 1
    ok, seats, won, scores, inter = entries(rec, size, size)
    names = [label] + [economics.resolve(t)[0] for t in table]
    print(f"\n**{label} at the {name} table** ({int(ok.sum())} games; fair share {1 / size:.2f})\n")
    print("| entry | win share | mean score | stock per decision | cheap share of stock | sold per game | bought per game | price paid | factories held | mean lot price laid out |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for e, n in enumerate(names):
        m = seats == e
        row = inter[m]
        placed = row[:, V.PLACED_AT:V.PLACED_AT + 6].sum(0)
        price = float((placed * np.arange(1, 7)).sum() / max(placed.sum(), 1))
        dec = max(row[:, V.MAIN_DECISIONS].sum(), 1)
        print(f"| {n} | {won[m].mean():.3f} | {scores[m].mean():.1f} | {row[:, V.OFFERED].sum() / dec:.2f} | "
              f"{row[:, V.OFFERED_COMPETITIVE].sum() / max(row[:, V.OFFERED].sum(), 1):.2f} | {row[:, V.SOLD].mean():.1f} | "
              f"{row[:, V.BOUGHT].mean():.1f} | {row[:, V.SPENT].sum() / max(row[:, V.BOUGHT].sum(), 1):.2f} | "
              f"{row[:, V.FACTORIES_HELD].sum() / dec:.2f} | {price:.2f} |")
    print("\nWin share by seat position (all entries): " + ", ".join(f"{s}: {won[:, s].mean():.3f}" for s in range(size)))
    print("Winners per game: mean " + f"{(rec.winners[ok] > 0).mean():.2f}" + f"; focus-entry wins by seat it sat in: " +
          ", ".join(f"{s}: {won[:, s][seats[:, s] == 0].mean():.3f}" for s in range(size)))
    colors = inter[..., V.BOUGHT_COLOR:V.BOUGHT_COLOR + 5]
    print("Containers bought per game by colour, per entry: " + "; ".join(
        f"{n} " + "/".join(f"{x:.1f}" for x in colors[seats == e].mean(0)) for e, n in enumerate(names)))


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("focus")
    ap.add_argument("--games", type=int, default=200)
    ap.add_argument("--compose", action="append", required=True)
    args = ap.parse_args()
    for item in args.compose:
        name, _, seats = item.partition("=")
        report(args.focus, name, tuple(seats.split(",")), args.games)


if __name__ == "__main__":
    main()
