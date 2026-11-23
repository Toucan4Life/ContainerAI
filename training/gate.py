"""Phase 2 divergence gate: does an archetype net differ from Peddler v2 in the way its objective intends?

    python -m training.gate producer runs/strategist/producer_s1/final.pt --games 300

Candidate and Peddler v2 each sit at every seat in turn of the same tables (same seeds, same opponents:
Peddler v2, Consortium, Magnate) under the same table rules (those the candidate was trained under), so games
are paired and only the focus net differs. The table's Peddler v2 seat plays by the focus net's rules (the
engine mirrors them, as in training); Consortium and Magnate play without rules, as deployed. For each metric
the paired difference per game is bootstrapped over seed clusters; effect size = mean difference / SD of the
baseline's per-game values.

Three things are reported apart, and a high win rate does not make up for a failed one:

* Fidelity (`diverges`): the primary metric and at least one corroborating metric each have effect >= `--effect`
  (in the intended direction) with a 95% interval excluding zero. The Producer's primary is containers produced;
  `sales_share` is only reported, because a reseller sells a lot without producing.
* Economic quality (`economic_ok`): none of the archetype's pathologies. The Producer's volume must reach buyers
  and the final score: it may not sell too little of what it produces (an absolute floor on sell-through, since
  Peddler v2 resells and so out-sells any Producer per container made), and its final score may not be materially
  below Peddler v2's (the effect and interval above), which is what giving the value away would show as.
* Game quality (`game_ok`): the candidate's tables do not hit the action horizon (nobody scores) materially more
  often than Peddler v2's. Stalled games leave the metrics above, so this is the only place they show.

`passes` = fidelity and economic quality. No action quotas: the metrics are positions and outcomes, and any legal
action may be used. The win-floor checks are reported separately.

The stock a net ends with is not derived here: 1 + produced + bought - sold - shipped is exact only for a seat that
never takes containers from the Bank, as the heuristics do not, and nets do (it came out negative in 31% of
Peddler v2's games). Sell-through is the unsold-stock check for a net; `training/egta.py` reports the exact stock
of the heuristics.
"""

from __future__ import annotations

import argparse
import json

import numpy as np

from training import evalcore as E
from training.audit_interaction import parse
from training.env import CARGO_CASH, DELIVERIES, EARNED, PRODUCED, SHIPPED, SOLD, SOLD_COLOR, SPENT, STANDARD
from training.strategy_metrics import NETS, configure_table, seat_metrics

RULES = NETS["peddler2"].partition(":")[2]
TABLE = ("peddler2", "consortium", "magnate")

# archetype -> (primary, corroborating alternatives); all are "higher is more of the archetype"
GATES = {
    "producer": ("produced", ("sell_through", "cheap_shelf_stock", "competitive_share")),
    "showcase": ("colors_taken", ("competitive_colors", "purchase_colors", "sold")),
    "shipper": ("delivery_cash_minus_purchases", ("delivery_cash", "deliveries", "cargo_size")),
}
# Reported next to a gate's metrics: what the Producer's volume came to. `score` is tested (see `verdict`), the others only read.
ECONOMICS = {"producer": ("sales_share", "avg_price", "cash", "score")}
WIN_FLOOR = 0.04  # the candidate may win at most this much less than Peddler v2 (absolute share)
# Provisional absolute guard: outright winner share (ties split; fair share 1/4 at this 4-seat table), lower 95% bound.
# `rank` (share of opponents beaten, ties half; fair share 0.5) is reported next to it, not thresholded.
ABS_WIN_FLOOR = 0.35
# Provisional absolute guard for the Producer: containers sold per container produced, upper 95% bound of the candidate's mean.
# What is left in a factory at the end scores nothing, so a Producer that sells well under what it makes is wasting its wages.
SELL_THROUGH_FLOOR = 0.8


