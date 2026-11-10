"""Writes the fixture that checks the Rust net against the numpy reference of the same weights.

    python -m training.make_fixture
"""

from __future__ import annotations

import struct
from pathlib import Path

import numpy as np
import torch

from training.env import MERCHANT, BatchEnv
from training.model import LAYERS, PolicyNet, export_weights, numpy_forward

OUT = Path(__file__).resolve().parent.parent / "crates/container-bots/tests/fixtures"


def read_weights(path: str) -> dict[str, tuple[np.ndarray, np.ndarray]]:
    raw = open(path, "rb").read()
    s, a, h = struct.unpack("<3I", raw[4:16])
    shapes = {"t1": (h, s), "t2": (h, h), "ws": (h, h), "wa": (h, a), "h1": (h, h),
              "h2": (1, h), "v1": (h // 2, h), "v2": (1, h // 2)}
    pos, out = 16, {}
    for name in LAYERS:
        o, i = shapes[name]
        w = np.frombuffer(raw, "<f4", o * i, pos).reshape(o, i)
        pos += 4 * o * i
        b = np.frombuffer(raw, "<f4", o, pos)
        pos += 4 * o
        out[name] = (w.astype(np.float64), b.astype(np.float64))
    assert pos == len(raw)
    return out


def main() -> None:
    torch.manual_seed(3)
    env = BatchEnv(4, threads=1)
    for i in range(4):
        env.reset(i, 4, 11 + i, [0, MERCHANT, MERCHANT, MERCHANT])
    env.advance()
    state, acts, mask, slot, _ = env.observe()
    net = PolicyNet(env.state_dim, env.action_dim, 8)
    OUT.mkdir(parents=True, exist_ok=True)
    export_weights(net, str(OUT / "parity.bin"))
    weights = read_weights(str(OUT / "parity.bin"))

    # The numpy reference must agree with torch before it is trusted as the Rust reference.
    n = int(mask[0].sum())
    s, a = state[0].astype(np.float64), acts[0, :n].astype(np.float64)
    logits, value = numpy_forward(weights, s, a)
    with torch.no_grad():
        tl, tv = net(torch.tensor(state[:1]), torch.tensor(acts[:1, :n]), torch.ones(1, n, dtype=torch.bool))
    assert np.allclose(logits, tl[0].numpy(), atol=1e-4) and abs(value - tv.item()) < 1e-4

    fmt = lambda x: " ".join(f"{v:.9g}" for v in np.asarray(x).ravel())
    lines = [str(n), fmt(state[0])] + [fmt(row) for row in acts[0, :n]] + [fmt(logits), fmt(value)]
    (OUT / "parity.txt").write_text("\n".join(lines) + "\n")
    print("fixture written:", n, "actions")


if __name__ == "__main__":
    main()
