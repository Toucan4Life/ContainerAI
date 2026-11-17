"""Turns a checkpoint into one that also reads the opponent memory: `python -m training.add_memory IN.pt OUT.pt`.

The new first-layer columns start at zero, so the net plays exactly as before until training moves them."""
import sys

import torch

from training.model import PolicyNet, load_checkpoint, save_checkpoint

MEM_DIM = 28  # container_bots::memory::MEM_DIM


def add_memory(net: PolicyNet) -> PolicyNet:
    state, action, hidden = net.dims
    out = PolicyNet(state + MEM_DIM, action, hidden)
    out.load_state_dict(net.state_dict() | {
        "t1.weight": torch.cat([net.t1.weight.data, torch.zeros(hidden, MEM_DIM)], dim=1)})
    return out


if __name__ == "__main__":
    save_checkpoint(add_memory(load_checkpoint(sys.argv[1])), sys.argv[2])
