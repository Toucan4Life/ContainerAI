"""Turns a checkpoint into one that also reads the opponent memory: `python -m training.add_memory IN.pt OUT.pt`.

The new first-layer columns start at zero, so the net plays exactly as before until training moves them."""
import sys

import torch

from training.model import PolicyNet, load_checkpoint, save_checkpoint

MEM_DIM = 28  # container_bots::memory::MEM_DIM


def widen_inputs(net: PolicyNet, extra: int) -> PolicyNet:
    """The same net reading `extra` more state inputs, whose first-layer columns are zero."""
    state, action, hidden = net.dims
    out = PolicyNet(state + extra, action, hidden)
    out.load_state_dict(net.state_dict() | {
        "t1.weight": torch.cat([net.t1.weight.data, torch.zeros(hidden, extra, device=net.t1.weight.device)], dim=1)})
    return out


def add_memory(net: PolicyNet) -> PolicyNet:
    return widen_inputs(net, MEM_DIM)


if __name__ == "__main__":
    save_checkpoint(add_memory(load_checkpoint(sys.argv[1])), sys.argv[2])
