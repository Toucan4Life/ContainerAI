"""Cross-play of a population: every composition of strategies at every table size, with full accounting.

    python -m training.crossplay run    --out reports/crossplay_deployed --seeds 20
    python -m training.crossplay report --out reports/crossplay_deployed

`run` plays each multiset of strategies `--seeds` times per rotation and keeps every game (`records.npz`);
`report` turns the records into `payoffs.json` (machine-readable) and `report.md`. The utility is the
canonical winner share of `evalcore.winner_share`: a stalled game pays nothing to anybody, and every
number below says how many games stalled. Intervals are 95% seed-clustered percentile bootstraps.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import time
from pathlib import Path

import numpy as np

from training import evalcore as E
from training.env import LENGTH_NAMES, MAX_PLAYERS, STANDARD
from training.population import Population

ROOT = E.ROOT
BLOCKS = {"dev": E.DEV, "holdout": E.HOLDOUT, "audit": E.AUDIT}


def run_crossplay(policies: list[E.Policy], players: list[int], length: int, seeds: int, block: int, out: Path, *,
                  seed_offset: int = 0, envs: int = 256, chunk: int = 24_000, torch_seed: int = 0, log=print,
                  include: list[int] | None = None, horizon: int = E.HORIZON) -> E.Records:
    """Plays the whole design in chunks, so an interrupted run resumes where it stopped.

    `include` restricts the design to the compositions that seat one of those policies (by index)."""
    out.mkdir(parents=True, exist_ok=True)
    specs = E.crossplay_specs(len(policies), players, seeds, block=block, length=length, seed_offset=seed_offset,
                              include=include)
    parts = []
    t0 = time.monotonic()
    for k, a in enumerate(range(0, len(specs), chunk)):
        path = out / f"chunk_{k:03d}.npz"
        if path.exists():
            parts.append(E.Records.load(path))
            continue
        rec = E.play(specs[a:a + chunk], policies, envs=envs, seed=torch_seed + k, log=None, horizon=horizon)
        rec.save(path)
        parts.append(rec)
        done = min(a + chunk, len(specs))
        log(f"  {done}/{len(specs)} games ({done / (time.monotonic() - t0):.0f}/s overall), "
            f"{int(rec.stalled.sum())} stalled in this chunk")
    rec = E.Records.concat(parts)
    rec.meta = {**parts[0].meta, "seconds": round(time.monotonic() - t0, 1), "players": players, "length": length,
                "seeds": seeds, "seed_offset": seed_offset, "block": block}
    rec.save(out / "records.npz")
    for k in range(len(parts)):
        (out / f"chunk_{k:03d}.npz").unlink(missing_ok=True)
    return rec


# --------------------------------------------------------------------------------------- analysis


def field_matrix(cells: dict, m: int, n: int) -> dict:
    """A[i][j] = payoff of a seat of strategy i at a table of one i and n-1 j (for i == j, the all-i table).

    Each entry carries its mean, a seed-clustered 95% interval and the per-seed values (clusters align
    across cells because every composition is played on the same seeds), so that differences can be paired.
    """
    mean, lo, hi = (np.full((m, m), np.nan) for _ in range(3))
    per_seed: dict = {}
    for i in range(m):
        for j in range(m):
            ms = tuple(sorted((i,) + (j,) * (n - 1)))
            cell = cells.get((n, ms))
            if cell is None:
                continue
            k = cell["distinct"].index(i)
            vals = cell["per_seed"][:, k]
            per_seed[(i, j)] = vals
            mean[i, j], lo[i, j], hi[i, j] = E.mean_ci(vals)
    return {"mean": mean, "lo": lo, "hi": hi, "per_seed": per_seed}


def deviation_gains(fm: dict, m: int, n: int) -> list[dict]:
    """For every field j (everyone else plays j), the best single deviation within the population.

    gain(j) = max over k of A[k][j] - A[j][j]: how much a seat gains by leaving a table of j's for k. The
    interval is a paired bootstrap on the best k *chosen on these seeds*, so it is optimistic: confirm any
    claim on the holdout block.
    """
    out = []
    for j in range(m):
        base = fm["per_seed"].get((j, j))
        if base is None:
            continue
        best = None
        for k in range(m):
            if k == j or (k, j) not in fm["per_seed"]:
                continue
            diff = fm["per_seed"][(k, j)] - base
            mean, lo, hi = E.mean_ci(diff)
            if best is None or mean > best["gain"]:
                best = {"field": j, "deviate_to": k, "gain": mean, "lo": lo, "hi": hi}
        out.append(best)
    return out


def elo_summary(rec: E.Records, names: list[str]) -> dict:
    """The ladder's rating of the strategies from these games, for continuity only (stalled games are left out)."""
    from training.elo import ratings

    done = np.nonzero(~rec.stalled)[0]
    games = [{"seats": [names[k] for k in rec.seats[g, : rec.players[g]]], "scores": rec.scores[g, : rec.players[g]].tolist()}
             for g in done]
    return {k: round(v, 1) for k, v in ratings(games, names[0]).items()}


