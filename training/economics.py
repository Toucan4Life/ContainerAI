"""Economic probes: does investment pay, and what does a delivery of each size bring?

    python -m training.economics build peddler2 --games 300
    python -m training.economics cargo peddler2 consortium magnate runs/strategist/control_s1/final.pt --games 100

`build`: the net plays the gate's tables (Peddler v2, Consortium, Magnate beside it, seats rotated, same seeds in every arm) in
three arms: as it is, with a scripted factory, and with a scripted warehouse. The script builds at the net's first Main decision
that leaves `--reserve` cash after paying the real build cost (the net plays all its other moves). It reports the paired
difference per game against the normal arm, and whether the capacity was used: factories held per decision, containers produced
and sold, harbor containers laid out per decision. A build the net does not use is not evidence that building is bad.

`cargo`: the share of Island trips carrying 1 to 5 containers, the deliverer's cash per trip and per container for each size (twice
the winning bid), the distinct colours per cargo, the sourcing cost (cash spent buying containers, per container bought; production
wages are not counted), and the sails per container shipped. Cargoes of 3 or more are rare, so sizes with fewer than `MIN_TRIPS`
trips are marked and nets can be pooled with `--pool`. Three options change the table:
`--bid-percent P` makes every rival bid P% of the cargo's value on its own card, so that bids scale with cargo;
`--lend` also lends a rival the cash it lacks for that bid, until the auction is settled, to separate what rivals can pay from
what the cargo is worth; `--min-cargo N...` also holds the net's ship until it carries N containers, to compare each size.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import numpy as np

from training import env as V
from training import evalcore as E
from training import gate
from training.strategy_metrics import NETS

MIN_TRIPS = 30
SAILS = ("sail_ocean", "sail_bank", "sail_harbor", "sail_island")
ARMS = {"normal": None, "factory": False, "warehouse": True}


def resolve(spec: str) -> tuple[str, str, dict]:
    """(label, checkpoint, rules): a name from `strategy_metrics.NETS`, or a checkpoint path (played under Peddler v2's rules)."""
    if spec in NETS:
        path, rules = NETS[spec].partition(":")[::2]
    else:
        path, rules = spec, gate.RULES
    return Path(spec).parent.name if spec not in NETS else spec, path, dict(r.partition("=")[::2] for r in rules.split(",") if r)


def per_game(rec: E.Records) -> dict[str, np.ndarray]:
    """Per-game economics of the focus seat (policy 0) over the games that finished, in spec order."""
    ok = ~rec.stalled
    rows = rec.interaction[(rec.seats == 0) & ok[:, None]].astype(np.float64)
    decisions = np.maximum(rows[:, V.MAIN_DECISIONS], 1)
    return {
        "produced": rows[:, V.PRODUCED], "sold": rows[:, V.SOLD], "bought": rows[:, V.BOUGHT],
        "trading_profit": rows[:, V.EARNED] - rows[:, V.SPENT],
        "factories_held": rows[:, V.FACTORIES_HELD] / decisions,
        "warehouses_held": rows[:, V.WAREHOUSES_HELD] / decisions,
        "harbor_offered": rows[:, V.HARBOR_OFFERED] / decisions,
        "deliveries": rows[:, V.DELIVERIES],
        "delivery_cash": rows[:, V.CARGO_CASH:V.CARGO_CASH + 5].sum(1),
        "script_builds": rows[:, V.SCRIPT_BUILDS],
        "win": (E.winner_share(rec) * (rec.seats == 0)).sum(1)[ok],
        "rank": (E.rank_share(rec) * (rec.seats == 0)).sum(1)[ok],
        "ok": np.nonzero(ok)[0],
    }


def build_probe(spec: str, games: int, reserve: int, after_main: int) -> str:
    label, path, rules = resolve(spec)
    arms = {}
    for arm, warehouse in ARMS.items():
        extra = None if warehouse is None else (
            lambda env, slot, w=warehouse: env.build_script(slot, warehouse=w, after_main=after_main, reserve=reserve))
        rec, specs = gate.play_focus(path, rules, games, extra=extra)
        arms[arm] = per_game(rec)
    metrics = ("script_builds", "factories_held", "warehouses_held", "produced", "sold", "bought", "harbor_offered",
               "trading_profit", "deliveries", "delivery_cash", "win", "rank")
    lines = [f"**Build probe: {label}** ({games} games per seat; reserve ${reserve}, after {after_main} own Main decisions; "
             "difference to the normal arm, paired, [95% CI])\n",
             "| metric | normal | + factory | diff | + warehouse | diff |", "|---|---|---|---|---|---|"]
    for m in metrics:
        cells = [f"{arms['normal'][m].mean():.3f}"]
        for arm in ("factory", "warehouse"):
            r = gate.compare(arms[arm], arms["normal"], specs, m)
            cells += [f"{r['cand']:.3f}", f"{r['diff']:+.3f} [{r['lo']:+.3f}, {r['hi']:+.3f}]"]
        lines.append(f"| {m} | " + " | ".join(cells) + " |")
    return "\n".join(lines) + "\n"


def table_extra(bid_percent: int, min_cargo: int, lend: bool = False):
    """Hook for `gate.play_focus`: rivals bid by value (the focus seat, slot `base`, does not), and the focus seat holds its ship until `min_cargo`."""
    def extra(env, base):
        if bid_percent:
            env.value_bidders(base, bid_percent, lend)
        if min_cargo > 1:
            env.require_min_cargo(base, min_cargo)
    return extra


def cargo_rows(spec: str, games: int, bid_percent: int = 0, min_cargo: int = 1, lend: bool = False):
    label, path, rules = resolve(spec)
    rec, _ = gate.play_focus(path, rules, games, extra=table_extra(bid_percent, min_cargo, lend))
    ok = ~rec.stalled
    summary = per_game(rec)
    summary["stalled"] = float(rec.stalled.mean())
    return label, rec.interaction[(rec.seats == 0) & ok[:, None]].astype(np.float64), V.move_names(), summary


def cargo_table(label: str, rows: np.ndarray, names: list[str]) -> str:
    s = rows.sum(0)
    trips = s[V.CARGO_SIZE:V.CARGO_SIZE + 5]
    cash = s[V.CARGO_CASH:V.CARGO_CASH + 5]
    sails = sum(s[V.MOVES + names.index(n)] for n in SAILS)
    shipped = s[V.SHIPPED]
    lines = [f"**Cargo economics: {label}** ({len(rows)} games, {int(trips.sum())} trips, deliverer cash from accepted deliveries; "
             f"sourcing ${s[V.SPENT] / max(s[V.BOUGHT], 1):.2f} per container bought, "
             f"{sails / max(shipped, 1):.1f} sails per container shipped, {s[V.CARGO_COLORS] / max(trips.sum(), 1):.2f} colours per cargo)\n",
             "| containers | trips | share | cash per trip | cash per container |", "|---|---|---|---|---|"]
    for k in range(5):
        n = trips[k]
        note = " (few)" if 0 < n < MIN_TRIPS else ""
        lines.append(f"| {k + 1} | {int(n)}{note} | {100 * n / max(trips.sum(), 1):.1f}% | "
                     f"{'-' if not n else f'${cash[k] / n:.2f}'} | {'-' if not n else f'${cash[k] / n / (k + 1):.2f}'} |")
    return "\n".join(lines) + "\n"


def sweep_row(label: str, n: int, rows: np.ndarray, names: list[str], summary: dict) -> str:
    s = rows.sum(0)
    trips, shipped = s[V.DELIVERIES], s[V.SHIPPED]
    cash = s[V.CARGO_CASH:V.CARGO_CASH + 5].sum()
    sails = sum(s[V.MOVES + names.index(k)] for k in SAILS)
    games = len(rows)
    return (f"| {label} | {n} | {trips / games:.1f} | {shipped / max(trips, 1):.2f} | {s[V.CARGO_COLORS] / max(trips, 1):.2f} | "
            f"${cash / max(shipped, 1):.2f} | "
            f"{cash / games:.1f} | {(cash - s[V.SPENT]) / games:.1f} | {sails / max(shipped, 1):.1f} | "
            f"{summary['win'].mean():.3f} | {summary['rank'].mean():.3f} | {100 * summary['stalled']:.0f}% |")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("probe", choices=("build", "cargo"))
    ap.add_argument("nets", nargs="+", help="a name from strategy_metrics.NETS or a checkpoint path")
    ap.add_argument("--games", type=int, default=300, help="games per seat")
    ap.add_argument("--reserve", type=int, default=5, help="cash the build must leave")
    ap.add_argument("--after-main", type=int, default=0, help="own Main decisions before the build")
    ap.add_argument("--pool", action="store_true", help="cargo: add a row pooling all the nets")
    ap.add_argument("--bid-percent", type=int, default=0,
                    help="cargo: rivals bid this percent of the cargo's value on their own card (0: the nets bid as trained)")
    ap.add_argument("--lend", action="store_true",
                    help="cargo, with --bid-percent: lend a rival the cash it lacks for its bid until the auction is settled")
    ap.add_argument("--min-cargo", type=int, nargs="*", default=[],
                    help="cargo: also hold the focus ship until it has this many containers, one run for each value")
    args = ap.parse_args()
    if args.probe == "build":
        for spec in args.nets:
            print(build_probe(spec, args.games, args.reserve, args.after_main), flush=True)
        return
    note = f"rivals bid {args.bid_percent}% of card value" if args.bid_percent else "rivals bid as trained"
    note += ", lent the cash they lack" if args.bid_percent and args.lend else ""
    got = [cargo_rows(spec, args.games, args.bid_percent, 1, args.lend) for spec in args.nets]
    for label, rows, names, _ in got:
        print(cargo_table(f"{label}, {note}", rows, names), flush=True)
    if args.pool and len(got) > 1:
        print(cargo_table(f"pooled, {note}", np.concatenate([r for _, r, _, _ in got]), got[0][2]))
    if args.min_cargo:
        print(f"**Holding the ship until it carries N containers ({note}; per game of the focus seat)**\n")
        print("| net | N | trips | containers/trip | colours/cargo | cash/container | delivery cash | cash less purchases | sails/container | win | rank | stalled |")
        print("|---|---|---|---|---|---|---|---|---|---|---|---|")
        for spec in args.nets:
            for n in args.min_cargo:
                label, rows, names, summary = cargo_rows(spec, args.games, args.bid_percent, n, args.lend)
                print(sweep_row(label, n, rows, names, summary), flush=True)


if __name__ == "__main__":
    main()