def game_metrics(rec: E.Records) -> dict[str, np.ndarray]:
    """Per-game values for the focus seat (policy 0) of every non-stalled game, in spec order."""
    ok = ~rec.stalled
    mine = (rec.seats == 0) & ok[:, None]
    rows = rec.interaction[mine]
    table_sold = rec.interaction[ok][:, :, SOLD].sum(1)
    out = {m: np.array([seat_metrics(r)[m] for r in rows]) for m in
           ("sell_through", "cheap_shelf_stock", "competitive_share", "competitive_colors", "purchase_colors",
            "island_share", "deliveries")}
    out["sales_share"] = rows[:, SOLD] / np.maximum(table_sold, 1)
    out["colors_taken"] = (rows[:, SOLD_COLOR:SOLD_COLOR + 5] > 0).sum(1).astype(float)
    out["sold"] = rows[:, SOLD].astype(float)
    out["produced"] = rows[:, PRODUCED].astype(float)
    out["earned"] = rows[:, EARNED].astype(float)
    out["island_points"] = rec.breakdown[mine][:, 1].astype(float)
    out["avg_price"] = rows[:, EARNED] / np.maximum(rows[:, SOLD], 1)
    final = rec.breakdown[mine]  # cash, island, leftovers, loans
    out["cash"] = final[:, 0].astype(float)
    out["score"] = (final[:, :3].sum(1) - final[:, 3]).astype(float)
    # Deliverer side: cash its accepted deliveries brought (twice the winning bid) less what it paid for containers. Not profit: see `training/ledger.py`.
    cash = rows[:, CARGO_CASH:CARGO_CASH + 5].sum(1).astype(float)
    out["delivery_cash"] = cash
    out["delivery_cash_minus_purchases"] = cash - rows[:, SPENT]
    out["cargo_size"] = rows[:, SHIPPED] / np.maximum(rows[:, DELIVERIES], 1)
    out["win"] = (E.winner_share(rec) * (rec.seats == 0)).sum(1)[ok]
    out["rank"] = (E.rank_share(rec) * (rec.seats == 0)).sum(1)[ok]
    out["ok"] = np.nonzero(ok)[0]
    return out


def table_entry(spec: str):
    """(label, checkpoint, rules) of a table seat: a name from `NETS`, or a checkpoint path (it plays under Peddler v2's rules, as the runs were trained)."""
    if spec in NETS:
        return parse(f"{spec}={NETS[spec]}")
    return parse(f"{spec.split('/')[-2]}={spec}:{RULES}")


def play_focus(path: str, rules: dict, games: int, envs: int = 256, extra=None, table=TABLE) -> tuple[E.Records, list]:
    parsed = [(None, path, rules)] + [table_entry(n) for n in table]
    pol = [E.Policy("focus", path, "guarded")] + [E.Policy(n, p, "guarded") for n, p, _ in parsed[1:]]
    n = len(table) + 1
    specs = []
    for seat in range(n):
        others = iter(range(1, n))
        seats = tuple(0 if s == seat else next(others) for s in range(n))
        for i in range(games):
            specs.append(E.GameSpec(n, STANDARD, E.game_seed(E.AUDIT, n, STANDARD, 10_000 + i), seats, seat, i))

    def configure(env):
        configure_table(env, [(E.SLOT_BASE + k, r) for k, (_, _, r) in enumerate(parsed)])
        if extra:
            extra(env, E.SLOT_BASE)

    return E.play(specs, pol, envs=envs, log=None, configure=configure), specs


def compare(cand: dict, base: dict, specs, metric: str) -> dict:
    """Paired effect of the candidate over the baseline on one metric (games stalled in either are dropped)."""
    common, ic, ib = np.intersect1d(cand["ok"], base["ok"], return_indices=True)
    diff = cand[metric][ic] - base[metric][ib]
    sd = float(np.std(base[metric][ib])) or 1.0
    clusters = np.array([specs[g].cluster + 1_000_000 * specs[g].cell for g in common])
    mean, lo, hi = E.mean_ci(diff, clusters)
    return dict(metric=metric, base=float(base[metric][ib].mean()), cand=float(cand[metric][ic].mean()),
                diff=mean, lo=lo, hi=hi, effect=mean / sd, effect_lo=lo / sd)