def secondary_metrics(rec: E.Records, names: list[str]) -> dict:
    """Descriptive statistics besides the payoff: per strategy over all the seats it occupies in the design
    (every composition weighted by its games), and the game length per player count.

    `fair` is the mean of n * utility, so 1 is the share of an equal player at any table size. The engine
    seat index is not turn order (the first player is drawn per game), so positions are in `turn_order`.
    """
    m = len(names)
    util = E.winner_share(rec)
    margin = E.score_margin(rec)
    done = ~rec.stalled
    won = (((rec.winners[:, None] >> np.arange(MAX_PLAYERS)) & 1) == 1) & done[:, None]
    shared = won & (won.sum(axis=1, keepdims=True) > 1)
    acc = {k: np.zeros(m) for k in ("seats", "fair", "wins", "ties", "margin", "margin_n", "length", "length_n")}
    for p in range(MAX_PLAYERS):
        rows = np.nonzero(rec.players > p)[0]
        s = rec.seats[rows, p].astype(np.int64)
        np.add.at(acc["seats"], s, 1)
        np.add.at(acc["fair"], s, util[rows, p] * rec.players[rows])
        np.add.at(acc["wins"], s, won[rows, p])
        np.add.at(acc["ties"], s, shared[rows, p])
        ok = done[rows]
        np.add.at(acc["margin"], s[ok], margin[rows, p][ok])
        np.add.at(acc["margin_n"], s[ok], 1)
        np.add.at(acc["length"], s[ok], rec.actions[rows][ok])
        np.add.at(acc["length_n"], s[ok], 1)
    strategies = {
        nme: dict(seat_games=int(acc["seats"][k]), fair_share=acc["fair"][k] / max(acc["seats"][k], 1),
                  win_rate=acc["wins"][k] / max(acc["seats"][k], 1), tie_rate=acc["ties"][k] / max(acc["seats"][k], 1),
                  score_margin=acc["margin"][k] / max(acc["margin_n"][k], 1),
                  actions=acc["length"][k] / max(acc["length_n"][k], 1))
        for k, nme in enumerate(names)
    }
    seats = {}
    for n in sorted({int(x) for x in rec.players}):
        rows = np.nonzero(rec.players == n)[0]
        seats[str(n)] = {
            "fair_share": [E.mean_ci(util[rows, p] * n, rec.cluster[rows]) for p in range(n)],  # by engine seat index
            "mean_actions": float(rec.actions[rows][done[rows]].mean()),
            "p99_actions": float(np.quantile(rec.actions[rows][done[rows]], 0.99)),
        }
    return {"strategies": strategies, "seats": seats}


def turn_order(rec: E.Records) -> dict:
    """Fair share (n x utility) by turn position over all games, with a seed-clustered 95% interval:
    position 0 moves first, position 1 next, and so on. Every strategy of a design sits in every position."""
    util = E.winner_share(rec)
    first = E.first_seats(rec)
    out = {}
    for n in sorted({int(x) for x in rec.players}):
        rows = np.nonzero(rec.players == n)[0]
        seat_at = (first[rows, None] + np.arange(n)[None, :]) % n  # engine seat holding each turn position
        fair = np.take_along_axis(util[rows], seat_at, axis=1) * n
        out[str(n)] = [E.mean_ci(fair[:, t], rec.cluster[rows]) for t in range(n)]
    return out


def build_payoffs(rec: E.Records, names: list[str], boot_seed: int = 0) -> dict:
    """Everything the report and the meta-solver need, as plain JSON types."""
    m = len(names)
    players = sorted({int(p) for p in rec.players})
    cells = E.payoff_cells(rec, m)
    out = {
        "strategies": names, "players": players, "accounting": E.accounting(rec), "meta": rec.meta,
        "utility": "winner share: 1/|W| per winner (engine tie-break), 0 otherwise; stalled games pay 0 to every seat",
        "cells": [], "by_players": {}, "secondary": secondary_metrics(rec, names),
    }
    for (n, ms), c in sorted(cells.items()):
        out["cells"].append({"players": n, "multiset": list(ms), "distinct": c["distinct"], "payoff": c["payoff"].tolist(),
                             "games": c["games"], "stalled": c["stalled"], "seeds": len(c["seeds"]),
                             "per_seed": np.round(c["per_seed"], 6).tolist()})
    for n in players:
        fm = field_matrix(cells, m, n)
        sel = rec.players == n
        out["by_players"][str(n)] = {
            "accounting": E.accounting(rec.subset(sel)),
            "field_matrix": {k: np.round(fm[k], 5).tolist() for k in ("mean", "lo", "hi")},
            "deviation_gains": deviation_gains(fm, m, n),
        }
    return out


