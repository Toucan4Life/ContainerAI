"""Strategy diagnostics from the env's per-seat counters (`BatchEnv.interaction`). Not rewards by themselves;
the shaping terms in `selfplay` use the same columns.

Formulas, over the counters of one seat summed (or averaged) over games:

* `produce_rate`      = produce actions / main decisions in which Produce was legal
* `produced_per_opp`  = containers put in the factory by Produce / main decisions in which Produce was legal
* `sell_through`      = containers sold to opponents / containers produced (not capped: resales of bought
                        containers count, so it can exceed 1; 0 when nothing was produced)
* `cheap_shelf_stock` = containers on the seat's own lots priced at most $3, summed over main decisions / main decisions. It is
                        a stock, not market activity: cheap inventory waiting on a shelf, whether or not anyone buys it (sales are
                        `sell_through`). Formerly `competitive_offers`, which is kept as an alias for historical logs and reports.
* `competitive_share` = containers on lots priced at most $3 / containers on lots (both summed over main decisions)
* `competitive_colors` = distinct colors on lots priced at most $3, per main decision
* `purchase_colors`   = distinct colors bought from opponents, per game
* `purchase_spread`   = 1 - (containers of the most-bought color / containers bought)
* `island_share`      = containers shipped to the Island / (produced + bought); `deliveries` is the absolute trip count
* `subsidy`           = money the supply created for the seat's accepted deliveries (equal to the winning bids)
* `bid_money`         = money of all the delivery bids the seat placed

`delivery_cash_minus_purchases` (formerly `delivery_net`, kept as an alias) is the cash a seat received for its accepted deliveries
(twice the winning bid) minus what it paid for containers. It is not profit: it ignores build costs, wages, interest, loans, the Bank
and the cargoes the seat won as bidder. `training/ledger.py` accounts for all of them.

A ratio of sums is used for every rate, so a game with few decisions weighs as little as it should.

    python -m training.strategy_metrics --games 300 --seeds 3 peddler2=training/league/peddler2.pt:min_bid=3,...

scores nets as the focus seat of mixed tables, with several seed blocks to show the seed-to-seed spread.
"""

from __future__ import annotations

from collections.abc import Sequence

import numpy as np

from training.env import (BID_MONEY, BOUGHT, BOUGHT_COLOR, COMPETITIVE_COLORS, DELIVERIES, MAIN_DECISIONS, OFFERED,
                          OFFERED_COMPETITIVE, PRODUCE_CHANCES, PRODUCED, PRODUCES, SHIPPED, SOLD, SUBSIDY)

METRICS = ("produce_rate", "produced_per_opp", "sell_through", "cheap_shelf_stock", "competitive_share",
           "competitive_colors", "purchase_colors", "purchase_spread", "island_share", "deliveries", "subsidy",
           "bid_money")
# Names that earlier logs and reports use for metrics that now have a more accurate one.
ALIASES = {"competitive_offers": "cheap_shelf_stock", "delivery_net": "delivery_cash_minus_purchases"}


def _ratio(num, den) -> float:
    return float(num / den) if den else 0.0


def seat_metrics(rows: np.ndarray) -> dict[str, float]:
    """The metrics of one seat from its counter rows, one row per game (a single row works too)."""
    rows = np.atleast_2d(rows).astype(np.float64)
    s = rows.sum(0)
    colors = rows[:, BOUGHT_COLOR:BOUGHT_COLOR + 5]
    out = dict(
        produce_rate=_ratio(s[PRODUCES], s[PRODUCE_CHANCES]),
        produced_per_opp=_ratio(s[PRODUCED], s[PRODUCE_CHANCES]),
        sell_through=_ratio(s[SOLD], s[PRODUCED]),
        cheap_shelf_stock=_ratio(s[OFFERED_COMPETITIVE], s[MAIN_DECISIONS]),
        competitive_share=_ratio(s[OFFERED_COMPETITIVE], s[OFFERED]),
        competitive_colors=_ratio(s[COMPETITIVE_COLORS], s[MAIN_DECISIONS]),
        purchase_colors=float((colors > 0).sum(1).mean()),
        purchase_spread=1.0 - _ratio(colors.sum(0).max(), s[BOUGHT]) if s[BOUGHT] else 0.0,
        island_share=_ratio(s[SHIPPED], s[PRODUCED] + s[BOUGHT]),
        deliveries=float(rows[:, DELIVERIES].mean()),
        subsidy=float(rows[:, SUBSIDY].mean()),
        bid_money=float(rows[:, BID_MONEY].mean()),
    )
    out["competitive_offers"] = out["cheap_shelf_stock"]  # the old name, so that logs stay comparable
    return out


# --------------------------------------------------------------------------------------- scoring nets

NETS = {
    "peddler2": "training/league/peddler2.pt:min_bid=3,no_same_turn_repay,buy_at_harbor,bank_needs_load,no_double_reprice",
    "consortium": "training/anchors/consortium.pt",
    "magnate": "populations/baselines/defense_run2_vs_exploiter_30m.pt",
}


