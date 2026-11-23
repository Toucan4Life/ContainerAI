"""Share of each kind of move per net: every action it plays, in any phase, at a table with Peddler v2, Consortium and Magnate.

    python -m training.move_table --games 100 > reports/strategist/move_table.md
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

import numpy as np

from training import env as V
from training import evalcore as E
from training.strategy_metrics import NETS, configure_table

# The rules of the Phase 2 runs (`reports/strategist/run_phase2.sh`).
RULES = {"min_bid": 3, "no_same_turn_repay": 1, "buy_at_harbor": 1, "bank_needs_load": 1, "no_double_reprice": 1}
TABLE = ("peddler2", "consortium", "magnate")
FAMILIES = ["control", "producer", "shipper", "shipper_capped", "showcase_fair", "showcase_sales", "showcase_variety"]

GROUPS = [
    ("Main action: making, building and buying", [
        ("produce", "Produce"), ("build_factory", "Build factory"), ("build_warehouse", "Build warehouse"),
        ("factory_purchase", "Start buying at a factory"), ("harbor_purchase", "Start buying at a harbor"),
        ("start_loading", "Start loading at the Bank"), ("reprice_factory", "Reprice factory"),
        ("reprice_harbor", "Reprice harbor"), ("pass", "Pass")]),
    ("Main action: ship, Bank and loans", [
        ("sail_ocean", "Sail to ocean"), ("sail_bank", "Sail to Bank"), ("sail_harbor", "Sail to a harbor"),
        ("sail_island", "Sail to Island"), ("call_container_lot", "Call Bank: container lot"),
        ("call_cash_lot", "Call Bank: cash lot"), ("outbid_container_lot", "Outbid: container lot"),
        ("outbid_cash_lot", "Outbid: cash lot"), ("request_loan", "Request loan"), ("repay_loan", "Repay loan")]),
    ("Inside an action: producing, buying, loading and laying out", [
        ("produce_color", "Pick a colour to produce"), ("buy_factory", "Take a container, factory lot"),
        ("buy_harbor", "Take a container, harbor lot"), ("done_buying", "Done buying"), ("load", "Load a container"),
        ("done_loading", "Done loading"), ("place", "Place a container in a lot")]),
    ("Inside an action: auctions and defaults", [
        ("bid", "Delivery bid"), ("accept", "Accept a bid"), ("buy_out", "Buy out"), ("bid_cash", "Bank bid: cash"),
        ("bid_container", "Bank bid: container"), ("done_bidding", "Done Bank bid"), ("seize", "Seize"),
        ("deal", "Deal")]),
]

NOTES = """**Notes**

- Build warehouse is the "harbor" building: the engine has no other, and the warehouses a player owns are its harbor capacity.
  Each player starts with one warehouse and one factory; the next warehouse costs $4 and the next factory $6.
- Pass is Reprice factory played with an empty factory district, which changes nothing.
- Produce makes one container per factory; Pick a colour to produce is only asked when the factory storage is too full for
  all of them.
- Request loan counts in every phase (between actions, when bidding on a delivery, when deciding on a bid); Repay loan only
  on the net's own turn.
- The "Take a container" columns count every container taken, so they are the purchases; the "Start buying" columns are the
  paid action that opens one. Delivery bid is a bid on another seat's delivery; Sail to Island is the seat's own delivery.
- Peddler v2 and the trained runs play under the Peddler table rules (min bid $3, no repaying a loan in the turn it was taken,
  a purchase after sailing to a harbor, a load to sail to the Bank, no repeated reprice). Consortium and Magnate play without
  table rules, as deployed, so their loan and Bank columns do not compare with the others. The table's Peddler v2 seat plays by
  the rules of the net being scored (the engine mirrors them, as in training).
