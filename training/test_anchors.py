import json

import numpy as np
import pytest
import torch

from training import anchors
from training.env import BatchEnv
from training.model import PolicyNet, save_checkpoint

pytestmark = pytest.mark.skipif(not torch.cuda.is_available(), reason="needs CUDA")


def test_a_net_is_rated_against_scripted_bots_and_anchor_nets(tmp_path, monkeypatch):
    monkeypatch.setattr(anchors, "DIR", tmp_path / "anchors")
    ladder_file = tmp_path / "ladder.json"
    ladder_file.write_text(json.dumps({"merchant": 932.0, "random": 0.0, "simple": 262.6}))
    monkeypatch.setattr(anchors, "LADDER", ladder_file)
    torch.manual_seed(0)
    env = BatchEnv(8, 1)
    first = PolicyNet(env.state_dim, env.action_dim, 16)
    save_checkpoint(first, str(tmp_path / "a.pt"))

    res = anchors.add("alpha", str(tmp_path / "a.pt"), games=48, threads=2)
    ladder = json.loads(ladder_file.read_text())
    assert ladder["alpha"] == res["elo"] and (tmp_path / "anchors" / "alpha.pt").exists()
    assert anchors.net_names() == ["alpha"]

    second = PolicyNet(env.state_dim, env.action_dim, 16)
    save_checkpoint(second, str(tmp_path / "b.pt"))
    res2 = anchors.add("beta", str(tmp_path / "b.pt"), games=64, threads=2)
    assert "vs_alpha" in res2, "the new net must have met the anchor net"
    assert set(json.loads(ladder_file.read_text())) == {"merchant", "random", "simple", "alpha", "beta"}


def test_evaluation_without_nets_matches_the_scripted_ladder():
    env = BatchEnv(8, 1)
    net = PolicyNet(env.state_dim, env.action_dim, 16).cuda().eval()
    res = anchors.evaluate(env, net, "cuda", np.random.default_rng(1), 24,
                           {"merchant": 932.0, "random": 0.0, "simple": 262.6})
    assert res["games"] == 24 and "vs_merchant" in res
