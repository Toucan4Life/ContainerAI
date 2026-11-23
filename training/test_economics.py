import numpy as np

from training import env as V
from training.economics import cargo_table


def test_cargo_table_reports_cash_per_trip_and_container_by_size():
    names = V.move_names()
    rows = np.zeros((2, V.INTERACTION))
    rows[:, V.CARGO_SIZE + 2] = [1, 0]  # one 3-container trip
    rows[:, V.CARGO_CASH + 2] = [12, 0]
    rows[:, V.CARGO_SIZE] = [0, 2]  # two single trips
    rows[:, V.CARGO_CASH] = [0, 10]
    text = cargo_table("x", rows, names)
    assert "| 3 | 1 (few) | 33.3% | $12.00 | $4.00 |" in text
    assert "| 1 | 2 (few) | 66.7% | $5.00 | $5.00 |" in text
