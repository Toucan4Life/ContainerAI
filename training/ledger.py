"""The cash ledger of a game: where each seat's money came from and where it went.

The training reward is a shaped number, and `delivery_cash_minus_purchases` (formerly `delivery_net`) is not profit: it is the cash a seat
received for its deliveries minus what it paid for containers, and leaves out builds, wages, interest, loans, the Bank and what the seat
paid for the cargoes it won. `flows` adds up everything the counters of `crates/container-env` can see, so that the change in a seat's
cash over a game is accounted for. What no counter follows (Bank auctions, seizures, deals) is left in `bank_and_other`; it is exactly
zero in a game without Bank calls and without seizures, which is how the counters are tested.

    flows = ledger.flows(rec.interaction, rec.breakdown, rec.players)   # arrays of games x MAX_PLAYERS, finished games only mean something
"""

from __future__ import annotations

import numpy as np

from training import env as V

START_CASH = 20
LOAN = 10  # cash a loan pays out
LOAN_PAYOFF = 11  # what an unpaid loan costs in the final score
# Paid to the supply for the first 0, 1, 2, ... factories (the starting one is free) and warehouses a seat builds.
FACTORY_SPEND = np.array([0, 6, 15, 27])
WAREHOUSE_SPEND = np.array([0, 4, 9, 15, 22])

IN = ("delivery_received", "containers_sold", "wages_received", "loans")
OUT = ("delivery_paid", "buyouts", "containers_bought", "builds", "wages_paid", "interest")
FLOWS = IN + OUT + ("bank_and_other",)


def flows(interaction: np.ndarray, breakdown: np.ndarray, players: np.ndarray, names: list[str] | None = None) -> dict[str, np.ndarray]:
    """Dollars per game and seat (games x MAX_PLAYERS; unused seats are zero) of each flow in `FLOWS`, plus `net_cash`,
    the change in the seat's cash over the game: its final cash minus `START_CASH`.

    `delivery_received` is the deliverer's cash for accepted deliveries (twice the winning bid), `delivery_paid` the winning bid
    paid for a cargo, `buyouts` the Bank payment for keeping one's own cargo, `containers_sold`/`containers_bought` the prices
    paid between players, `wages_*` the $1 union wage of Produce (paid to the seat on the right), `loans` the loan principal not
    repaid by the end, and `interest` the $1 per loan held at the start of each turn. Only finished games have a final cash."""
    names = names or V.move_names()
    column = {name: V.MOVES + k for k, name in enumerate(names)}
    x = interaction.astype(np.int64)
    seats = x.shape[1]
    seat = np.arange(seats)[None, :]
    count = np.asarray(players)[:, None]
    used = seat < count
    produces = x[:, :, V.PRODUCES]
    # The wage goes to the seat on the right, the one that plays just before: a seat is paid by the one after it.
    paid_by = np.take_along_axis(produces, (seat + 1) % count, axis=1)
    f = {
        "delivery_received": x[:, :, V.CARGO_CASH:V.CARGO_CASH + 5].sum(2),
        "containers_sold": x[:, :, V.EARNED],
        "wages_received": paid_by * used,
        "loans": LOAN * (breakdown[:, :, 3].astype(np.int64) // LOAN_PAYOFF),
        "delivery_paid": x[:, :, V.DEAL_PAID],
        "buyouts": x[:, :, V.BUYOUT_PAID],
        "containers_bought": x[:, :, V.SPENT],
        "builds": FACTORY_SPEND[x[:, :, column["build_factory"]]] + WAREHOUSE_SPEND[x[:, :, column["build_warehouse"]]],
        "wages_paid": produces,
        "interest": x[:, :, V.INTEREST],
    }
    f["net_cash"] = (breakdown[:, :, 0].astype(np.int64) - START_CASH) * used
    f["bank_and_other"] = f["net_cash"] - sum(f[k] for k in IN) + sum(f[k] for k in OUT)
    return f
