import copy

import numpy as np
import pytest
import torch

from training.env import MERCHANT, SIMPLE, BatchEnv
from training.model import PolicyNet
from training.selfplay import LEARNER, Roller, advantages

pytestmark = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")


def make(n=32):
    torch.manual_seed(0)
    env = BatchEnv(n, 4)
    net = PolicyNet(env.state_dim, env.action_dim, 32).cuda().eval()
    rng = np.random.default_rng(0)

    def table():
        players = int(rng.integers(3, 6))
        kinds = [LEARNER] + [int(rng.choice([MERCHANT, SIMPLE, LEARNER, 1])) for _ in range(players - 1)]
        return players, kinds, int(rng.integers(1, 1 << 30))

    return env, net, table, rng


def test_recorded_logp_matches_a_fresh_forward_pass():
    env, net, table, rng = make()
    frozen = {1: copy.deepcopy(net)}
    roller = Roller(env, table, "cuda", rng)
    rec, status, players, scores, tables = roller.collect({0: net, **frozen}, target=16)
    data, _, _ = rec.tensors()
    logits, value = net(data["state"], data["acts"].float(), data["mask"])
    logp = torch.log_softmax(logits, dim=1).gather(1, data["action"][:, None]).squeeze(1)
    assert len(status) == 16 and len(data["action"]) > 0
    assert torch.allclose(logp, data["logp"], atol=1e-3)
    assert torch.allclose(value, data["value"], atol=1e-3)
    assert bool(data["mask"].gather(1, data["action"][:, None]).all()), "an illegal slot was chosen"


def test_games_carry_over_and_every_decision_belongs_to_one_finished_game():
    env, net, table, rng = make()
    roller = Roller(env, table, "cuda", rng)
    seen = 0
    for _ in range(3):
        rec, status, players, scores, tables = roller.collect({0: net, 1: net}, target=10)
        ids = np.concatenate(rec.env)
        assert ids.min() >= 0 and ids.max() < len(status)
        adv, ret, keep = advantages(rec, ids, np.concatenate(rec.seat), status, players, scores)
        assert np.isfinite(adv).all()
        seen += len(status)
    assert seen >= 30 and roller.running() == env.n


def test_greedy_evaluation_plays_exactly_the_requested_games():
    env, net, table, rng = make()
    roller = Roller(env, table, "cuda", rng, limit=40, record=False)
    _, status, _, _, tables = roller.collect({0: net, 1: net}, mode="greedy")
    assert len(status) == 40 and roller.running() == 0


def test_sampling_follows_the_policy():
    env, net, table, rng = make(8)
    roller = Roller(env, table, "cuda", rng)
    fw = roller.forward
    state = np.random.default_rng(1).normal(size=(8, env.state_dim)).astype(np.float32)
    acts = np.random.default_rng(2).normal(size=(8, 48, env.action_dim)).astype(np.float32)
    mask = np.zeros((8, 48), dtype=np.uint8)
    mask[:, :5] = 1
    w = fw.load(state, acts, mask, 5)
    logits, _, _, _ = fw.run(net, w)
    probs = torch.softmax(logits[0], dim=0).cpu().numpy()
    counts = np.zeros(w)
    for _ in range(4000):
        counts[int(fw.run(net, w)[2][0])] += 1
    assert np.abs(counts / 4000 - probs).max() < 0.04


def test_stalled_games_are_penalised_not_dropped():
    from training.env import STALLED
    from training.selfplay import STALL_REWARD, Decisions

    rec = Decisions()
    z = torch.zeros(3)
    rec.add(torch.zeros(3, 2), torch.zeros(3, 1, 1), torch.ones(3, 1, dtype=torch.bool), z.long(), z, z, z.long(),
            np.array([0, 0, 1]), np.array([0, 0, 0]))
    status = np.array([STALLED, 2])
    scores = np.array([[0, 0, 0, 0, 0], [30, 10, 5, 0, 0]])
    adv, ret, keep = advantages(rec, np.array([0, 0, 1]), np.array([0, 0, 0]), status, np.array([3, 3]), scores)
    assert keep.all() and np.allclose(ret[:2], STALL_REWARD) and ret[2] > 0.5
