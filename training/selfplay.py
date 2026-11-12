"""Plays batches of games for training and evaluation."""

from __future__ import annotations

import gc
from types import SimpleNamespace

import numpy as np
import torch
from torch.nn import functional as F

from training.env import DONE, MAX_PLAYERS, MERCHANT, PENDING, RANDOM, SIMPLE, STALLED, BatchEnv, win_fraction

LEARNER = 0
STALL_REWARD = 0.0  # a game that never ends is worse than any loss; the engine has no rule against it


def reward(scores: np.ndarray, players: int) -> np.ndarray:
    """Outcome per seat in 0..1: the share of opponents beaten, blended with the score margin."""
    s = scores[:players].astype(np.float64)
    margin = (s - (s.sum() - s) / (players - 1)) / 50.0
    return 0.6 * win_fraction(scores, players) + 0.4 * (0.5 + 0.5 * np.tanh(margin))


class Decisions:
    """Decisions of the learner seats, in the order they were taken."""

    def __init__(self):
        self.state, self.acts, self.mask = [], [], []
        self.action, self.logp, self.value, self.label = [], [], [], []
        self.env, self.seat = [], []

    def add(self, st, ac, mk, action, logp, value, label, env, seat):
        self.state.append(st)
        self.acts.append(ac)
        self.mask.append(mk)
        self.action.append(action)
        self.logp.append(logp)
        self.value.append(value)
        self.label.append(label)
        self.env.append(env)
        self.seat.append(seat)

    def __len__(self):
        return sum(len(x) for x in self.action)

    def tensors(self, keep: np.ndarray | None = None):
        """Concatenates everything, acts padded to a common width. `keep` selects rows."""
        width = max(a.shape[1] for a in self.acts)
        acts = torch.cat([F.pad(a, (0, 0, 0, width - a.shape[1])) for a in self.acts])
        mask = torch.cat([F.pad(m, (0, width - m.shape[1])) for m in self.mask])
        out = dict(
            state=torch.cat(self.state), acts=acts, mask=mask,
            action=torch.cat(self.action), logp=torch.cat(self.logp),
            value=torch.cat(self.value), label=torch.cat(self.label),
        )
        env = np.concatenate(self.env)
        seat = np.concatenate(self.seat)
        if keep is not None:
            idx = torch.from_numpy(np.nonzero(keep)[0]).to(out["state"].device)
            out = {k: v[idx] for k, v in out.items()}
            env, seat = env[keep], seat[keep]
        return out, env, seat


MAX_ACTIONS = 48
WIDTHS = (8, 16, 32, MAX_ACTIONS)


class Guard:
    """The repetition guard of the deployed greedy nets, for batches of games.

    `Neural::best_untried` (crates/container-bots/src/neural.rs) keeps a greedy net from freezing a
    table: it does not play the same action twice from the same input (state and offered actions)
    until it has played all the others. The memory belongs to one seat of one game, like the one
    `Neural` per seat in the arena. A net that plays this rule at temperature 0 is the deployed
    "guarded greedy" policy; plain argmax is the unguarded one.
    """

    def __init__(self, envs: int, device: str):
        self.device = device
        self.tried: list[dict[int, int]] = [{} for _ in range(envs * MAX_PLAYERS)]
        self._mult: torch.Tensor | None = None

    def reset(self, env: int) -> None:
        for seat in range(MAX_PLAYERS):
            self.tried[env * MAX_PLAYERS + seat].clear()

    def fingerprint(self, state: torch.Tensor, acts: torch.Tensor) -> torch.Tensor:
        """A 64-bit hash of the exact input bits per row. Padding is zero, so it does not depend on width."""
        bits = torch.cat([state, acts.flatten(1)], dim=1).contiguous().view(torch.int32).to(torch.int64)
        if self._mult is None or self._mult.numel() != bits.shape[1]:
            gen = torch.Generator().manual_seed(20240607)
            mult = torch.randint(-(2**62), 2**62, (bits.shape[1],), generator=gen, dtype=torch.int64) | 1
            self._mult = mult.to(self.device)
        h = (bits * self._mult).sum(dim=1)
        return h ^ (h >> 29)

    def choose(self, rows: np.ndarray, seats: np.ndarray, logits: torch.Tensor, mask: torch.Tensor,
               state: torch.Tensor, acts: torch.Tensor) -> tuple[torch.Tensor, torch.Tensor]:
        """Picks for the envs `rows` (seat `seats` to move). Returns (row index tensor, picked index)."""
        ix = torch.from_numpy(rows).to(self.device)
        fps = self.fingerprint(state[ix], acts[ix]).cpu().tolist()
        keys = (rows * MAX_PLAYERS + seats).tolist()
        seen = torch.tensor([self.tried[k].get(f, 0) for k, f in zip(keys, fps)],
                            dtype=torch.int64, device=self.device)
        lg, valid = logits[ix], mask[ix]
        width = lg.shape[1]
        pos = torch.arange(width, device=self.device, dtype=torch.int64)
        fresh = valid & (((seen[:, None] >> pos) & 1) == 0)
        exhausted = ~fresh.any(dim=1)  # every action has been played from this input: start afresh
        allowed = torch.where(exhausted[:, None], valid, fresh)
        scored = lg.masked_fill(~allowed, float("-inf"))
        pick = (width - 1) - scored.flip(1).argmax(dim=1)  # the last best on ties, as `max_by` does in Rust
        now = torch.where(exhausted, torch.zeros_like(seen), seen) | (torch.ones_like(seen) << pick)
        for k, f, v in zip(keys, fps, now.tolist()):
            self.tried[k][f] = v
        return ix, pick