def configure_rules(env, slot: int, rules: dict) -> None:
    """Binds the seats of `slot` to the rules a net was trained under."""
    if "min_bid" in rules:
        env.require_min_bid(slot, int(rules["min_bid"]))
    if "max_price" in rules:
        env.require_max_price(slot, int(rules["max_price"]))
    for name, bind in (("no_same_turn_repay", env.forbid_same_turn_repay), ("buy_at_harbor", env.require_harbor_purchase),
                       ("bank_needs_load", env.require_bank_load), ("no_double_reprice", env.forbid_double_reprice),
                       ("bid_within_value", env.require_bid_within_value)):
        if name in rules:
            bind(slot)


def configure_table(env, nets: Sequence[tuple[int, dict]]) -> None:
    """Binds the rules of every net at a table, given as (slot, rules).

    The engine holds one slot per rule, so binding a second net overwrites the first, and only one net would
    play by its rules. The net with the most rules owns them; a second net with rules plays by the owner's
    through the engine's single mirror, as a frozen Peddler seat does in training. Its rules must therefore be
    among the owner's (a capped run's Peddler seat inherits the cap, as in that run's training).
    """
    # a flag rule is bound by its name alone; only the amounts of the others have to agree
    amounts = ("min_bid", "max_price")
    ruled = sorted(((slot, {k: str(v) if k in amounts else "" for k, v in r.items()}) for slot, r in nets if r),
                   key=lambda n: -len(n[1]))
    if not ruled:
        return
    (owner, rules), *mirrored = ruled
    configure_rules(env, owner, rules)
    for _, other in mirrored:
        if not other.items() <= rules.items():
            raise ValueError(f"rules {other} cannot be mirrored from {rules}: they are not among them")
    if len(mirrored) == 1:
        env.mirror_rules(mirrored[0][0], owner)
    elif mirrored:
        base = min(slot for slot, _ in nets)
        env.mirror_rules_set(sum(1 << (slot - base) for slot, _ in mirrored), owner)


def score(focus: str, table: tuple[str, ...], games: int, block: int, envs: int = 256) -> dict:
    """Metrics of the `focus` net at `table`, sitting at every seat in turn (`games` per seat), on seed block `block`."""
    from training import evalcore as E
    from training.audit_interaction import parse
    from training.env import STANDARD

    parsed = [parse(f"{n}={NETS[n]}") for n in (focus, *table)]
    pol = [E.Policy(n if i else "focus", p, "guarded") for i, (n, p, _) in enumerate(parsed)]

    def configure(env):
        configure_table(env, [(E.SLOT_BASE + k, rules) for k, (_, _, rules) in enumerate(parsed)])

    n = len(table) + 1
    specs = []
    for seat in range(n):
        others = iter(range(1, n))
        seats = tuple(0 if s == seat else next(others) for s in range(n))
        for i in range(games):
            index = block * games + i
            specs.append(E.GameSpec(n, STANDARD, E.game_seed(E.AUDIT, n, STANDARD, index), seats, seat, index))
    rec = E.play(specs, pol, envs=envs, log=None, configure=configure)
    ok = ~rec.stalled
    mine = rec.seats == 0
    rows = rec.interaction[mine & ok[:, None]]
    share = (E.winner_share(rec) * mine).sum(1)[ok]
    return seat_metrics(rows) | dict(win=float(share.mean()), stalled=int(rec.stalled.sum()), games=int(ok.sum()),
                                     sd_game={m: float(np.std(_per_game(rows, m))) for m in METRICS})


def _per_game(rows: np.ndarray, metric: str) -> np.ndarray:
    return np.array([seat_metrics(r)[metric] for r in rows])


def main() -> None:
    import argparse

    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("nets", nargs="*", choices=list(NETS), help="default: all")
    ap.add_argument("--games", type=int, default=300, help="games per seat and seed block")
    ap.add_argument("--blocks", type=int, default=3, help="seed blocks; their spread calibrates the effect-size threshold")
    ap.add_argument("--table", nargs="+", default=["peddler2", "consortium", "magnate"], choices=list(NETS),
                    help="the other seats; the scored net sits at every seat in turn")
    args = ap.parse_args()
    print(f"table: focus + {' + '.join(args.table)}; {args.games} games per seat, {args.blocks} seed blocks")
    for name in args.nets or NETS:
        runs = [score(name, tuple(args.table), args.games, b) for b in range(args.blocks)]
        print(f"\n{name}: win {np.mean([r['win'] for r in runs]):.3f}, stalled {sum(r['stalled'] for r in runs)}")
        print("| metric | mean | block SD | game SD | block SD / game SD |\n|---|---|---|---|---|")
        for m in METRICS:
            vals = np.array([r[m] for r in runs])
            game_sd = float(np.mean([r["sd_game"][m] for r in runs]))
            print(f"| {m} | {vals.mean():.3f} | {vals.std(ddof=1) if len(vals) > 1 else float('nan'):.3f} | {game_sd:.3f} "
                  f"| {(vals.std(ddof=1) / game_sd if game_sd and len(vals) > 1 else float('nan')):.2f} |")


if __name__ == "__main__":
    main()
