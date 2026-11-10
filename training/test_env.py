import numpy as np

from training.env import DONE, MERCHANT, PENDING, RANDOM, SIMPLE, BatchEnv, win_fraction


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