def fmt(mean, lo, hi) -> str:
    return "—" if np.isnan(mean) else f"{mean:.3f} [{lo:.3f}, {hi:.3f}]"


def render_report(payoffs: dict, rec: E.Records, title: str) -> str:
    names, m = payoffs["strategies"], len(payoffs["strategies"])
    acc, meta = payoffs["accounting"], payoffs["meta"]
    lines = [f"# {title}", "",
             f"Population: {', '.join(names)}.  Horizon {acc['horizon']} actions, game length "
             f"{LENGTH_NAMES.get(int(meta.get('length', STANDARD)), meta.get('length'))}, "
             f"seed block `{hex(int(rec.seed.min()))[:5]}…`, {meta.get('seeds')} seeds per composition and table size, "
             f"every composition played in all its seat rotations. Commit `{meta.get('git')}`.", "",
             f"**{acc['games']:,} games: {acc['completed']:,} completed, {acc['stalled']:,} stalled "
             f"({100 * acc['stall_rate']:.2f}%).** Utility: {payoffs['utility']}.", "",
             "Policy modes (never pooled): " + ", ".join(f"{p['name']}={'scripted' if p['scripted'] is not None else p['mode']}"
                                                         for p in meta["policies"]) + ".", ""]
    games = np.zeros(m, dtype=np.int64)
    stalled = np.zeros(m, dtype=np.int64)
    for g in range(len(rec)):
        for k in set(rec.seats[g, : rec.players[g]].tolist()):
            games[k] += 1
            stalled[k] += rec.status[g] != 2
    lines += ["## Accounting per strategy", "", "| strategy | games seated in | stalled | stall rate |", "|---|---|---|---|"]
    for k, nme in enumerate(names):
        lines.append(f"| {nme} | {games[k]:,} | {stalled[k]:,} | {100 * stalled[k] / max(games[k], 1):.2f}% |")
    for n in payoffs["players"]:
        block = payoffs["by_players"][str(n)]
        fm = block["field_matrix"]
        a = block["accounting"]
        lines += ["", f"## {n} players: payoff of one seat of the row strategy at a table whose other {n - 1} seats play the "
                      f"column strategy", "",
                  f"{a['games']:,} games, {a['stalled']:,} stalled ({100 * a['stall_rate']:.2f}%). The diagonal is the "
                  f"all-same table: (1 − stall rate) / {n}. 95% seed-clustered CI in brackets.", "",
                  "| row \\ column | " + " | ".join(names) + " |", "|---|" + "---|" * m]
        for i in range(m):
            lines.append(f"| **{names[i]}** | " + " | ".join(fmt(fm["mean"][i][j], fm["lo"][i][j], fm["hi"][i][j]) for j in range(m)) + " |")
        lines += ["", f"Best single deviation within the population against each field of {n} (an optimistic lower bound on exploitability: the "
                      "best strategy is picked on the same seeds, and the real best response is not in this population):", "",
                  "| field | best deviation | gain over staying | 95% CI |", "|---|---|---|---|"]
        for d in block["deviation_gains"]:
            lines.append(f"| {names[d['field']]} | {names[d['deviate_to']]} | {d['gain']:+.3f} | [{d['lo']:+.3f}, {d['hi']:+.3f}] |")
    sec = payoffs.get("secondary")
    if sec:
        lines += ["", "## Per strategy over all the seats it occupies in the design", "",
                  "Compositions are weighted by their games. `fair share` is n × utility, averaged (1.00 = an equal player at any "
                  "table size). A tie is a shared win. Score margin is own total minus the mean of the opponents' totals and "
                  "length is in engine actions; both over completed games.", "",
                  "| strategy | seat-games | fair share | win rate | tie rate | score margin | actions per game |",
                  "|---|---|---|---|---|---|---|"]
        for nme, s in sec["strategies"].items():
            lines.append(f"| {nme} | {s['seat_games']:,} | {s['fair_share']:.3f} | {s['win_rate']:.3f} | {s['tie_rate']:.3f} | "
                         f"{s['score_margin']:+.1f} | {s['actions']:.0f} |")
        lines += ["", "## By turn position", "",
                  "Fair share (n × utility) of the player in each turn position over all games, with 95% seed-clustered CI; "
                  "position 0 moves first. The engine draws the first player at random for every game and every design "
                  "seats each strategy in every position equally often, so a position effect is averaged out of the payoffs "
                  "above, not part of them. 1.00 means no position effect; a position that is above 1.00 is above an equal "
                  "player's share.", "",
                  "| players | " + " | ".join(f"position {t}" for t in range(max(map(int, sec["seats"])))) +
                  " | actions per game (mean, p99) |", "|---|" + "---|" * max(map(int, sec["seats"])) + "---|"]
        turns = turn_order(rec)
        for n, s in sec["seats"].items():
            cells = [fmt(*ci) for ci in turns[n]] + ["—"] * (max(map(int, sec["seats"])) - len(turns[n]))
            lines.append(f"| {n} | " + " | ".join(cells) + f" | {s['mean_actions']:.0f}, {s['p99_actions']:.0f} |")
    elo = payoffs.get("elo")
    if elo:
        lines += ["", "## Secondary: ladder-style ELO of these games", "",
                  "For continuity with `training/ladder.json` only. It ignores stalled games and weights compositions "
                  "uniformly, so it is not comparable with the ladder's mixed-pool sampling, and it says nothing about "
                  "exploitability.", "", "| strategy | ELO |", "|---|---|"]
        lines += [f"| {k} | {v} |" for k, v in sorted(elo.items(), key=lambda kv: -kv[1])]
    return "\n".join(lines) + "\n"


