import numpy as np
import pytest

from training import env as env_module
from training.env import (BID_MONEY, BID_VALUE, BIDS, DELIVERIES, DONE, INTERACTION, MERCHANT, MOVES, OVERBID, PENDING,
                          PRODUCES, RANDOM, SHIPPED, SIMPLE, SOLITAIRE, BatchEnv, move_names, win_fraction)
from training.move_table import bid_summary, delivery_summary, harbor_gap


def play_scripted(n=16, players=4, kinds=(MERCHANT, SIMPLE, RANDOM, SIMPLE), seed=1):
    env = BatchEnv(n, threads=4)
    for i in range(n):
        env.reset(i, players, seed + i, list(kinds))
    env.advance()
    return env


def test_scripted_games_finish():
    env = play_scripted()
    status, players, scores = env.results()
    assert (status == DONE).all()
    assert (players == 4).all()
    assert (scores[:, :4].sum(axis=1) != 0).any()


def test_solitaire_seats_finish_their_games():
    env = play_scripted(players=3, kinds=(SOLITAIRE, SOLITAIRE, MERCHANT))
    status, _, _ = env.results()
    assert (status == DONE).all()


def test_the_move_counters_name_every_column_and_add_up_to_the_actions():
    env = play_scripted(players=4, kinds=(MERCHANT, MERCHANT, SIMPLE, SIMPLE))
    names = move_names()
    assert len(names) == INTERACTION - MOVES and len(set(names)) == len(names)
    inter = env.interaction()
    actions = env.results_ex()[4]
    assert (inter[:, :, MOVES:].sum(axis=(1, 2)) == actions).all(), "scripted seats and forced moves count too"
    column = {name: MOVES + k for k, name in enumerate(names)}
    assert inter[:, :, column["produce"]].sum() > 0
    assert (inter[:, :, column["produce"]] == inter[:, :, PRODUCES]).all()
    assert (inter[:, :, column["sail_island"]] == inter[:, :, DELIVERIES]).all()


def test_a_library_with_another_counter_width_is_refused(monkeypatch):
    monkeypatch.setattr(env_module, "INTERACTION", INTERACTION + 1)
    with pytest.raises(RuntimeError, match="rebuild"):
        BatchEnv(1)


def test_the_harbor_gap_lists_the_nets_bound_by_the_rule_that_sailed_without_buying():
    def row(sails, taken):
        return 0.0, {}, {"sail_harbor": sails, "buy_harbor": taken}, {}, {}

    rows = {"even": row(100, 100), "short": row(100, 90), "unbound": row(100, 70)}
    bound = {"buy_at_harbor": 1}
    nets = {"even": ("e.pt", bound), "short": ("s.pt", bound), "unbound": ("u.pt", {})}
    text = harbor_gap(rows, nets)
    assert "short 10 of 100 (10.0%)" in text and "unbound 30" not in text and "zero for the other 1" in text
    assert "none" in harbor_gap({"even": rows["even"]}, nets)


def test_the_bid_summary_compares_bids_with_the_value_of_the_cargo_and_survives_no_bids():
    rows = np.zeros((2, INTERACTION), dtype=np.int32)
    rows[0, [BIDS, BID_MONEY, BID_VALUE, OVERBID]] = [3, 12, 6, 6]
    rows[1, [BIDS, BID_MONEY, BID_VALUE, OVERBID]] = [1, 0, 6, 0]
    assert bid_summary(rows) == {"per_game": 2.0, "bid": 3.0, "value": 3.0, "over": 0.5}
    assert bid_summary(np.zeros((1, INTERACTION), dtype=np.int32)) == {"per_game": 0.0, "bid": 0.0, "value": 0.0, "over": 0.0}


