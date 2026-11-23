import numpy as np
import pytest
import torch

from training import evalcore as E
from training import selfplay as sp
from training.add_memory import widen_inputs
from training.env import (BIDS, BOUGHT, CARGO_CASH, COMPETITIVE_COLORS, DELIVERIES, EARNED, INTERACTION, MOVES, OFFERED,
                          OFFERED_COMPETITIVE, OVERBID, PRODUCE_CHANCES, PRODUCED, SHIPPED, SOLD, SOLD_COLOR, SPENT, STANDARD,
                          BatchEnv, move_names)
from training.model import PolicyNet, save_checkpoint
from training.strategy_metrics import configure_table, seat_metrics
from training.train import resolve_shaping

needs_cuda = pytest.mark.skipif(not torch.cuda.is_available(), reason="the rollout graphs need CUDA")


def step(mu, win, base, slack=0.05):
    return sp.dual_step(mu, win, base, slack, lr=0.05, ceiling=0.6, warmup=2)


def test_the_controller_only_measures_during_warmup():
    base = []
    assert step(0.3, 0.30, base) == 0.3 and step(0.3, 0.34, base) == 0.3
    assert base == [0.30, 0.34]


def test_a_violated_floor_cuts_the_coefficient_and_slack_raises_it():
    base = [0.30, 0.30]  # floor 0.25
    assert step(0.3, 0.20, base) < 0.3
    assert step(0.3, 0.40, base) > 0.3
    assert step(0.3, 0.25, base) == pytest.approx(0.3)


def test_the_coefficient_stays_within_zero_and_the_ceiling():
    base = [0.30, 0.30]
    assert step(0.01, 0.0, base) == 0.0
    assert step(0.59, 1.0, base) == 0.6


def test_every_objective_is_a_share_and_pays_what_it_names():
    inter = np.zeros((3, INTERACTION), dtype=np.int32)
    island = np.array([40, 0, 0])
    for o in sp.OBJECTIVES:
        assert 0.0 <= sp.shaping_term(o, inter, island, 0) <= 1.0
    assert sp.shaping_term("island", inter, island, 0) == pytest.approx(0.5)


def test_shipper_pays_the_deliverers_proceeds_not_the_bidders_island_points():
    inter = np.zeros((3, INTERACTION), dtype=np.int32)
    island = np.array([80, 0, 0])
    assert sp.shaping_term("shipper", inter, island, 0) == 0.0  # island points of the bidder side pay nothing
    inter[0, [CARGO_CASH + 2, DELIVERIES, SHIPPED]] = (60, 1, 3)
    full = sp.shaping_term("shipper", inter, None, 0)
    inter[0, SPENT] = 60  # buying the cargo at what it fetched leaves no net contribution
    assert sp.shaping_term("shipper", inter, None, 0) < full
    assert sp.shaping_term("shipper", inter, None, 0) == pytest.approx(
        0.3 * np.tanh(3 / sp.DELIVERED_SCALE) + 0.2 * 0.6)


def test_producer_pays_market_position_not_production():
    inter = np.zeros((3, INTERACTION), dtype=np.int32)
    inter[0, [PRODUCED, PRODUCE_CHANCES]] = (30, 10)  # production alone pays nothing
    assert sp.shaping_term("producer", inter, None, 0) == 0.0
    inter[0, [SOLD, EARNED, OFFERED, OFFERED_COMPETITIVE]] = (10, 20, 20, 20)
    assert sp.shaping_term("producer", inter, None, 0) == pytest.approx(1.0)  # all the table's sales
    inter[1, [SOLD, EARNED]] = (10, 20)
    assert sp.shaping_term("producer", inter, None, 0) == pytest.approx(0.7 + 0.3)  # half of sales still earns the full share


def test_producer_term_does_not_care_where_the_stock_came_from():
    inter = np.zeros((2, INTERACTION), dtype=np.int32)
    inter[0, [SOLD, EARNED]] = (4, 8)
    inter[1, [SOLD, EARNED]] = (12, 24)
    own = sp.shaping_term("producer", inter, None, 0)
    inter[0, [PRODUCED, BOUGHT]] = (0, 4)  # the same sales made of containers bought from a rival
    assert sp.shaping_term("producer", inter, None, 0) == own


def test_showcase_pays_the_colours_buyers_took():
    inter = np.zeros((2, INTERACTION), dtype=np.int32)
    inter[0, COMPETITIVE_COLORS] = 50  # offering colours nobody buys pays nothing
    assert sp.shaping_term("showcase", inter, None, 0) == 0.0
    inter[0, SOLD_COLOR:SOLD_COLOR + 5] = 1
    assert sp.shaping_term("showcase", inter, None, 0) == pytest.approx(1.0)


def test_the_shaping_term_replaces_the_interaction_term_in_the_reward():
    scores = np.array([30, 10, 5])
    inter = np.zeros((3, INTERACTION), dtype=np.int32)
    inter[0, SOLD_COLOR:SOLD_COLOR + 5] = 1
    plain = sp.reward(scores, 3, inter_weight=0.5, inter=inter, island=np.zeros(3), shaping="interaction")
    show = sp.reward(scores, 3, inter_weight=0.5, inter=inter, island=np.zeros(3), shaping="showcase")
    assert show[0] == pytest.approx(plain[0] + 0.5 * 1.0)


def test_metrics_are_ratios_of_sums():
    rows = np.zeros((2, INTERACTION), dtype=np.int32)
    rows[0, [PRODUCE_CHANCES, PRODUCED]] = (10, 20)
    rows[1, [PRODUCE_CHANCES, PRODUCED]] = (30, 20)
    assert seat_metrics(rows)["produced_per_opp"] == pytest.approx(1.0)
    assert seat_metrics(np.zeros(INTERACTION))["sell_through"] == 0.0