def manifest_digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()[:16]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("cmd", choices=["run", "report"])
    ap.add_argument("--population", default=str(ROOT / "populations/baselines/manifest.json"))
    ap.add_argument("--strategies", help="comma-separated strategy names (default: all of the manifest)")
    ap.add_argument("--include", help="comma-separated strategy names: play only the compositions that seat one of them "
                                      "(the cells of a new member; the others are already simulated)")
    ap.add_argument("--mode", choices=E.MODES, help="play every net in this mode instead of the strategy's own")
    ap.add_argument("--players", default="3,4,5")
    ap.add_argument("--length", choices=["short", "standard", "extended"], default="standard")
    ap.add_argument("--seeds", type=int, default=20)
    ap.add_argument("--block", choices=list(BLOCKS), default="dev")
    ap.add_argument("--seed-offset", type=int, default=0)
    ap.add_argument("--envs", type=int, default=256)
    ap.add_argument("--horizon", type=int, default=E.HORIZON, help="engine actions after which a game counts as stalled")
    ap.add_argument("--out", required=True)
    ap.add_argument("--title", default="Cross-play report")
    args = ap.parse_args()
    out = Path(args.out)
    out = out if out.is_absolute() else ROOT / out
    pop = Population.load(args.population)
    names = args.strategies.split(",") if args.strategies else list(pop.strategies)
    policies = pop.policies(names)
    if args.mode:
        policies = [p.with_mode(args.mode) for p in policies]
    length = {"short": 0, "standard": 1, "extended": 2}[args.length]
    if args.cmd == "run":
        problems = pop.verify()
        if problems:
            raise SystemExit("population does not verify:\n" + "\n".join(problems))
        players = [int(p) for p in args.players.split(",")]
        if BLOCKS[args.block] == E.HOLDOUT:
            E.log_holdout(dict(command="crossplay", population=args.population, strategies=names, players=players,
                               length=args.length, seeds=args.seeds, offset=args.seed_offset, out=args.out))
        include = [names.index(x) for x in args.include.split(",")] if args.include else None
        rec = run_crossplay(policies, players, length, args.seeds, BLOCKS[args.block], out, seed_offset=args.seed_offset,
                            envs=args.envs, horizon=args.horizon, include=include)
        print(f"{len(rec):,} games in {rec.meta['seconds']}s; {E.accounting(rec)}")
    rec = E.Records.load(out / "records.npz")
    payoffs = build_payoffs(rec, names)
    payoffs["population"] = {"manifest": str(Path(args.population).relative_to(ROOT)) if Path(args.population).is_relative_to(ROOT)
                             else args.population, "manifest_sha256_16": manifest_digest(Path(args.population))}
    payoffs["elo"] = elo_summary(rec, names)
    (out / "payoffs.json").write_text(json.dumps(payoffs, indent=1) + "\n")
    (out / "report.md").write_text(render_report(payoffs, rec, args.title))
    print(f"wrote {out / 'payoffs.json'} and {out / 'report.md'}")


if __name__ == "__main__":
    main()