- The allowance of a bidder for a cargo is what the cargo is worth on that seat's own secret scoring card: every container at
  its value on the card, a full set of colours assumed (so the card's second colour is worth $10) and no colour discarded, or
  $5 if that is more (the engine's `BID_ALLOWANCE_FLOOR`). The mean allowance is over all of a net's delivery bids, whatever it
  bid; "bid money above the allowance" is the share of the money it put on the table beyond it (a runoff bid counts only what
  it adds).
- Containers/delivery is the number of containers on the ship when the seat sails to the Island (`SHIPPED / DELIVERIES`),
  averaged over its trips; a ship holds 5, and an empty ship may not sail there. Deliveries/game are the seat's Sail to Island
  moves per game. The mean does not show how the cargoes are spread, but every delivery carries at least one container, so at
  most (mean - 1) / (k - 1) of a net's deliveries carry k or more.
- Under the harbor rule a sail to a harbor must open a purchase, so Sail to a harbor and Take a container, harbor lot match for
  the nets bound by it; the last line lists the exceptions. The engine waives a rule when every legal move is forbidden. A
  one-off trace of shipper_s2 (100 games per seat) followed its 3307 harbor sails: 3030 opened a purchase, 218 did not, and 59
  ended without a readable outcome. In all 218 the legal moves were the three harbor sails, one sail to the Bank and one
  reprice; the only rules that can forbid the last two are Bank-load (nothing to load) and no repeated reprice.
"""

CAPPED_NOTES = """- The capped runs are played with `--bid-within-value`: they may not have more on the table than their allowance for the
  cargo, so their bid money above the allowance is 0% unless the engine waived the rule (it does so only when every legal move
  is forbidden). Beside one, the table's Peddler v2 seat is bound the same way. shipper_capped_s1 and s2 were trained under an
  earlier, tighter allowance (what the cargo adds to the seat's own island, $5 at least) and have not been retrained since.
"""


def split(spec: str) -> tuple[str, dict]:
    path, _, rules = spec.partition(":")
    return path, dict(r.partition("=")[::2] for r in rules.split(",") if r)


def run_nets() -> dict[str, tuple[str, dict]]:
    """The trained runs with their rules, grouped by family."""
    def key(d: Path):
        family = re.sub(r"_s\d+$", "", d.name)
        return (FAMILIES.index(family) if family in FAMILIES else len(FAMILIES), d.name)

    dirs = sorted((d for d in Path("runs/strategist").iterdir() if (d / "final.pt").exists()), key=key)
    return {d.name: (str(d / "final.pt"), RULES | ({"bid_within_value": ""} if "capped" in d.name else {})) for d in dirs}


def bid_summary(rows: np.ndarray) -> dict[str, float]:
    """Delivery bids per game, the mean bid and the mean allowance for the cargo over them (what it is worth on the bidder's card,
    $5 at least), and the share of the bid money that was above it (`BID_VALUE` and `OVERBID` are per bid, so a runoff bid counts again)."""
    s = rows.sum(0)
    bids = s[V.BIDS]
    return {"per_game": float(bids / len(rows)), "bid": float(s[V.BID_MONEY] / bids) if bids else 0.0,
            "value": float(s[V.BID_VALUE] / bids) if bids else 0.0,
            "over": float(s[V.OVERBID] / s[V.BID_MONEY]) if s[V.BID_MONEY] else 0.0}


def delivery_summary(rows: np.ndarray) -> dict[str, float]:
    """The seat's own deliveries per game (its sails to the Island) and the containers on its ship at each, on average."""
    s = rows.sum(0)
    trips = s[V.DELIVERIES]
    return {"per_game": float(trips / len(rows)), "cargo": float(s[V.SHIPPED] / trips) if trips else 0.0}


def moves_of(name: str, path: str, rules: dict, games: int, names: list[str]) -> tuple[float, dict[str, float], dict[str, int], dict[str, float], dict[str, float]]:
    """Moves per game of the net, the share of each kind among them, the counts, the bid summary and the delivery summary,
    over `games` games at each seat."""
    nets = [(name, path, rules)] + [(n, *split(NETS[n])) for n in TABLE]
    pol = [E.Policy(n if i else "focus", p, "guarded") for i, (n, p, _) in enumerate(nets)]

    def configure(env):
        configure_table(env, [(E.SLOT_BASE + k, r) for k, (_, _, r) in enumerate(nets)])

    n = len(pol)
    specs = []
    for seat in range(n):
        others = iter(range(1, n))
        seats = tuple(0 if s == seat else next(others) for s in range(n))
        for i in range(games):
            specs.append(E.GameSpec(n, V.STANDARD, E.game_seed(E.AUDIT, n, V.STANDARD, i), seats, seat, i))
    rec = E.play(specs, pol, envs=256, log=None, configure=configure)
    ok = ~rec.stalled
    mine = rec.interaction[(rec.seats == 0) & ok[:, None]]
    moves = mine[:, V.MOVES:]
    total = moves.sum(0)
    return (float(moves.sum(1).mean()), dict(zip(names, total / total.sum())), dict(zip(names, map(int, total))),
            bid_summary(mine), delivery_summary(mine))


def harbor_gap(rows: dict, nets: dict) -> str:
    """Sail to a harbor minus Take a container, harbor lot, for the nets bound by the harbor rule.

    A sail must open a purchase that takes a container, so a positive value means that at least that many sails
    bought nothing: the engine waived the rule for a seat whose every legal move was forbidden.
    """
    gaps = {n: (c["sail_harbor"] - c["buy_harbor"], c["sail_harbor"]) for n, (_, _, c, _, _) in rows.items()
            if "buy_at_harbor" in nets[n][1]}
    shown = [f"{n} {g} of {s} ({100 * g / s:.1f}%)" for n, (g, s) in gaps.items() if g]
    zero = sum(1 for g, _ in gaps.values() if not g)
    return ("Sail to a harbor minus Take a container, harbor lot, in counts over each net's games, for the nets bound by the "
            f"harbor rule: {', '.join(shown) or 'none'}; zero for the other {zero}.\n")


def bid_table(rows: dict) -> str:
    """The delivery bids of each net against what the cargo is worth on the bidder's own scoring card."""
    lines = ["**Delivery bids against the allowance: what the cargo is worth on the bidder's scoring card, $5 at least**\n",
             "| net | bids/game | mean bid | mean allowance | bid money above the allowance |", "|---|---|---|---|---|"]
    for name, (_, _, _, b, _) in rows.items():
        lines.append(f"| {name} | {b['per_game']:.1f} | ${b['bid']:.2f} | ${b['value']:.2f} | {100 * b['over']:.0f}% |")
    return "\n".join(lines) + "\n"


def delivery_table(rows: dict) -> str:
    """How full each net's ship is when it sails to the Island."""
    lines = ["**Deliveries: containers on the ship when the net sails to the Island (a ship holds 5)**\n",
             "| net | deliveries/game | containers/delivery |", "|---|---|---|"]
    for name, (_, _, _, _, d) in rows.items():
        lines.append(f"| {name} | {d['per_game']:.1f} | {d['cargo']:.2f} |")
    return "\n".join(lines) + "\n"


def percent(x: float) -> str:
    return "0" if x == 0 else "<0.1%" if x < 0.0005 else f"{100 * x:.1f}%"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--games", type=int, default=100, help="games per seat")
    args = ap.parse_args()
    names = V.move_names()
    listed = [m for _, moves in GROUPS for m, _ in moves]
    assert sorted(listed) == sorted(names), f"the tables must list every move once: {set(names) ^ set(listed)}"
    nets = {n: split(NETS[n]) for n in NETS} | run_nets()
    rows = {}
    for name, (path, rules) in nets.items():
        rows[name] = moves_of(name, path, rules, args.games, names)
        print(f"{name} done", file=sys.stderr, flush=True)
    print(f"Share of all the moves a net plays, in every phase, forced moves included. Each net sits at every seat of a "
          f"table with {', '.join(TABLE)} ({args.games} games per seat, {4 * args.games} per net). A row adds up to 100% "
          "over the four tables.\n")
    for k, (title, moves) in enumerate(GROUPS):
        print(f"**{title}**\n")
        print("| net |" + (" moves/game |" if k == 0 else "") + "".join(f" {label} |" for _, label in moves))
        print("|---|" + ("---|" if k == 0 else "") + "---|" * len(moves))
        for name, (per_game, share, _, _, _) in rows.items():
            print(f"| {name} |" + (f" {per_game:.0f} |" if k == 0 else "") + "".join(f" {percent(share[m])} |" for m, _ in moves))
        print()
    print(bid_table(rows))
    print(delivery_table(rows))
    print(NOTES + (CAPPED_NOTES if any("capped" in n for n in nets) else ""))
    print(harbor_gap(rows, nets))


if __name__ == "__main__":
    main()
