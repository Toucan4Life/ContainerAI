"""Empirical meta-game analysis for a symmetric n-player game played by a population of m policies.

PSRO has to know which mixture over the policies trained so far is hard to exploit. Simulated tables give
the mean payoff of every strategy in every multiset of n strategies; this module turns them into a game
object, evaluates symmetric mixed strategies exactly, searches for approximate symmetric Nash equilibria
with several solvers, and checks any result with a verifier that shares no code with them.

Each solver states in `Solution.concept` the solution concept it really targets: replicator dynamics and
the NashConv minimiser aim at Nash and only earn the label "Nash candidate" when the measured deviation
gain is tiny, support enumeration solves the indifference equations and verifies the roots, and regret
matching reaches a coarse correlated equilibrium only. The deviation gain of a symmetric mixture sigma,
max_i u_i(sigma) - sigma . u(sigma), is the epsilon of the epsilon-Nash equilibrium "everyone plays sigma";
NashConv, the sum over the n players of their best-response gains, is n times that.

Payoff thresholds are absolute, in payoff units, so utilities are expected to be of order one.
"""

from __future__ import annotations

import itertools
import math
from collections.abc import Callable, Mapping, Sequence
from dataclasses import dataclass, field
from typing import Protocol

import numpy as np

__all__ = [
    "MixtureGame",
    "PayoffOracle",
    "Solution",
    "SymmetricGame",
    "bootstrap_gain",
    "deviation_gain",
    "mean_payoff",
    "nash_conv",
    "nash_conv_minimizer",
    "project_simplex",
    "pure_nash",
    "regret_matching",
    "regrets",
    "replicator",
    "solve_all",
    "support_enumeration",
    "verify_regret",
]

NASH_TOL = 1e-6  # solvers that only aim at Nash are called "Nash candidate" below this deviation gain
ROOT_TOL = 1e-7  # support enumeration keeps a root only if its deviation gain is at most this
PURE_TOL = 1e-12  # a pure strategy counts as a Nash equilibrium below this deviation gain
MAX_ENUMERATION_M = 14  # support enumeration visits 2**m supports
_SIGMA_TOL = 1e-6  # how far from a probability vector an input sigma may be
_FD_STEP = 1e-5  # central-difference step in log-odds, used only when an oracle has no analytic gradient


class PayoffOracle(Protocol):
    """What every solver needs: the number of strategies and the exact payoffs of a mixture."""

    m: int

    def expected_payoffs(self, sigma: np.ndarray) -> np.ndarray:
        """u_i(sigma) for every strategy i when all other players draw iid from sigma; shape (m,)."""
        ...


def _multisets(m: int, size: int) -> np.ndarray:
    """All sorted tuples of `size` strategies out of range(m), one per row, in lexicographic order."""
    count = math.comb(m + size - 1, size)
    flat = itertools.chain.from_iterable(itertools.combinations_with_replacement(range(m), size))
    return np.fromiter(flat, dtype=np.int64, count=count * size).reshape(count, size)


def _ranks(rows: np.ndarray, m: int) -> np.ndarray:
    """Index of each sorted multiset row among all multisets of that size.

    A sorted multiset a_0 <= ... <= a_{s-1} maps to the combination {a_j + j}, whose colex rank is
    sum_j C(a_j + j, j + 1). That is a bijection onto range(C(m + s - 1, s)), so payoffs can live in a dense
    array and be gathered with numpy instead of being looked up tuple by tuple in a dict.
    """
    size = rows.shape[1]
    binom = np.array([[math.comb(a, c) for c in range(size + 1)] for a in range(m + size)], dtype=np.int64)
    j = np.arange(size)
    return binom[rows + j, j + 1].sum(axis=1)


def _softmax(theta: np.ndarray) -> np.ndarray:
    z = np.exp(theta - theta.max(axis=-1, keepdims=True))
    return z / z.sum(axis=-1, keepdims=True)


def _as_batch(sigmas: np.ndarray, m: int) -> np.ndarray:
    """Validated float array of mixtures with shape (S, m); rounding noise below zero is clipped."""
    s = np.asarray(sigmas, dtype=np.float64)
    if s.ndim != 2 or s.shape[1] != m:
        raise ValueError(f"expected mixtures of shape (S, {m}), got {s.shape}")
    if s.size == 0:
        return s
    if not s.min() >= -1e-12 or np.abs(s.sum(axis=1) - 1.0).max() > _SIGMA_TOL:
        raise ValueError("sigma must be a probability vector: non-negative and summing to 1")
    return np.maximum(s, 0.0)


def _as_distribution(sigma: np.ndarray, m: int) -> np.ndarray:
    s = np.asarray(sigma, dtype=np.float64)
    if s.shape != (m,):
        raise ValueError(f"sigma must have shape ({m},), got {s.shape}")
    return _as_batch(s[None, :], m)[0]


@dataclass
class _Tables:
    """Lazily built enumeration of the multisets of the other n-1 players, shared by every evaluation."""

    others: np.ndarray  # [K, n-1] sorted tuples of strategies
    counts: np.ndarray  # [K, m] how often each strategy occurs in each multiset
    counts_t: np.ndarray  # [m, K] contiguous transpose of `counts`
    logcoef: np.ndarray  # [K] log multinomial coefficient of each multiset
    pay: np.ndarray  # [K, m] payoff of strategy i against multiset k, 0 where missing
    pay_t: np.ndarray  # [m, K] contiguous transpose of `pay`
    missing: np.ndarray | None  # [K, m] bool mask of NaN payoffs, None for a complete game
    missing_f: np.ndarray | None  # the same mask as floats, for counting with matrix products


