import pytest
import torch

from training import evalcore as E
from training.env import STANDARD
from training.loanprobe import ACCEPT, BID, LOAN, Rule

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")
NET = E.ROOT / "populations/psro/psro03.pt"
DIM = 32


def offered(*kinds, over_cash=None):
    """Feature rows of the offered actions, padded to 4: one-hot kind, and amount / cash as the last-but-one number."""
    acts = torch.zeros(4, DIM)
    for k, kind in enumerate(kinds):
        acts[k, kind] = 1.0
        if over_cash is not None and k < len(over_cash):
            acts[k, -2] = over_cash[k]
    mask = torch.tensor([k < len(kinds) for k in range(4)])
    return acts, mask


def tables():
    """Four envs: an all-in bid pick, a modest bid pick, a decision of the deliverer, and a main-phase turn."""
    rows = [offered(BID, BID, LOAN, over_cash=[0.2, 1.0]), offered(BID, BID, LOAN, over_cash=[0.2, 1.0]),
            offered(ACCEPT, 15, LOAN), offered(0, LOAN)]
    return torch.stack([a for a, _ in rows]), torch.stack([m for _, m in rows]), torch.tensor([1, 0, 0, 0])


def run(spec):
    acts, mask, pick = tables()
    rule = Rule(spec)
    after = rule(pick.clone(), torch.arange(4), mask, acts)
    return rule, after.tolist()


def test_an_all_in_rule_borrows_only_when_the_pick_is_the_bid_of_all_the_cash():
    rule, after = run("bid:allin")
    assert after == [2, 0, 0, 0]
    assert (rule.seen, rule.fired) == (2, 1)


def test_a_phase_rule_applies_in_its_phase_and_nowhere_else():
    rule, after = run("bid:any")
    assert after == [2, 2, 0, 0] and (rule.seen, rule.fired) == (2, 2)
    rule, after = run("decide:any")
    assert after == [1, 0, 2, 0] and (rule.seen, rule.fired) == (1, 1)


def test_a_rule_with_a_zero_chance_never_fires_and_a_bad_rule_is_refused():
    rule, after = run("bid:any:0")
    assert after == [1, 0, 0, 0] and (rule.seen, rule.fired) == (2, 0)
    for bad in ("main:any", "decide:allin", "bid:sometimes", "bid:any:0.5:2"):
        with pytest.raises(ValueError):
            Rule(bad)


@needs_cuda
@pytest.mark.skipif(not NET.exists(), reason="the PSRO nets are not checked in")
def test_a_forced_loan_is_played_through_the_whole_game():
    rule = Rule("bid:any")
    pol = [E.Policy("net", str(NET), "guarded", adjust=rule), E.Policy("plain", str(NET), "guarded")]
    specs = E.crossplay_specs(2, [3], seeds_per_cell=1, length=STANDARD)[:12]
    rec = E.play(specs, pol, envs=8, log=None, seed=1)
    assert len(rec) == len(specs) and not rec.stalled.any()
    assert rule.fired == rule.seen > 0
