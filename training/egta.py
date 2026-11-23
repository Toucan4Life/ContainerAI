"""EGTA Lite for the three heuristics: a cross-play, an empirical payoff table, and the cash behind it.

    python -m training.egta run    --out reports/strategist/egta_h0 --seeds 400
    python -m training.egta report --out reports/strategist/egta_h0

`run` seats every composition of Producer, Showcase and Shipper (10, 15 and 21 multisets at 3, 4 and 5 seats), `--seeds` seeds each,
every seat rotation of every seed, on the DEV block, and keeps the per-seat counters (`counters.npz`) next to the records
(`records.npz`). `report` turns them into `payoffs.json` and `crossplay_report.md` (the standard cross-play tables), `economics.json`
and `report.md`. Nothing is trained and no heuristic is changed: this measures what the three do to one another. Peddler v2, Magnate,
Consortium and every net stay out of the population.

The reward and the cash are reported apart. The reward is the canonical winner share of `evalcore.winner_share` (a stalled game pays
nothing to anybody), with the rank share and the score margin beside it. The cash is the ledger of `training/ledger.py`; none of its sums
is a profit. Every table size is its own game and is never pooled with another (docs/equilibrium_spec.md). Intervals are 95% percentile
bootstraps over seeds, and every difference between two cells is paired on the seeds they share.

Two definitions the tables rely on:

* A seat *defaults* when it misses an interest payment. The heuristics never call the Bank, so in their games the ledger's unexplained
  residual (`bank_and_other`) is exactly the interest a seat did not pay, and it is zero for every seat that did not default.
* `unsold` is the stock a seat ends with in its factory, harbor and ship: 1 + produced + bought - sold - shipped. It is exact for a seat that
  neither takes containers from the Bank nor loses any to a seizure, so a defaulting seat has none. The nets use the Bank, which is why the
  gate (`training/gate.py`) cannot use it for them.
"""

from __future__ import annotations

import argparse
import itertools
import json
import time
from dataclasses import dataclass
from pathlib import Path

import numpy as np

from training import crossplay, ledger, psro
from training import env as V
from training import evalcore as E
from training.env import MAX_PLAYERS, PRODUCER, SHIPPER, SHOWCASE, STANDARD

ROOT = E.ROOT
NAMES = ("producer", "showcase", "shipper")
KINDS = (PRODUCER, SHOWCASE, SHIPPER)
SHORT = ("P", "Sc", "Sh")
SIZES = (3, 4, 5)
REPS = 1000  # bootstrap resamples of the seeds
EFFECT = 0.2  # SDs: from here a difference between two fields is called a response (the gate's threshold)
BANK_MOVES = ("sail_bank", "call_container_lot", "call_cash_lot", "outbid_container_lot", "outbid_cash_lot", "bid_cash", "bid_container",
              "done_bidding")
FLOWS = ledger.IN + ledger.OUT + ("net_cash", "bank_and_other")

# name: (numerator, denominator). A metric is the sum of the first over the sum of the second, so a rate is a ratio of sums and a plain
# mean (no denominator) counts every seat once. Quantities of a stalled game are undefined and left out, except `win` and `rank`,
# which pay nothing in a stalled game and are counted.
METRICS = {
    "produced": ("produced", None), "sold": ("sold", None), "sell_through": ("sold", "produced"),
    "avg_sale_price": ("earned", "sold"), "bought": ("bought", None), "avg_purchase_price": ("spent", "bought"),
    "unsold": ("unsold", None), "reprices": ("reprices", None), "deliveries": ("deliveries", None),
    "cargo_size": ("shipped", "deliveries"), "loans_taken": ("loans_taken", None), "buy_outs": ("buy_outs", None),
    "factories_built": ("factories_built", None), "warehouses_built": ("warehouses_built", None),
    "cash": ("cash", None), "score": ("score", None), "win": ("win", None), "rank": ("rank", None), "margin": ("margin", None),
    "colors_sold": ("colors_sold", None), "cheap_shelf_stock": ("offered_competitive", "main_decisions"),
    "competitive_share": ("offered_competitive", "offered"),
    "delivery_cash_minus_purchases": ("delivery_cash_minus_purchases", None),
    "defaulted": ("defaulted", None), "unpaid_interest": ("bank_and_other", None),
    **{f"flow_{k}": (f"flow_{k}", None) for k in FLOWS if k != "bank_and_other"},
}
UNDEFINED_WHEN_STALLED = {k for pair in METRICS.values() for k in pair if k} - {"win", "rank"}


def label(ms) -> str:
    return " ".join(SHORT[i] for i in ms)


def policies() -> list[E.Policy]:
    return [E.Policy(name, scripted=kind) for name, kind in zip(NAMES, KINDS)]


# --------------------------------------------------------------------------------------- run


def read_chunk(path: Path) -> tuple[E.Records, np.ndarray, np.ndarray]:
    with np.load(path, allow_pickle=False) as z:
        return E.Records(**{f: z[f] for f in E.ARRAYS}, meta=json.loads(str(z["meta"]))), z["interaction"], z["breakdown"]


