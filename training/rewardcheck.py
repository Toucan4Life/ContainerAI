"""What does the corrected Shipper reward actually pay? Evaluation only.

    python -m training.rewardcheck peddler2 runs/strategist/shipper_s1/final.pt --games 100

For each focus net, at the current table and at a table of three copies of itself (what a learner meets in the
league), prints the spread of the three reward terms, their weighted contribution to the 0..1 term, and the
cash terms in dollars: gross delivery cash (twice the winning bid, deliverer side), container purchases (SPENT,
the only cost deducted) and their difference.
"""

from __future__ import annotations

import argparse

import numpy as np

from training import economics, gate
from training import env as V
from training import selfplay as S


def terms(inter: np.ndarray) -> dict:
    net = inter[:, V.CARGO_CASH:V.CARGO_CASH + 5].sum(1) - inter[:, V.SPENT]
    shipped, deliveries = inter[:, V.SHIPPED], inter[:, V.DELIVERIES]
    used = np.divide(shipped, S.SHIP_CAPACITY * deliveries, out=np.zeros_like(shipped), where=deliveries > 0)
    parts = np.stack([np.clip(net / S.NET_SCALE, 0, 1), np.tanh(shipped / S.DELIVERED_SCALE), used])
    gross = inter[:, V.CARGO_CASH:V.CARGO_CASH + 5].sum(1)
    return dict(gross=gross, spent=inter[:, V.SPENT], net=net, parts=parts, bids=inter[:, V.BID_MONEY],
                weighted=parts * np.array(S.SHIPPER_WEIGHTS)[:, None], total=np.dot(S.SHIPPER_WEIGHTS, parts))


def report(label: str, focus: str, games: int, table: tuple[str, ...], name: str) -> None:
    _, path, rules = economics.resolve(focus)
    rec, _ = gate.play_focus(path, rules, games, table=table)
    ok = ~rec.stalled
    n = len(table) + 1
    inter = rec.interaction[ok][:, :n].astype(np.float64)
    mine = rec.seats[ok][:, :n] == 0
    t = terms(inter[mine])
    q = lambda a: f"{a.mean():.2f} [p10 {np.percentile(a, 10):.2f}, p90 {np.percentile(a, 90):.2f}]"
    print(f"| {label} | {name} | {q(t['gross'])} | {q(t['spent'])} | {q(t['net'])} | {t['bids'].mean():.1f} | "
          f"{t['parts'][0].mean():.2f} / {t['parts'][1].mean():.2f} / {t['parts'][2].mean():.2f} | "
          f"{t['weighted'][0].mean():.3f} / {t['weighted'][1].mean():.3f} / {t['weighted'][2].mean():.3f} | "
          f"{t['total'].mean():.3f} | {float((t['net'] < 0).mean()):.2f} | {float((t['net'] > S.NET_SCALE).mean()):.2f} |")


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("focus", nargs="+")
    p.add_argument("--games", type=int, default=100)
    args = p.parse_args()
    print("| net | table | gross delivery cash $ | container purchases $ | net $ (gross - purchases) | $ the seat bid as a rival (not deducted) | terms net/volume/utilisation | weighted | reward | share net<0 | share net>60 |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for f in args.focus:
        _, path, _ = economics.resolve(f)
        report(f, f, args.games, gate.TABLE, "current")
        report(f, f, args.games, (f, f, f), "3 copies of itself")


if __name__ == "__main__":
    main()
