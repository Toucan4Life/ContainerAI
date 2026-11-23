"""Stage E: price-lot deviations of the Producer and the Showcase, against the baseline of `training.egta`.

    python -m training.stage_e run    --baseline reports/strategist/egta_h0b --out reports/strategist/stage_e
    python -m training.stage_e report --baseline reports/strategist/egta_h0b --out reports/strategist/stage_e

`run` replays, for every variant of `container_bots::archetype::VARIANTS`, each baseline game that seats the variant's archetype, with
the first such seat in seat order deviating (same game seed, same seats, same rotation), and stores the per-game outcomes of the
deviating seat next to those of the base seat of the same game (`variants.npz`). `report` pairs them: per table size, never pooled,
the difference deviating minus base with seed-clustered percentile bootstrap intervals (unadjusted 95% and Bonferroni-adjusted for every
variant and size), plus the table-level stalls and defaults and the Producer's and Shipper's dead and stuck seats. The decision rule is
pre-registered in docs/strategist_plan.md (Stage E). Development evidence on `DEV` seeds: nothing here is a confirmation, and nothing is
adopted by it.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import numpy as np

from training import egta as G
from training import evalcore as E
from training import ledger

# (archetype, factory lot, harbor lot) of the variants, in the order of `archetype::VARIANTS`; seat kind -11 - i. Index 0 and 5 are null.
VARIANTS = [(0, 0, 1), (0, 1, 1), (0, 2, 1), (0, 0, 2), (0, 1, 2), (1, 1, 1), (1, 2, 1), (1, 1, 2), (1, 2, 2)]
FIRST_KIND = -11
BASE_LOTS = {0: (0, 1), 1: (1, 1)}
DEAD = (0, 0, 2 * ledger.LOAN_PAYOFF)
STUCK_CASH = 1
SIZES = G.SIZES
DRAWS = 4000
ALPHA = 0.05 / (7 * len(SIZES))  # Bonferroni over 7 deviating variants and 3 table sizes
SEAT_METRICS = ("score", "cash_change", "win", "produced", "bought", "sold", "shipped", "unsold", "resale", "warehouses_built")
THRESHOLDS = dict(score=2.0, sold=-1.0, sell_through=-0.05, produced=-2.0, resale=-1.0, table_points=1.0)  # proposed, not approved


def is_null(i: int) -> bool:
    return (VARIANTS[i][1], VARIANTS[i][2]) == BASE_LOTS[VARIANTS[i][0]]


def name(i: int) -> str:
    who, f, h = VARIANTS[i]
    return f"{G.NAMES[who]} ({f},{h})" + (" null" if is_null(i) else "")


def policies() -> list[E.Policy]:
    return G.policies() + [E.Policy(f"variant{i}", scripted=FIRST_KIND - i) for i in range(len(VARIANTS))]


def specs_of(baseline: Path) -> list[E.GameSpec]:
    d = json.loads((baseline / "design.json").read_text())
    return E.crossplay_specs(3, d["sizes"], d["seeds"], block=E.DEV, length=G.STANDARD, seed_offset=d["seed_offset"])


def deviating(specs: list[E.GameSpec], who: int, vi: int):
    """Indices of the baseline games that seat `who`, and those games with their first such seat given to variant `vi`."""
    idx = [j for j, s in enumerate(specs) if who in s.seats]
    out = []
    for j in idx:
        s = specs[j]
        k = s.seats.index(who)
        out.append(s._replace(seats=s.seats[:k] + (3 + vi,) + s.seats[k + 1:]))
    return np.array(idx), out


# --------------------------------------------------------------------------------------- per-game outcomes


def outcomes(rec: E.Records, interaction: np.ndarray, breakdown: np.ndarray, who_of: np.ndarray, pos: np.ndarray | None = None) -> dict:
    """Per game: the table-level facts and the outcomes of the deviating seat (`pos` is its seat, or the seat of a policy >= 3)."""
    t = G.seat_table(rec, interaction, breakdown)
    n = len(rec)
    start = np.cumsum(rec.players) - rec.players
    if pos is None:
        rows = np.nonzero(t.arche >= 3)[0]
        if len(rows) != n:
            raise ValueError("every game must seat exactly one deviating seat")
        pos = rows - start[t.game[rows]]
    row = start + pos
    raw = t.raw
    seat_bd = breakdown[t.game, np.arange(len(t.game)) - start[t.game]].astype(np.int64)
    dead = (seat_bd[:, [0, 2, 3]] == DEAD).all(1)
    stuck = (seat_bd[:, 0] <= STUCK_CASH) & (seat_bd[:, 2] == DEAD[1]) & (seat_bd[:, 3] == DEAD[2])
    resale = np.minimum(raw["bought"], np.maximum(0, raw["sold"] - raw["produced"] - 1))
    resale[np.isnan(raw["sold"])] = np.nan
    who = who_of[t.arche]
    out = {"stalled": rec.stalled.copy(), "size": rec.players.astype(np.int64), "seed": t.game_seed,
           "default": np.bincount(t.game, np.nan_to_num(raw["defaulted"]), n) > 0}
    for g, label in ((0, "P"), (1, "Sc"), (2, "Sh")):
        m = who == g
        out[f"seats_{label}"] = np.bincount(t.game[m], minlength=n)
        out[f"dead_{label}"] = np.bincount(t.game[m], dead[m], n)
        out[f"stuck_{label}"] = np.bincount(t.game[m], stuck[m], n)
        out[f"resale_{label}"] = np.bincount(t.game[m], np.nan_to_num(resale[m]), n)
    seat = {"score": raw["score"], "cash_change": raw["flow_net_cash"], "win": raw["win"], "produced": raw["produced"],
            "bought": raw["bought"], "sold": raw["sold"], "shipped": raw["shipped"], "unsold": raw["unsold"], "resale": resale,
            "warehouses_built": raw["warehouses_built"]}
    for k, v in seat.items():
        out[k] = v[row]
    out["pos"] = pos
    return out


def run(baseline: Path, out: Path, log=print) -> None:
    out.mkdir(parents=True, exist_ok=True)
    rec_b, inter_b, bd_b = G.load(baseline)
    specs = specs_of(baseline)
    if len(specs) != len(rec_b):
        raise SystemExit("the baseline does not match its design")
    who_of = np.array([0, 1, 2] + [v[0] for v in VARIANTS])
    pol = policies()
    arrays: dict[str, np.ndarray] = {}
    for vi, (who, _, _) in enumerate(VARIANTS):
        idx, vspecs = deviating(specs, who, vi)
        rec_v = E.play(vspecs, pol, log=None)
        v = outcomes(rec_v, rec_v.interaction, rec_v.breakdown, who_of)
        sub = rec_b.subset(np.isin(np.arange(len(rec_b)), idx))
        b = outcomes(sub, inter_b[idx], bd_b[idx], who_of, pos=v["pos"])
        if is_null(vi):
            same = (rec_v.scores == rec_b.scores[idx]).all() and (rec_v.stalled == rec_b.stalled[idx]).all() and (
                rec_v.interaction == inter_b[idx]).all() and (rec_v.breakdown == bd_b[idx]).all()
            log(f"  null variant {vi} ({name(vi)}): {'identical to the baseline' if same else 'DIFFERS from the baseline'} in {len(idx):,} games")
            if not same:
                raise SystemExit(f"the null variant {vi} does not reproduce the baseline")
        for k, a in v.items():
            arrays[f"v{vi}_{k}"] = np.asarray(a)
        for k, a in b.items():
            arrays[f"b{vi}_{k}"] = np.asarray(a)
        arrays[f"idx{vi}"] = idx
        log(f"  variant {vi} {name(vi)}: {len(idx):,} games, {int(rec_v.stalled.sum())} stalled (baseline {int(rec_b.stalled[idx].sum())})")
    np.savez_compressed(out / "variants.npz", **arrays)


# --------------------------------------------------------------------------------------- statistics


def draw(seeds: int) -> np.ndarray:
    return np.random.default_rng(seeds).integers(0, seeds, size=(DRAWS, seeds))


def paired(v_top, v_bot, b_top, b_bot, seed, ok, seeds) -> list[float]:
    """[difference of ratios of sums v - b, lo, hi, adjusted lo, adjusted hi], resampling seeds."""
    def sums(a):
        return np.bincount(seed[ok], a[ok], seeds)
    vt, vb, bt, bb = sums(v_top), sums(v_bot), sums(b_top), sums(b_bot)
    d = draw(seeds)

    def ratio(t, b):
        s = b[d].sum(1)
        return np.divide(t[d].sum(1), s, out=np.full(len(d), np.nan), where=s > 0)

    est = vt.sum() / vb.sum() - bt.sum() / bb.sum() if vb.sum() > 0 and bb.sum() > 0 else float("nan")
    r = ratio(vt, vb) - ratio(bt, bb)
    q = lambda p: float(np.nanquantile(r, p))  # noqa: E731
    return [est, q(0.025), q(0.975), q(ALPHA / 2), q(1 - ALPHA / 2)]


def level(top, bot, seed, ok, seeds) -> float:
    a, b = np.bincount(seed[ok], top[ok], seeds).sum(), np.bincount(seed[ok], bot[ok], seeds).sum()
    return float(a / b) if b > 0 else float("nan")


def fmt(x: list[float], d: int = 2, adjusted: bool = False) -> str:
    lo, hi = (x[3], x[4]) if adjusted else (x[1], x[2])
    return f"{x[0]:+.{d}f} [{lo:+.{d}f}, {hi:+.{d}f}]"


def compare(z, vi: int, size: int) -> dict:
    """Everything the report and the verdict need for one variant and one table size."""
    k = lambda s: z[f"v{vi}_{s}"]  # noqa: E731
    c = lambda s: z[f"b{vi}_{s}"]  # noqa: E731
    sel = k("size") == size
    seed_ids = np.unique(k("seed")[sel])
    remap = {s: j for j, s in enumerate(seed_ids)}
    seed = np.array([remap.get(s, 0) for s in k("seed")])
    seeds = len(seed_ids)
    done = sel & ~k("stalled") & ~c("stalled")
    one = np.ones(len(seed))
    res = {"games": int(sel.sum()), "seeds": seeds, "completed_both": int(done.sum())}
    for m in SEAT_METRICS:
        ok = done & np.isfinite(k(m)) & np.isfinite(c(m)) if m != "win" else sel
        res[m] = paired(np.nan_to_num(k(m)), one, np.nan_to_num(c(m)), one, seed, ok, seeds)
        res[m + "_base"] = level(np.nan_to_num(c(m)), one, seed, ok, seeds)
    ok = done & (k("produced") > 0) & (c("produced") > 0)
    res["sell_through"] = paired(np.nan_to_num(k("sold")), np.nan_to_num(k("produced")), np.nan_to_num(c("sold")), np.nan_to_num(c("produced")), seed, ok, seeds)
    res["sell_through_base"] = level(np.nan_to_num(c("sold")), np.nan_to_num(c("produced")), seed, ok, seeds)
    res["stalled"] = paired(k("stalled").astype(float) * 100, one, c("stalled").astype(float) * 100, one, seed, sel, seeds)
    res["stalled_base"] = float(c("stalled")[sel].mean() * 100)
    res["default"] = paired(k("default").astype(float) * 100, one, c("default").astype(float) * 100, one, seed, done, seeds)
    res["default_base"] = float(c("default")[done].mean() * 100)
    for g in ("P", "Sh"):
        for kind in ("dead", "stuck"):
            res[f"{kind}_{g}"] = paired(k(f"{kind}_{g}") * 100.0, k(f"seats_{g}").astype(float), c(f"{kind}_{g}") * 100.0,
                                        c(f"seats_{g}").astype(float), seed, done & (k(f"seats_{g}") > 0), seeds)
            res[f"{kind}_{g}_base"] = level(c(f"{kind}_{g}") * 100.0, c(f"seats_{g}").astype(float), seed, done & (c(f"seats_{g}") > 0), seeds)
    allthree = done & (k("seats_P") + (VARIANTS[vi][0] == 0) > 0) & (k("seats_Sc") + (VARIANTS[vi][0] == 1) > 0) & (k("seats_Sh") > 0)
    for g in ("P", "Sc", "Sh"):
        res[f"rank_{g}"] = [level(k(f"resale_{g}"), k(f"seats_{g}").astype(float), seed, allthree & (k(f"seats_{g}") > 0), seeds),
                            level(c(f"resale_{g}"), c(f"seats_{g}").astype(float), seed, allthree & (c(f"seats_{g}") > 0), seeds)]
    return res


def verdict(vi: int, r: dict) -> dict:
    t = THRESHOLDS
    economics = r["score"][0] >= t["score"] and r["score"][3] > 0
    sales = r["sold"][0] >= t["sold"] and r["sell_through"][0] >= t["sell_through"]
    if VARIANTS[vi][0] == 0:
        identity = r["produced"][0] >= t["produced"]
    else:
        identity = r["resale"][0] >= t["resale"] and (r["rank_Sc"][0] > r["rank_P"][0])
    quality = r["stalled"][0] <= t["table_points"] and r["default"][0] <= t["table_points"]
    return dict(economics=economics, sales=sales, identity=identity, quality=quality, all=economics and sales and identity and quality)


# --------------------------------------------------------------------------------------- report


def stall_panel(baseline: Path) -> list[str]:
    rec, _, _ = G.load(baseline)
    lines = ["| table size | composition | games | stalled | stalled share |", "|---|---|---|---|---|"]
    for n in SIZES:
        cells = {}
        for c, ms in enumerate(E.multisets(3, n)):
            sel = rec.cell == n * 10_000 + c
            cells[ms] = (int(sel.sum()), int(rec.stalled[sel].sum()))
        tot = [sum(v[i] for v in cells.values()) for i in (0, 1)]
        lines.append(f"| {n} | all compositions | {tot[0]:,} | {tot[1]:,} | {100 * tot[1] / tot[0]:.2f}% |")
        if n == 3:
            for ms, (g, s) in cells.items():
                if s:
                    lines.append(f"| 3 | {G.label(ms)} | {g:,} | {s} | {100 * s / g:.2f}% |")
    return lines


def interact_panel(folder: Path) -> list[str]:
    """Stall rates of the `interact` runs (the other seats are drawn at random from the pool), one row per focus and table size."""
    rows = []
    for f in sorted(folder.glob("*.json")):
        d = json.loads(f.read_text())
        who = d["focus"] or d["pool"][0]
        rows.append((who, ",".join(d["pool"]) if d["focus"] else "classic only", d["players"][0], d["games"], d["horizon"], d["seed"],
                     d["bots"]["focus" if d["focus"] else who]["stall_rate"]))
    key = lambda r: (r[2], r[1] != "classic only", r[1], r[0])  # noqa: E731
    lines = ["| table size | seat of interest | other seats drawn from | games | horizon | seed | games stalled |", "|---|---|---|---|---|---|---|"]
    for who, pool, n, games, horizon, seed, stall in sorted(rows, key=key):
        lines.append(f"| {n} | {who if pool != 'classic only' else 'all seats'} | {pool if pool != 'classic only' else 'classic (all seats)'} | {games} | {horizon} | {seed} | {100 * stall:.1f}% |")
    return lines


def render(z, baseline: Path, folder: Path | None = None) -> str:
    L = ["# Stage E: price-lot deviations (development evidence)", "",
         "One seat of the archetype deviates (the first in seat order) in every baseline game that seats it; each figure is the deviating seat "
         "minus the base seat in the same game (same seed, seats and rotation), per table size, never pooled. Baseline: "
         f"`{baseline}` (the new appetite profiles). `DEV` seeds, 400 per composition; cyclic rotations of one arrangement per composition do "
         "not cover every permutation. Intervals: 95% percentile bootstrap over seeds (all games of a seed stay together), unadjusted, and "
         f"for the primary outcome (score) also Bonferroni-adjusted over 21 tests (alpha {ALPHA:.4f} per test). The grid is exploratory. "
         "Thresholds in the verdict are the proposed ones pre-registered in the plan, not approved ones.", "",
         "Caveats beside every figure: `cash change` is final cash - $10 start (`flow_net_cash`), not profit and not the score (it leaves out island "
         "value, leftover containers and loans). `unsold` is exact only because the heuristics make no Bank move and a defaulting seat is "
         "left out. `sold` includes resales, so `sold/produced` can exceed 1 (legacy diagnostic). Metrics other than win share cover games "
         "completed in both versions; dead and stuck shares cover completed games only (stalled games record no breakdown).", ""]
    summary = ["| variant | size | score Δ (adj. 95%) | meets practical threshold | sales kept | identity kept | quality kept | all |", "|---|---|---|---|---|---|---|---|"]
    body: list[str] = []
    for n in SIZES:
        body += [f"## {n} seats", ""]
        for vi in range(len(VARIANTS)):
            r = compare(z, vi, n)
            v = verdict(vi, r)
            summary.append(f"| {name(vi)} | {n} | {fmt(r['score'], adjusted=True)} | {'yes' if v['economics'] else 'no'} | {'yes' if v['sales'] else 'no'} | "
                           f"{'yes' if v['identity'] else 'no'} | {'yes' if v['quality'] else 'no'} | {'**yes**' if v['all'] and not is_null(vi) else 'no'} |")
            body += [f"### {name(vi)}: {r['games']:,} games, {r['seeds']} seeds, {r['completed_both']:,} completed in both", "",
                     "| quantity | base seat | deviating − base [95%] |", "|---|---|---|"]
            for m, d in (("score", 2), ("cash_change", 2), ("win", 3), ("sold", 2), ("sell_through", 3), ("unsold", 2), ("produced", 2), ("bought", 2),
                         ("shipped", 2), ("resale", 2), ("warehouses_built", 2)):
                body.append(f"| {m} | {r[m + '_base']:.{d}f} | {fmt(r[m], d)}" + (f"; adjusted {fmt(r[m], d, True)}" if m == "score" else "") + " |")
            body += [f"| stalled games (%) | {r['stalled_base']:.2f} | {fmt(r['stalled'])} |", f"| games with a default (%) | {r['default_base']:.2f} | {fmt(r['default'])} |"]
            for g, gn in (("P", "Producer"), ("Sh", "Shipper")):
                body += [f"| {gn} seats dead (%) | {r[f'dead_{g}_base']:.2f} | {fmt(r[f'dead_{g}'])} |", f"| {gn} seats stuck (%) | {r[f'stuck_{g}_base']:.2f} | {fmt(r[f'stuck_{g}'])} |"]
            body += ["", "Resale per seat-game in tables of all three archetypes (variant run / baseline): "
                     + ", ".join(f"{g} {r[f'rank_{g}'][0]:.1f} / {r[f'rank_{g}'][1]:.1f}" for g in ("P", "Sc", "Sh")), ""]
    L += ["## Summary against the pre-registered rule", ""] + summary + [""] + body
    L += ["## Stall baseline (two panels, never in one column)", "",
          "**Panel 1, EGTA cross-play of the three heuristics** (`" + str(baseline) + "`, no nets, every composition and rotation, 400 seeds, horizon 2,500 actions). "
          "Rates are comparable within this panel only.", ""] + stall_panel(baseline)
    if folder is not None and any(folder.glob("*.json")):
        L += ["", "**Panel 2, `interact`**: each row plays 300 games with the same seed (1), the same horizon (2,500) and the same table size; the seats not "
              "of interest are drawn at random from the pool, so these rates are not comparable with panel 1. A stalled game stalls for every seat, so the "
              "rate of the seat of interest is the game stall rate. Command: `interact [--focus H] --pool P --players N --games 300 --seed 1`.", ""]
        L += interact_panel(folder)
    return "\n".join(L)


def report(baseline: Path, out: Path) -> None:
    with np.load(out / "variants.npz") as z:
        z = {k: z[k] for k in z.files}
    (out / "report.md").write_text(render(z, baseline, out / "interact") + "\n")
    print(f"wrote {out / 'report.md'}")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=("run", "report"))
    ap.add_argument("--baseline", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()
    if a.cmd == "run":
        run(a.baseline, a.out)
    report(a.baseline, a.out)


if __name__ == "__main__":
    main()