class GraphedForward:
    """Runs nets on the whole batch of envs through CUDA graphs, which removes the per-step launch cost.

    Inputs sit in static buffers per width bucket; each net gets one captured graph per bucket. Rows of
    envs with nothing pending hold stale data and their outputs are ignored.
    """

    def __init__(self, n: int, state_dim: int, action_dim: int, device: str):
        self.n, self.device = n, device
        self.state = torch.zeros(n, state_dim, device=device)
        self.raw_acts = torch.zeros(n, MAX_ACTIONS, action_dim, device=device)
        self.bufs = {w: (torch.zeros(n, w, action_dim, device=device), torch.zeros(n, w, dtype=torch.bool, device=device))
                     for w in WIDTHS}
        self.graphs: dict = {}
        self.pool = torch.cuda.graph_pool_handle() if device == "cuda" else None  # graphs replay one at a time

    def load(self, state, acts, mask, width: int) -> int:
        w = next(x for x in WIDTHS if x >= width)
        ac, mk = self.bufs[w]
        self.state.copy_(torch.from_numpy(state), non_blocking=True)
        self.raw_acts.copy_(torch.from_numpy(acts), non_blocking=True)
        ac.copy_(self.raw_acts[:, :w])
        mk.copy_(torch.from_numpy(mask).to(self.device, non_blocking=True)[:, :w])
        return w

    @staticmethod
    def _pass(net, st, ac, mk):
        logits, value = net(st, ac, mk, dense=True)
        gumbel = -torch.log(-torch.log(torch.rand_like(logits).clamp_(1e-10, 1.0)))
        return logits, value, (logits + gumbel).argmax(dim=1), logits.argmax(dim=1)

    @torch.no_grad()
    def run(self, net, w: int):
        """Returns static tensors (logits, value, sampled action, greedy action).

        All graphs share one memory pool, so the next `run` overwrites them: copy what you keep.
        """
        key = (id(net), w)
        if key not in self.graphs:
            ac, mk = self.bufs[w]
            side = torch.cuda.Stream()
            side.wait_stream(torch.cuda.current_stream())
            with torch.cuda.stream(side):
                for _ in range(3):
                    self._pass(net, self.state, ac, mk)
            torch.cuda.current_stream().wait_stream(side)
            graph = torch.cuda.CUDAGraph()
            with torch.cuda.graph(graph, pool=self.pool):
                out = self._pass(net, self.state, ac, mk)
            self.graphs[key] = (graph, out, net)
        graph, out, _ = self.graphs[key]
        graph.replay()
        return out

    def release(self) -> None:
        """Drops the captured graphs and their memory."""
        self.graphs.clear()
        self.pool = torch.cuda.graph_pool_handle()
        gc.collect()
        torch.cuda.empty_cache()

    def forget_except(self, nets) -> None:
        keep = {id(n) for n in nets}
        self.graphs = {k: v for k, v in self.graphs.items() if k[0] in keep}