class SymmetricGame:
    """Symmetric n-player normal-form game given by the payoff of every multiset of n strategies.

    All players share one strategy set, so only the multiset of strategies in a profile matters. `payoffs`
    maps each sorted n-tuple of strategy indices to one payoff per position of the tuple, the way simulated
    tables report them; positions holding the same strategy are averaged, which makes the payoff of strategy
    i against any multiset of opponents well defined. With `allow_missing=True` absent multisets (and NaN
    payoffs) are kept as NaN and `expected_payoffs` raises only if a NaN entry has positive probability.

    `expected_payoffs` is exact: the other n-1 players draw iid from sigma, so each multiset of opponents
    has a multinomial probability. The enumeration of those multisets and the payoff matrix against them are
    built once, on first use, after which an evaluation is two small matrix products.
    """

    def __init__(
        self,
        n: int,
        m: int,
        payoffs: Mapping[tuple[int, ...], Sequence[float]],
        names: Sequence[str] | None = None,
        allow_missing: bool = False,
    ) -> None:
        if n < 2 or m < 1:
            raise ValueError(f"need at least 2 players and 1 strategy, got n={n}, m={m}")
        self.n, self.m = int(n), int(m)
        self.names = [str(x) for x in names] if names is not None else [str(i) for i in range(self.m)]
        if len(self.names) != self.m:
            raise ValueError(f"expected {self.m} names, got {len(self.names)}")
        self.payoffs = payoffs  # kept as given (not copied): verify_regret works from exactly this input
        self.allow_missing = allow_missing
        self._full = self._dense(payoffs)
        total = len(self._full)
        self.n_missing = total - int((~np.isnan(self._full).any(axis=1)).sum())
        if self.n_missing and not allow_missing:
            raise ValueError(
                f"{self.n_missing} of {total} multisets of {self.n} strategies out of {self.m} have no "
                "(finite) payoff entry; pass allow_missing=True to treat them as NaN"
            )
        self._cache: _Tables | None = None

    def _dense(self, payoffs: Mapping[tuple[int, ...], Sequence[float]]) -> np.ndarray:
        """Payoff of the strategy at each sorted position of every multiset, by colex rank (NaN if absent)."""
        n, m = self.n, self.m
        full = np.full((math.comb(m + n - 1, n), n), np.nan)
        if not payoffs:
            return full
        try:
            keys = np.array(list(payoffs.keys()))
            vals = np.array([list(v) for v in payoffs.values()], dtype=np.float64)
        except (TypeError, ValueError) as exc:
            raise ValueError("payoffs must map sorted n-tuples to n payoffs each") from exc
        if keys.shape != (len(payoffs), n) or vals.shape != keys.shape or keys.dtype.kind not in "iu":
            raise ValueError(f"payoffs must map sorted {n}-tuples of strategy indices to {n} payoffs each")
        if keys.min() < 0 or keys.max() >= m or (np.diff(keys, axis=1) < 0).any():
            raise ValueError(f"keys must be sorted tuples of strategy indices in range({m})")
        if np.isinf(vals).any():
            raise ValueError("payoffs must be finite or NaN")
        same = keys[:, :, None] == keys[:, None, :]
        shared = np.where(same, vals[:, None, :], 0.0).sum(axis=2) / same.sum(axis=2)
        full[_ranks(keys, m)] = shared
        return full

    @classmethod
    def from_function(
        cls,
        n: int,
        m: int,
        fn: Callable[[int, tuple[int, ...]], float],
        names: Sequence[str] | None = None,
    ) -> SymmetricGame:
        """Game from fn(i, others): the payoff of a player using i against the sorted tuple `others`."""
        payoffs: dict[tuple[int, ...], list[float]] = {}
        for key in itertools.combinations_with_replacement(range(m), n):
            by_strategy: dict[int, float] = {}
            for p, s in enumerate(key):
                if s not in by_strategy:  # first occurrence of s, so removing it keeps `others` sorted
                    by_strategy[s] = float(fn(s, key[:p] + key[p + 1 :]))
            payoffs[key] = [by_strategy[s] for s in key]
        return cls(n, m, payoffs, names)

    def __repr__(self) -> str:
        return f"SymmetricGame(n={self.n}, m={self.m}, missing={self.n_missing})"

    def raw(self) -> list[tuple[float, Mapping[tuple[int, ...], Sequence[float]], int]]:
        """The game as the (weight, payoffs, n) components that `verify_regret` takes."""
        return [(1.0, self.payoffs, self.n)]

    def payoff(self, i: int, others: Sequence[int]) -> float:
        """Payoff of a player using strategy i against the other players' strategies `others` (any order)."""
        rest = sorted(int(o) for o in others)
        if len(rest) != self.n - 1 or not all(0 <= s < self.m for s in [*rest, int(i)]):
            raise ValueError(f"need {self.n - 1} opponent strategies and i in range({self.m})")
        key = np.array(sorted([*rest, int(i)]), dtype=np.int64)
        position = int(np.searchsorted(key, i))
        return float(self._full[_ranks(key[None, :], self.m)[0], position])

    def restrict(self, keep: Sequence[int]) -> SymmetricGame:
        """Sub-game on the strategies `keep`, reindexed 0..len(keep)-1 in the order given."""
        keep = [int(k) for k in keep]
        if not keep or len(set(keep)) != len(keep) or min(keep) < 0 or max(keep) >= self.m:
            raise ValueError(f"keep must be distinct strategy indices in range({self.m})")
        new_keys = _multisets(len(keep), self.n)
        old = np.asarray(keep, dtype=np.int64)[new_keys]
        ranks = _ranks(np.sort(old, axis=1), self.m)
        vals = np.empty(old.shape)
        for p in range(self.n):
            # a strategy's slot in the sorted old multiset is the number of strictly smaller entries
            vals[:, p] = self._full[ranks, (old < old[:, p : p + 1]).sum(axis=1)]
        payoffs = {tuple(k): v for k, v in zip(new_keys.tolist(), vals.tolist(), strict=True)}
        return SymmetricGame(
            self.n, len(keep), payoffs, [self.names[k] for k in keep], allow_missing=self.allow_missing
        )

    def _tables(self) -> _Tables:
        if self._cache is None:
            n, m = self.n, self.m
            others = _multisets(m, n - 1)
            k = len(others)
            counts = np.zeros((k, m))
            for c in range(n - 1):
                counts[np.arange(k), others[:, c]] += 1.0
            log_fact = np.concatenate([[0.0], np.cumsum(np.log(np.arange(1, n)))])
            logcoef = log_fact[n - 1] - log_fact[counts.astype(np.int64)].sum(axis=1)
            pay = np.empty((k, m))
            for i in range(m):
                with_i = np.sort(np.concatenate([others, np.full((k, 1), i)], axis=1), axis=1)
                pay[:, i] = self._full[_ranks(with_i, m), (others < i).sum(axis=1)]
            nan = np.isnan(pay)
            gaps = nan if nan.any() else None
            pay = np.where(nan, 0.0, pay)
            self._cache = _Tables(
                others=others,
                counts=counts,
                counts_t=np.ascontiguousarray(counts.T),
                logcoef=logcoef,
                pay=pay,
                pay_t=np.ascontiguousarray(pay.T),
                missing=gaps,
                missing_f=None if gaps is None else gaps.astype(np.float64),
            )
        return self._cache

    def _probs(self, sig: np.ndarray) -> np.ndarray:
        """Multinomial probability of every multiset of opponents under each mixture, shape (S, K).

        Computed as exp(log coefficient + counts . log sigma) with log(0) masked out: a multiset that
        contains a strategy played with probability zero gets probability exactly 0 (and 0**0 = 1).
        """
        t = self._tables()
        zero = sig <= 0.0
        prob = np.exp(np.log(np.where(zero, 1.0, sig)) @ t.counts_t + t.logcoef)
        if zero.any():
            prob[(zero.astype(np.float64) @ t.counts_t) > 0.0] = 0.0
        return prob

    def _payoffs_from(self, prob: np.ndarray) -> np.ndarray:
        t = self._tables()
        if t.missing is not None and t.missing_f is not None:
            bad = (prob > 0.0).astype(np.float64) @ t.missing_f
            if bad.any():
                s, i = (int(x) for x in np.argwhere(bad > 0)[0])
                k = int(np.argmax((prob[s] > 0.0) & t.missing[:, i]))
                raise ValueError(
                    f"payoff of strategy {i} against {tuple(t.others[k].tolist())} is missing (NaN) but "
                    f"that multiset of opponents has probability {prob[s, k]:.3g}"
                )
        return prob @ t.pay

    def _vjp_from(self, prob: np.ndarray, sig: np.ndarray, cot: np.ndarray) -> np.ndarray:
        """sum_i cot[s, c, i] * d u_i / d theta_a for sigma = softmax(theta), shape (S, C, m).

        For one multiset, d prob / d theta_a = prob * (count_a - (n-1) sigma_a), which stays exact when some
        sigma_a is zero (then prob and the derivative both vanish wherever the multiset contains a).
        """
        t = self._tables()
        s, c, m = cot.shape
        w = ((cot.reshape(s * c, m) @ t.pay_t) * np.repeat(prob, c, axis=0)).reshape(s, c, -1)
        grad = (w.reshape(s * c, -1) @ t.counts).reshape(s, c, m)
        return grad - (self.n - 1) * sig[:, None, :] * w.sum(axis=2, keepdims=True)

    def expected_payoffs(self, sigma: np.ndarray) -> np.ndarray:
        """u_i(sigma): payoff of strategy i when the other n-1 players draw iid from sigma; shape (m,)."""
        s = _as_distribution(sigma, self.m)
        return self._payoffs_from(self._probs(s[None, :]))[0]

    def expected_payoffs_batch(self, sigmas: np.ndarray) -> np.ndarray:
        """`expected_payoffs` for S mixtures at once: (S, m) -> (S, m). Optional accelerator for solvers."""
        return self._payoffs_from(self._probs(_as_batch(sigmas, self.m)))

    def payoffs_and_vjp(
        self, sigmas: np.ndarray, cotangent: Callable[[np.ndarray], np.ndarray]
    ) -> tuple[np.ndarray, np.ndarray]:
        """Payoffs (S, m) plus a vector-Jacobian product with respect to the log-odds theta of sigma.

        `cotangent(u)` receives the payoffs and returns weights b of shape (S, C, m); the result is
        G[s, c, a] = sum_i b[s, c, i] * d u_i / d theta_a. Optional accelerator: solvers fall back to
        finite differences for oracles without it.
        """
        sig = _as_batch(sigmas, self.m)
        prob = self._probs(sig)
        u = self._payoffs_from(prob)
        return u, self._vjp_from(prob, sig, cotangent(u))