def verdict(archetype: str, cand: dict, base: dict, specs, effect: float) -> dict:
    primary, corroborating = GATES[archetype]
    res = {m: compare(cand, base, specs, m)
           for m in (primary, *corroborating, "win", "rank", "sell_through", "sold", "deliveries",
                     *ECONOMICS.get(archetype, ()))}
    clusters = np.array([specs[g].cluster + 1_000_000 * specs[g].cell for g in cand["ok"]])
    win_lo = E.mean_ci(cand["win"], clusters)[1]

    def passes(m):
        r = res[m]
        return r["effect"] >= effect and r["lo"] > 0

    hits = [m for m in corroborating if passes(m)]
    win = res["win"]
    pathology = []
    if archetype == "producer":
        # Volume is not enough: it has to be sold, and the value has to show in the score.
        if E.mean_ci(cand["sell_through"], clusters)[2] < SELL_THROUGH_FLOOR:
            pathology.append("production without sales")
        if res["score"]["effect"] <= -effect and res["score"]["hi"] < 0:
            pathology.append("gives its value away")
    # The check is on containers sold (not bought), and only a fall of at least `effect` SD counts.
    if archetype == "showcase" and passes(primary) and res["sold"]["hi"] < 0 and res["sold"]["effect"] < -effect:
        pathology.append("variety without sales")
    if archetype == "shipper" and passes(primary) and win["hi"] < 0:
        pathology.append("deliveries that hurt the result")
    diverges = passes(primary) and bool(hits)
    return dict(archetype=archetype, effect_threshold=effect, primary=primary, primary_pass=passes(primary),
                corroborated_by=hits, diverges=diverges, pathology=pathology, economic_ok=not pathology,
                passes=diverges and not pathology,
                win_floor_ok=win["lo"] > -WIN_FLOOR, abs_win_floor_ok=bool(win_lo > ABS_WIN_FLOOR),
                win_lower_bound=win_lo, metrics=res)


def stalls(cand: E.Records, base: E.Records, specs) -> dict:
    """Game quality: the share of tables that hit the action horizon (nobody scores), candidate against Peddler v2 on the same seeds."""
    clusters = np.array([s.cluster + 1_000_000 * s.cell for s in specs])
    mean, lo, hi = E.mean_ci(cand.stalled.astype(float) - base.stalled.astype(float), clusters)
    return dict(stall_rate=float(cand.stalled.mean()), base_stall_rate=float(base.stalled.mean()), stall_diff=mean,
                stall_lo=lo, stall_hi=hi, game_ok=bool(lo <= 0))


def render(v: dict) -> str:
    lines = [f"{v['archetype']}: passes={v['passes']} (fidelity: diverges={v['diverges']}, primary {v['primary']} "
             f"pass={v['primary_pass']}, corroborated by {v['corroborated_by'] or 'nothing'}; economic_ok={v['economic_ok']}, "
             f"pathology={v['pathology'] or 'none'}); win floor ok={v['win_floor_ok']}; absolute win >{ABS_WIN_FLOOR} "
             f"ok={v['abs_win_floor_ok']} (lower bound {v['win_lower_bound']:.3f})",
             "| metric | peddler2 | candidate | diff [95% CI] | effect (SD) |", "|---|---|---|---|---|"]
    for r in v["metrics"].values():
        lines.append(f"| {r['metric']} | {r['base']:.3f} | {r['cand']:.3f} | {r['diff']:+.3f} [{r['lo']:+.3f}, {r['hi']:+.3f}] "
                     f"| {r['effect']:+.2f} |")
    if "game_ok" in v:
        lines.append(f"game quality: game_ok={v['game_ok']}; stalled tables {v['stall_rate']:.3f} against Peddler v2's "
                     f"{v['base_stall_rate']:.3f} (diff {v['stall_diff']:+.3f} [{v['stall_lo']:+.3f}, {v['stall_hi']:+.3f}])")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("archetype", choices=list(GATES))
    ap.add_argument("checkpoint")
    ap.add_argument("--rules", default=RULES, help="comma-separated rules the candidate was trained under")
    ap.add_argument("--games", type=int, default=300, help="games per seat")
    ap.add_argument("--effect", type=float, default=0.2)
    ap.add_argument("--json", help="write the verdict here")
    args = ap.parse_args()
    rules = dict(r.partition("=")[::2] for r in args.rules.split(",") if r)
    cand_rec, specs = play_focus(args.checkpoint, rules, args.games)
    base_rec, _ = play_focus(NETS["peddler2"].partition(":")[0], rules, args.games)
    v = verdict(args.archetype, game_metrics(cand_rec), game_metrics(base_rec), specs, args.effect)
    v.update(stalls(cand_rec, base_rec, specs))
    print(render(v))
    if args.json:
        with open(args.json, "w") as f:
            json.dump(v, f, indent=2)


if __name__ == "__main__":
    main()