def test_the_delivery_summary_is_the_cargo_per_sail_to_the_island_and_survives_no_deliveries():
    rows = np.zeros((2, INTERACTION), dtype=np.int32)
    rows[0, [DELIVERIES, SHIPPED]] = [3, 5]
    rows[1, [DELIVERIES, SHIPPED]] = [1, 3]
    assert delivery_summary(rows) == {"per_game": 2.0, "cargo": 2.0}
    assert delivery_summary(np.zeros((1, INTERACTION), dtype=np.int32)) == {"per_game": 0.0, "cargo": 0.0}


def test_slots_get_decisions_and_can_finish_a_game():
    n = 8
    env = BatchEnv(n, threads=2)
    for i in range(n):
        env.reset(i, 3, 100 + i, [0, 0, SIMPLE])
    rng = np.random.default_rng(0)
    for _ in range(5000):
        env.advance()
        _, _, mask, slot, _ = env.observe()
        status, _, _ = env.results()
        if (status != PENDING).all():
            break
        assert (slot[status == PENDING] == 0).all()
        counts = mask.sum(axis=1)
        env.step((rng.random(n) * np.maximum(counts, 1)).astype(np.int32))
    status, _, scores = env.results()
    assert (status == DONE).all()


REQUEST_LOAN, REPAY_LOAN = 16, 17  # the action kinds, one-hot at the start of an action's features


def repays_offered_after_a_request(rule: bool, n=16) -> int:
    """Plays random games with every seat a slot; counts the decisions that offer a repayment right
    after the same seat chose to request a loan (a free action, so the seat is still in its turn)."""
    env = BatchEnv(n, threads=2)
    if rule:
        env.forbid_same_turn_repay(0)
    for i in range(n):
        env.reset(i, 3, 200 + i, [0, 0, 0])
    rng = np.random.default_rng(0)
    asked = np.zeros(n, dtype=bool)
    offered = 0
    for _ in range(3000):
        env.advance()
        _, acts, mask, slot, _ = env.observe()
        pending = slot >= 0
        if not pending.any():
            break
        offers = ((acts[:, :, REPAY_LOAN] > 0.5) & (mask > 0)).any(axis=1)
        offered += int((pending & asked & offers).sum())
        counts = mask.sum(axis=1)
        pick = (rng.random(n) * np.maximum(counts, 1)).astype(np.int32)
        asked = pending & (acts[np.arange(n), pick, REQUEST_LOAN] > 0.5)
        env.step(pick)
    return offered


def test_a_seat_that_may_not_repay_in_the_turn_it_borrowed_is_not_offered_to():
    assert repays_offered_after_a_request(rule=False) > 0
    assert repays_offered_after_a_request(rule=True) == 0


def test_teacher_returns_a_legal_index():
    env = BatchEnv(4, threads=1)
    for i in range(4):
        env.reset(i, 4, 7 + i, [0, MERCHANT, MERCHANT, MERCHANT])
    env.advance()
    _, _, mask, _, _ = env.observe()
    t = env.teacher()
    for i in range(4):
        assert 0 <= t[i] < mask[i].sum()


def test_win_fraction():
    w = win_fraction(np.array([5, 3, 3, 0, 0]), 4)
    assert w[0] == 1.0 and w[3] == 0.0 and w[1] == w[2] == (1 + 0.5 + 0) / 3


def test_compact_forward_equals_padded_forward():
    import torch

    from training import model
    from training.model import PolicyNet

    torch.manual_seed(0)
    net = PolicyNet(20, 7, 16)
    state, acts = torch.randn(6, 20), torch.randn(6, 12, 7)
    mask = torch.zeros(6, 12, dtype=torch.bool)
    for i, n in enumerate([1, 2, 3, 5, 12, 2]):
        mask[i, :n] = True
    old = model.COMPACT_BELOW
    try:
        model.COMPACT_BELOW = 0.0
        padded, v1 = net(state, acts, mask)
        model.COMPACT_BELOW = 1.1
        compact, v2 = net(state, acts, mask)
    finally:
        model.COMPACT_BELOW = old
    assert torch.allclose(padded, compact, atol=1e-5) and torch.allclose(v1, v2, atol=1e-6)
