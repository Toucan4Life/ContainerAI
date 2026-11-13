"""Strength and interactivity of a net at tables with strong, different opponents.

One seat is the candidate; the others are fixed strong bots (Consortium, Magnate, Broker, `scratch_trade`, or a
Merchant standing in for a human). The candidate sits at every seat in turn. Reported per candidate and table:

* `share`: the candidate's winner share (1/players is fair), with a 95% interval over games;
* what it does with the others, per game: containers bought from opponents' factories and harbors, bids of at
  least $2 on their deliveries, containers sold to them, the mean price it got and the share of its containers
  laid out in a lot of $3 or less (a container priced in the top lots is one nobody buys).

    python -m training.interplay --games 300 new=runs/x/final.pt:min_bid=3,no_same_turn_repay broker=runs/scratch_constrained/final.pt:min_bid=3,no_same_turn_repay

A net may carry the rules it was trained under after a colon (`min_bid=N`, `max_price=N`, `no_same_turn_repay`, `classic`).
"""

from __future__ import annotations

import argparse
import itertools

import numpy as np

from training import evalcore as E
from training.audit_interaction import parse
from training.env import BOUGHT, EARNED, MERCHANT, PLACED_AT, REAL_BIDS, SOLD, SPENT, STANDARD

OPPONENTS = {
    "consortium": "training/anchors/consortium.pt",
    "magnate": "populations/baselines/defense_run2_vs_exploiter_30m.pt",
    "broker": "runs/scratch_constrained/final.pt:min_bid=3,no_same_turn_repay",
    "trade": "runs/scratch_trade/final.pt:no_same_turn_repay",
}
TABLES = {
    "3p C+M": ("consortium", "magnate"),
    "3p C+B": ("consortium", "broker"),
    "4p C+M+B": ("consortium", "magnate", "broker"),
    "4p C+M+T": ("consortium", "magnate", "trade"),
    "4p C+M+merchant": ("consortium", "magnate", "merchant"),
}
FAIR = 3


def evaluate(name: str, path: str, rules: dict, table: tuple[str, ...], games: int, envs: int = 256) -> dict:
    opp = [o for o in table if o != "merchant"]
    specs_of = {n: parse(f"{n}={OPPONENTS[n]}") for n in opp}
    pol = [E.Policy(name, path, "guarded")] + [E.Policy(n, specs_of[n][1], "guarded") for n in opp]
    if "merchant" in table:
        pol.append(E.Policy("merchant", scripted=MERCHANT))
    index = {n: i + 1 for i, n in enumerate(opp)} | ({"merchant": len(pol) - 1} if "merchant" in table else {})
    rule_of = {0: rules} | {i + 1: specs_of[n][2] for i, n in enumerate(opp)}

    def configure(env):
        for k, r in rule_of.items():
            if "classic" in r:
                env.ban_bank_all()
            if "min_bid" in r:
                env.require_min_bid(E.SLOT_BASE + k, int(r["min_bid"]))
            if "max_price" in r:
                env.require_max_price(E.SLOT_BASE + k, int(r["max_price"]))
            if "no_same_turn_repay" in r:
                env.forbid_same_turn_repay(E.SLOT_BASE + k)

    n = len(table) + 1
    specs = []
    for seat in range(n):
        others = iter(index[o] for o in table)
        seats = tuple(0 if s == seat else next(others) for s in range(n))
        for i in range(games):
            specs.append(E.GameSpec(n, STANDARD, E.game_seed(E.AUDIT, n, STANDARD, i), seats, seat, i))
    rec = E.play(specs, pol, envs=envs, log=None, configure=configure)
    ok = ~rec.stalled
    mine = rec.seats == 0
    share = (E.winner_share(rec) * mine).sum(1)[ok]
    row = rec.interaction[mine & ok[:, None]]  # one row per game: the candidate's counters
    placed = row[:, PLACED_AT:PLACED_AT + 6].sum(1)
    sold = row[:, SOLD].sum()
    return dict(
        name=name, table=" + ".join(table), players=n, games=int(ok.sum()), stalled=int(rec.stalled.sum()),
        share=share.mean(), ci=1.96 * share.std() / np.sqrt(len(share)), fair=1 / n,
        bought=row[:, BOUGHT].mean(), bids=row[:, REAL_BIDS].mean(), sold=row[:, SOLD].mean(),
        sell_price=row[:, EARNED].sum() / sold if sold else float("nan"),
        buy_price=row[:, SPENT].sum() / max(row[:, BOUGHT].sum(), 1),
        fair_share=row[:, PLACED_AT:PLACED_AT + FAIR].sum() / max(placed.sum(), 1),
        price_mix=row[:, PLACED_AT:PLACED_AT + 6].sum(0) / max(placed.sum(), 1),
        cash=rec.breakdown[mine & ok[:, None]][:, 0].mean(),
    )


def render(rows: list[dict]) -> str:
    out = ["| net | table | share (fair) | bought | bids >= $2 | sold | sell $ | buy $ | priced <= $3 | priced $1..$6 (%) | stalled |",
           "|---|---|---|---|---|---|---|---|---|---|---|"]
    for r in rows:
        out.append(f"| {r['name']} | {r['table']} | {r['share']:.3f} ± {r['ci']:.3f} ({r['fair']:.2f}) | {r['bought']:.1f} "
                   f"| {r['bids']:.1f} | {r['sold']:.1f} | {r['sell_price']:.2f} | {r['buy_price']:.2f} "
                   f"| {r['fair_share']:.2f} | {' '.join(f'{100 * x:.0f}' for x in r['price_mix'])} | {r['stalled']} |")
    return "\n".join(out)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("nets", nargs="+", help="name=path[:rule,rule]")
    ap.add_argument("--games", type=int, default=300, help="games per seat of the candidate")
    ap.add_argument("--tables", nargs="+", default=list(TABLES), choices=list(TABLES))
    args = ap.parse_args()
    rows = []
    for spec in args.nets:
        name, path, rules = parse(spec)
        for t in args.tables:
            rows.append(evaluate(name, path, rules, TABLES[t], args.games))
            print(render(rows[-1:]).splitlines()[-1], flush=True)
    print()
    print(render(rows))


if __name__ == "__main__":
    main()
