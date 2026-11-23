"""Trains a neural opponent within a time budget.

    python -m training.train --name first --budget-minutes 30

Stage 1 copies the merchant (imitation with DAgger-style relabelling), stage 2 improves the net by
league self-play with PPO: tables mix the learner with itself, scripted bots, frozen earlier
checkpoints, the anchor nets picked by `--league-nets` (Consortium), the frozen Peddler v2 of `--peddler` and, with
`--heuristic-share`, the Producer, Showcase and Shipper heuristics. Everything lands in `runs/<name>/`: `metrics.jsonl` (what the dashboard reads),
`status.json`, `ckpt_*.pt`, and at the end `final.pt` and `weights.bin` for the Rust bot.
"""

from __future__ import annotations

import argparse
import copy
import json
import os
import time
from pathlib import Path

os.environ.setdefault("PYTORCH_CUDA_ALLOC_CONF", "expandable_segments:True")  # rollouts vary in size

import numpy as np  # noqa: E402
import torch  # noqa: E402
from torch.nn import functional as F

from training import anchors as anchor_lib
from training import ladder
from training.env import BOUGHT, CARGO_CASH, DONE, MERCHANT, PLACED_AT, PRODUCER, RANDOM, REAL_BIDS, SHIPPER, SHOWCASE, SIMPLE, SOLD, SPENT, STALLED, TRADER, BARGAIN, DISCOUNTER, SOLITAIRE, BatchEnv
from training.model import PolicyNet, export_weights, save_checkpoint
from training.selfplay import (LEARNER, OBJECTIVES as SHAPING_OBJECTIVES, SHOWCASE_VARIANTS, STALL_REWARD, Roller, advantages,
                               dual_step, reward, table_stats, win_fraction)
from training.strategy_metrics import seat_metrics

SCRIPTED = [MERCHANT, SIMPLE, RANDOM]
HEURISTICS = [PRODUCER, SHOWCASE, SHIPPER]  # the Phase H0 bots, seated by `--heuristic-share`
KIND_NAME = {MERCHANT: "merchant", SIMPLE: "simple", RANDOM: "random", TRADER: "trader",
             DISCOUNTER: "discounter", BARGAIN: "bargain", SOLITAIRE: "solitaire"}
PEDDLER_SLOT = 900  # the frozen copy of a shipped net, clear of the snapshots (1..) and the anchor nets (1000..)
LEAGUE_PEDDLER = Path(__file__).with_name("league") / "peddler2.pt"  # Peddler v2, the shipped net `--peddler` seats by default
# The learner's rules that Peddler v2 was trained and is shipped with; its seat plays by whichever of them the learner has.
PEDDLER_RULES = ("--no-same-turn-repay", "--min-bid 3", "--buy-at-harbor", "--bank-needs-load", "--no-double-reprice")


class Run:
    def __init__(self, root: Path, budget: float):
        self.dir = root
        self.dir.mkdir(parents=True, exist_ok=True)
        self.t0 = time.monotonic()
        self.budget = budget
        self.metrics = open(self.dir / "metrics.jsonl", "a", buffering=1)

    def elapsed(self) -> float:
        return time.monotonic() - self.t0

    def log(self, **row) -> None:
        row["t"] = round(self.elapsed(), 2)
        self.metrics.write(json.dumps(row) + "\n")

    def status(self, state: str, stage: str, **extra) -> None:
        body = dict(state=state, stage=stage, elapsed=round(self.elapsed(), 1), budget=self.budget, **extra)
        (self.dir / "status.json").write_text(json.dumps(body))


seed_of = anchor_lib.seed_of


def imitation_table(rng):
    players = int(rng.choice([3, 4, 5]))
    if rng.random() < 0.6:
        kinds = [LEARNER] * players
    else:
        kinds = [int(rng.choice(SCRIPTED)) for _ in range(players)]
        for s in rng.choice(players, size=int(rng.integers(1, 3)), replace=False):
            kinds[s] = LEARNER
    return players, kinds, seed_of(rng)


