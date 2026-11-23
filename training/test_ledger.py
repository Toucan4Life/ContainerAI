import numpy as np

from training import ledger
from training.env import DONE, INTERACTION, MERCHANT, MOVES, PRODUCER, PRODUCES, SHIPPER, SHOWCASE, SIMPLE, SOLITAIRE, BatchEnv, move_names


def simulate(players, kinds, ban_bank, games=24, seed=1):
    env = BatchEnv(games, threads=4)
    if ban_bank:
        env.ban_bank_all()
    for i in range(games):
        env.reset(i, players, seed + i, list(kinds))
    env.advance()
    status, count, _ = env.results()
    assert (status == DONE).all()
    return env.interaction(), env.breakdown(), count


def play(players, kinds, ban_bank, games=24, seed=1):
    interaction, breakdown, count = simulate(players, kinds, ban_bank, games, seed)
    return ledger.flows(interaction, breakdown, count)


def test_without_the_bank_the_ledger_accounts_for_every_dollar_of_every_table_size():
    # None of these seats defaults, so the interest due is the interest paid; a default leaves its unpaid part in `bank_and_other`.
    tables = {3: (SOLITAIRE, MERCHANT, MERCHANT), 4: (MERCHANT,) * 4, 5: (MERCHANT,) * 5}
    for players, kinds in tables.items():
        f = play(players, kinds, ban_bank=True)
        assert (f["bank_and_other"] == 0).all(), f"{players} seats"
        for flow in ("delivery_received", "delivery_paid", "containers_bought", "wages_paid") + (("interest",) if players == 3 else ()):
            assert f[flow].sum() > 0, f"{players} seats: no {flow}"
        assert f["wages_paid"].sum() == f["wages_received"].sum(), "every wage has a payer and a payee"
        assert (f["delivery_paid"].sum(1) <= f["delivery_received"].sum(1)).all(), "the supply adds to every winning bid"


def test_builds_and_wages_are_priced_from_the_counters():
    names = move_names()
    x = np.zeros((1, 5, INTERACTION), dtype=np.int32)
    x[0, 0, [MOVES + names.index("build_factory"), MOVES + names.index("build_warehouse")]] = [2, 3]
    x[0, 1, MOVES + names.index("build_factory")] = 3
    x[0, 0, PRODUCES], x[0, 1, PRODUCES], x[0, 2, PRODUCES] = 4, 7, 1
    breakdown = np.zeros((1, 5, 4), dtype=np.int32)
    breakdown[0, :3, 0] = [20, 20, 20]
    f = ledger.flows(x, breakdown, np.array([3]), names)
    assert f["builds"][0].tolist() == [15 + 15, 27, 0, 0, 0], "6+9 for two factories and 4+5+6 for three warehouses; 6+9+12 for three"
    assert f["wages_received"][0].tolist() == [7, 1, 4, 0, 0], "a seat is paid by the one after it, and the last wraps to the first"
    assert f["bank_and_other"][0].tolist() == [30 + 4 - 7, 27 + 7 - 1, 1 - 4, 0, 0], "cash that did not move, so what was paid out and not taken in is unexplained"


def test_the_bank_flows_are_what_the_ledger_cannot_see():
    f = play(4, (MERCHANT, MERCHANT, MERCHANT, SIMPLE), ban_bank=False)
    assert (f["bank_and_other"] != 0).any(), "merchants call the Bank, and its auctions move cash that no counter follows"
    assert np.array_equal(f["net_cash"], sum(f[k] for k in ledger.IN) - sum(f[k] for k in ledger.OUT) + f["bank_and_other"])


def test_the_ledger_closes_for_the_heuristics_and_prices_what_they_build():
    names = move_names()
    tables = {3: (PRODUCER, SHOWCASE, SHIPPER), 4: (PRODUCER, SHOWCASE, SHIPPER, MERCHANT), 5: (PRODUCER, SHOWCASE, SHIPPER, SHOWCASE, PRODUCER)}
    built = np.zeros(2)
    for players, kinds in tables.items():
        interaction, breakdown, count = simulate(players, kinds, ban_bank=True, games=60)
        f = ledger.flows(interaction, breakdown, count, names)
        # A default leaves its unpaid interest in `bank_and_other`; the heuristics default rarely.
        defaulted = interaction[:, :, [MOVES + names.index("seize"), MOVES + names.index("deal")]].sum((1, 2)) > 0
        assert (~defaulted).sum() >= 30, f"{players} seats: most games should be free of defaults"
        assert (f["bank_and_other"][~defaulted] == 0).all(), f"{players} seats"
        built += f["builds"][:, :2].sum(0)
    assert (built > 0).all(), f"the Producer and the Showcase build: {built}"