def test_a_shaping_objective_needs_a_starting_share_and_the_old_flag_is_the_interaction_objective():
    assert resolve_shaping(None, None, 0.0) == (None, 0.0)
    assert resolve_shaping(None, None, 0.4) == ("interaction", 0.4)
    assert resolve_shaping("producer", 0.2, 0.4) == ("producer", 0.2)
    assert resolve_shaping("island", None, 0.4) == ("island", 0.4)
    with pytest.raises(SystemExit):
        resolve_shaping("showcase", None, 0.0)


class Recorder:
    """An env that only records the calls that bind rules."""

    def __init__(self):
        self.calls = []

    def __getattr__(self, name):
        return lambda *args: self.calls.append((name, *args))


def test_the_net_with_most_rules_owns_them_and_a_second_net_with_rules_mirrors_it():
    peddler = {"min_bid": "3", "buy_at_harbor": ""}
    env = Recorder()
    configure_table(env, [(10, {}), (11, {"min_bid": 3, "buy_at_harbor": 1}), (12, peddler | {"bid_within_value": ""})])
    assert ("require_bid_within_value", 12) in env.calls and ("mirror_rules", 11, 12) in env.calls
    assert {c[1] for c in env.calls if c[0] != "mirror_rules"} == {12}  # nothing is bound to the mirrored or the plain net
    env = Recorder()
    configure_table(env, [(10, {}), (11, {})])
    assert env.calls == []


def test_rules_that_the_engine_cannot_share_at_one_table_are_refused():
    with pytest.raises(ValueError):
        configure_table(Recorder(), [(1, {"min_bid": "3"}), (2, {"min_bid": "4"})])
    with pytest.raises(ValueError):
        configure_table(Recorder(), [(1000, {"min_bid": "3"}), (1001, {"min_bid": "3"}), (1002, {"min_bid": "4"})])


def test_several_nets_with_the_same_rules_all_mirror_the_owner():
    env = Recorder()
    same = {"min_bid": 3, "buy_at_harbor": 1}
    configure_table(env, [(1000, same), (1001, {}), (1002, same), (1003, same)])
    assert ("mirror_rules_set", 0b1100, 1000) in env.calls  # slots 1002 and 1003; the plain net 1001 is left alone


@needs_cuda
def test_every_net_at_a_table_plays_under_its_rules(tmp_path):
    # With buy_at_harbor, every sail to a harbor ends in a purchase there. Binding the rules of one net after
    # another left only the last net bound, so the others declined at the harbor.
    env = BatchEnv(1, 1)
    pol = []
    for k in range(2):
        torch.manual_seed(k)
        path = str(tmp_path / f"net{k}.pt")
        save_checkpoint(PolicyNet(env.state_dim, env.action_dim, 32), path)
        pol.append(E.Policy(f"net{k}", path, "sample"))
    specs = [E.GameSpec(3, STANDARD, E.game_seed(E.AUDIT, 3, STANDARD, i), seats, 0, i)
             for i in range(24) for seats in ((0, 1, 0), (1, 0, 1))]
    names = move_names()
    rules = {"buy_at_harbor": ""}

    def harbors(bound):
        rec = E.play(specs, pol, envs=16, log=None, seed=1,
                     configure=lambda e: configure_table(e, [(E.SLOT_BASE + k, rules) for k in bound]))
        counts = [rec.interaction[rec.seats == k][:, [MOVES + names.index(m) for m in ("sail_harbor", "buy_harbor")]].sum(0)
                  for k in range(2)]
        return [(int(sails), int(buys)) for sails, buys in counts]

    for sails, buys in harbors([0, 1]):
        assert 0 < sails <= buys
    (sails, buys), _ = harbors([1])
    assert sails > buys  # unbound, the same net does decline: the check can fail


@needs_cuda
def test_a_net_bound_to_the_value_of_the_cargo_never_bids_above_it(tmp_path):
    env = BatchEnv(1, 1)
    pol = []
    for k in range(2):
        torch.manual_seed(k)
        path = str(tmp_path / f"net{k}.pt")
        save_checkpoint(PolicyNet(env.state_dim, env.action_dim, 32), path)
        pol.append(E.Policy(f"net{k}", path, "sample"))
    specs = [E.GameSpec(3, STANDARD, E.game_seed(E.AUDIT, 3, STANDARD, i), seats, 0, i)
             for i in range(24) for seats in ((0, 1, 0), (1, 0, 1))]
    rules = {"min_bid": "3", "bid_within_value": ""}
    rec = E.play(specs, pol, envs=16, log=None, seed=1, configure=lambda e: configure_table(e, [(E.SLOT_BASE, rules)]))
    bids, over = ([int(rec.interaction[rec.seats == k][:, column].sum()) for k in range(2)] for column in (BIDS, OVERBID))
    assert bids[0] > 0 and over[0] == 0
    assert over[1] > 0  # unbound, the same kind of net does bid above the allowance: the check can fail


def test_widening_the_inputs_keeps_the_policy():
    net = PolicyNet(12, 5, 16)
    wide = widen_inputs(net, 7)
    assert wide.dims == (19, 5, 16)
    state, acts, mask = torch.randn(4, 12), torch.randn(4, 3, 5), torch.ones(4, 3, dtype=torch.bool)
    pad = torch.randn(4, 7)
    for a, b in zip(net(state, acts, mask), wide(torch.cat([state, pad], 1), acts, mask)):
        assert torch.allclose(a, b, atol=1e-6)