BLOCKS = {"dev": E.DEV, "holdout": E.HOLDOUT}


def run(out: Path, seeds: int, sizes=SIZES, seed_offset: int = 0, chunk: int = 24_000, log=print, block: str = "dev") -> E.Records:
    """Plays the whole design in chunks (an interrupted run resumes) and writes `records.npz` and `counters.npz`."""
    out.mkdir(parents=True, exist_ok=True)
    design = dict(sizes=list(sizes), seeds=seeds, seed_offset=seed_offset, chunk=chunk, block=block)
    pinned = out / "design.json"
    if pinned.exists() and json.loads(pinned.read_text()) != design:
        raise SystemExit(f"{out} was started with another design ({pinned.read_text().strip()}); use a fresh --out")
    pinned.write_text(json.dumps(design) + "\n")
    pol = policies()
    specs = E.crossplay_specs(len(pol), sizes, seeds, block=BLOCKS[block], length=STANDARD, seed_offset=seed_offset)
    t0 = time.monotonic()
    paths = []
    for k, a in enumerate(range(0, len(specs), chunk)):
        paths.append(out / f"chunk_{k:03d}.npz")
        if paths[-1].exists():
            continue
        rec = E.play(specs[a:a + chunk], pol, log=None)
        np.savez_compressed(paths[-1], **{f: getattr(rec, f) for f in E.ARRAYS}, interaction=rec.interaction,
                            breakdown=rec.breakdown, meta=json.dumps(rec.meta))
        log(f"  {min(a + chunk, len(specs)):,}/{len(specs):,} games, {int(rec.stalled.sum())} stalled in this chunk")
    parts = [read_chunk(p) for p in paths]
    rec = E.Records.concat([p[0] for p in parts])
    rec.meta = {**rec.meta, "seconds": round(time.monotonic() - t0, 1), "players": list(sizes), "length": STANDARD, "seeds": seeds,
                "seed_offset": seed_offset, "block": BLOCKS[block]}
    rec.save(out / "records.npz")
    np.savez_compressed(out / "counters.npz", interaction=np.concatenate([p[1] for p in parts]),
                        breakdown=np.concatenate([p[2] for p in parts]))
    for p in paths:
        p.unlink()
    return rec


def load(out: Path) -> tuple[E.Records, np.ndarray, np.ndarray]:
    rec = E.Records.load(out / "records.npz")
    with np.load(out / "counters.npz", allow_pickle=False) as z:
        return rec, z["interaction"], z["breakdown"]


# --------------------------------------------------------------------------------------- seats


@dataclass
class Seats:
    """One row per seat of every game. `raw` holds what happened to the seat as floats (NaN where it is undefined)."""

    size: np.ndarray  # table size of the row's game
    arche: np.ndarray  # policy index: 0 producer, 1 showcase, 2 shipper
    game: np.ndarray
    seed: np.ndarray  # index of the game's seed among those of its table size
    game_seed: np.ndarray  # the same, one per game
    seeds: dict  # table size -> number of seeds
    raw: dict
    by: dict  # (cell, archetype) -> rows


