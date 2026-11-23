"""Does a table of complementary strategies change what a Shipper can earn and buy? Evaluation only.

    python -m training.ecosystem select                                  # which Producer / Showcase to seat
    python -m training.ecosystem table peddler2 runs/strategist/shipper_s1/final.pt --games 200 \
        --compose current=peddler2,consortium,magnate \
        --compose complementary=runs/strategist/producer_s1/final.pt,runs/strategist/showcase_variety_s1/final.pt,runs/strategist/control_s1/final.pt

`table` seats each focus net at every seat in turn of every composition, on the same seeds (games are paired across
compositions), and reports two channels next to the focus seat's economic outcome: supply (what the focus seat
could buy and at what price; what rivals offered) and delivery demand (rival bids: how many could afford $3 and $10,
their cash, the winning bid against the bidder's cash and against the cargo's nominal value). Liquidity (rival cash)
is reported apart from behaviour so that a richer table is not mistaken for better trading partners.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import numpy as np

from training import economics, gate
from training import env as V
from training import evalcore as E


def ratio(num, den) -> float:
    return float(num / den) if den else float("nan")


def summarise(rec: E.Records, specs, table_size: int) -> dict:
    """Per-composition numbers: per-game arrays for the paired outcome metrics, plus pooled channel ratios."""
    pg = economics.per_game(rec)
    ok = ~rec.stalled
    inter = rec.interaction[ok][:, :table_size].astype(np.float64)
    mine = (rec.seats[ok][:, :table_size] == 0)
    focus = inter[mine]
    rivals = inter[~mine].reshape(len(focus), table_size - 1, -1)
    ids = np.nonzero(ok)[0]
    pg["ok"] = ids
    pg["bought"] = focus[:, V.BOUGHT]
    pg["delivery_cash_minus_purchases"] = pg["delivery_cash"] - focus[:, V.SPENT]  # not profit: see `training/ledger.py`
    deliveries = focus[:, V.DELIVERIES].sum()
    bids_made = rivals[:, :, V.BIDS].sum()
    channels = {
        # supply: what the focus seat could buy, at what price, and what rivals put on sale
        "bought_per_game": focus[:, V.BOUGHT].mean(),
        "price_per_container": ratio(focus[:, V.SPENT].sum(), focus[:, V.BOUGHT].sum()),
        "rival_offers_per_game": rivals[:, :, V.OFFERED].sum(1).mean(),
        "rival_competitive_share": ratio(rivals[:, :, V.OFFERED_COMPETITIVE].sum(), rivals[:, :, V.OFFERED].sum()),
        "rival_sold_per_game": rivals[:, :, V.SOLD].sum(1).mean(),
        # delivery demand: who could pay, and what they paid
        "rival_bids_per_delivery": ratio(focus[:, V.RIVAL_BIDS].sum(), deliveries),
        "able_to_pay_3": ratio(focus[:, V.RIVAL_CASH_3].sum(), focus[:, V.RIVAL_BIDS].sum()),
        "able_to_pay_10": ratio(focus[:, V.RIVAL_CASH_10].sum(), focus[:, V.RIVAL_BIDS].sum()),
        "rival_cash_at_bid": ratio(rivals[:, :, V.BID_CASH].sum(), bids_made),
        "mean_rival_bid": ratio(rivals[:, :, V.BID_MONEY].sum(), bids_made),
        "bid_over_cash": ratio(rivals[:, :, V.BID_MONEY].sum(), rivals[:, :, V.BID_CASH].sum()),
        "bid_over_value": ratio(rivals[:, :, V.BID_MONEY].sum(), rivals[:, :, V.BID_VALUE].sum()),
        "winning_bid": ratio(focus[:, V.CARGO_CASH:V.CARGO_CASH + 5].sum() / 2, deliveries),
        "rival_trading_profit": (rivals[:, :, V.EARNED] - rivals[:, :, V.SPENT]).sum(1).mean(),
        "stalled": float(rec.stalled.mean()),
    }
    return dict(pg=pg, channels=channels)


def play(focus: str, table: tuple[str, ...], games: int):
    label, path, rules = economics.resolve(focus)
    rec, specs = gate.play_focus(path, rules, games, table=table)
    return label, summarise(rec, specs, len(table) + 1), specs


def ci(pg: dict, metric: str, specs) -> tuple[float, float, float]:
    clusters = np.array([specs[g].cluster + 1_000_000 * specs[g].cell for g in pg["ok"]])
    return E.mean_ci(pg[metric], clusters)


OUTCOMES = ("delivery_cash", "bought", "delivery_cash_minus_purchases", "deliveries", "trading_profit", "win", "rank")
CHANNELS = ("bought_per_game", "price_per_container", "rival_offers_per_game", "rival_competitive_share",
            "rival_sold_per_game", "rival_bids_per_delivery", "able_to_pay_3", "able_to_pay_10", "rival_cash_at_bid",
            "mean_rival_bid", "bid_over_cash", "bid_over_value", "winning_bid", "rival_trading_profit", "stalled")


def render(results: dict) -> str:
    lines = []
    for focus, per in results.items():
        names = list(per)
        lines += [f"\n**Focus: {focus}** (per game of the focus seat, 95% CI over seed clusters; same seeds in every composition)\n",
                  "| metric | " + " | ".join(names) + " |", "|---|" + "---|" * len(names)]
        for m in OUTCOMES:
            cells = []
            for n in names:
                s = per[n]
                mean, lo, hi = ci(s["pg"], m, s["specs"])
                cells.append(f"{mean:.2f} [{lo:.2f}, {hi:.2f}]")
            lines.append(f"| {m} | " + " | ".join(cells) + " |")
        for m in CHANNELS:
            lines.append(f"| {m} | " + " | ".join(f"{per[n]['channels'][m]:.3f}" for n in names) + " |")
    return "\n".join(lines)


def table_cmd(args) -> None:
    comps = {}
    for item in args.compose:
        name, _, seats = item.partition("=")
        comps[name] = tuple(seats.split(","))
    results: dict[str, dict] = {}
    for focus in args.focus:
        for name, table in comps.items():
            label, s, specs = play(focus, table, args.games)
            s["specs"] = specs
            results.setdefault(label, {})[name] = s
            print(f"done {label} / {name}", flush=True)
    print(render(results))


SELECT_METRICS = {
    "producer": ("factories_held", "produced", "sell_through", "cheap_shelf_stock", "sales_share", "bought"),
    "showcase": ("warehouses_held", "competitive_colors", "purchase_colors", "colors_taken", "sold"),
}


def select_cmd(args) -> None:
    runs = Path("runs/strategist")
    names = sorted(p.name for p in runs.iterdir() if p.is_dir() and (p / "final.pt").exists()
                   and p.name.startswith(("producer", "showcase", "control")))
    cols = ("factories_held", "warehouses_held", "produced", "sold", "bought", "sell_through", "cheap_shelf_stock",
            "sales_share", "competitive_colors", "purchase_colors", "colors_taken", "delivery_cash", "win", "rank")
    print("| net | " + " | ".join(cols) + " | stalled |\n|---|" + "---|" * (len(cols) + 1))
    for n in names:
        label, path, rules = economics.resolve(str(runs / n / "final.pt"))
        rec, _ = gate.play_focus(path, rules, args.games)
        g = gate.game_metrics(rec)
        pg = economics.per_game(rec)
        vals = {**{k: v.mean() for k, v in g.items() if k != "ok"}, **{k: v.mean() for k, v in pg.items() if k != "ok"}}
        print(f"| {n} | " + " | ".join(f"{vals[c]:.3f}" for c in cols) + f" | {float(rec.stalled.mean()):.3f} |", flush=True)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    sel = sub.add_parser("select")
    sel.add_argument("--games", type=int, default=150)
    tab = sub.add_parser("table")
    tab.add_argument("focus", nargs="+")
    tab.add_argument("--games", type=int, default=200)
    tab.add_argument("--compose", action="append", required=True, help="name=seat,seat,seat (names from NETS or checkpoint paths)")
    args = ap.parse_args()
    {"select": select_cmd, "table": table_cmd}[args.cmd](args)


if __name__ == "__main__":
    main()