class Roller:
    """Keeps every env busy: a game that ends is replaced at once by the next table.

    Games that are still running when `collect` returns carry over to the next call, so no work is
    thrown away and long games are not under-represented. With `limit` set, only that many games
    are ever started (used for evaluation, where every game must be played to the end).

    `next_table` returns `(players, kinds, seed)` or `(players, kinds, seed, length)`. After every
    `collect`, `finished` holds the winners (bit mask per seat) and action counts of the games it
    returned, in the same order.
    """

    def __init__(self, env: BatchEnv, next_table, device: str, rng=None, limit: int | None = None, record=True):
        self.env, self.next_table, self.device = env, next_table, device
        self.rng = rng or np.random.default_rng()
        self.limit, self.record = limit, record
        self.rec = Decisions()
        self.gid = np.full(env.n, -1, dtype=np.int64)
        self.table: list = [None] * env.n
        self.started = 0
        self.choice = np.zeros(env.n, dtype=np.int32)
        self.forward = GraphedForward(env.n, env.state_dim, env.action_dim, device)
        self.guard = Guard(env.n, device)
        self.finished = SimpleNamespace(gid=np.zeros(0, np.int64), winners=np.zeros(0, np.int32),
                                        actions=np.zeros(0, np.int32))
        for i in range(env.n):
            self._start(i)

    def _start(self, i: int) -> None:
        if self.limit is not None and self.started >= self.limit:
            self.gid[i] = -1
            return
        players, kinds, seed, *length = self.next_table()
        self.env.reset(i, players, seed, kinds, *length)
        self.guard.reset(i)
        self.gid[i] = self.started
        self.table[i] = (players, kinds, seed, *length)
        self.started += 1

    def close(self) -> None:
        self.forward.release()

    def running(self) -> int:
        return int((self.gid >= 0).sum())

    @torch.no_grad()
    def collect(self, policies, target: int | None = None, mode="sample", beta=0.0, greedy_slots=(),
                guarded_slots=(), adjust=None):
        """Plays until `target` games have finished (all started games when None).

        `greedy_slots` are opponent slots that play their best action even when `mode` samples;
        `guarded_slots` play it with the repetition guard (see `Guard`), as the deployed nets do.
        `adjust` maps a slot to `fn(action, rows, mask, acts) -> action`, which may replace the pick of
        the envs `rows` (a tensor of env indices) after the net and the guard have chosen.
        Returns `(rec, status, players, scores, tables)`; rec's env ids index those arrays.
        """
        env, dev = self.env, self.device
        done = []
        self.forward.forget_except(policies.values())
        while True:
            env.advance()
            status, players, scores, winners, actions = env.results_ex()
            ended = np.nonzero(((status == DONE) | (status == STALLED)) & (self.gid >= 0))[0]
            for i in ended:
                done.append((int(self.gid[i]), self.table[i], int(status[i]), int(players[i]), scores[i].copy(),
                             int(winners[i]), int(actions[i])))
                self._start(int(i))
            if len(ended):
                env.advance()
            if target is not None and len(done) >= target:
                break
            state, acts, mask, slot, seat = env.observe()
            pend = np.nonzero(slot >= 0)[0]
            if len(pend) == 0:
                break
            retired = [i for i in pend if int(slot[i]) not in policies]
            for i in retired:
                self._start(int(i))  # the frozen net this game was seated against is gone
            if retired:
                pend = np.array([i for i in pend if int(slot[i]) in policies], dtype=np.int64)
                if len(pend) == 0:
                    continue
            w = self.forward.load(state, acts, mask, int(mask[pend].sum(axis=1).max()))
            st_all, (ac_all, mk_all) = self.forward.state, self.forward.bufs[w]
            teacher = env.teacher().copy() if mode == "teacher" else None
            used = [int(x) for x in np.unique(slot[pend])]
            picks = []
            for s in used:
                logits, value, sampled, best = self.forward.run(policies[s], w)
                if s in guarded_slots:
                    rows = pend[slot[pend] == s]
                    action = best.clone()
                    ix, pick = self.guard.choose(rows, seat[rows], logits, mk_all, st_all, self.forward.raw_acts)
                    action[ix] = pick
                else:
                    action = best if mode == "greedy" or s in greedy_slots else sampled
                if adjust and s in adjust:
                    rows = torch.from_numpy(pend[slot[pend] == s]).to(dev)
                    action = adjust[s](action.clone(), rows, mk_all, self.forward.raw_acts)
                if s == LEARNER:
                    idx = pend[slot[pend] == s]
                    ix = torch.from_numpy(idx).to(dev)
                    st, ac, mk, lg, chosen = st_all[ix], ac_all[ix], mk_all[ix], logits[ix], action[ix]
                    label = torch.zeros_like(chosen)
                    if teacher is not None:
                        label = torch.from_numpy(teacher[idx].astype(np.int64)).to(dev).clamp(min=0)
                        follow = torch.from_numpy(self.rng.random(len(idx)) >= beta).to(dev)
                        chosen = torch.where(follow, label, chosen)
                    if self.record:
                        logp = torch.log_softmax(lg, dim=1).gather(1, chosen[:, None]).squeeze(1)
                        self.rec.add(st, ac.half(), mk, chosen, logp, value[ix], label,
                                     self.gid[idx].copy(), seat[idx].copy())
                    action = action.clone()
                    action[ix] = chosen
                picks.append(action.clone())
            table = torch.stack(picks).cpu().numpy()
            row_of = np.zeros(max(used) + 1, dtype=np.int64)
            row_of[used] = np.arange(len(used))
            self.choice[:] = table[row_of[np.clip(slot, 0, None)], np.arange(env.n)]
            env.step(self.choice)
        return self._package(done)

    def _package(self, done):
        """Splits the recorded decisions into those of finished games and those still running."""
        order = {g: k for k, (g, *_rest) in enumerate(done)}
        rec, rest = Decisions(), Decisions()
        old = self.rec
        self.rec = rest
        if len(old.action):
            gids, seats = np.concatenate(old.env), np.concatenate(old.seat)
            finished = np.array(sorted(order), dtype=np.int64)
            lut = np.array([order[g] for g in finished], dtype=np.int64)
            local = np.full(len(gids), -1, dtype=np.int64)
            if len(finished):
                at = np.searchsorted(finished, gids).clip(max=len(finished) - 1)
                local = np.where(finished[at] == gids, lut[at], -1)
            running = np.zeros(len(gids), dtype=bool)
            live = np.sort(self.gid[self.gid >= 0])
            if len(live):
                at = np.searchsorted(live, gids).clip(max=len(live) - 1)
                running = live[at] == gids
            fin = np.nonzero(local >= 0)[0]
            rem = np.nonzero(running & (local < 0))[0]
            dev = old.action[0].device
            parts = {}
            for name in ("state", "acts", "mask", "action", "logp", "value", "label"):
                chunks = getattr(old, name)
                if name in ("acts", "mask"):
                    width = max(c.shape[1] for c in chunks)
                    big = torch.zeros((len(gids), width) + tuple(chunks[0].shape[2:]), dtype=chunks[0].dtype, device=dev)
                    at = 0
                    for c in chunks:
                        big[at:at + len(c), :c.shape[1]] = c
                        at += len(c)
                else:
                    big = torch.cat(chunks)
                chunks.clear()
                parts[name] = [big[torch.from_numpy(rows).to(dev)] if len(rows) else None for rows in (fin, rem)]
                del big
            torch.cuda.empty_cache()
            for k, (target, rows, ids) in enumerate(((rec, fin, local[fin]), (rest, rem, gids[rem]))):
                if len(rows):
                    target.add(*(parts[n][k] for n in ("state", "acts", "mask", "action", "logp", "value", "label")),
                               ids, seats[rows])
        status = np.array([d[2] for d in done], dtype=np.int32)
        players = np.array([d[3] for d in done], dtype=np.int32)
        scores = np.array([d[4] for d in done], dtype=np.int32).reshape(len(done), MAX_PLAYERS)
        self.finished = SimpleNamespace(
            gid=np.array([d[0] for d in done], dtype=np.int64),
            winners=np.array([d[5] for d in done], dtype=np.int32),
            actions=np.array([d[6] for d in done], dtype=np.int32),
        )
        return rec, status, players, scores, [d[1] for d in done]