def league_table(rng, frozen: list[int], scripted_share: float = 0.55, trader_share: float = 0.0,
                 seller_share: float = 0.0, solitaire_share: float = 0.0, merchant_share: float = 0.0,
                 peddler_share: float = 0.0, heuristic_share: float = 0.0):
    players = int(rng.choice([3, 4, 5]))
    if rng.random() < 0.25:
        return players, [LEARNER] * players, seed_of(rng)
    learners = rng.choice(players, size=int(rng.integers(1, players)), replace=False)
    kinds = []
    for s in range(players):
        if s in learners:
            kinds.append(LEARNER)
        elif rng.random() < peddler_share:
            kinds.append(PEDDLER_SLOT)
        elif heuristic_share and rng.random() < heuristic_share:  # no draw when off, so older runs keep their tables
            kinds.append(int(rng.choice(HEURISTICS)))
        elif rng.random() < solitaire_share:
            kinds.append(SOLITAIRE)
        elif rng.random() < merchant_share:
            kinds.append(MERCHANT)
        elif frozen and rng.random() >= scripted_share:
            kinds.append(int(rng.choice(frozen)))
        elif rng.random() < seller_share:
            kinds.append(int(rng.choice([DISCOUNTER, BARGAIN])))
        elif rng.random() < trader_share:
            kinds.append(TRADER)
        else:
            kinds.append(int(rng.choice(SCRIPTED, p=[0.6, 0.3, 0.1])))
    return players, kinds, seed_of(rng)


def league_slots(anchor_nets: dict, chosen: str) -> dict:
    """The anchor nets that are training opponents, by slot. `chosen` is `all`, `none` or comma-separated names.

    Every anchor stays on the ladder the evaluations rate against; this only picks whom the learner trains against.
    A net keeps the slot `NET_SLOT_BASE` plus its place among all the anchors, chosen or not.
    """
    names = list(anchor_nets)
    if chosen == "all":
        picked = names
    elif chosen in ("", "none"):
        picked = []
    else:
        picked = [n.strip() for n in chosen.split(",") if n.strip()]
        unknown = [n for n in picked if n not in anchor_nets]
        if unknown:
            raise SystemExit(f"--league-nets: {unknown} are not rated anchor nets; the anchors are {names}, or use 'all' or 'none'")
    return {anchor_lib.NET_SLOT_BASE + names.index(n): anchor_nets[n] for n in picked}


def missing_peddler_rules(args) -> list[str]:
    """The `PEDDLER_RULES` flags the learner does not set, which its Peddler seat then plays without."""
    have = (args.no_same_turn_repay, args.min_bid == 3, args.buy_at_harbor, args.bank_needs_load, args.no_double_reprice)
    return [flag for flag, on in zip(PEDDLER_RULES, have) if not on]


def resolve_shaping(objective, weight, interaction_weight):
    """`(objective, starting share)` of the shaping reward; `--interaction-weight` alone means the interaction objective."""
    if objective is None:
        return ("interaction", interaction_weight) if interaction_weight else (None, 0.0)
    share = interaction_weight if weight is None else weight
    if not share:
        raise SystemExit("--shaping-objective needs a starting --shaping-weight above 0")
    return objective, share