class MixtureGame:
    """The same strategy set played at several player counts: expected payoff = sum_k w_k * game_k's.

    PSRO policies sit at 3-, 4- and 5-player tables with a fixed probability each; a policy's payoff against
    a mixture is then the weighted average over those tables. Weights are normalised to sum to one.
    """

    def __init__(self, games: Sequence[SymmetricGame], weights: Sequence[float] | None = None) -> None:
        self.games = list(games)
        if not self.games:
            raise ValueError("a mixture needs at least one game")
        self.m = self.games[0].m
        if any(g.m != self.m for g in self.games):
            raise ValueError("all games of a mixture must share the same strategy set (same m)")
        w = np.ones(len(self.games)) if weights is None else np.asarray(weights, dtype=np.float64)
        if w.shape != (len(self.games),) or not np.isfinite(w).all() or (w < 0).any() or w.sum() <= 0:
            raise ValueError("weights must be one non-negative number per game, not all zero")
        self.weights = w / w.sum()
        self.names = self.games[0].names
        self.ns = [g.n for g in self.games]

    def __repr__(self) -> str:
        return f"MixtureGame(n={self.ns}, m={self.m}, weights={np.round(self.weights, 4).tolist()})"

    def raw(self) -> list[tuple[float, Mapping[tuple[int, ...], Sequence[float]], int]]:
        """The mixture as the (weight, payoffs, n) components that `verify_regret` takes."""
        return [(float(w), g.payoffs, g.n) for w, g in zip(self.weights, self.games, strict=True)]

    def _live(self) -> list[tuple[float, SymmetricGame]]:
        return [(float(w), g) for w, g in zip(self.weights, self.games, strict=True) if w > 0]

    def expected_payoffs(self, sigma: np.ndarray) -> np.ndarray:
        """sum_k w_k u^{(k)}(sigma); shape (m,)."""
        s = _as_distribution(sigma, self.m)[None, :]
        return sum(w * g._payoffs_from(g._probs(s)) for w, g in self._live())[0]

    def expected_payoffs_batch(self, sigmas: np.ndarray) -> np.ndarray:
        """`expected_payoffs` for S mixtures at once: (S, m) -> (S, m)."""
        sig = _as_batch(sigmas, self.m)
        return sum(w * g._payoffs_from(g._probs(sig)) for w, g in self._live())

    def payoffs_and_vjp(
        self, sigmas: np.ndarray, cotangent: Callable[[np.ndarray], np.ndarray]
    ) -> tuple[np.ndarray, np.ndarray]:
        """See `SymmetricGame.payoffs_and_vjp`; the weights apply to payoffs and gradients alike."""
        sig = _as_batch(sigmas, self.m)
        parts = [(w, g, g._probs(sig)) for w, g in self._live()]
        u = sum(w * g._payoffs_from(p) for w, g, p in parts)
        cot = cotangent(u)
        return u, sum(w * g._vjp_from(p, sig, cot) for w, g, p in parts)