def seat_table(rec: E.Records, interaction: np.ndarray, breakdown: np.ndarray) -> Seats:
    names = V.move_names()
    used = np.arange(MAX_PLAYERS)[None, :] < rec.players[:, None]
    g, s = np.nonzero(used)
    x = interaction[g, s].astype(np.int64)
    bd = breakdown[g, s].astype(np.int64)
    done = ~rec.stalled[g]
    flow = ledger.flows(interaction, breakdown, rec.players, names)

    def move(name: str) -> np.ndarray:
        return x[:, V.MOVES + names.index(name)]

    score = bd[:, :3].sum(1) - bd[:, 3]
    if not np.array_equal(score[done], rec.scores[g, s][done]):
        raise ValueError("the breakdown (cash + island + leftovers - loans) does not add up to the engine's scores")
    arche = rec.seats[g, s].astype(np.int64)
    raw = {
        "one": np.ones(len(g)), "produced": x[:, V.PRODUCED], "sold": x[:, V.SOLD], "earned": x[:, V.EARNED], "bought": x[:, V.BOUGHT],
        "spent": x[:, V.SPENT], "shipped": x[:, V.SHIPPED], "deliveries": x[:, V.DELIVERIES], "buy_outs": move("buy_out"),
        "reprices": move("reprice_factory") + move("reprice_harbor"), "loans_taken": move("request_loan"),
        "factories_built": move("build_factory"), "warehouses_built": move("build_warehouse"), "cash": bd[:, 0], "score": score,
        "colors_sold": (x[:, V.SOLD_COLOR:V.SOLD_COLOR + 5] > 0).sum(1), "offered": x[:, V.OFFERED],
        "offered_competitive": x[:, V.OFFERED_COMPETITIVE], "main_decisions": x[:, V.MAIN_DECISIONS],
        "win": E.winner_share(rec)[g, s], "rank": E.rank_share(rec)[g, s], "margin": E.score_margin(rec)[g, s],
        **{f"flow_{k}": flow[k][g, s] for k in FLOWS},
    }
    raw = {k: np.asarray(v, dtype=np.float64) for k, v in raw.items()}
    raw["bank_and_other"] = raw.pop("flow_bank_and_other")
    raw["defaulted"] = (raw["bank_and_other"] > 0).astype(np.float64)
    raw["delivery_cash_minus_purchases"] = raw["flow_delivery_received"] - raw["spent"]
    raw["unsold"] = np.where(raw["defaulted"] > 0, np.nan, 1 + raw["produced"] + raw["bought"] - raw["sold"] - x[:, V.SHIPPED])
    for k in UNDEFINED_WHEN_STALLED:
        raw[k][~done] = np.nan
    game_seed, seeds = np.zeros(len(rec), dtype=np.int64), {}
    for n in np.unique(rec.players):
        rows = np.nonzero(rec.players == n)[0]
        values, inverse = np.unique(rec.cluster[rows], return_inverse=True)
        game_seed[rows], seeds[int(n)] = inverse.reshape(-1), len(values)
    key = rec.cell[g].astype(np.int64) * 8 + arche
    order = np.argsort(key, kind="stable")
    keys, starts = np.unique(key[order], return_index=True)
    by = {(int(k) // 8, int(k) % 8): order[a:b] for k, a, b in zip(keys, starts, list(starts[1:]) + [len(order)])}
    return Seats(rec.players[g].astype(np.int64), arche, g, game_seed[g], game_seed, seeds, raw, by)


# --------------------------------------------------------------------------------------- statistics

_DRAWS: dict[int, np.ndarray] = {}


def draws(seeds: int) -> np.ndarray:
    """The same resamples of the seeds for every statistic of a table size, so that differences are paired."""
    if seeds not in _DRAWS:
        _DRAWS[seeds] = np.random.default_rng(seeds).integers(0, seeds, size=(REPS, seeds))
    return _DRAWS[seeds]


def seed_sums(t: Seats, rows: np.ndarray, metric: str) -> tuple[np.ndarray, np.ndarray]:
    """Per-seed sums of the numerator and the denominator of `metric` over `rows`; rows where either is undefined are left out."""
    top, bottom = METRICS[metric]
    a = t.raw[top][rows]
    b = t.raw[bottom][rows] if bottom else t.raw["one"][rows]
    ok = ~(np.isnan(a) | np.isnan(b))
    seeds = t.seeds[int(t.size[rows[0]])]
    seed = t.seed[rows][ok]
    return np.bincount(seed, a[ok], seeds), np.bincount(seed, b[ok], seeds)


def point(sums: tuple[np.ndarray, np.ndarray]) -> float:
    return float(sums[0].sum() / sums[1].sum()) if sums[1].sum() > 0 else float("nan")


def resampled(sums: tuple[np.ndarray, np.ndarray]) -> np.ndarray:
    d = draws(len(sums[0]))
    top, bottom = sums[0][d].sum(1), sums[1][d].sum(1)
    return np.divide(top, bottom, out=np.full(len(top), np.nan), where=bottom > 0)


def interval(est: float, r: np.ndarray) -> list[float]:
    if np.isnan(r).all():
        return [est, float("nan"), float("nan")]
    return [est, float(np.nanquantile(r, 0.025)), float(np.nanquantile(r, 0.975))]


def stat(t: Seats, rows: np.ndarray, metric: str) -> list[float]:
    """[value, lo, hi] of a metric over `rows`."""
    if not len(rows):
        return [float("nan")] * 3
    s = seed_sums(t, rows, metric)
    return interval(point(s), resampled(s))


def contrast(a: tuple[np.ndarray, np.ndarray], b: tuple[np.ndarray, np.ndarray]) -> list[float]:
    """[a - b, lo, hi], paired on the seeds."""
    return interval(point(a) - point(b), resampled(a) - resampled(b))


def spread(t: Seats, groups: list[np.ndarray], metric: str) -> float:
    """The within-group SD of the metric per seat and game (the scale an effect is measured in), pooled over `groups`."""
    top, bottom = METRICS[metric]
    var = []
    for rows in groups:
        a = t.raw[top][rows]
        b = t.raw[bottom][rows] if bottom else np.ones(len(rows))
        with np.errstate(divide="ignore", invalid="ignore"):
            v = a / b
        v = v[np.isfinite(v)]
        if len(v) > 1:
            var.append(v.var())
    return float(np.sqrt(np.mean(var))) if var else 0.0


# --------------------------------------------------------------------------------------- analysis


def cell_of(size: int, ms) -> int:
    return size * 10_000 + E.multisets(len(NAMES), size).index(tuple(sorted(ms)))


def members(rec: E.Records) -> dict:
    """cell id -> (table size, composition) for the table sizes the records hold."""
    return E.cell_members(len(NAMES), [int(n) for n in np.unique(rec.players)])


def field_rows(t: Seats, size: int, focal: int, field: int) -> np.ndarray:
    """The seats of `focal` at the tables of one `focal` and `size - 1` of `field` (all the seats of a mirror table)."""
    return t.by.get((cell_of(size, (focal,) + (field,) * (size - 1)), focal), np.zeros(0, dtype=np.int64))


def mixed_rows(t: Seats, size: int, archetype: int) -> np.ndarray:
    """The seats of `archetype` at every table of this size that seats at least two archetypes."""
    parts = [t.by[(cell, archetype)] for cell, (n, ms) in E.cell_members(len(NAMES), [size]).items()
             if len(set(ms)) > 1 and (cell, archetype) in t.by]
    return np.concatenate(parts) if parts else np.zeros(0, dtype=np.int64)


def per_game_mean(t: Seats, rec: E.Records, metric: str) -> np.ndarray:
    """[games, 3]: the mean of `metric` over the seats of each archetype in each game (NaN if it has none)."""
    top, _ = METRICS[metric]
    v = t.raw[top]
    ok = ~np.isnan(v)
    total, count = np.zeros((len(rec), 3)), np.zeros((len(rec), 3))
    np.add.at(total, (t.game[ok], t.arche[ok]), v[ok])
    np.add.at(count, (t.game[ok], t.arche[ok]), 1.0)
    return np.divide(total, count, out=np.full(total.shape, np.nan), where=count > 0)


def producer_leads(t: Seats, rec: E.Records) -> list[dict]:
    """In every table that seats the Producer next to another archetype: how much more (or less) it produces than each of them,
    within the same games, with the seed-paired interval. The identity the Producer gate asks of the nets, asked of the heuristic."""
    mean = per_game_mean(t, rec, "produced")
    out = []
    for cell, (n, ms) in members(rec).items():
        kinds = sorted(set(ms))
        if 0 not in kinds or len(kinds) < 2:
            continue
        g = np.nonzero((rec.cell == cell) & ~rec.stalled)[0]
        if not len(g):
            continue
        row = dict(players=n, composition=label(ms), games=len(g), produced={NAMES[k]: float(np.nanmean(mean[g, k])) for k in kinds})
        for k in kinds[1:]:
            d = mean[g, 0] - mean[g, k]
            seeds = t.seeds[n]
            s = (np.bincount(t.game_seed[g], d, seeds), np.bincount(t.game_seed[g], np.ones(len(g)), seeds))
            row[f"minus_{NAMES[k]}"] = interval(point(s), resampled(s))
        out.append(row)
    return out


def responses(t: Seats, metrics: tuple[str, ...]) -> list[dict]:
    """Does a focal archetype behave differently when the other seats are of another kind? For each focal archetype, table size and
    metric, the value against each homogeneous field and the largest paired contrast between two fields, in SDs of the seat's per-game
    value. The heuristics keep no model of their opponents: a difference is their fixed rules meeting a different market."""
    out = []
    for n, i in itertools.product(sorted(t.seeds), range(len(NAMES))):
        rows = {j: field_rows(t, n, i, j) for j in range(len(NAMES))}
        for m in metrics:
            sums = {j: seed_sums(t, r, m) for j, r in rows.items()}
            best = None
            for j, k in itertools.combinations(range(len(NAMES)), 2):
                d, lo, hi = contrast(sums[j], sums[k])
                sd = spread(t, [rows[j], rows[k]], m) or 1.0
                if best is None or abs(d / sd) > abs(best["effect"]):
                    best = dict(pair=[NAMES[j], NAMES[k]], diff=d, lo=lo, hi=hi, effect=d / sd)
            best["responds"] = bool(abs(best["effect"]) >= EFFECT and (best["lo"] > 0 or best["hi"] < 0))
            out.append(dict(players=n, focal=NAMES[i], metric=m, kind="decision" if m in DECISIONS else "outcome",
                            values={NAMES[j]: point(sums[j]) for j in rows}, **best))
    return out


def quality(t: Seats, rec: E.Records) -> dict:
    """Per composition: games, stalled games, completed games in which some seat defaulted, and the mean length of the completed ones."""
    defaulted = np.bincount(t.game, np.nan_to_num(t.raw["defaulted"]), len(rec)) > 0
    out = {}
    for cell, (n, ms) in members(rec).items():
        g = np.nonzero(rec.cell == cell)[0]
        if not len(g):
            continue
        done = g[~rec.stalled[g]]
        out[(n, ms)] = dict(games=len(g), stalled=int(rec.stalled[g].sum()), completed=len(done), defaults=int(defaulted[done].sum()),
                            actions=float(rec.actions[done].mean()) if len(done) else float("nan"))
    return out


def ledger_check(t: Seats, rec: E.Records, interaction: np.ndarray) -> dict:
    """What the ledger can and cannot see in these games: no Bank move at all, a residual that is never negative, and a residual that
    is positive where interest went unpaid: where the engine had the Bank seize something, and rarely where a seat had nothing to seize."""
    names = V.move_names()
    x = interaction.astype(np.int64)
    seize = x[:, :, [V.MOVES + names.index("seize"), V.MOVES + names.index("deal")]].sum((1, 2))
    residual = t.raw["bank_and_other"]
    ok = ~np.isnan(residual)
    by_ledger = np.bincount(t.game[ok], (residual[ok] > 0).astype(float), len(rec)) > 0
    done = ~rec.stalled
    # without the Bank every container a seat buys is one another seat sold, at the same price
    open_market = (x[:, :, V.BOUGHT].sum(1) != x[:, :, V.SOLD].sum(1)) | (x[:, :, V.SPENT].sum(1) != x[:, :, V.EARNED].sum(1))
    return dict(
        bank_moves={m: int(x[:, :, V.MOVES + names.index(m)].sum()) for m in BANK_MOVES}, games_with_open_market=int(open_market.sum()),
        seat_games=int(ok.sum()), residual_negative=int((residual[ok] < 0).sum()), residual_positive=int((residual[ok] > 0).sum()),
        games_defaulted_by_ledger=int((by_ledger & done).sum()), games_with_seize_or_deal=int(((seize > 0) & done).sum()),
        games_both=int((by_ledger & (seize > 0) & done).sum()),
    )


def analyse(rec: E.Records, interaction: np.ndarray, breakdown: np.ndarray) -> dict:
    """Every number of the report, as plain Python and NumPy values."""
    t = seat_table(rec, interaction, breakdown)
    cells = E.payoff_cells(rec, len(NAMES))
    sizes, by_cell = sorted(t.seeds), members(rec)
    out = dict(sizes=sizes, accounting=E.accounting(rec), cells=[], field={}, mixed={}, groups={})
    for (n, ms), c in sorted(cells.items()):
        out["cells"].append(dict(players=n, composition=label(ms), multiset=list(ms), games=c["games"], stalled=c["stalled"],
                                 seeds=len(c["seeds"]),
                                 payoff={NAMES[p]: E.mean_ci(c["per_seed"][:, k]) for k, p in enumerate(c["distinct"])}))
    for n in sizes:
        out["field"][str(n)] = {NAMES[i]: {NAMES[j]: {m: stat(t, field_rows(t, n, i, j), m) for m in METRICS} for j in range(len(NAMES))}
                                for i in range(len(NAMES))}
        out["mixed"][str(n)] = {NAMES[i]: {m: stat(t, mixed_rows(t, n, i), m) for m in METRICS} for i in range(len(NAMES))}
    for (cell, a), rows in t.by.items():
        n, ms = by_cell[cell]
        out["groups"][f"{n}|{label(ms)}|{NAMES[a]}"] = {m: stat(t, rows, m) for m in METRICS}
    out["producer_leads"] = producer_leads(t, rec)
    out["responses"] = responses(t, RESPONSE_METRICS)
    out["quality"] = {f"{n}|{label(ms)}": q for (n, ms), q in quality(t, rec).items()}
    out["ledger"] = ledger_check(t, rec, interaction)
    return out


# What a seat chooses to do, and what the market and the scoring give back for it.
DECISIONS = ("produced", "bought", "reprices", "deliveries", "cargo_size", "loans_taken", "buy_outs", "factories_built",
             "warehouses_built")
OUTCOMES = ("sold", "avg_sale_price", "avg_purchase_price", "cash", "score")
RESPONSE_METRICS = DECISIONS + OUTCOMES


# --------------------------------------------------------------------------------------- report


def table(head: list[str], rows: list[list[str]]) -> list[str]:
    return ["| " + " | ".join(head) + " |", "|" + "---|" * len(head)] + ["| " + " | ".join(r) + " |" for r in rows] + [""]


def pm(v, digits: int = 3) -> str:
    return "—" if np.isnan(v[0]) else f"{v[0]:.{digits}f} ±{(v[2] - v[1]) / 2:.{digits}f}"


def num(v, digits: int = 2) -> str:
    return "—" if np.isnan(v[0]) else f"{v[0]:.{digits}f}"


def pct(v) -> str:
    return "—" if np.isnan(v[0]) else f"{100 * v[0]:.1f}%"


def field_table(block: dict, columns: list[tuple[str, str, object]]) -> list[str]:
    """One row per (focal archetype, field): a seat of the focal archetype at a table whose other seats are all of the field."""
    rows = []
    for i in NAMES:
        for j in NAMES:
            cells = block[i][j]
            rows.append([f"**{i}**" if i == j else i, f"{j} (mirror)" if i == j else j] + [fmt(cells[m]) for _, m, fmt in columns])
    return table(["focal seat", "other seats"] + [h for h, _, _ in columns], rows)


def render(rec: E.Records, a: dict, meta_game: list[str]) -> str:
    acc, meta, sizes = a["accounting"], rec.meta, a["sizes"]
    seeds = meta["seeds"]
    counts = ", ".join(f"{sum(c['players'] == n for c in a['cells'])} at {n} seats" for n in sizes)
    lines = [
        "# EGTA Lite, Stage D: cross-play of Producer, Showcase and Shipper", "",
        f"Population: producer, showcase, shipper (scripted, the hand-set profiles of `crates/container-bots/src/archetype.rs`; no Bank play, "
        f"no nets). Peddler v2, Magnate and Consortium are not in it. Design: every composition of the three archetypes ({counts}), "
        f"{seeds} seeds each (seed block `{'DEV' if meta['block'] == E.DEV else 'HOLDOUT'}`, offset {meta['seed_offset']}), every composition seated in all its rotations on every "
        f"seed, `Standard` length, horizon {acc['horizon']} actions, commit `{meta['git']}`. No training, no reward, no change to a "
        f"heuristic.", "",
        f"**{acc['games']:,} games: {acc['completed']:,} completed, {acc['stalled']:,} stalled ({100 * acc['stall_rate']:.3f}%).** "
        f"Utility: winner share, a stalled game pays 0 to every seat. Every table size is its own game and is never pooled with another. "
        f"Intervals are 95% percentile bootstraps over seeds ({REPS:,} resamples), written `±` for half the interval; differences are paired "
        f"on the seeds the two cells share. Tables are for one seat of the archetype named.", ""]
    for n in sizes:
        lines += [f"## {n} seats: the empirical payoff table", "",
                  f"Winner share of one seat of each archetype in every composition (fair share {1 / n:.3f}). `games` are the n rotations of "
                  f"{seeds} seeds; `defaults` are completed games in which some seat missed an interest payment.", ""]
        q = a["quality"]
        rows = []
        for c in (c for c in a["cells"] if c["players"] == n):
            k = q[f"{n}|{c['composition']}"]
            rows.append([c["composition"], f"{c['games']:,}", f"{k['stalled']}", f"{100 * k['defaults'] / max(k['completed'], 1):.1f}%",
                         f"{k['actions']:.0f}"] + [pm(c["payoff"][nm]) if nm in c["payoff"] else "—" for nm in NAMES])
        lines += table(["composition", "games", "stalled", "defaults", "actions", *NAMES], rows)
        lines += ["Final ranking in the same tables: rank share (the share of the other seats a seat out-scores, ties half, 0.5 is par; a "
                  "stalled game counts 0) ± half-interval, then the mean score margin (own total minus the mean of the others'; completed "
                  "games).", ""]
        rows = []
        for c in (c for c in a["cells"] if c["players"] == n):
            g = {nm: a["groups"][f"{n}|{c['composition']}|{nm}"] for nm in c["payoff"]}
            rows.append([c["composition"]] + [f"{pm(g[nm]['rank'])} · {num(g[nm]['margin'], 1)}" if nm in g else "—" for nm in NAMES])
        lines += table(["composition", *NAMES], rows)
    lines += ["## Identity: does the Producer produce more than the others?", "",
              "In every table that seats the Producer next to another archetype, the containers a Producer seat produces per game and the "
              "paired difference to the other archetype's seats of the same games (completed games; a positive difference means the "
              "Producer produces more).", ""]
    rows = []
    for r in a["producer_leads"]:
        rows.append([str(r["players"]), r["composition"]] + [f"{r['produced'][nm]:.2f}" if nm in r["produced"] else "—" for nm in NAMES]
                    + [pm(r[f"minus_{nm}"], 2) if f"minus_{nm}" in r else "—" for nm in NAMES[1:]])
    lines += table(["seats", "composition", *(f"{nm} produced" for nm in NAMES), "producer − showcase", "producer − shipper"], rows)
    wins = {}
    for r in a["producer_leads"]:
        for nm in NAMES[1:]:
            if f"minus_{nm}" in r:
                w = wins.setdefault((r["players"], nm), [0, 0])
                w[0] += r[f"minus_{nm}"][1] > 0
                w[1] += 1
    lines += ["Tables in which the lower bound of the difference is above zero: " +
              "; ".join(f"{n} seats vs {nm}: {w[0]} of {w[1]}" for (n, nm), w in sorted(wins.items())) + ".", ""]
    lines += ["### Each archetype's own metric in the mixed tables", "",
              "Seats of each archetype pooled over the tables of one size that seat at least two archetypes (mirrors left out; "
              "compositions weighted by their seats).", ""]
    for n in sizes:
        mixed = a["mixed"][str(n)]
        rows = [[h] + [pm(mixed[nm][m], d) for nm in NAMES] for h, m, d in IDENTITY]
        lines += [f"{n} seats:", ""] + table(["metric", *NAMES], rows)
    lines += ["## Economic outcomes of one archetype against each field", "",
              "A seat of the focal archetype at a table whose other seats are all the second archetype (the mirror row is the all-same "
              "table). Counts are per game, completed games only; average prices are total income over total containers; "
              "`unsold` is the exact stock at the end of seats that did not default.", ""]
    market = [("won", "win", lambda v: pm(v)), ("rank share", "rank", lambda v: num(v, 3)), ("score margin", "margin", lambda v: num(v, 1)),
              ("score", "score", num), ("final cash", "cash", num)]
    play = [("produced", "produced", num), ("sold", "sold", num), ("sold/produced", "sell_through", num),
            ("avg sale $", "avg_sale_price", num), ("bought", "bought", num), ("avg purchase $", "avg_purchase_price", num),
            ("unsold", "unsold", num), ("reprices", "reprices", num)]
    ship = [("deliveries", "deliveries", num), ("per delivery", "cargo_size", num), ("loans taken", "loans_taken", num),
            ("buy-outs", "buy_outs", num), ("factories", "factories_built", num), ("warehouses", "warehouses_built", num),
            ("defaulted", "defaulted", pct)]
    for n in sizes:
        lines += [f"### {n} seats", "", "Result:", ""] + field_table(a["field"][str(n)], market)
        lines += ["Market:", ""] + field_table(a["field"][str(n)], play) + ["Delivery and finance:", ""] + field_table(a["field"][str(n)], ship)
    lines += ["## Cash ledger flows, apart from the reward", "",
              "Dollars per game and seat from the counters of the engine (`training/ledger.py`). In: containers sold, delivery proceeds (twice "
              "the winning bid), wages received, loan principal not repaid. Out: cargo auctions won, containers bought, buy-outs, factories "
              "and warehouses built, wages paid, interest due. `net cash change` is final cash minus the $20 start. None of these sums is a "
              "profit: the reward is the score, which also counts the Island, leftovers and loans, and `delivery_cash_minus_purchases` "
              "ignores most of this table. `unpaid` is the interest due that a defaulting seat did not pay (what the ledger cannot "
              "explain).", ""]
    flows = [("sales", "flow_containers_sold"), ("delivery", "flow_delivery_received"), ("wages in", "flow_wages_received"),
             ("loans", "flow_loans"), ("auctions", "flow_delivery_paid"), ("purchases", "flow_containers_bought"),
             ("buy-outs", "flow_buyouts"), ("builds", "flow_builds"), ("wages out", "flow_wages_paid"), ("interest", "flow_interest"),
             ("net cash change", "flow_net_cash"), ("unpaid", "unpaid_interest")]
    for n in sizes:
        lines += [f"### {n} seats", ""] + field_table(a["field"][str(n)], [(h, m, num) for h, m in flows])
    lines += ["## Strategic interaction: does behaviour depend on who else is at the table?", "",
              f"For each archetype, table size and metric, the largest paired contrast between two of the three fields (all other seats "
              f"Producers, Showcases or Shippers), in SDs of the seat's per-game value. A response is a contrast of at least {EFFECT} SD "
              f"whose interval excludes zero. *Decisions* are what the seat chooses to do (produce, buy, reprice, deliver, borrow, buy out, "
              f"build); *outcomes* are what the market and the scoring give back (containers sold, prices, cash, score). The heuristics "
              f"model no opponent: each follows fixed rules on what it sees, so a response is a state-dependent reaction to the market "
              f"the others create, not opponent modelling. With this many comparisons some flagged contrasts are chance; the effect "
              f"threshold is what keeps the list short.", ""]
    for n in sizes:
        rs = [r for r in a["responses"] if r["players"] == n]
        for kind, cap in (("decision", 10), ("outcome", 6)):
            of = [r for r in rs if r["kind"] == kind]
            hit = sorted((r for r in of if r["responds"]), key=lambda r: -abs(r["effect"]))
            lines += [f"### {n} seats, {kind}s: {len(hit)} of {len(of)} (archetype, metric) pairs respond", ""]
            rows = [[r["focal"], r["metric"]] + [f"{r['values'][nm]:.2f}" for nm in NAMES] +
                    [f"{r['pair'][0]} vs {r['pair'][1]}", f"{r['diff']:+.2f} [{r['lo']:+.2f}, {r['hi']:+.2f}]", f"{r['effect']:+.2f}"]
                    for r in hit[:cap]]
            lines += table(["focal", "metric", *(f"vs {nm}s" for nm in NAMES), "largest contrast", "difference [95%]", "SD"], rows)
            if len(hit) > cap:
                lines += [f"({len(hit) - cap} smaller responses are in `economics.json`.)", ""]
    q, led = a["quality"], a["ledger"]
    lines += ["## Game quality", "",
              "Stalled games reach the action horizon with nobody scoring; a default is a seat that misses an interest payment. The "
              "per-composition counts are in the payoff tables above.", ""]
    rows = []
    for n in sizes:
        ks = [k for key, k in q.items() if key.startswith(f"{n}|")]
        games, done = sum(k["games"] for k in ks), sum(k["completed"] for k in ks)
        worst = max((key for key in q if key.startswith(f"{n}|")), key=lambda key: q[key]["defaults"] / max(q[key]["completed"], 1))
        rows.append([str(n), f"{games:,}", str(sum(k["stalled"] for k in ks)), f"{100 * sum(k['defaults'] for k in ks) / max(done, 1):.1f}%",
                     f"{worst.split('|')[1]} ({100 * q[worst]['defaults'] / max(q[worst]['completed'], 1):.1f}%)",
                     f"{np.average([k['actions'] for k in ks if k['completed']], weights=[k['completed'] for k in ks if k['completed']]):.0f}"])
    lines += table(["seats", "games", "stalled", "games with a default", "most default-prone composition", "mean actions"], rows)
    lines += [f"Ledger check: Bank moves of any kind in these games: {sum(led['bank_moves'].values())} "
              f"({', '.join(f'{m} {c}' for m, c in led['bank_moves'].items())}). The ledger's unexplained residual is negative in "
              f"{led['residual_negative']} of {led['seat_games']:,} seat-games and positive in {led['residual_positive']}; it marks a default "
              f"in {led['games_defaulted_by_ledger']} games, and {led['games_with_seize_or_deal']} games show a Bank seizure or deal "
              f"({led['games_both']} in both). The two detectors agree when these are equal. Every container bought was sold by another "
              f"seat at the same price in all but {led['games_with_open_market']} games.", ""]
    lines += ["## The empirical game of the three heuristics", ""] + meta_game
    lines += ["## Limits of what this shows", "",
              "* The three heuristics are the whole population and each is one hand-set profile. A solution of this game is an equilibrium of "
              "the *empirical game*, relative to this tested population, these table sizes and no best-response search beyond the three "
              "strategies; it says nothing about deviations outside them and is not an equilibrium of ContainerAI.",
              "* Their games are Bank-free by construction; the Bank-open and Bank-closed comparison is a later stage.",
              ("* The seed block is `HOLDOUT`: a final-claim block; log every look at it." if meta["block"] == E.HOLDOUT else
               "* The seed block is `DEV`: these are development numbers, not a claim. Any decision made from them is confirmed on seeds "
               "that were not used to make it."),
              "* The heuristics lose to the nets (Peddler v2, Magnate, Consortium) in earlier evaluation; this report does not test that "
              "and does not try to close it.", ""]
    return "\n".join(lines)


# Rows of the identity table: (label, metric, digits)
IDENTITY = [("produced", "produced", 2), ("sold", "sold", 2), ("sold/produced", "sell_through", 2), ("avg sale $", "avg_sale_price", 2),
            ("unsold at the end", "unsold", 2), ("cheap shelf stock (own lots at $3 or less, per decision)", "cheap_shelf_stock", 2),
            ("colors sold", "colors_sold", 2), ("reprices", "reprices", 1), ("deliveries", "deliveries", 2),
            ("containers per delivery", "cargo_size", 2), ("delivery cash minus purchases", "delivery_cash_minus_purchases", 1),
            ("win share", "win", 3), ("score", "score", 1)]


def meta_game_lines(payoffs: dict, out: Path) -> list[str]:
    store = psro.CellStore()
    store.add(payoffs)
    meta = psro.solve_meta(store, list(NAMES), payoffs["players"], boot_reps=200)
    (out / "meta_game.json").write_text(json.dumps(meta, indent=1) + "\n")
    return ["Each table size is solved on its own, with the solvers of `training/metagame.py` on the 3-strategy symmetric game (the payoff "
            "of a seat against opponents drawn iid from the mixture). The chosen output is verified by the independent checker, and the "
            "bootstrap column re-measures the gain of that fixed mixture on resampled seeds: a gain below it cannot be told from sampling "
            "error. What a row shows is an empirically estimated approximate equilibrium relative to the tested policy population (the "
            "three heuristics), these table sizes and a best-response search budget of nothing beyond the three strategies. Several "
            "solutions can exist, and the others are listed below.", ""] + psro.render_meta(meta, list(NAMES))


def report(out: Path) -> None:
    rec, interaction, breakdown = load(out)
    analysis = analyse(rec, interaction, breakdown)
    payoffs = crossplay.build_payoffs(rec, list(NAMES))
    (out / "payoffs.json").write_text(json.dumps(payoffs, indent=1) + "\n")
    (out / "crossplay_report.md").write_text(crossplay.render_report(payoffs, rec, "Cross-play of the three heuristics (standard tables)"))
    (out / "economics.json").write_text(json.dumps(analysis, indent=1, default=float) + "\n")
    (out / "report.md").write_text(render(rec, analysis, meta_game_lines(payoffs, out)))
    print(f"wrote {out / 'report.md'}, payoffs.json, economics.json, meta_game.json and crossplay_report.md")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("cmd", choices=["run", "report"])
    ap.add_argument("--out", default="reports/strategist/egta_h0")
    ap.add_argument("--seeds", type=int, default=400)
    ap.add_argument("--seed-offset", type=int, default=0)
    ap.add_argument("--block", choices=sorted(BLOCKS), default="dev", help="seed block; holdout is for final claims only")
    ap.add_argument("--players", default="3,4,5")
    args = ap.parse_args()
    out = Path(args.out)
    out = out if out.is_absolute() else ROOT / out
    if args.cmd == "run":
        rec = run(out, args.seeds, [int(p) for p in args.players.split(",")], args.seed_offset, block=args.block)
        print(f"{len(rec):,} games in {rec.meta['seconds']}s; {E.accounting(rec)}")
    report(out)


if __name__ == "__main__":
    main()
