"""Where the scripted bots would sit on the ladder: Producer, Showcase and Shipper rated the way `anchors rate` rates a net.

    python -m training.heuristic_elo [--games 3000] [--reps 200] > reports/strategist/heuristic_elo.md

The protocol is `anchors.evaluate`'s: the rated bot sits at a table of 3 to 5 seats, Merchant is always in the field and the other
opponents are drawn uniformly from the ladder (Random, Simple, Merchant and the anchor nets of `training/ladder.json`, which play their
best action). Each opponent gives one duel (the higher score wins, a tie counts half) and `elo.rate_against` turns the duels into a rating
on the ladder's scale (Random = 0). Two ladders: the full one, and the scripted one (Random, Simple and Merchant only), which is how
Solitaire is rated in the UI list. Every rated bot plays the same tables and seeds (the `DEV` block). A stalled game is not rated, as in
the ladder, and is counted. Random, Simple, Merchant and Solitaire are rated by the same code as a check: their listed ratings (0, 263,
932 and 1200 on the scripted ladder) should come back. Intervals are 95% bootstraps over games.
"""

from __future__ import annotations

import argparse
import sys

import numpy as np
import torch

from training import anchors
from training import evalcore as E
from training.elo import rate_against
from training.env import DONE, MERCHANT, PRODUCER, RANDOM, SHIPPER, SHOWCASE, SIMPLE, SOLITAIRE, STANDARD

RATED = {"producer": PRODUCER, "showcase": SHOWCASE, "shipper": SHIPPER,
         "random": RANDOM, "simple": SIMPLE, "merchant": MERCHANT, "solitaire": SOLITAIRE}
LISTED = {"random": 0, "simple": 263, "merchant": 932, "solitaire": 1200}


def field_policies(ladder: dict[str, float]) -> list[E.Policy]:
    scripted = [E.Policy(n, scripted=k) for n, k in anchors.SCRIPTED.items()]
    nets = [E.Policy(n, str(anchors.DIR / f"{n}.pt"), "greedy") for n in anchors.net_names(ladder)]
    return scripted + nets


def tables(field: list[E.Policy], names: list[str], games: int) -> list[E.GameSpec]:
    """Policy 0 is the rated bot, policy `1 + k` is `field[k]`; the draws do not depend on who is rated."""
    index = {p.name: 1 + k for k, p in enumerate(field)}
    specs = []
    for g in range(games):
        rng = np.random.default_rng([7, g])
        n = int(rng.integers(2, 5))
        seats = [0, index["merchant"]] + [index[str(rng.choice(names))] for _ in range(n - 1)]
        specs.append(E.GameSpec(n + 1, STANDARD, E.game_seed(E.DEV, n + 1, STANDARD, g),
                                tuple(int(s) for s in rng.permutation(seats)), g, g))
    return specs


def duels(rec: E.Records, policies: list[E.Policy]) -> list[list[tuple[str, float]] | None]:
    """Per game, the rated seat's duels against each other seat; None for a stalled game."""
    out = []
    for g in range(len(rec)):
        if rec.status[g] != DONE:
            out.append(None)
            continue
        n = int(rec.players[g])
        seats, score = rec.seats[g, :n], rec.scores[g, :n]
        me = int(np.flatnonzero(seats == 0)[0])
        out.append([(policies[seats[s]].name, 1.0 if score[me] > score[s] else 0.5 if score[me] == score[s] else 0.0)
                    for s in range(n) if s != me])
    return out


def rate(played: list, known: dict[str, float], reps: int) -> tuple[float, float, float, dict[str, float]]:
    done = [g for g in played if g is not None]
    flat = [d for g in done for d in g]
    rng = np.random.default_rng(0)
    boot = [rate_against(known, [d for i in rng.integers(0, len(done), len(done)) for d in done[i]]) for _ in range(reps)]
    per = {n: float(np.mean([o for m, o in flat if m == n])) for n in known if any(m == n for m, _ in flat)}
    lo, hi = np.percentile(boot, [2.5, 97.5])
    return rate_against(known, flat), float(lo), float(hi), per


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--games", type=int, default=3000)
    ap.add_argument("--reps", type=int, default=200)
    args = ap.parse_args()
    ladder = anchors.load_ladder()
    field = field_policies(ladder)
    nets = E.load_nets(field, "cuda" if torch.cuda.is_available() else "cpu")
    scripted = list(anchors.SCRIPTED)
    ladders = {"full": ([p.name for p in field], ladder),
               "scripted": (scripted, {n: ladder[n] for n in scripted})}
    rows, per_opponent = {}, {}
    for kind, (names, known) in ladders.items():
        specs = tables(field, names, args.games)
        for bot, code in RATED.items():
            policies = [E.Policy(bot, scripted=code), *field]
            rec = E.play(specs, policies, nets=nets, log=None)
            res = rate(duels(rec, policies), known, args.reps)
            rows[bot, kind] = (*res[:3], int(rec.stalled.sum()))
            per_opponent[bot, kind] = res[3]
            print(f"{kind} / {bot}: {res[0]:.0f}, {int(rec.stalled.sum())} stalled", flush=True, file=sys.stderr)
    print(f"# Ladder ratings of the scripted bots ({args.games} games per bot and ladder, `DEV` seeds)\n")
    print("Rating on the ladder of `training/ladder.json` (Random = 0, Simple 263, Merchant 932, the anchor nets 1907 to 2286), 95% bootstrap "
          "interval over games. The last four rows are the check: their listed ratings are in the last column "
          "(Solitaire's 1200 is its scripted-ladder rating).\n")
    print("| bot | full ladder | scripted ladder (Random, Simple, Merchant) | stalled games (full / scripted) | listed |")
    print("|---|---|---|---|---|")
    for bot in RATED:
        (f, flo, fhi, fs), (s, slo, shi, ss) = rows[bot, "full"], rows[bot, "scripted"]
        print(f"| {bot} | {f:.0f} [{flo:.0f}, {fhi:.0f}] | {s:.0f} [{slo:.0f}, {shi:.0f}] | {fs} / {ss} of {args.games} | {LISTED.get(bot, '')} |")
    for kind, (names, _) in ladders.items():
        print(f"\nShare of the duels each bot wins against each member of the {kind} ladder (the rated bot against that opponent only):\n")
        print("| bot | " + " | ".join(names) + " |")
        print("|---|" + "---|" * len(names))
        for bot in RATED:
            per = per_opponent[bot, kind]
            print(f"| {bot} | " + " | ".join(f"{per[n]:.2f}" if n in per else "" for n in names) + " |")


if __name__ == "__main__":
    main()