def advantages(rec: Decisions, env_ids, seats, status, players, scores, lam=0.97, payoff=None):
    """Per-decision advantage and return. The only reward is the outcome at the game's end.

    The outcome is `reward` of the final scores, or `payoff[game, seat]` when given (the canonical
    utility, say). A stalled game pays `STALL_REWARD` either way.
    """
    n_total = len(env_ids)
    value = torch.cat(rec.value).float().cpu().numpy()
    keep = np.array([status[e] in (DONE, STALLED) for e in env_ids]) if n_total else np.zeros(0, bool)
    key = env_ids * MAX_PLAYERS + seats
    adv = np.zeros(n_total)
    ret = np.zeros(n_total)
    order = np.argsort(key, kind="stable")
    bounds = np.nonzero(np.diff(key[order]))[0] + 1
    for group in np.split(order, bounds):
        if len(group) == 0 or not keep[group[0]]:
            continue
        e, p = env_ids[group[0]], seats[group[0]]
        if status[e] == STALLED:
            r = STALL_REWARD
        else:
            r = reward(scores[e], int(players[e]))[p] if payoff is None else payoff[e, p]
        v = value[group]
        nxt = np.append(v[1:], r)
        delta = nxt - v
        a = 0.0
        for t in range(len(group) - 1, -1, -1):
            a = delta[t] + lam * a
            adv[group[t]] = a
        ret[group] = adv[group] + v
    return adv, ret, keep


def table_stats(status, players, scores, tables, learner_seats_only=True):
    """Mean learner reward over finished games, and how many games finished."""
    rewards = []
    for i, (n, kinds, *_) in enumerate(tables):
        if status[i] != DONE:
            continue
        r = reward(scores[i], n)
        rewards += [r[s] for s in range(n) if kinds[s] == LEARNER]
    return (float(np.mean(rewards)) if rewards else 0.0), len(rewards)
