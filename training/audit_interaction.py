"""How much a net interacts with the other players, and where its points come from.

Each net sits at three seats of a four-player table; the fourth seat is a scripted Merchant that stands in
for a human. Every figure is a mean per game over the games that ended, summed over the net's three seats
unless it says "per seat". The nets play the guarded greedy way they are deployed.

    python -m training.audit_interaction --games 200 \\
        magnate=populations/baselines/defense_run2_vs_exploiter_30m.pt \\
        mine=runs/x/final.pt:min_bid=3,no_same_turn_repay

A net may carry the rules it was trained under after a colon (`min_bid=N`, `no_same_turn_repay`).
"""

from __future__ import annotations

import argparse

import numpy as np
import torch

from training import evalcore as E
from training.env import MERCHANT, STANDARD, TRADER

KINDS = 26  # action kinds, one-hot at the start of an action's feature vector
SEAT0, LOC0 = KINDS + 5 + 6, KINDS + 5 + 6 + 5  # seat one-hot (relative to the viewer) and location one-hot
BUY, FACTORY_PURCHASE, SAIL, HARBOR_PURCHASE, BID, ACCEPT, PLACE = 5, 4, 7, 8, 13, 14, 12
LOT0 = KINDS + 5  # lot one-hot, six slots
BANK_KINDS = (20, 21, 22)
HARBOR_LOCATION = 3
MERCHANT_SEAT = 3


class Tally:
    wants_seat = True

    def __init__(self, players: int = 4):
        self.n = players
        self.count = np.zeros(KINDS)
        self.bid_money = 0.0
        self.factory_from = np.zeros(players)
        self.harbor_to = np.zeros(players)
        self.placed = np.zeros(6)  # containers priced, by lot (a lot's price is its index + 1 in a factory, + 2 in a harbor)
        self.bought = np.zeros(6)  # containers bought, by lot

    def __call__(self, action, rows, mask, acts, seat=None):
        a = acts[rows, : mask.shape[1]]
        pick = a[torch.arange(len(rows), device=a.device), action[rows]].cpu().numpy()
        kind = pick[:, :KINDS].argmax(1)
        rel = pick[:, SEAT0:SEAT0 + 5].argmax(1)
        loc = pick[:, LOC0:LOC0 + 4].argmax(1)
        self.count += np.bincount(kind, minlength=KINDS)
        lot = pick[:, LOT0:LOT0 + 6].argmax(1)
        self.placed += np.bincount(lot[kind == PLACE], minlength=6)
        self.bought += np.bincount(lot[kind == BUY], minlength=6)
        self.bid_money += float((pick[kind == BID, -3] * 30).sum())
        for k, r, l, s in zip(kind, rel, loc, seat):
            if k == FACTORY_PURCHASE:
                self.factory_from[(s + r) % self.n] += 1
            elif k == SAIL and l == HARBOR_LOCATION:
                self.harbor_to[(s + r) % self.n] += 1
        return action


def parse(spec: str):
    name, _, rest = spec.partition("=")
    path, _, rules = rest.partition(":")
    return name, path, dict(r.partition("=")[::2] for r in rules.split(",") if r)


def audit(name: str, path: str, rules: dict, games: int, envs: int = 256, stand_in: int = MERCHANT) -> dict:
    tally = Tally()
    pol = [E.Policy(name, path, "guarded", adjust=tally), E.Policy("stand-in", scripted=stand_in)]

    def configure(env):
        if "classic" in rules:
            env.ban_bank_all()
        if "min_bid" in rules:
            env.require_min_bid(E.SLOT_BASE, int(rules["min_bid"]))
        if "max_price" in rules:
            env.require_max_price(E.SLOT_BASE, int(rules["max_price"]))
        if "no_same_turn_repay" in rules:
            env.forbid_same_turn_repay(E.SLOT_BASE)

    specs = [E.GameSpec(4, STANDARD, E.game_seed(E.AUDIT, 4, STANDARD, i), (0, 0, 0, 1), 0, i) for i in range(games)]
    rec = E.play(specs, pol, envs=envs, log=None, configure=configure)
    ok = ~rec.stalled
    parts = rec.breakdown[ok][:, :3].reshape(-1, 4).mean(0)
    c = tally.count / games
    return dict(
        name=name, games=games, stalled=int(rec.stalled.sum()),
        bought=c[BUY], factory_other=tally.factory_from[:3].sum() / games, factory_merchant=tally.factory_from[3] / games,
        harbor_sails_merchant=tally.harbor_to[3] / games, harbor_sails_nets=tally.harbor_to[:3].sum() / games,
        harbor_purchases=c[HARBOR_PURCHASE], bids=c[BID], accepted=c[ACCEPT],
        mean_bid=tally.bid_money / max(tally.count[BID], 1), bank=c[list(BANK_KINDS)].sum(),
        bid_share=c[BID] / max(c[BID] + c[list(BANK_KINDS)].sum(), 1),
        placed=tally.placed / max(tally.placed.sum(), 1), bought_lots=tally.bought / max(tally.bought.sum(), 1),
        cash=parts[0], island=parts[1], leftovers=parts[2], loans=parts[3],
    )


def render(rows: list[dict]) -> str:
    head = ("| net | games (stalled) | containers bought | factory purchases: stand-in / other nets | harbor sails: stand-in / nets "
            "| harbor purchases | bids on players | accepted | mean bid | Bank bids | bid share | cash | island | leftovers | loans |")
    out = [head, "|" + "---|" * 15]
    for r in rows:
        out.append(
            f"| {r['name']} | {r['games']} ({r['stalled']}) | {r['bought']:.1f} | {r['factory_merchant']:.1f} / {r['factory_other']:.1f} "
            f"| {r['harbor_sails_merchant']:.1f} / {r['harbor_sails_nets']:.1f} | {r['harbor_purchases']:.1f} | {r['bids']:.1f} "
            f"| {r['accepted']:.1f} | ${r['mean_bid']:.2f} | {r['bank']:.1f} | {r['bid_share']:.2f} | {r['cash']:.1f} | {r['island']:.1f} "
            f"| {r['leftovers']:.1f} | -{r['loans']:.1f} |")
    out += ["", "Share of containers by lot (a factory lot costs lot+1, a harbor lot lot+2):", "",
            "| net | priced at lot 0 / 1 / 2 / 3 / 4 | bought from lot 0 / 1 / 2 / 3 / 4 |", "|---|---|---|"]
    for r in rows:
        out.append(f"| {r['name']} | {' / '.join(f'{x:.2f}' for x in r['placed'][:5])} "
                   f"| {' / '.join(f'{x:.2f}' for x in r['bought_lots'][:5])} |")
    return "\n".join(out)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("nets", nargs="+", help="name=path[:rule,rule]")
    ap.add_argument("--games", type=int, default=200)
    ap.add_argument("--stand-in", choices=["merchant", "trader"], default="merchant",
                    help="the scripted seat at the table: Merchant, or a trader that buys only at fair prices")
    args = ap.parse_args()
    stand_in = TRADER if args.stand_in == "trader" else MERCHANT
    print(render([audit(n, p, r, args.games, stand_in=stand_in) for n, p, r in map(parse, args.nets)]))


if __name__ == "__main__":
    main()
