"""Offer exposure: what the bots of a population do at a table, for the three heuristics and the scripted bots the trainer draws today.

    python -m training.exposure --out reports/strategist/exposure.md [--seeds 100]

The *heuristics* are the three hand-set Phase H0 bots (Producer, Showcase, Shipper). The *scripted bots* are the older built-in opponents
the trainer seats by default: Merchant, Simple and Random, drawn 60% / 30% / 10% (`train.league_table`). Both are scripted Rust bots; the
names only separate the new from the old.

Each population plays among itself (every composition and rotation, `DEV` seeds, completed games, no learner seated) and the figures are
per seat-game of each bot: containers per delivery, the share of deliveries of two or more containers, factories and warehouses built, and
the share of seat-games that built at least one warehouse (a harbor with a warehouse offers more colors at once). They describe what a
bot does, which is a proxy for what a learner would meet, not a measurement of a learner at the table. It reads only counters that exist;
nothing is trained and no bot changes. The nets of the pool (Peddler v2, Consortium) and snapshots of the learner are not in this table.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import numpy as np

from training import egta as G
from training import env as V
from training import evalcore as E

HEURISTICS = {"Producer": V.PRODUCER, "Showcase": V.SHOWCASE, "Shipper": V.SHIPPER}
SCRIPTED = {"Merchant": V.MERCHANT, "Simple": V.SIMPLE, "Random": V.RANDOM}
SCRIPTED_DRAW = {"Merchant": 0.6, "Simple": 0.3, "Random": 0.1}  # `train.league_table`, among the scripted seats


def measure(kinds: list[int], size: int, seeds: int) -> dict:
    """Per bot of the population: the sums over its seat-games in completed games."""
    pol = [E.Policy(f"k{k}", scripted=k) for k in kinds]
    specs = E.crossplay_specs(len(pol), [size], seeds, block=E.DEV, length=V.STANDARD)
    rec = E.play(specs, pol, log=None)
    used = np.arange(V.MAX_PLAYERS)[None, :] < rec.players[:, None]
    g, s = np.nonzero(used & ~rec.stalled[:, None])
    x = rec.interaction[g, s].astype(np.int64)
    who = rec.seats[g, s].astype(np.int64)
    names = V.move_names()
    fac, wh = V.MOVES + names.index("build_factory"), V.MOVES + names.index("build_warehouse")
    bots = []
    for i in range(len(kinds)):
        m = who == i
        trips = x[m, V.CARGO_SIZE:V.CARGO_SIZE + 5].sum(0)
        bots.append(dict(n=int(m.sum()), shipped=int(x[m, V.SHIPPED].sum()), deliveries=int(x[m, V.DELIVERIES].sum()), multi=int(trips[1:].sum()),
                         trips=int(trips.sum()), factories=int(x[m, fac].sum()), warehouses=int(x[m, wh].sum()), owns=int((x[m, wh] > 0).sum())))
    return {"games": len(rec), "stalled": int(rec.stalled.sum()), "bots": bots}


def figures(bots: list[dict], weights: list[float]) -> dict:
    """The figures of several bots together, each seat-game of bot i counting `weights[i] / n_i` (its share of the seats)."""
    w = [wt / b["n"] for wt, b in zip(weights, bots)]

    def total(key: str) -> float:
        return sum(wi * b[key] for wi, b in zip(w, bots))

    return dict(per_delivery=total("shipped") / total("deliveries"), multi=total("multi") / total("trips"), factories=total("factories") / sum(weights),
                warehouses=total("warehouses") / sum(weights), owns=total("owns") / sum(weights))


def row(size: int, name: str, f: dict) -> str:
    return (f"| {size} | {name} | {f['per_delivery']:.2f} | {100 * f['multi']:.1f}% | {f['factories']:.2f} | {f['warehouses']:.2f} | "
            f"{100 * f['owns']:.0f}% |")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--seeds", type=int, default=100)
    a = ap.parse_args()
    lines = ["# Offer exposure: the heuristics and the scripted bots", "",
             f"{a.seeds} `DEV` seeds per composition, all rotations, completed games, per seat-game of each bot, each population playing among itself "
             "(no learner seated). The heuristics are Producer, Showcase and Shipper; the scripted bots are Merchant, Simple and Random, which the trainer "
             "seats 60% / 30% / 10% of its scripted seats. The nets of the pool and the learner's own snapshots are not measured.", "",
             "| seats | bot | containers / delivery | deliveries of 2+ containers | factories built | warehouses built | seat-games with a warehouse built |",
             "|---|---|---|---|---|---|---|"]
    stalls = []
    for size in G.SIZES:
        for title, group, draw in (("heuristics", HEURISTICS, None), ("scripted", SCRIPTED, SCRIPTED_DRAW)):
            m = measure(list(group.values()), size, a.seeds)
            stalls.append(f"{title} at {size} seats: {m['stalled']} of {m['games']:,} games stalled and left out")
            for (name, _), bot in zip(group.items(), m["bots"]):
                lines.append(row(size, name, figures([bot], [1.0])))
            weights = [draw[n] for n in group] if draw else [1.0] * len(group)
            lines.append(row(size, "**" + ("all three, equal weights" if not draw else "the trainer's mix 60/30/10") + "**", figures(m["bots"], weights)))
    lines += ["", "Left out as stalled: " + "; ".join(stalls) + "."]
    a.out.parent.mkdir(parents=True, exist_ok=True)
    a.out.write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