def mean_payoff(game: PayoffOracle, sigma: np.ndarray) -> float:
    """sum_i sigma_i u_i(sigma): what a player gets when everyone plays sigma."""
    s = np.asarray(sigma, dtype=np.float64)
    return float(s @ game.expected_payoffs(s))


def regrets(game: PayoffOracle, sigma: np.ndarray) -> np.ndarray:
    """u_i(sigma) - mean_payoff(sigma) for each strategy i (negative for strategies worse than sigma)."""
    s = np.asarray(sigma, dtype=np.float64)
    u = game.expected_payoffs(s)
    return u - float(s @ u)


def deviation_gain(game: PayoffOracle, sigma: np.ndarray) -> float:
    """max_i u_i(sigma) - mean_payoff(sigma), the symmetric epsilon of "everyone plays sigma"; >= 0."""
    return max(0.0, float(regrets(game, sigma).max()))


def nash_conv(game: PayoffOracle, sigma: np.ndarray, n_players: int) -> float:
    """Sum over the players of their best-response gain when all play sigma: n_players * deviation_gain."""
    return n_players * deviation_gain(game, sigma)


@dataclass
class Solution:
    """One solver output together with what it does and does not certify."""

    method: str  # solver name, e.g. "replicator"
    concept: str  # the solution concept this output actually targets or verifies
    sigma: np.ndarray  # symmetric mixed strategy, sums to 1
    gain: float  # deviation_gain(game, sigma), computed with the exact evaluator
    iterations: int
    extra: dict = field(default_factory=dict)  # method-specific diagnostics


def _payoffs_batch(game: PayoffOracle, sigmas: np.ndarray) -> np.ndarray:
    """u(sigma) for each row of `sigmas`, batched when the oracle can and looped otherwise."""
    batch = getattr(game, "expected_payoffs_batch", None)
    if batch is not None:
        return batch(sigmas)
    return np.stack([game.expected_payoffs(s) for s in sigmas])


def _tilt(sigmas: np.ndarray, a: int, h: float) -> np.ndarray:
    """Move every mixture by h along coordinate a of its log-odds (stays on the simplex, keeps zeros)."""
    out = sigmas.copy()
    out[:, a] *= math.exp(h)
    return out / out.sum(axis=1, keepdims=True)


def _payoffs_and_vjp(
    game: PayoffOracle, sigmas: np.ndarray, cotangent: Callable[[np.ndarray], np.ndarray]
) -> tuple[np.ndarray, np.ndarray]:
    """`payoffs_and_vjp` of the oracle if it has one, else central differences in the log-odds."""
    native = getattr(game, "payoffs_and_vjp", None)
    if native is not None:
        return native(sigmas, cotangent)
    u = _payoffs_batch(game, sigmas)
    cot = cotangent(u)
    grad = np.zeros_like(cot)
    for a in range(game.m):
        up = _payoffs_batch(game, _tilt(sigmas, a, _FD_STEP))
        down = _payoffs_batch(game, _tilt(sigmas, a, -_FD_STEP))
        grad[:, :, a] = np.einsum("sci,si->sc", cot, up - down) / (2.0 * _FD_STEP)
    return u, grad


def _theta_jacobian(game: PayoffOracle, sigma: np.ndarray, cols: Sequence[int]) -> np.ndarray:
    """D[i, a] = d u_i / d theta_a at sigma (theta are log-odds) for a in `cols`; other columns are 0."""
    m = game.m
    if hasattr(game, "payoffs_and_vjp"):
        return game.payoffs_and_vjp(sigma[None, :], lambda u: np.eye(m)[None])[1][0]
    jac = np.zeros((m, m))
    for a in cols:
        up = game.expected_payoffs(_tilt(sigma[None, :], a, _FD_STEP)[0])
        down = game.expected_payoffs(_tilt(sigma[None, :], a, -_FD_STEP)[0])
        jac[:, a] = (up - down) / (2.0 * _FD_STEP)
    return jac


def _gains(game: PayoffOracle, sigmas: np.ndarray) -> np.ndarray:
    """Deviation gain of each row of `sigmas`."""
    u = _payoffs_batch(game, sigmas)
    return np.maximum(u.max(axis=1) - (sigmas * u).sum(axis=1), 0.0)


def _payoff_scale(game: PayoffOracle) -> float:
    """Typical spread max_i u_i - min_i u_i of the payoffs, probed at pure, uniform and random mixtures."""
    m = game.m
    probes = np.vstack(
        [np.eye(m), np.full((1, m), 1.0 / m), np.random.default_rng(0).dirichlet(np.ones(m), size=16)]
    )
    u = _payoffs_batch(game, probes)
    spread = float((u.max(axis=1) - u.min(axis=1)).max())
    return spread if spread > 1e-12 else 1.0


def project_simplex(x: np.ndarray) -> np.ndarray:
    """Euclidean projection of a vector onto the probability simplex (sort-based, O(m log m))."""
    v = np.asarray(x, dtype=np.float64)
    u = np.sort(v)[::-1]
    css = np.cumsum(u) - 1.0
    k = np.arange(1, v.size + 1)
    rho = k[u - css / k > 0][-1]
    return np.maximum(v - css[rho - 1] / rho, 0.0)


def pure_nash(game: PayoffOracle) -> list[int]:
    """Strategies i for which "everyone plays i" is a Nash equilibrium (deviation gain <= PURE_TOL)."""
    u = _payoffs_batch(game, np.eye(game.m))  # u[i, j] = payoff of j when everyone else plays i
    gain = u.max(axis=1) - np.diag(u)
    return [int(i) for i in np.nonzero(gain <= PURE_TOL)[0]]