def ppo_update(net, opt, data, adv, ret, epochs=4, minibatch=8192, clip=0.2, ent_coef=0.01, vf_coef=0.5):
    n = len(adv)
    adv_t = torch.from_numpy(adv).float().to(data["state"].device)
    ret_t = torch.from_numpy(ret).float().to(data["state"].device)
    adv_t = (adv_t - adv_t.mean()) / (adv_t.std() + 1e-8)
    stats = dict(loss_pi=0.0, loss_v=0.0, entropy=0.0, kl=0.0)
    steps = 0
    for _ in range(epochs):
        perm = torch.randperm(n, device=adv_t.device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            logits, value = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            logp_all = torch.log_softmax(logits, dim=1)
            logp = logp_all.gather(1, data["action"][ix, None]).squeeze(1)
            ratio = torch.exp(logp - data["logp"][ix])
            a = adv_t[ix]
            pi_loss = -torch.min(ratio * a, torch.clamp(ratio, 1 - clip, 1 + clip) * a).mean()
            v_loss = F.mse_loss(value, ret_t[ix])
            entropy = -(logp_all.exp() * logp_all).sum(dim=1).mean()
            loss = pi_loss + vf_coef * v_loss - ent_coef * entropy
            opt.zero_grad(set_to_none=True)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(net.parameters(), 0.5)
            opt.step()
            stats["loss_pi"] += pi_loss.item()
            stats["loss_v"] += v_loss.item()
            stats["entropy"] += entropy.item()
            stats["kl"] += (data["logp"][ix] - logp).mean().item()
            steps += 1
    return {k: round(v / steps, 5) for k, v in stats.items()}


def imitate(net, opt, data, epochs=2, minibatch=4096):
    n = len(data["state"])
    total, correct, count = 0.0, 0, 0
    for _ in range(epochs):
        perm = torch.randperm(n, device=data["state"].device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            logits, _ = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            loss = F.cross_entropy(logits, data["label"][ix])
            opt.zero_grad(set_to_none=True)
            loss.backward()
            torch.nn.utils.clip_grad_norm_(net.parameters(), 1.0)
            opt.step()
            total += loss.item() * len(ix)
            correct += (logits.argmax(dim=1) == data["label"][ix]).sum().item()
            count += len(ix)
    return {"loss_ce": round(total / count, 5), "accuracy": round(correct / count, 4)}


def value_fit(net, data, ret, opt, epochs=1, minibatch=8192):
    """Teaches the value head from game outcomes while the policy is still being imitated."""
    ret_t = torch.from_numpy(ret).float().to(data["state"].device)
    n = len(ret_t)
    for _ in range(epochs):
        perm = torch.randperm(n, device=ret_t.device)
        for start in range(0, n, minibatch):
            ix = perm[start:start + minibatch]
            mask = data["mask"][ix]
            width = int(mask.sum(dim=1).max())
            _, value = net(data["state"][ix], data["acts"][ix, :width].float(), mask[:, :width])
            loss = F.mse_loss(value, ret_t[ix])
            opt.zero_grad(set_to_none=True)
            loss.backward()
            opt.step()


def outcome_returns(rec, status, players, scores, welfare_weight=0.0, win_only=False, geomean_weight=0.0,
                    trade_weight=0.0, traded=None, min_trade=0.0, penalty=0.0, island_weight=0.0, island=None, inter_weight=0.0, inter=None, shaping="interaction"):
    """Return of every learner decision: the seat's final reward (no bootstrapping)."""
    env_ids = np.concatenate(rec.env)
    seats = np.concatenate(rec.seat)
    keep = np.array([status[e] in (DONE, STALLED) for e in env_ids])
    ret = np.zeros(len(env_ids))
    cache = {}
    for k, (e, s) in enumerate(zip(env_ids, seats)):
        if keep[k]:
            if status[e] == STALLED:
                ret[k] = STALL_REWARD
                continue
            if e not in cache:
                cache[e] = reward(scores[e], int(players[e]), welfare_weight, win_only, geomean_weight, trade_weight,
                                 None if traded is None else traded[e], min_trade, penalty,
                                 island_weight, None if island is None else island[e],
                                 inter_weight, None if inter is None else inter[e], shaping)
            ret[k] = cache[e][s]
    return ret, keep


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--name", default=time.strftime("run-%Y%m%d-%H%M%S"))
    ap.add_argument("--budget-minutes", type=float, default=30.0)
    ap.add_argument("--hidden", type=int, default=256)
    ap.add_argument("--envs", type=int, default=256)
    ap.add_argument("--threads", type=int, default=12)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--init", help="checkpoint to start from (skips imitation if --no-imitation)")
    ap.add_argument("--no-imitation", action="store_true")
    ap.add_argument("--imitation-share", type=float, default=0.17)
    ap.add_argument("--eval-seconds", type=float, default=90.0)
    ap.add_argument("--runs", default="runs")
    ap.add_argument("--lr", type=float, default=3e-4)
    ap.add_argument("--exploit", metavar="ANCHOR",
                    help="train a best-response exploiter of this anchor net: one seat for the learner, "
                         "every other seat plays the anchor (starts from its weights, skips imitation)")
    ap.add_argument("--target-mode", choices=["greedy", "sample"], default="greedy",
                    help="how the exploited anchor plays in training; ladder ratings are greedy")
    ap.add_argument("--no-call-bank", action="store_true",
                    help="the learner may never call the Bank (start or outbid an auction); "
                         "opponents still do, and evaluations apply the same ban")
    ap.add_argument("--buy-at-harbor", action="store_true",
                    help="the learner must buy something whenever it sails to a harbor: it may only sail "
                         "to one where something can be bought and may not decline; evaluations too")
    ap.add_argument("--no-double-reprice", action="store_true",
                    help="the learner may not reprice the same district twice in a row within a turn; evaluations too")
    ap.add_argument("--bank-needs-load", action="store_true",
                    help="the learner may only sail to the Bank when it has a container there to load; evaluations too")
    ap.add_argument("--no-same-turn-repay", action="store_true",
                    help="the learner may not repay a loan in a turn in which it requested one, so a "
                         "request is no longer free; evaluations too. The net's Rust bot needs "
                         "`Neural::without_same_turn_repay` to play under the same rule")
    ap.add_argument("--welfare-weight", type=float, default=0.0,
                    help="share of the learner's reward (0..1) that is the table's mean final score instead "
                         "of its own rank and margin, so that it also gains when the others do well")
    ap.add_argument("--win-only", action="store_true",
                    help="the learner's reward is only the share of opponents it beats, with no score-margin "
                         "term, so a dollar an opponent gains costs it nothing unless it changes the ranking")
    ap.add_argument("--geomean-weight", type=float, default=0.0,
                    help="share of the reward (0..1) that is the geometric mean of the table's final scores; "
                         "the rest is the share of opponents beaten, with no margin term")
    ap.add_argument("--trade-weight", type=float, default=0.0,
                    help="share of the reward (0..1) that is the money of the player-to-player deliveries the "
                         "learner took part in, as deliverer or winning bidder; the rest is the share of "
                         "opponents beaten")
    ap.add_argument("--min-bid", type=int, default=0,
                    help="the learner opens every delivery auction with at least this bid (its whole cash if "
                         "less); evaluations too. The deployed bot has no matching filter yet")
    ap.add_argument("--max-price", type=int, default=0,
                    help="the learner lays no container out in a lot that costs more than this many dollars "
                         "(a container left on the ship is worth $3 at the end); evaluations too. The deployed "
                         "bot needs `Neural::with_max_price`")
    ap.add_argument("--min-trade", type=float, default=0.0,
                    help="constraint: the money of the player-to-player deliveries each learner seat takes part "
                         "in should reach this. The reward is the share of opponents beaten minus a penalty, "
                         "raised while the learners fall short and lowered when they exceed it")
    ap.add_argument("--penalty-lr", type=float, default=0.05, help="step of the --min-trade penalty per iteration")
    ap.add_argument("--scripted-share", type=float, default=0.55,
                    help="chance that an opponent seat of a league table is a scripted bot rather than a frozen "
                         "copy of the learner (the scripted bots stand in for a human)")
    ap.add_argument("--trader-share", type=float, default=0.0,
                    help="chance that a scripted opponent seat is a trader (Merchant that buys only at fair prices "
                         "and prices its own containers around one lot) instead of Merchant, Simple or Random")
    ap.add_argument("--seller-share", type=float, default=0.0,
                    help="chance that a scripted opponent seat is a cheap seller (discounter: prices at $1 and buys "
                         "at any price; bargain: prices around $2 and buys up to $3)")
    ap.add_argument("--solitaire-share", type=float, default=0.0,
                    help="chance that an opponent seat of a league table is Solitaire, drawn before the other kinds")
    ap.add_argument("--merchant-share", type=float, default=0.0,
                    help="chance that an opponent seat of a league table is Merchant, drawn before the other kinds "
                         "(on top of the Merchants among the scripted seats)")
    ap.add_argument("--league-nets", default="consortium",
                    help="the rated anchor nets the learner trains against: 'all', 'none' or comma-separated names. "
                         "Every anchor stays on the ladder that evaluations rate against, so ELOs keep their meaning; "
                         "this only picks the training opponents ('all' is the league before Peddler v2 joined)")
    ap.add_argument("--peddler", default=str(LEAGUE_PEDDLER),
                    help="checkpoint of a frozen net that holds a permanent seat in the league, under the table rules "
                         "the learner plays by (default: Peddler v2, the shipped net the learner is to improve on; "
                         "'' for none). Train with its five rules or it plays without them")
    ap.add_argument("--peddler-share", type=float, default=0.15,
                    help="chance that an opponent seat of a league table is the --peddler net, drawn first "
                         "(0 turns the seat off)")
    ap.add_argument("--heuristic-share", type=float, default=0.0,
                    help="chance that an opponent seat of a league table is one of the three Phase H0 heuristics (Producer, "
                         "Showcase, Shipper, each as likely), drawn right after the --peddler seat (0 turns them off)")
    ap.add_argument("--classic", action="store_true",
                    help="the classic rule: nobody at the table, scripted bots and frozen copies included, may call "
                         "the Bank. The ladder ratings were measured with the Bank, so the eval ELO is not comparable; "
                         "read the win rates")
    ap.add_argument("--interaction-weight", type=float, default=0.0,
                    help="share of the reward paid for dealing with the others (`selfplay.interaction_term`: containers "
                         "bought from opponents, bids on their deliveries, containers sold, fair prices); the rest is "
                         "the default reward (rank and margin)")
    ap.add_argument("--shaping-objective", choices=SHAPING_OBJECTIVES, default=None,
                    help="what the shaping share of the reward pays for (`selfplay.shaping_term`): interaction, island "
                         "(Shipper), producer or showcase. --interaction-weight alone means interaction")
    ap.add_argument("--shaping-weight", type=float, default=None,
                    help="starting share of the reward paid by --shaping-objective (the dual variable's start with "
                         "--win-slack); defaults to --interaction-weight")
    ap.add_argument("--showcase-variant", choices=sorted(SHOWCASE_VARIANTS), default="variety",
                    help="weights of the showcase objective: colours offered at $3 or less alone, plus fair pricing, plus sales")
    ap.add_argument("--bid-within-value", action="store_true",
                    help="the learner never has more on the table in a delivery auction than the cargo is worth on its own "
                         "scoring card (every container at its value, a full set of colours assumed, none discarded), or $5 "
                         "if that is more; an ablation for the island objective, whose pay can come from bids that create money")
    ap.add_argument("--island-weight", type=float, default=0.0,
                    help="share of the reward paid for the points the seat's score took from the Island, "
                         "the rest being the share of opponents beaten (`selfplay.island_term`)")
    ap.add_argument("--win-slack", type=float, default=None,
                    help="constrained mode: keep the learner's win share in the training league above its starting "
                         "level (the mean of the first iterations) minus this slack. The interaction share of the "
                         "reward is a dual variable, raised while the constraint holds and cut while it is broken")
    ap.add_argument("--dual-lr", type=float, default=0.05, help="step of the --win-slack dual variable per iteration")
    ap.add_argument("--dual-max", type=float, default=0.6, help="ceiling of the --win-slack dual variable")
    ap.add_argument("--dual-warmup", type=int, default=6, help="iterations at the start used to measure the win share")
    ap.add_argument("--no-margin", action="store_true", help="with --interaction-weight, the rest of the reward is the win share only, with no score margin")
    args = ap.parse_args()
    objective, shaping_weight = resolve_shaping(args.shaping_objective, args.shaping_weight, args.interaction_weight)
    if args.no_margin:
        if not objective:
            raise SystemExit("--no-margin needs a shaping share (--interaction-weight or --shaping-objective)")
        import training.selfplay as _sp
        _sp.MARGIN_IN_INTERACTION_MIX = False
    import training.selfplay as _sp
    _sp.SHOWCASE_VARIANT = args.showcase_variant
    if args.win_slack is not None and not objective:
        raise SystemExit("--win-slack needs a shaping share, the starting value of the dual variable")
    if objective and any((args.island_weight, args.min_trade, args.win_only, args.welfare_weight,
                          args.geomean_weight, args.trade_weight)):
        raise SystemExit("the shaping share is exclusive with the other reward options")
    if not 0.0 <= shaping_weight <= 1.0:
        raise SystemExit("--shaping-weight and --interaction-weight are shares of the reward, between 0 and 1")
    if args.island_weight and any((args.min_trade, args.win_only, args.welfare_weight, args.geomean_weight, args.trade_weight)):
        raise SystemExit("--island-weight is exclusive with the other reward options")
    if not 0.0 <= args.island_weight <= 1.0:
        raise SystemExit("--island-weight is a share of the reward, between 0 and 1")
    if args.min_trade and any((args.win_only, args.welfare_weight, args.geomean_weight, args.trade_weight)):
        raise SystemExit("--min-trade is exclusive with the other reward options")
    if sum(map(bool, (args.win_only, args.welfare_weight, args.geomean_weight, args.trade_weight))) > 1:
        raise SystemExit("--win-only, --welfare-weight, --geomean-weight and --trade-weight are exclusive")
    if not 0.0 <= args.trade_weight <= 1.0:
        raise SystemExit("--trade-weight is a share of the reward, between 0 and 1")
    if not 0.0 <= args.geomean_weight <= 1.0:
        raise SystemExit("--geomean-weight is a share of the reward, between 0 and 1")
    if not 0.0 <= args.welfare_weight <= 1.0:
        raise SystemExit("--welfare-weight is a share of the reward, between 0 and 1")
    if not 0.0 <= args.peddler_share <= 1.0:
        raise SystemExit("--peddler-share is a chance, between 0 and 1")
    if not 0.0 <= args.heuristic_share <= 1.0:
        raise SystemExit("--heuristic-share is a chance, between 0 and 1")
    seats_peddler = bool(args.peddler) and args.peddler_share > 0 and not args.exploit
    if seats_peddler and not Path(args.peddler).is_file():
        raise SystemExit(f"--peddler {args.peddler} is not a checkpoint; pass --peddler '' to train without that seat")
    if args.exploit:
        if args.exploit not in anchor_lib.net_names():
            raise SystemExit(f"--exploit needs a rated anchor net, one of {anchor_lib.net_names()}")
        args.no_imitation = True
        args.init = args.init or str(anchor_lib.DIR / f"{args.exploit}.pt")

    device = "cuda" if torch.cuda.is_available() else "cpu"
    torch.backends.cuda.matmul.allow_tf32 = True
    rng = np.random.default_rng(args.seed)
    torch.manual_seed(args.seed)
    budget = args.budget_minutes * 60
    run = Run(Path(args.runs) / args.name, budget)
    (run.dir / "config.json").write_text(json.dumps(vars(args) | {"device": device}, indent=2))
    anchors = ladder.load()
    anchor_nets = anchor_lib.load_nets(device, anchors)
    if args.exploit:
        target_slot = anchor_lib.NET_SLOT_BASE + list(anchor_nets).index(args.exploit)
        anchor_slots = {target_slot: anchor_nets[args.exploit]}
    else:
        anchor_slots = league_slots(anchor_nets, args.league_nets)  # slot -> net; evaluations still use every anchor
    stage_name = "exploit" if args.exploit else "selfplay"
    print(f"ladder: {anchors}  (net anchors: {list(anchor_nets)})", flush=True)
    env = BatchEnv(args.envs, args.threads, pinned=device == "cuda")
    eval_env = BatchEnv(256, args.threads, pinned=device == "cuda")  # evaluation must not disturb the games carried over in `env`

    if args.classic:
        env.ban_bank_all()
        eval_env.ban_bank_all()
    if args.no_call_bank:
        env.ban_bank(LEARNER)
        eval_env.ban_bank(LEARNER)
    if args.buy_at_harbor:
        env.require_harbor_purchase(LEARNER)
        eval_env.require_harbor_purchase(LEARNER)
    penalty = 0.0
    mu = shaping_weight
    base_win: list[float] = []
    if args.bid_within_value:
        env.require_bid_within_value(LEARNER)
        eval_env.require_bid_within_value(LEARNER)
    if args.min_bid:
        env.require_min_bid(LEARNER, args.min_bid)
        eval_env.require_min_bid(LEARNER, args.min_bid)
    if args.max_price:
        env.require_max_price(LEARNER, args.max_price)
        eval_env.require_max_price(LEARNER, args.max_price)
    if args.no_double_reprice:
        env.forbid_double_reprice(LEARNER)
        eval_env.forbid_double_reprice(LEARNER)
    if args.bank_needs_load:
        env.require_bank_load(LEARNER)
        eval_env.require_bank_load(LEARNER)
    if args.no_same_turn_repay:
        env.forbid_same_turn_repay(LEARNER)
        eval_env.forbid_same_turn_repay(LEARNER)

    net = PolicyNet(env.state_dim, env.action_dim, args.hidden).to(device)
    if args.init:
        from training.model import load_checkpoint
        net = load_checkpoint(args.init, device)
        if net.dims[0] < env.state_dim:
            from training.add_memory import widen_inputs
            print(f"warm start: {args.init} reads {net.dims[0]} inputs, the env writes {env.state_dim}; "
                  f"the new columns start at zero", flush=True)
            net = widen_inputs(net, env.state_dim - net.dims[0]).to(device)
        elif net.dims[0] != env.state_dim:
            raise SystemExit(f"{args.init} reads {net.dims[0]} inputs but the env writes {env.state_dim}")
    peddler_slots: dict[int, torch.nn.Module] = {}
    if seats_peddler:
        from training.model import load_checkpoint
        peddler_slots[PEDDLER_SLOT] = load_checkpoint(args.peddler, device).eval()
        env.mirror_rules(PEDDLER_SLOT, LEARNER)
    if not args.exploit:
        in_league = [n for i, n in enumerate(anchor_nets) if anchor_lib.NET_SLOT_BASE + i in anchor_slots]
        peddler_note = f"{Path(args.peddler).name} on {args.peddler_share:.0%} of the opponent seats" if peddler_slots else "none"
        heuristic_note = f", Producer/Showcase/Shipper on {args.heuristic_share:.0%} of the opponent seats" if args.heuristic_share else ""
        print(f"league: anchor nets {in_league}, peddler seat {peddler_note}{heuristic_note}, up to 5 snapshots, scripted bots", flush=True)
        if peddler_slots and (missing := missing_peddler_rules(args)):
            print(f"warning: the peddler seat plays by the learner's rules and this run does not set {' '.join(missing)}; "
                  f"the Peddler nets were trained with all five, so it plays outside them here", flush=True)
    games = decisions = 0
    ckpts: list[tuple[str, float]] = []
    frozen: dict[int, torch.nn.Module] = {}
    last_eval = -1e9

    def checkpoint(stage: str, big: bool = False) -> dict:
        net.eval()
        n_games = 1024 if big else 256
        res = anchor_lib.evaluate(eval_env, net, device, rng, n_games, anchors, anchor_nets)
        if args.exploit:
            # `elo` stays the ladder rating, which is the strength that counts; the duel only says how
            # well the target is beaten when every other seat plays it.
            duel = anchor_lib.evaluate_exploit(eval_env, net, anchor_nets[args.exploit], device, rng, n_games,
                                               args.exploit, anchors[args.exploit])
            res |= {f"duel_{k}": v for k, v in duel.items() if k not in ("elo", "games")}
        net.train()
        name = f"ckpt_{len(ckpts):03d}.pt"
        save_checkpoint(net, str(run.dir / name))
        ckpts.append((name, res["elo"]))
        run.log(type="eval", stage=stage, ckpt=name, games_trained=games, decisions=decisions, **res)
        print(f"[{run.elapsed():6.0f}s] eval {stage}: {res}", flush=True)
        return res

    imitation_end = budget * args.imitation_share
    reserve = budget * 0.08
    it = 0

    if not args.no_imitation:
        opt = torch.optim.Adam(net.parameters(), lr=1e-3)
        roller = Roller(env, lambda: imitation_table(rng), device, rng)
        while run.elapsed() < imitation_end:
            run.status("running", "imitation", games=games, decisions=decisions)
            t = time.monotonic()
            rec = data = None
            rec, status, players, scores, _ = roller.collect({0: net}, target=env.n, mode="teacher", beta=0.3)
            ret, keep = outcome_returns(rec, status, players, scores, args.welfare_weight, args.win_only, args.geomean_weight,
                                      args.trade_weight, roller.finished.traded,
                                      args.min_trade, penalty, args.island_weight, roller.finished.breakdown[:, :, 1],
                                      mu, roller.finished.interaction, objective or "interaction")
            data, _, _ = rec.tensors(keep)
            stats = imitate(net, opt, data)
            value_fit(net, data, ret[keep], opt)
            games += int(sum(status == DONE))
            decisions += len(data["state"])
            run.log(type="train", stage="imitation", iter=it, games=games, decisions=decisions,
                    games_per_sec=round(len(status) / (time.monotonic() - t), 1), **stats)
            it += 1
            if run.elapsed() - last_eval > args.eval_seconds:
                checkpoint("imitation")
                last_eval = run.elapsed()
        checkpoint("imitation")
        last_eval = run.elapsed()
        roller.close()
        del roller

    opt = torch.optim.Adam(net.parameters(), lr=args.lr)
    ppo_end = budget - reserve
    snapshot_at = run.elapsed()
    if args.exploit:
        def next_table():
            return anchor_lib.exploit_table(rng, target_slot)
    else:
        def next_table():
            return league_table(rng, list(frozen) + list(anchor_slots), args.scripted_share, args.trader_share, args.seller_share,
                                args.solitaire_share, args.merchant_share, args.peddler_share if peddler_slots else 0.0,
                                args.heuristic_share)
    roller = Roller(env, next_table, device, rng)
    greedy = set(anchor_slots) if args.exploit and args.target_mode == "greedy" else set()
    while run.elapsed() < ppo_end:
        run.status("running", stage_name, games=games, decisions=decisions,
                   elo=ckpts[-1][1] if ckpts else None)
        t = time.monotonic()
        rec = data = None  # last iteration's tensors would otherwise stay on the GPU during the rollout
        policies = {0: net, **frozen, **anchor_slots, **peddler_slots}
        net.eval()
        rec, status, players, scores, tables = roller.collect(policies, target=env.n, mode="sample",
                                                              greedy_slots=greedy)
        net.train()
        env_ids, seats = np.concatenate(rec.env), np.concatenate(rec.seat)
        adv, ret, keep = advantages(rec, env_ids, seats, status, players, scores,
                                    welfare_weight=args.welfare_weight, win_only=args.win_only, geomean_weight=args.geomean_weight,
                                    trade_weight=args.trade_weight, traded=roller.finished.traded,
                                    min_trade=args.min_trade, penalty=penalty, island_weight=args.island_weight,
                                    island=roller.finished.breakdown[:, :, 1], inter_weight=mu,
                                    inter=roller.finished.interaction, shaping=objective or "interaction")
        data, _, _ = rec.tensors(keep)
        mean_reward, finished = table_stats(status, players, scores, tables,
                                            welfare_weight=args.welfare_weight, win_only=args.win_only, geomean_weight=args.geomean_weight,
                                    trade_weight=args.trade_weight, traded=roller.finished.traded,
                                    min_trade=args.min_trade, penalty=penalty, island_weight=args.island_weight,
                                    island=roller.finished.breakdown[:, :, 1], inter_weight=mu,
                                    inter=roller.finished.interaction, shaping=objective or "interaction")
        stats = ppo_update(net, opt, data, adv[keep], ret[keep])
        fin = roller.finished
        mine = np.array([fin.interaction[i][s] for i, (n, kinds, *_rest) in enumerate(tables) if status[i] == DONE
                         for s in range(n) if kinds[s] == LEARNER]).reshape(-1, fin.interaction.shape[2])
        deal = {}
        if len(mine):
            placed = mine[:, PLACED_AT:PLACED_AT + 6]
            deal = dict(bought=round(float(mine[:, BOUGHT].mean()), 2), bids=round(float(mine[:, REAL_BIDS].mean()), 2),
                        sold=round(float(mine[:, SOLD].mean()), 2),
                        delivery_cash=round(float(mine[:, CARGO_CASH:CARGO_CASH + 5].sum(1).mean()), 1),
                        delivery_net=round(float((mine[:, CARGO_CASH:CARGO_CASH + 5].sum(1) - mine[:, SPENT]).mean()), 1),
                        fair=round(float(placed[:, :3].sum() / max(placed.sum(), 1)), 3),
                        **{k: round(v, 3) for k, v in seat_metrics(mine).items()})
        trade_now = None
        if args.min_trade:
            fin = roller.finished
            seen = [fin.traded[i][s] for i, (_, kinds, *_rest) in enumerate(tables) if status[i] == DONE
                    for s in range(len(kinds)) if kinds[s] == LEARNER]
            if seen:
                trade_now = float(np.mean(seen))
                penalty = float(np.clip(penalty + args.penalty_lr * (args.min_trade - trade_now) / args.min_trade, 0.0, 2.0))
        win_now = None
        if args.win_slack is not None:
            mine_win = [win_fraction(scores[i], n)[s] for i, (n, kinds, *_rest) in enumerate(tables) if status[i] == DONE
                        for s in range(n) if kinds[s] == LEARNER]
            if mine_win:
                win_now = float(np.mean(mine_win))
                mu = dual_step(mu, win_now, base_win, args.win_slack, args.dual_lr, args.dual_max, args.dual_warmup)
        games += finished
        decisions += len(data["state"])
        run.log(type="train", stage=stage_name, iter=it, games=games, decisions=decisions,
                mean_reward=round(mean_reward, 4), penalty=round(penalty, 3), mu=round(mu, 3), win=win_now, traded=trade_now, frozen=len(frozen), stalled=int((status == STALLED).sum()), **deal,
                games_per_sec=round(len(status) / (time.monotonic() - t), 1), **stats)
        it += 1
        if run.elapsed() - last_eval > args.eval_seconds:
            res = checkpoint(stage_name)
            last_eval = run.elapsed()
            if not args.exploit:
                # Freeze the net as a future opponent; keep the most recent few.
                slot = max(frozen, default=0) + 1
                frozen[slot] = copy.deepcopy(net).eval()
                if len(frozen) > 5:
                    del frozen[min(frozen)]

    final = checkpoint("final", big=True)
    save_checkpoint(net, str(run.dir / "final.pt"))
    export_weights(net, str(run.dir / "weights.bin"))
    run.log(type="done", reason="budget", games_trained=games, decisions=decisions, **final)
    run.status("finished", "done", games=games, decisions=decisions, elo=final["elo"], reason="budget")
    print(f"finished after {run.elapsed():.0f}s: {final}")


if __name__ == "__main__":
    main()
