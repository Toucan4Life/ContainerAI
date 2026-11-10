"""The policy/value network.

An action is scored from the state and the action's own features, so one net handles every phase
and any number of legal actions:

    h      = trunk(state)                      # two ReLU layers
    logit  = head(relu(Ws(h) + Wa(action)))    # one scalar per legal action
    value  = value_head(h)                     # expected outcome in 0..1 for the seat to move

The layers are all small matrix products, so the same net runs in pure Rust in the browser.
"""

from __future__ import annotations

import struct

import numpy as np
import torch
from torch import nn
from torch.nn import functional as F

MAGIC = b"CNTW"
COMPACT_BELOW = 0.6  # share of real actions under which padding is skipped


class PolicyNet(nn.Module):
    def __init__(self, state_dim: int, action_dim: int, hidden: int = 256):
        super().__init__()
        self.dims = (state_dim, action_dim, hidden)
        self.t1 = nn.Linear(state_dim, hidden)
        self.t2 = nn.Linear(hidden, hidden)
        self.ws = nn.Linear(hidden, hidden)
        self.wa = nn.Linear(action_dim, hidden)
        self.h1 = nn.Linear(hidden, hidden)
        self.h2 = nn.Linear(hidden, 1)
        self.v1 = nn.Linear(hidden, hidden // 2)
        self.v2 = nn.Linear(hidden // 2, 1)

    def forward(self, state, acts, mask, dense=False):
        """state [B,S], acts [B,A,D], mask [B,A] (bool). Returns logits [B,A] and value [B].

        Batches are padded to their widest decision, so when most slots are padding only the real
        actions go through the action layers; the result is the same. `dense` forces the padded path,
        which has static shapes and so can be captured in a CUDA graph.
        """
        h = F.relu(self.t2(F.relu(self.t1(state))))
        value = self.v2(F.relu(self.v1(h))).squeeze(-1)
        if not dense and mask.shape[1] > 1 and mask.float().mean() < COMPACT_BELOW:
            row, col = mask.nonzero(as_tuple=True)
            z = F.relu(self.ws(h)[row] + self.wa(acts[row, col]))
            flat = self.h2(F.relu(self.h1(z))).squeeze(-1)
            logits = torch.full(mask.shape, -1e9, dtype=flat.dtype, device=flat.device)
            return logits.index_put((row, col), flat), value
        z = F.relu(self.ws(h).unsqueeze(1) + self.wa(acts))
        logits = self.h2(F.relu(self.h1(z))).squeeze(-1)
        return logits.masked_fill(~mask, -1e9), value


LAYERS = ["t1", "t2", "ws", "wa", "h1", "h2", "v1", "v2"]


def export_weights(net: PolicyNet, path: str) -> None:
    """Writes the net as a flat little-endian file the Rust bot reads.

    Layout: magic, u32 state_dim, action_dim, hidden, then per layer in LAYERS order its weight
    matrix (out x in, row-major) and its bias, all f32.
    """
    with open(path, "wb") as f:
        f.write(MAGIC)
        f.write(struct.pack("<3I", *net.dims))
        for name in LAYERS:
            layer = getattr(net, name)
            f.write(layer.weight.detach().cpu().numpy().astype("<f4").tobytes())
            f.write(layer.bias.detach().cpu().numpy().astype("<f4").tobytes())


def load_checkpoint(path: str, device: str = "cpu") -> PolicyNet:
    blob = torch.load(path, map_location=device, weights_only=True)
    net = PolicyNet(*blob["dims"]).to(device)
    net.load_state_dict(blob["state"])
    return net


def save_checkpoint(net: PolicyNet, path: str) -> None:
    torch.save({"dims": net.dims, "state": net.state_dict()}, path)


def numpy_forward(weights: dict[str, tuple[np.ndarray, np.ndarray]], state, acts) -> tuple[np.ndarray, float]:
    """Reference forward pass in numpy for one decision, mirroring the Rust code."""

    def lin(name, x):
        w, b = weights[name]
        return x @ w.T + b

    relu = lambda x: np.maximum(x, 0)
    h = relu(lin("t2", relu(lin("t1", state))))
    z = relu(lin("ws", h)[None, :] + lin("wa", acts))
    logits = lin("h2", relu(lin("h1", z)))[:, 0]
    value = float(lin("v2", relu(lin("v1", h)))[0])
    return logits, value