def bootstrap_gain(sigma: np.ndarray, games: Sequence[PayoffOracle]) -> np.ndarray:
    """Deviation gain of the FIXED mixture sigma under each (resampled) game, to see how stable it is."""
    s = np.asarray(sigma, dtype=np.float64)
    return np.array([deviation_gain(g, s) for g in games])


def _nash_label(gain: float, what: str) -> str:
    """Concept label of a solver that aims at Nash: the claim is made only if the measured gain backs it."""
    if gain <= NASH_TOL:
        return "Nash candidate (verified by gain)"
    return f"not verified ({what} has gain {gain:.2e} > {NASH_TOL:g}); no Nash claim"


def replicator(
    game: PayoffOracle,
    sigma0: np.ndarray | None = None,
    steps: int = 20000,
    step_size: float = 0.05,
    average: bool = True,
    tol: float = 1e-9,
    seed: int = 0,
) -> Solution:
    """Discrete-time replicator dynamics sigma_i <- sigma_i * (1 + step_size * (u_i - mean) / scale).

    `scale` is the largest payoff spread seen so far: every factor stays positive and `step_size` does not
    depend on the payoff units. An attracting rest point is a symmetric Nash equilibrium, but the dynamics
    can also cycle (rock-paper-scissors), so the last iterate is returned only if it stopped moving (L1
    change below `tol`); otherwise, with `average=True`, the time-averaged iterate, which often approaches
    the equilibrium of a cycle. The result is labelled a Nash candidate only if its measured deviation gain
    is tiny. The dynamics are deterministic: `seed` exists for a uniform solver signature and is unused.
    """
    m = game.m
    sigma = np.full(m, 1.0 / m) if sigma0 is None else _as_distribution(sigma0, m)
    scale = _payoff_scale(game)
    total = np.zeros(m)
    stride = max(1, steps // 100)
    trajectory: list[float] = []
    evaluated = 0
    converged = False
    for t in range(steps):
        u = game.expected_payoffs(sigma)
        mean = float(sigma @ u)
        scale = max(scale, float(u.max() - u.min()))
        total += sigma
        evaluated += 1
        if t % stride == 0:
            trajectory.append(max(0.0, float(u.max()) - mean))
        nxt = np.maximum(sigma * (1.0 + step_size * (u - mean) / scale), 0.0)
        nxt /= nxt.sum()
        change = float(np.abs(nxt - sigma).sum())
        sigma = nxt
        if change < tol:
            converged = True
            break
    averaged = total / evaluated if evaluated else sigma
    last_gain, average_gain = deviation_gain(game, sigma), deviation_gain(game, averaged)
    use_average = average and not converged
    chosen, gain = (averaged, average_gain) if use_average else (sigma, last_gain)
    returned = "average" if use_average else "last"
    extra = {
        "converged": converged,
        "last_gain": last_gain,
        "average_gain": average_gain,
        "returned": returned,
        "trajectory_gain": trajectory,
        "scale": scale,
    }
    concept = _nash_label(gain, f"replicator {returned} iterate")
    return Solution("replicator", concept, chosen, gain, evaluated, extra)


def regret_matching(game: PayoffOracle, iters: int = 20000, seed: int = 0) -> Solution:
    """Symmetric regret matching self-play; returns the average strategy.

    All n players use the same sigma_t, proportional to the positive part of the accumulated regrets
    (uniform when there is none), and each regret grows by u_i(sigma_t) - mean_payoff(sigma_t). Average
    regret vanishes, which makes the uniform mixture over rounds of sigma_t^(x n) an approximate coarse
    correlated equilibrium of the n-player game: `extra["cce_gap"]`, max_i of the average regret, is exactly
    its gap. That is weaker than Nash, and the average strategy's own gain is not controlled by it (it can
    be large, e.g. in games with a cyclic structure); the concept string says so. Deterministic: `seed` is
    unused.
    """
    if iters < 1:
        raise ValueError("iters must be at least 1")
    m = game.m
    uniform = np.full(m, 1.0 / m)
    cumulative = np.zeros(m)
    total = np.zeros(m)
    stride = max(1, iters // 40)
    trajectory: list[float] = []
    for t in range(iters):
        positive = np.maximum(cumulative, 0.0)
        norm = positive.sum()
        sigma = positive / norm if norm > 0.0 else uniform
        u = game.expected_payoffs(sigma)
        cumulative += u - float(sigma @ u)
        total += sigma
        if (t + 1) % stride == 0:
            trajectory.append(deviation_gain(game, total / (t + 1)))
    averaged = total / iters
    gain = deviation_gain(game, averaged)
    concept = (
        "CCE (coarse correlated equilibrium of the n-player game, via average regret); "
        "the average strategy's own Nash gain is NOT guaranteed small"
    )
    extra = {"cce_gap": float(cumulative.max() / iters), "trajectory_gain": trajectory}
    return Solution("regret_matching", concept, averaged, gain, iters, extra)


_FROZEN = 1e-20  # a start whose scaled surrogate is below this already meets the equilibrium conditions
_LOGIT_FLOOR = 40.0  # logits stay within this distance of the largest one, so softmax never underflows
_BETA1, _BETA2, _EPS = 0.9, 0.99, 1e-8  # Adam; the short second-moment memory suits vanishing gradients
_POLISH_TOP = 3  # this many best starts are polished at the end
_SNAP_CUTOFFS = (1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-9, 1e-12)  # entries below these may be dropped


def _surrogate(game: PayoffOracle, sigma: np.ndarray, scale: float) -> tuple[np.ndarray, np.ndarray]:
    """Scaled surrogate f = sum_i max(0, r_i / scale)^2 and its gradient in the softmax logits.

    r = u - sigma . u are the regrets. With a_i = 2 max(0, r_i) / scale and A = sum_i a_i the chain rule
    gives df/dtheta_a = (sum_i (a_i - A sigma_i) du_i/dtheta_a - A sigma_a r_a) / scale.
    """
    box: dict[str, np.ndarray] = {}

    def cotangent(u: np.ndarray) -> np.ndarray:
        r = u - (sigma * u).sum(axis=1, keepdims=True)
        a = 2.0 * np.maximum(r, 0.0) / scale
        total = a.sum(axis=1, keepdims=True)
        box["r"], box["total"] = r, total
        return (a - total * sigma)[:, None, :]

    _, vjp = _payoffs_and_vjp(game, sigma, cotangent)
    r = box["r"]
    grad = (vjp[:, 0, :] - box["total"] * sigma * r) / scale
    return grad, (np.maximum(r, 0.0) ** 2).sum(axis=1) / scale**2


def _adam_descent(
    game: PayoffOracle, theta: np.ndarray, scale: float, steps: int, lr: float, lr_end: float
) -> np.ndarray:
    """Adam on the surrogate in softmax logits; every row of `theta` is an independent start.

    The learning rate decays like a cosine from `lr` to `lr_end`: Adam moves a coordinate by about the
    learning rate even when the gradient is tiny, so only a vanishing rate lets the iterates settle on the
    zero set of the surrogate. Rows that reach it are frozen and leave the batch.
    """
    theta = theta.copy()
    first, second = np.zeros_like(theta), np.zeros_like(theta)
    live = np.ones(len(theta), dtype=bool)
    for t in range(1, steps + 1):
        rows = np.nonzero(live)[0]
        if rows.size == 0:
            break
        grad, f = _surrogate(game, _softmax(theta[rows]), scale)
        done = f < _FROZEN
        live[rows[done]] = False
        rate = lr_end + 0.5 * (lr - lr_end) * (1.0 + math.cos(math.pi * (t - 1) / steps))
        first[rows] = _BETA1 * first[rows] + (1.0 - _BETA1) * grad
        second[rows] = _BETA2 * second[rows] + (1.0 - _BETA2) * grad**2
        step = rate * (first[rows] / (1.0 - _BETA1**t)) / (np.sqrt(second[rows] / (1.0 - _BETA2**t)) + _EPS)
        step[done] = 0.0
        moved = theta[rows] - step
        theta[rows] = np.maximum(moved - moved.max(axis=1, keepdims=True), -_LOGIT_FLOOR)
    return theta


_NEWTON_ITERS = 30
_STEP_CAP = 5.0  # largest log-odds change of one Newton step
_THETA_SPREAD = 30.0  # log-odds this far apart mean "the root is on the boundary of the support"
_LINE_SEARCH = 8  # halvings tried per Newton step


def _newton_support(
    game: PayoffOracle, support: np.ndarray, start: np.ndarray, tol: float
) -> tuple[np.ndarray, int] | None:
    """Damped Newton on the indifference equations u_i(sigma) = u_j(sigma) for i, j in `support`.

    The unknowns are the log-odds of the support strategies with the first pinned at 0, so sigma is a strictly
    positive probability vector on the support at every step and no boundary handling is needed (a root on
    the boundary is a root of a smaller support). Returns (sigma, Newton steps) or None if the iteration
    stalls, diverges or runs out of steps.
    """
    m = game.m

    def to_sigma(theta: np.ndarray) -> np.ndarray:
        sigma = np.zeros(m)
        sigma[support] = _softmax(theta)
        return sigma

    def residual(theta: np.ndarray) -> np.ndarray:
        u = game.expected_payoffs(to_sigma(theta))[support]
        return u[1:] - u[0]

    theta = np.log(start) - np.log(start[0])
    f = residual(theta)
    for steps in range(_NEWTON_ITERS):
        err = float(np.abs(f).max())
        if err <= tol:
            return to_sigma(theta), steps
        jac = _theta_jacobian(game, to_sigma(theta), support)[np.ix_(support, support)]
        jac = jac[1:, 1:] - jac[0, 1:]
        try:
            direction = np.linalg.solve(jac, -f)
        except np.linalg.LinAlgError:
            direction = np.linalg.lstsq(jac, -f, rcond=None)[0]
        peak = float(np.abs(direction).max())
        if not np.isfinite(peak) or peak < 1e-14:
            return None
        alpha = min(1.0, _STEP_CAP / peak)
        for _ in range(_LINE_SEARCH):
            trial = theta.copy()
            trial[1:] += alpha * direction
            if np.abs(trial).max() <= _THETA_SPREAD:
                f_trial = residual(trial)
                if np.abs(f_trial).max() < (1.0 - 1e-4 * alpha) * err:
                    break
            alpha *= 0.5
        else:
            return None
        theta, f = trial, f_trial
    return (to_sigma(theta), _NEWTON_ITERS) if np.abs(f).max() <= tol else None


def _clipped(sigma: np.ndarray) -> np.ndarray:
    """sigma with entries below 1e-12 set to zero and renormalised."""
    out = np.where(sigma < 1e-12, 0.0, sigma)
    return out / out.sum()


def _refine(game: PayoffOracle, sigma: np.ndarray, gain: float, tol: float) -> tuple[np.ndarray, float, str]:
    """Final clean-up of an optimiser result: the best of sigma, snapped copies and Newton roots.

    Softmax logits only approach the boundary of the simplex and a first-order method converges slowly on
    ill-conditioned games, so the result is typically an equilibrium with leftover entries around 1e-5 and
    a gain around 1e-5 as well. For each support obtained by dropping the entries below a cutoff, the
    snapped sigma and the root of the indifference system on that support (Newton started at the snapped
    sigma) are candidates; whatever has the smallest measured gain wins, so this never makes things worse.
    Returns (sigma, gain, "adam" | "snap" | "newton").
    """
    best, best_gain, how = sigma, gain, "adam"
    tried: set[tuple[int, ...]] = set()
    for cutoff in _SNAP_CUTOFFS:
        support = np.nonzero(sigma >= cutoff)[0]
        if support.size == 0 or tuple(support.tolist()) in tried:
            continue
        tried.add(tuple(support.tolist()))
        snapped = np.zeros_like(sigma)
        snapped[support] = sigma[support]
        snapped /= snapped.sum()
        candidates = [("snap", snapped)]
        root = _newton_support(game, support, snapped[support], tol) if support.size >= 2 else None
        if root is not None:
            candidates.append(("newton", _clipped(root[0])))
        for label, cand in candidates:
            cand_gain = deviation_gain(game, cand)
            if cand_gain < best_gain:
                best, best_gain, how = cand, cand_gain, label
    return best, best_gain, how


def nash_conv_minimizer(
    game: PayoffOracle, starts: int = 64, steps: int = 3000, lr: float = 0.2, seed: int = 0
) -> Solution:
    """Multi-start minimisation of sum_i max(0, u_i(sigma) - mean_payoff(sigma))^2 over the simplex.

    The surrogate is smooth and zero exactly at symmetric Nash equilibria; sigma = softmax(theta) is
    optimised with a numpy Adam using the analytic gradient (one batched pass for all starts). `starts` is the
    total number of starting points: the m near-pure strategies and the uniform mixture always run, the rest
    are Dirichlet draws (alternately flat and sparse). The few starts with the smallest deviation gain are
    polished with a lower learning rate; the best of them is then cleaned up by `_refine` (small entries
    snapped to zero, Newton on the indifference equations of the remaining support), which keeps a change
    only if the measured gain drops. The result is labelled a Nash candidate only if its final gain is tiny.
    """
    m = game.m
    rng = np.random.default_rng(seed)
    scale = _payoff_scale(game)
    n_random = max(0, starts - (m + 1))
    init = [0.99 * np.eye(m) + 0.01 / m, np.full((1, m), 1.0 / m)]
    for k in range(n_random):
        draw = np.maximum(rng.dirichlet(np.full(m, 1.0 if k % 2 == 0 else 0.3)), 1e-9)
        init.append((draw / draw.sum())[None, :])
    labels = [f"pure:{j}" for j in range(m)] + ["uniform"] + [f"dirichlet:{k}" for k in range(n_random)]
    theta = _adam_descent(game, np.log(np.vstack(init)), scale, steps, lr, lr * 1e-3)
    sigmas = _softmax(theta)
    gains = _gains(game, sigmas)
    top = np.argsort(gains, kind="stable")[:_POLISH_TOP]
    polish_steps = min(500, max(100, steps // 6))
    polished = _softmax(_adam_descent(game, theta[top], scale, polish_steps, lr / 10.0, lr * 1e-5))
    pool = np.vstack([sigmas[top], polished])
    best = int(np.argmin(_gains(game, pool)))
    sigma = pool[best] / pool[best].sum()
    adam_gain = deviation_gain(game, sigma)
    sigma, gain, refined_by = _refine(game, sigma, adam_gain, 1e-11 * scale)
    extra = {
        "starts": len(labels),
        "best_start": labels[int(top[best % len(top)])],
        "start_gains": gains.tolist(),
        "adam_gain": adam_gain,
        "refined_by": refined_by,
        "surrogate": float((np.maximum(regrets(game, sigma), 0.0) ** 2).sum()),
        "polish_steps": polish_steps,
    }
    concept = _nash_label(gain, "NashConv-surrogate minimiser result")
    return Solution("nash_conv_minimizer", concept, sigma, gain, steps + polish_steps, extra)


_SUPPORT_CONCEPT = f"Nash equilibrium (support enumeration, verified by gain <= {ROOT_TOL:g})"


def support_enumeration(
    game: PayoffOracle,
    max_support: int | None = None,
    tol: float = 1e-9,
    seed: int = 0,
    starts_per_support: int = 6,
) -> list[Solution]:
    """All symmetric Nash equilibria found by solving the indifference system on every support (m <= 14).

    A pure support is checked directly. For a support S with at least two strategies, damped Newton solves
    u_i(sigma) = v for i in S with sigma supported on S, from the uniform mixture on S and
    `starts_per_support - 1` random Dirichlet mixtures; a polynomial system can have several roots, so
    several starts are tried, and a support can have none (e.g. two strategies of rock-paper-scissors).
    A root is kept only if its measured deviation gain, after clipping tiny entries, is at most ROOT_TOL,
    which also checks that no strategy outside S does better. Solutions closer than 1e-6 in L1 are merged
    and the list is sorted by (support size, gain). The search is not exhaustive for large systems:
    Newton may miss a root, so an empty or short list does not prove that no equilibrium exists.
    """
    m = game.m
    if m > MAX_ENUMERATION_M:
        raise ValueError(f"support enumeration visits 2**m supports; refusing m={m} > {MAX_ENUMERATION_M}")
    largest = m if max_support is None else min(int(max_support), m)
    rng = np.random.default_rng(seed)
    found: list[Solution] = []

    def keep(sigma: np.ndarray, iterations: int) -> None:
        sigma = _clipped(sigma)
        gain = deviation_gain(game, sigma)
        if gain > ROOT_TOL or any(np.abs(s.sigma - sigma).sum() < 1e-6 for s in found):
            return
        extra = {"support": np.nonzero(sigma)[0].tolist(), "v": mean_payoff(game, sigma)}
        found.append(Solution("support_enumeration", _SUPPORT_CONCEPT, sigma, gain, iterations, extra))

    for k in range(1, largest + 1):
        for members in itertools.combinations(range(m), k):
            support = np.array(members)
            if k == 1:
                keep(np.eye(m)[members[0]], 0)
                continue
            random_starts = np.maximum(rng.dirichlet(np.ones(k), size=max(0, starts_per_support - 1)), 1e-6)
            for start in [np.full(k, 1.0 / k), *random_starts][:starts_per_support]:
                root = _newton_support(game, support, start, tol)
                if root is not None:
                    keep(*root)
    found.sort(key=lambda s: (len(s.extra["support"]), s.gain))
    return found


def solve_all(
    game: PayoffOracle, n_players: int | None = None, seed: int = 0, quick: bool = False
) -> list[Solution]:
    """Run replicator, regret matching, the NashConv minimiser and (if m <= 10) support enumeration.

    `quick=True` cuts the iteration counts about tenfold, for tests and large games. If the number of
    players is known (`n_players`, else the game's own `n`) every solution also gets
    `extra["nash_conv"]`, the sum over the players of their best-response gain.
    """
    shrink = 10 if quick else 1
    solutions = [
        replicator(game, steps=20000 // shrink, seed=seed),
        regret_matching(game, iters=20000 // shrink, seed=seed),
        nash_conv_minimizer(game, starts=8 if quick else 64, steps=3000 // shrink, seed=seed),
    ]
    if game.m <= 10:
        solutions += support_enumeration(game, seed=seed, starts_per_support=3 if quick else 6)
    players = n_players if n_players is not None else getattr(game, "n", None)
    if players is not None:
        for s in solutions:
            s.extra["nash_conv"] = nash_conv(game, s.sigma, players)
    return solutions


# ---------------------------------------------------------------------------------------------------------
# Independent verifier. It works straight from the raw payoff dictionaries and deliberately uses none of
# the classes, tables or solvers above (only numpy and itertools), so a bug in them cannot hide in its own
# check. Every helper and constant here is prefixed `_verify_` / `_VERIFY_`.
# ---------------------------------------------------------------------------------------------------------

_VERIFY_ENUMERATE_LIMIT = 20_000  # "auto" enumerates every ordered opponent tuple up to this many


def _verify_vector(
    payoffs: Mapping[tuple[int, ...], Sequence[float]],
    m: int,
    others: tuple[int, ...],
    cache: dict[tuple[int, ...], np.ndarray],
) -> np.ndarray:
    """Payoff of every strategy against the sorted opponent multiset `others`, read from the raw dict.

    The profile (others + i) is looked up under its sorted key; positions that hold strategy i itself are
    averaged. A missing or NaN entry raises, because the caller only asks for multisets that can occur.
    """
    hit = cache.get(others)
    if hit is None:
        values = []
        for i in range(m):
            key = tuple(sorted((*others, i)))
            entry = payoffs.get(key)
            mine = [float(v) for s, v in zip(key, entry, strict=True) if s == i] if entry is not None else []
            value = sum(mine) / len(mine) if mine else float("nan")
            if math.isnan(value):
                raise ValueError(f"payoff of strategy {i} in profile {key} is missing or NaN but is needed")
            values.append(value)
        hit = cache[others] = np.array(values)
    return hit


def _verify_enumerated(
    payoffs: Mapping[tuple[int, ...], Sequence[float]], n: int, m: int, sig: np.ndarray
) -> np.ndarray:
    """u_i(sigma) as an explicit sum over ALL ordered tuples of the n-1 other players."""
    u = np.zeros(m)
    cache: dict[tuple[int, ...], np.ndarray] = {}
    for ordered in itertools.product(range(m), repeat=n - 1):
        prob = 1.0
        for s in ordered:
            prob *= float(sig[s])
        if prob > 0.0:
            u += prob * _verify_vector(payoffs, m, tuple(sorted(ordered)), cache)
    return u


def _verify_drawn(
    payoffs: Mapping[tuple[int, ...], Sequence[float]],
    n: int,
    m: int,
    sig: np.ndarray,
    samples: int,
    rng: np.random.Generator,
) -> tuple[np.ndarray, np.ndarray]:
    """Payoff vectors of the distinct opponent multisets in `samples` iid draws, with their draw counts."""
    draws = np.sort(rng.choice(m, size=(samples, n - 1), p=sig / sig.sum()), axis=1)
    rows, counts = np.unique(draws, axis=0, return_counts=True)
    cache: dict[tuple[int, ...], np.ndarray] = {}
    return np.array([_verify_vector(payoffs, m, tuple(int(s) for s in r), cache) for r in rows]), counts


def verify_regret(
    raw: Sequence[tuple[float, Mapping[tuple[int, ...], Sequence[float]], int]],
    m: int,
    sigma: Sequence[float],
    method: str = "auto",
    samples: int = 200_000,
    seed: int = 0,
) -> dict:
    """Independent check of the regrets of "everyone plays sigma", computed only from the raw payoff dicts.

    `raw` lists (weight, payoffs, n) components in the format `SymmetricGame` takes (a plain game is one
    component of weight 1.0, a `MixtureGame` has several, see their `raw()`); u_i(sigma) is the weighted sum
    over components of the expected payoff of strategy i against n-1 iid draws from sigma. With
    method="enumerate" (chosen by "auto" when m**(n-1) <= 20000 for every component) all ordered opponent
    tuples are summed explicitly; with "sampled", `samples` seeded Monte-Carlo draws per component are
    used and a standard error is reported for the regret of the best deviation.

    Returns {"u", "mean", "gain", "regrets", "method", "stderr"}; "gain" = max(0, max_i regret_i) and
    "stderr" is None when enumerated.
    """
    sig = np.asarray(sigma, dtype=np.float64)
    if sig.shape != (m,) or not sig.min() >= -1e-12 or abs(float(sig.sum()) - 1.0) > 1e-6:
        raise ValueError(f"sigma must be a probability vector of length {m}")
    sig = np.maximum(sig, 0.0)
    if method not in ("auto", "enumerate", "sampled"):
        raise ValueError(f"unknown method {method!r}")
    parts = [(float(w), payoffs, int(n)) for w, payoffs, n in raw]
    if any(not math.isfinite(w) or w < 0.0 for w, _, _ in parts):
        raise ValueError("component weights must be finite and non-negative")
    parts = [p for p in parts if p[0] > 0.0]
    if not parts:
        raise ValueError("no component with a positive weight")
    exact = method == "enumerate" or (
        method == "auto" and all(m ** (n - 1) <= _VERIFY_ENUMERATE_LIMIT for _, _, n in parts)
    )
    u = np.zeros(m)
    if exact:
        for w, payoffs, n in parts:
            u += w * _verify_enumerated(payoffs, n, m, sig)
        regs = u - float(sig @ u)
        return {
            "u": u,
            "mean": float(sig @ u),
            "gain": max(0.0, float(regs.max())),
            "regrets": regs,
            "method": "enumerate",
            "stderr": None,
        }
    if samples < 2:
        raise ValueError("need at least 2 samples")
    rng = np.random.default_rng(seed)
    drawn = []
    for w, payoffs, n in parts:
        x, counts = _verify_drawn(payoffs, n, m, sig, samples, rng)
        u += w * (counts @ x) / samples
        drawn.append((w, x, counts))
    mean = float(sig @ u)
    regs = u - mean
    worst = int(np.argmax(regs))
    variance = 0.0
    for w, x, counts in drawn:
        y = x[:, worst] - x @ sig  # regret of the best deviation in each draw
        centred = y - counts @ y / samples
        variance += w * w * float(counts @ centred**2) / (samples - 1) / samples
    return {
        "u": u,
        "mean": mean,
        "gain": max(0.0, float(regs.max())),
        "regrets": regs,
        "method": "sampled",
        "stderr": math.sqrt(variance),
    }
