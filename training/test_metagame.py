"""Tests for training.metagame: exact payoffs, honestly labelled solvers and the independent verifier.

Games are tiny and seeded, so the whole file runs in seconds. Solvers are pinned to answers known in closed
form: rock-paper-scissors (uniform), a dominance game (pure strategy 0), a 3-player 2-strategy game whose
indifference condition is a quadratic, and symmetric constant-sum games (mean payoff 1/n).
"""

from __future__ import annotations

import ast
import inspect
import itertools
import math
from collections.abc import Sequence

import numpy as np
import pytest

from training import metagame as mg
from training.metagame import (
    MixtureGame,
    PayoffOracle,
    SymmetricGame,
    bootstrap_gain,
    deviation_gain,
    mean_payoff,
    nash_conv,
    nash_conv_minimizer,
    project_simplex,
    pure_nash,
    regret_matching,
    regrets,
    replicator,
    solve_all,
    support_enumeration,
    verify_regret,
)

Payoffs = dict[tuple[int, ...], list[float]]

RPS = np.array([[0.0, -1.0, 1.0], [1.0, 0.0, -1.0], [-1.0, 1.0, 0.0]])
UNIFORM3 = np.full(3, 1.0 / 3.0)
SIGMA_STAR = np.array([math.sqrt(7.0) - 2.0, 3.0 - math.sqrt(7.0)])  # equilibrium of `interior_game`


def random_payoffs(n: int, m: int, seed: int) -> Payoffs:
    """Gaussian payoff of each strategy in every multiset; equal strategies of a profile share one value."""
    rng = np.random.default_rng(seed)
    table: Payoffs = {}
    for key in itertools.combinations_with_replacement(range(m), n):
        value = {s: float(rng.normal()) for s in sorted(set(key))}
        table[key] = [value[s] for s in key]
    return table


def random_game(n: int, m: int, seed: int) -> SymmetricGame:
    return SymmetricGame(n, m, random_payoffs(n, m, seed))


def random_sigma(m: int, seed: int, zeros: Sequence[int] = ()) -> np.ndarray:
    sigma = np.random.default_rng(seed).dirichlet(np.ones(m))
    sigma[list(zeros)] = 0.0
    return sigma / sigma.sum()


def naive_expected_payoffs(table: Payoffs, n: int, m: int, sigma: np.ndarray) -> np.ndarray:
    """u_i(sigma) by looping over i and over every ordered profile of the other n-1 players."""
    u = np.zeros(m)
    for i in range(m):
        for others in itertools.product(range(m), repeat=n - 1):
            prob = math.prod(float(sigma[j]) for j in others)
            key = tuple(sorted((i, *others)))
            u[i] += prob * table[key][key.index(i)]
    return u


def rps_game(n: int) -> SymmetricGame:
    """Rock-paper-scissors as an n-player game: u(i; others) = mean over the others of A[i][j]."""
    return SymmetricGame.from_function(n, 3, lambda i, others: float(np.mean([RPS[i][j] for j in others])))


def dominance_game(n: int = 4, m: int = 4) -> SymmetricGame:
    """Strategy 0 earns 1 more than any other strategy plus an interaction term of size at most 0.2."""

    def payoff(i: int, others: tuple[int, ...]) -> float:
        interaction = sum(math.sin(3.0 * i + 5.0 * j + 1.0) for j in others) / len(others)
        return (1.0 if i == 0 else 0.0) + 0.2 * interaction

    return SymmetricGame.from_function(n, m, payoff)


def two_strategy_game(a: Sequence[float], b: Sequence[float]) -> SymmetricGame:
    """3 players, 2 strategies: with c of the others on strategy 1, strategy 0 pays a[c], strategy 1 b[c]."""
    return SymmetricGame.from_function(3, 2, lambda i, others: (a, b)[i][sum(others)])


def interior_game() -> SymmetricGame:
    """Anti-coordination game with one symmetric Nash equilibrium, in the interior, known in closed form.

    If each other player picks strategy 1 with probability p, the number c of them that do is Binomial(2, p).
    Strategy 0 pays a = (0, 1, 2)[c] and strategy 1 pays b = (2, 0, -1)[c], so the indifference condition is
    sum_c Bin(c; p) (a_c - b_c) = -2(1-p)^2 + 2p(1-p) + 3p^2 = -2 + 6p - p^2 = 0, i.e. p = 3 - sqrt(7) (the
    root 3 + sqrt(7) is not a probability). The left side is -2 at p = 0 and 3 at p = 1, so neither pure
    strategy is an equilibrium, and the common payoff is v = u_0 = 2p = 6 - 2 sqrt(7).
    """
    return two_strategy_game(a=(0.0, 1.0, 2.0), b=(2.0, 0.0, -1.0))


def constant_sum_game(n: int, m: int, seed: int) -> SymmetricGame:
    """Every profile pays positive weights normalised to sum to 1 over the n positions."""
    rng = np.random.default_rng(seed)
    table: Payoffs = {}
    for key in itertools.combinations_with_replacement(range(m), n):
        weight = {s: float(rng.uniform(0.1, 1.0)) for s in sorted(set(key))}
        total = sum(weight[s] for s in key)
        table[key] = [weight[s] / total for s in key]
    return SymmetricGame(n, m, table)


class ThinOracle:
    """Exposes only what the PayoffOracle protocol promises, so solvers cannot lean on any extras."""

    def __init__(self, game: SymmetricGame) -> None:
        self.m = game.m
        self._game = game

    def expected_payoffs(self, sigma: np.ndarray) -> np.ndarray:
        return self._game.expected_payoffs(sigma)


# --- 1. exact evaluation ------------------------------------------------------------------------------------


@pytest.mark.parametrize(("n", "m"), [(3, 4), (4, 5), (5, 4)])
def test_expected_payoffs_match_verifier_and_naive_loop(n: int, m: int) -> None:
    table = random_payoffs(n, m, seed=100 * n + m)
    game = SymmetricGame(n, m, table)
    cases = [(0, ()), (1, ()), (2, (0, m - 1)), (3, tuple(range(1, m)))]  # the last one is a pure strategy
    for seed, zeros in cases:
        sigma = random_sigma(m, seed, zeros)
        checked = verify_regret([(1.0, table, n)], m, sigma, method="enumerate")
        u = game.expected_payoffs(sigma)
        assert checked["method"] == "enumerate"
        np.testing.assert_allclose(u, checked["u"], rtol=0, atol=1e-9)
        np.testing.assert_allclose(u, naive_expected_payoffs(table, n, m, sigma), rtol=0, atol=1e-9)
        np.testing.assert_allclose(regrets(game, sigma), checked["regrets"], rtol=0, atol=1e-9)
        assert mean_payoff(game, sigma) == pytest.approx(checked["mean"], abs=1e-9)
        assert deviation_gain(game, sigma) == pytest.approx(checked["gain"], abs=1e-9)


def test_mixture_expected_payoffs_match_verifier_and_naive_loop() -> None:
    m = 4
    specs = [(3, 11), (4, 12), (5, 13)]  # (n, seed): the same four policies at 3-, 4- and 5-player tables
    tables = [random_payoffs(n, m, seed) for n, seed in specs]
    games = [SymmetricGame(n, m, t) for (n, _), t in zip(specs, tables, strict=True)]
    mixture = MixtureGame(games, weights=[2.0, 1.0, 1.0])
    weights = [0.5, 0.25, 0.25]
    np.testing.assert_allclose(mixture.weights, weights)
    raw = [(w, t, n) for w, t, (n, _) in zip(weights, tables, specs, strict=True)]
    for seed, zeros in [(0, ()), (1, (2,))]:
        sigma = random_sigma(m, seed, zeros)
        u = mixture.expected_payoffs(sigma)
        by_hand = sum(
            w * naive_expected_payoffs(t, n, m, sigma)
            for w, t, (n, _) in zip(weights, tables, specs, strict=True)
        )
        np.testing.assert_allclose(u, by_hand, rtol=0, atol=1e-9)
        for components in (raw, mixture.raw()):
            checked = verify_regret(components, m, sigma, method="enumerate")
            np.testing.assert_allclose(u, checked["u"], rtol=0, atol=1e-9)
            assert deviation_gain(mixture, sigma) == pytest.approx(checked["gain"], abs=1e-9)


def test_expected_payoffs_on_a_large_game_match_the_verifier() -> None:
    n, m = 5, 20
    table = random_payoffs(n, m, seed=7)
    game = SymmetricGame(n, m, table)
    sigma = random_sigma(m, 1, zeros=(3, 4, 17))
    checked = verify_regret([(1.0, table, n)], m, sigma, method="enumerate")
    np.testing.assert_allclose(game.expected_payoffs(sigma), checked["u"], rtol=0, atol=1e-9)


def test_sigma_with_zeros_gives_no_nans() -> None:
    n, m = 4, 5
    game = random_game(n, m, seed=5)
    for j in range(m):
        u = game.expected_payoffs(np.eye(m)[j])
        assert np.isfinite(u).all()
        for i in range(m):
            assert u[i] == pytest.approx(game.payoff(i, [j] * (n - 1)), abs=1e-12)
    sigma = np.array([0.0, 0.7, 0.0, 0.3, 0.0])
    assert np.isfinite(game.expected_payoffs(sigma)).all()
    assert np.isfinite(regrets(game, sigma)).all()
    _, grad = game.payoffs_and_vjp(sigma[None, :], lambda u: np.ones((1, 1, m)))
    assert np.isfinite(grad).all()
    u = game.expected_payoffs(sigma)
    for i in range(m):
        # k of the n-1 opponents play strategy 3 (probability 0.3 each), the rest play strategy 1
        by_hand = sum(
            math.comb(n - 1, k) * 0.3**k * 0.7 ** (n - 1 - k) * game.payoff(i, [1] * (n - 1 - k) + [3] * k)
            for k in range(n)
        )
        assert u[i] == pytest.approx(by_hand, abs=1e-12)


def test_expected_payoffs_reject_things_that_are_not_mixtures() -> None:
    game = random_game(3, 3, seed=1)
    for bad in ([0.5, 0.5], [0.5, 0.5, 0.5], [1.5, -0.5, 0.0], [math.nan, 0.5, 0.5]):
        with pytest.raises(ValueError, match="sigma|shape"):
            game.expected_payoffs(np.array(bad))


def test_regret_helpers_agree_with_their_definitions() -> None:
    n = 4
    game = random_game(n, 4, seed=6)
    sigma = random_sigma(4, 2)
    u = game.expected_payoffs(sigma)
    mean = float(sigma @ u)
    np.testing.assert_allclose(regrets(game, sigma), u - mean, rtol=0, atol=1e-14)
    assert mean_payoff(game, sigma) == pytest.approx(mean, abs=1e-14)
    assert deviation_gain(game, sigma) == pytest.approx(float((u - mean).max()), abs=1e-14)
    assert regrets(game, sigma).min() < 0.0 < deviation_gain(game, sigma)
    assert nash_conv(game, sigma, n) == pytest.approx(n * deviation_gain(game, sigma), abs=1e-14)


# --- 2. rock-paper-scissors ---------------------------------------------------------------------------------


@pytest.mark.parametrize("n", [3, 4, 5])
def test_rps_uniform_is_an_equilibrium(n: int) -> None:
    game = rps_game(n)
    u = game.expected_payoffs(UNIFORM3)
    assert np.ptp(u) < 1e-12  # every strategy earns the same against the uniform mixture ...
    np.testing.assert_allclose(u, np.zeros(3), rtol=0, atol=1e-12)  # ... namely 0, the value of the game
    assert deviation_gain(game, UNIFORM3) == pytest.approx(0.0, abs=1e-12)
    assert deviation_gain(game, np.array([0.6, 0.3, 0.1])) == pytest.approx(0.5, abs=1e-12)
    assert pure_nash(game) == []


@pytest.mark.parametrize("n", [3, 4, 5])
def test_rps_support_enumeration_finds_the_uniform_equilibrium(n: int) -> None:
    solutions = support_enumeration(rps_game(n))
    assert len(solutions) == 1  # the two-strategy supports have no root and must fail quietly
    found = solutions[0]
    np.testing.assert_allclose(found.sigma, UNIFORM3, rtol=0, atol=1e-9)
    assert found.method == "support_enumeration"
    assert found.extra["support"] == [0, 1, 2]
    assert found.gain <= 1e-7
    assert "Nash equilibrium" in found.concept


@pytest.mark.parametrize("n", [3, 5])
def test_rps_regret_matching_average_approaches_uniform_and_claims_only_a_cce(n: int) -> None:
    game = rps_game(n)
    solution = regret_matching(game, iters=20000)
    assert solution.gain < 2e-2
    np.testing.assert_allclose(solution.sigma, UNIFORM3, rtol=0, atol=2e-2)
    assert solution.extra["cce_gap"] < 1e-2
    assert solution.gain == pytest.approx(deviation_gain(game, solution.sigma), abs=1e-12)
    assert solution.concept.startswith("CCE")
    assert "NOT guaranteed" in solution.concept
    assert "Nash candidate" not in solution.concept


def test_rps_averaged_replicator_approaches_uniform_while_the_last_iterate_cycles() -> None:
    game = rps_game(3)
    start = np.array([0.6, 0.3, 0.1])
    averaged = replicator(game, sigma0=start, steps=20000)
    assert not averaged.extra["converged"]
    assert averaged.extra["returned"] == "average"
    assert averaged.gain < 2e-2
    assert averaged.gain == pytest.approx(averaged.extra["average_gain"], abs=1e-15)
    assert averaged.extra["last_gain"] > 0.1
    assert len(averaged.extra["trajectory_gain"]) > 10
    assert "Nash candidate" not in averaged.concept  # a gain of ~6e-3 does not earn the label
    last = replicator(game, sigma0=start, steps=20000, average=False)
    assert last.extra["returned"] == "last"
    assert last.gain == pytest.approx(averaged.extra["last_gain"], abs=1e-12)
    assert "Nash candidate" not in last.concept


def test_replicator_started_at_the_equilibrium_stays_there_and_earns_the_label() -> None:
    solution = replicator(rps_game(3))
    assert solution.extra["converged"]
    assert solution.extra["returned"] == "last"
    assert solution.gain < 1e-12
    assert solution.concept == "Nash candidate (verified by gain)"


@pytest.mark.parametrize("n", [3, 4])
def test_rps_nash_conv_minimizer_reaches_the_equilibrium(n: int) -> None:
    solution = nash_conv_minimizer(rps_game(n))
    assert solution.method == "nash_conv_minimizer"
    assert solution.gain < 1e-3
    np.testing.assert_allclose(solution.sigma, UNIFORM3, rtol=0, atol=1e-3)
    assert solution.concept == "Nash candidate (verified by gain)"
    assert solution.extra["starts"] == 64
    assert len(solution.extra["start_gains"]) == 64
    assert np.median(solution.extra["start_gains"]) < 1e-3


# --- 3. dominance -------------------------------------------------------------------------------------------


def test_dominance_game_every_solver_ends_at_the_dominant_strategy() -> None:
    game = dominance_game()
    assert pure_nash(game) == [0]
    solutions = solve_all(game, quick=True)
    assert {s.method for s in solutions} == {
        "replicator",
        "regret_matching",
        "nash_conv_minimizer",
        "support_enumeration",
    }
    for solution in solutions:
        assert solution.sigma[0] > 0.99, (solution.method, solution.sigma)
        exact_solver = solution.method != "regret_matching"
        assert solution.gain < (1e-7 if exact_solver else 1e-2), (solution.method, solution.gain)
    assert regret_matching(game).sigma[0] > 0.999
    exact = support_enumeration(game)
    assert len(exact) == 1
    np.testing.assert_allclose(exact[0].sigma, np.eye(4)[0], rtol=0, atol=0)


# --- 4. closed-form interior equilibrium --------------------------------------------------------------------


def test_interior_equilibrium_is_recovered_in_closed_form() -> None:
    game = interior_game()
    assert deviation_gain(game, SIGMA_STAR) == pytest.approx(0.0, abs=1e-12)
    assert pure_nash(game) == []
    exact = support_enumeration(game)
    assert len(exact) == 1
    np.testing.assert_allclose(exact[0].sigma, SIGMA_STAR, rtol=0, atol=1e-4)
    assert exact[0].extra["support"] == [0, 1]
    assert exact[0].extra["v"] == pytest.approx(6.0 - 2.0 * math.sqrt(7.0), abs=1e-9)
    minimized = nash_conv_minimizer(game)
    np.testing.assert_allclose(minimized.sigma, SIGMA_STAR, rtol=0, atol=1e-4)
    assert minimized.concept == "Nash candidate (verified by gain)"


def test_interior_equilibrium_is_stable_for_replicator_and_regret_matching() -> None:
    game = interior_game()
    dynamics = replicator(game)
    assert dynamics.extra["converged"]
    np.testing.assert_allclose(dynamics.sigma, SIGMA_STAR, rtol=0, atol=1e-4)
    assert dynamics.concept == "Nash candidate (verified by gain)"
    matching = regret_matching(game)
    assert matching.gain < 1e-3
    np.testing.assert_allclose(matching.sigma, SIGMA_STAR, rtol=0, atol=1e-2)


def test_support_enumeration_finds_every_equilibrium_of_a_two_strategy_game() -> None:
    """With d_c = a_c - b_c, f(p) = 0.14(1-p)^2 - 0.62p(1-p) + 0.24p^2 = (p - 0.2)(p - 0.7).

    f(0) > 0 makes the pure strategy 0 an equilibrium and f(1) > 0 rules out the pure strategy 1, so the
    equilibria are exactly e_0, (0.8, 0.2) and (0.3, 0.7).
    """
    game = two_strategy_game(a=(0.14, -0.31, 0.24), b=(0.0, 0.0, 0.0))
    assert pure_nash(game) == [0]
    found = support_enumeration(game)
    assert [len(s.extra["support"]) for s in found] == [1, 2, 2]
    sigmas = sorted((s.sigma for s in found), key=lambda s: s[1])
    for sigma, expected in zip(sigmas, [[1.0, 0.0], [0.8, 0.2], [0.3, 0.7]], strict=True):
        np.testing.assert_allclose(sigma, expected, rtol=0, atol=1e-6)
    assert nash_conv_minimizer(game).gain < 1e-6


def test_support_enumeration_max_support_limits_the_search() -> None:
    game = interior_game()
    assert support_enumeration(game, max_support=1) == []
    assert len(support_enumeration(game, max_support=2)) == 1


def test_support_enumeration_refuses_large_games() -> None:
    class Huge:
        m = 15

        def expected_payoffs(self, sigma: np.ndarray) -> np.ndarray:
            raise AssertionError("must refuse before evaluating anything")

    with pytest.raises(ValueError, match="15"):
        support_enumeration(Huge())


# --- 5. constant-sum ----------------------------------------------------------------------------------------


@pytest.mark.parametrize(("n", "m"), [(3, 4), (4, 3), (5, 4)])
def test_constant_sum_game_pays_one_over_n_to_a_player_whatever_sigma(n: int, m: int) -> None:
    game = constant_sum_game(n, m, seed=n + m)
    for seed, zeros in [(0, ()), (1, ()), (2, (0,)), (3, (1, 2))]:
        sigma = random_sigma(m, seed, zeros)
        assert mean_payoff(game, sigma) == pytest.approx(1.0 / n, abs=1e-12)
    pure = np.eye(m)[1]
    assert mean_payoff(game, pure) == pytest.approx(1.0 / n, abs=1e-12)


# --- 6. MixtureGame -----------------------------------------------------------------------------------------


def test_single_component_mixture_equals_the_plain_game() -> None:
    game = random_game(4, 5, seed=2)
    mixture = MixtureGame([game])
    np.testing.assert_allclose(mixture.weights, [1.0])
    assert mixture.m == game.m
    for seed, zeros in [(0, ()), (1, ()), (2, (1,))]:
        sigma = random_sigma(5, seed, zeros)
        np.testing.assert_allclose(
            mixture.expected_payoffs(sigma), game.expected_payoffs(sigma), rtol=0, atol=1e-12
        )
        assert deviation_gain(mixture, sigma) == pytest.approx(deviation_gain(game, sigma), abs=1e-12)
    assert MixtureGame([game], weights=[7.5]).weights[0] == 1.0
    small = random_game(3, 4, seed=3)
    plain = support_enumeration(small)
    through_mixture = support_enumeration(MixtureGame([small]))
    assert len(plain) == len(through_mixture) > 0
    for a, b in zip(plain, through_mixture, strict=True):
        np.testing.assert_allclose(a.sigma, b.sigma, rtol=0, atol=1e-9)


def test_mixture_weights_are_normalised_and_validated() -> None:
    games = [random_game(n, 3, seed=n) for n in (3, 4, 5)]
    np.testing.assert_allclose(MixtureGame(games).weights, np.full(3, 1.0 / 3.0))
    mixture = MixtureGame(games, [1.0, 2.0, 1.0])
    np.testing.assert_allclose(mixture.weights, [0.25, 0.5, 0.25])
    sigma = random_sigma(3, 4)
    expected = sum(w * g.expected_payoffs(sigma) for w, g in zip([0.25, 0.5, 0.25], games, strict=True))
    np.testing.assert_allclose(mixture.expected_payoffs(sigma), expected, rtol=0, atol=1e-12)
    dropped = MixtureGame(games, [1.0, 0.0, 1.0])  # a zero weight switches a game off
    np.testing.assert_allclose(
        dropped.expected_payoffs(sigma),
        0.5 * (games[0].expected_payoffs(sigma) + games[2].expected_payoffs(sigma)),
        rtol=0,
        atol=1e-12,
    )
    for bad in ([1.0, -1.0, 1.0], [0.0, 0.0, 0.0], [1.0, 1.0], [1.0, math.nan, 1.0]):
        with pytest.raises(ValueError, match="weights"):
            MixtureGame(games, bad)
    with pytest.raises(ValueError, match="same"):
        MixtureGame([games[0], random_game(3, 4, seed=1)])
    with pytest.raises(ValueError, match="at least one"):
        MixtureGame([])


def test_solve_all_on_a_mixture_reports_gains_the_independent_verifier_confirms() -> None:
    m = 4
    games = [random_game(n, m, seed=20 + n) for n in (3, 4, 5)]
    mixture = MixtureGame(games, weights=[0.5, 0.3, 0.2])
    solutions = solve_all(mixture, n_players=4, quick=True)
    assert {s.method for s in solutions} == {
        "replicator",
        "regret_matching",
        "nash_conv_minimizer",
        "support_enumeration",
    }
    for solution in solutions:
        checked = verify_regret(mixture.raw(), m, solution.sigma)
        assert checked["method"] == "enumerate"
        assert solution.gain == pytest.approx(checked["gain"], abs=1e-9)
        assert solution.sigma.sum() == pytest.approx(1.0, abs=1e-9)
        assert solution.extra["nash_conv"] == pytest.approx(4.0 * solution.gain, abs=1e-12)
    assert min(s.gain for s in solutions) < 1e-9
    assert all(s.gain <= 1e-7 for s in solutions if s.method == "support_enumeration")
    assert all("nash_conv" not in s.extra for s in solve_all(mixture, quick=True))  # no single n known


# --- 7. restrict, missing multisets, malformed tables -------------------------------------------------------


def test_restrict_keeps_payoffs_consistent() -> None:
    n, m = 4, 6
    game = SymmetricGame(n, m, random_payoffs(n, m, seed=3), names=list("abcdef"))
    keep = [4, 1, 3]  # unsorted on purpose: the sub-game is indexed in the order given
    sub = game.restrict(keep)
    assert (sub.n, sub.m, sub.names) == (n, 3, ["e", "b", "d"])
    for i in range(3):
        for others in itertools.combinations_with_replacement(range(3), n - 1):
            assert sub.payoff(i, others) == game.payoff(keep[i], [keep[o] for o in others])
    for sigma in (random_sigma(3, 1), random_sigma(3, 2, zeros=(1,))):
        embedded = np.zeros(m)
        embedded[keep] = sigma
        expected = game.expected_payoffs(embedded)[keep]
        np.testing.assert_allclose(sub.expected_payoffs(sigma), expected, rtol=0, atol=1e-12)
    everything = game.restrict(list(range(m)))
    sigma = random_sigma(m, 5)
    np.testing.assert_allclose(everything.expected_payoffs(sigma), game.expected_payoffs(sigma), atol=1e-12)
    for bad in ([], [1, 1], [m], [-1]):
        with pytest.raises(ValueError, match="keep"):
            game.restrict(bad)


def test_missing_multisets_raise_by_default_and_report_how_many() -> None:
    n, m = 3, 3
    table = random_payoffs(n, m, seed=2)
    total = math.comb(m + n - 1, n)
    gone = {(2, 2, 2), (0, 1, 2), (1, 1, 2)}
    partial = {k: v for k, v in table.items() if k not in gone}
    with pytest.raises(ValueError, match=f"3 of {total}"):
        SymmetricGame(n, m, partial)
    with pytest.raises(ValueError, match=f"{total} of {total}"):
        SymmetricGame(n, m, {})
    assert SymmetricGame(n, m, partial, allow_missing=True).n_missing == 3


def test_allow_missing_defers_the_error_to_the_first_use_of_a_missing_entry() -> None:
    n, m = 3, 3
    table = random_payoffs(n, m, seed=2)
    complete = SymmetricGame(n, m, table)
    game = SymmetricGame(n, m, {k: v for k, v in table.items() if k != (2, 2, 2)}, allow_missing=True)
    assert game.n_missing == 1
    assert math.isnan(game.payoff(2, (2, 2)))
    assert game.payoff(2, (0, 2)) == complete.payoff(2, (0, 2))
    avoiding = np.array([0.5, 0.5, 0.0])  # the missing entry has probability sigma_2**2 = 0
    np.testing.assert_allclose(game.expected_payoffs(avoiding), complete.expected_payoffs(avoiding))
    for hits in (np.full(3, 1.0 / 3.0), np.array([0.0, 0.0, 1.0])):
        with pytest.raises(ValueError, match="missing"):
            game.expected_payoffs(hits)
        with pytest.raises(ValueError, match="missing"):
            deviation_gain(game, hits)
    assert game.restrict([0, 1]).n_missing == 0
    assert game.restrict([1, 2]).n_missing == 1


def test_malformed_payoff_tables_are_rejected() -> None:
    table = random_payoffs(3, 3, seed=1)
    for key, value in [((1, 0, 2), [0.0] * 3), ((0, 0, 3), [0.0] * 3), ((0, 1, 2), [0.0] * 2)]:
        broken = {k: v for k, v in table.items() if k != (0, 1, 2)}
        broken[key] = value
        with pytest.raises(ValueError, match="sorted|payoffs"):
            SymmetricGame(3, 3, broken)
    with pytest.raises(ValueError, match="names"):
        SymmetricGame(3, 3, table, names=["only", "two"])


def test_positions_holding_the_same_strategy_are_averaged() -> None:
    table = random_payoffs(3, 2, seed=1)
    table[(0, 0, 1)] = [1.0, 3.0, 5.0]
    game = SymmetricGame(3, 2, table)
    assert game.payoff(0, (0, 1)) == pytest.approx(2.0)  # strategy 0 holds positions 0 and 1: mean(1, 3)
    assert game.payoff(0, (1, 0)) == pytest.approx(2.0)
    assert game.payoff(1, (0, 0)) == pytest.approx(5.0)


def test_from_function_passes_sorted_opponents_and_payoff_accepts_any_order() -> None:
    calls: list[tuple[int, tuple[int, ...]]] = []

    def fn(i: int, others: tuple[int, ...]) -> float:
        calls.append((i, others))
        return 10.0 * i + sum(others) + 0.01 * others[0]

    game = SymmetricGame.from_function(3, 3, fn, names=["x", "y", "z"])
    assert calls and all(len(o) == 2 and list(o) == sorted(o) for _, o in calls)
    assert game.names == ["x", "y", "z"]
    for i, others in itertools.product(range(3), itertools.combinations_with_replacement(range(3), 2)):
        assert game.payoff(i, others) == pytest.approx(fn(i, others))
        assert game.payoff(i, others[::-1]) == game.payoff(i, others)
    assert SymmetricGame.from_function(3, 2, lambda i, others: 0.0).names == ["0", "1"]


# --- 8. bootstrap, helpers ----------------------------------------------------------------------------------


def test_bootstrap_gain_of_a_fixed_sigma_matches_deviation_gain() -> None:
    game = random_game(4, 5, seed=8)
    sigma = random_sigma(5, 3, zeros=(2,))
    assert bootstrap_gain(sigma, [game])[0] == pytest.approx(deviation_gain(game, sigma), abs=1e-12)
    resampled = [random_game(4, 5, seed=s) for s in range(30, 33)] + [ThinOracle(game)]
    gains = bootstrap_gain(sigma, resampled)
    assert gains.shape == (4,)
    np.testing.assert_allclose(gains, [deviation_gain(g, sigma) for g in resampled], rtol=0, atol=1e-12)
    assert gains[-1] == pytest.approx(deviation_gain(game, sigma), abs=1e-12)
    assert len(set(np.round(gains, 6))) == 4  # different resampled games give different gains


def test_project_simplex_is_the_euclidean_projection() -> None:
    np.testing.assert_allclose(project_simplex(np.array([2.0, 0.0])), [1.0, 0.0])
    np.testing.assert_allclose(project_simplex(np.array([1.0, 1.0, 1.0])), UNIFORM3)
    point = random_sigma(5, 1)
    np.testing.assert_allclose(project_simplex(point), point, rtol=0, atol=1e-15)
    rng = np.random.default_rng(0)
    for _ in range(20):
        x = rng.normal(scale=2.0, size=6)
        p = project_simplex(x)
        assert p.min() >= 0.0
        assert p.sum() == pytest.approx(1.0, abs=1e-12)
        # KKT: x - p equals one common tau on the support and is at most tau off it
        tau = float((x - p)[p > 0][0])
        np.testing.assert_allclose((x - p)[p > 0], tau, rtol=0, atol=1e-12)
        assert (x - p)[p == 0].max(initial=-np.inf) <= tau + 1e-12


def test_pure_nash_lists_every_strategy_that_is_a_best_response_to_itself() -> None:
    coordination = SymmetricGame.from_function(3, 3, lambda i, o: sum(j == i for j in o) / len(o))
    assert pure_nash(coordination) == [0, 1, 2]
    ties = SymmetricGame.from_function(3, 3, lambda i, o: 1.0 if i < 2 else 0.0)
    assert pure_nash(ties) == [0, 1]
    assert pure_nash(rps_game(3)) == []
    assert pure_nash(ThinOracle(dominance_game())) == [0]


# --- extras: gradient, thin oracles, honest labels ----------------------------------------------------------


def finite_difference_vjp(game: PayoffOracle, sigma: np.ndarray, cot: np.ndarray) -> np.ndarray:
    """d/d theta_a of cot . u(softmax(theta)) at sigma, by central differences (zero entries stay zero)."""
    h = 1e-5

    def tilted(a: int, step: float) -> np.ndarray:
        moved = sigma.copy()
        moved[a] *= math.exp(step)
        return moved / moved.sum()

    grad = np.zeros(len(sigma))
    for a in range(len(sigma)):
        up, down = (game.expected_payoffs(tilted(a, s)) for s in (h, -h))
        grad[a] = cot @ (up - down) / (2.0 * h)
    return grad


@pytest.mark.parametrize("kind", ["plain", "mixture"])
def test_analytic_gradient_matches_central_differences(kind: str) -> None:
    m = 5
    plain = random_game(4, m, seed=3)
    game = plain if kind == "plain" else MixtureGame([plain, random_game(3, m, seed=4)], [0.7, 0.3])
    sigmas = np.stack([random_sigma(m, 1), random_sigma(m, 2, zeros=(0, 3))])
    cot = np.random.default_rng(0).normal(size=(2, 3, m))
    u, grad = game.payoffs_and_vjp(sigmas, lambda _: cot)
    assert grad.shape == (2, 3, m)
    for s in range(2):
        np.testing.assert_allclose(u[s], game.expected_payoffs(sigmas[s]), rtol=0, atol=1e-12)
        for c in range(3):
            expected = finite_difference_vjp(game, sigmas[s], cot[s, c])
            np.testing.assert_allclose(grad[s, c], expected, rtol=0, atol=1e-7)


def test_solvers_only_need_the_payoff_oracle_protocol() -> None:
    oracle = ThinOracle(interior_game())
    assert pure_nash(oracle) == []
    np.testing.assert_allclose(support_enumeration(oracle)[0].sigma, SIGMA_STAR, rtol=0, atol=1e-4)
    minimized = nash_conv_minimizer(oracle, starts=8, steps=300)
    np.testing.assert_allclose(minimized.sigma, SIGMA_STAR, rtol=0, atol=1e-4)
    assert replicator(oracle).gain < 1e-6
    assert regret_matching(oracle, iters=2000).gain < 1e-2
    solutions = solve_all(oracle, quick=True)
    assert all("nash_conv" not in s.extra for s in solutions)
    assert all(s.extra["nash_conv"] == pytest.approx(3.0 * s.gain) for s in solve_all(oracle, 3, quick=True))


@pytest.mark.parametrize("name", ["rps", "interior", "dominance"])
def test_every_solution_is_honestly_labelled_and_its_gain_is_exact(name: str) -> None:
    game = {"rps": rps_game(3), "interior": interior_game(), "dominance": dominance_game(3, 3)}[name]
    for solution in solve_all(game, quick=True):
        assert solution.sigma.shape == (game.m,)
        assert solution.sigma.min() >= 0.0
        assert solution.sigma.sum() == pytest.approx(1.0, abs=1e-9)
        assert solution.gain == pytest.approx(deviation_gain(game, solution.sigma), abs=1e-12)
        assert solution.extra["nash_conv"] == pytest.approx(game.n * solution.gain, abs=1e-12)
        if solution.method == "regret_matching":
            assert solution.concept.startswith("CCE")
        elif solution.method == "support_enumeration":
            assert solution.gain <= 1e-7
            assert "Nash equilibrium" in solution.concept
        elif "Nash candidate" in solution.concept:
            assert solution.gain <= 1e-6
        else:
            assert solution.gain > 1e-6
            assert "no Nash claim" in solution.concept


# --- regret matching against a from-the-definition reference ------------------------------------------------


def reference_regret_matching(
    table: Payoffs, n: int, m: int, iters: int
) -> tuple[np.ndarray, list[np.ndarray]]:
    """Symmetric regret matching written from its definition with the naive evaluator."""
    cumulative = np.zeros(m)
    visited = []
    for _ in range(iters):
        positive = np.maximum(cumulative, 0.0)
        sigma = positive / positive.sum() if positive.sum() > 0.0 else np.full(m, 1.0 / m)
        u = naive_expected_payoffs(table, n, m, sigma)
        cumulative += u - sigma @ u
        visited.append(sigma)
    return cumulative, visited


def cce_gap_of_visited_mixture(table: Payoffs, n: int, m: int, visited: list[np.ndarray]) -> float:
    """max_i E_mu[u_0(i, a_-0) - u_0(a)] for mu = mean_t sigma_t^(x n), summed over all m**n profiles."""
    gap = np.zeros(m)
    for profile in itertools.product(range(m), repeat=n):
        mu = float(np.mean([math.prod(float(s[a]) for a in profile) for s in visited]))
        key = tuple(sorted(profile))
        current = table[key][key.index(profile[0])]
        for i in range(m):
            deviated = tuple(sorted((i, *profile[1:])))
            gap[i] += mu * (table[deviated][deviated.index(i)] - current)
    return float(gap.max())


def test_regret_matching_reports_the_exact_cce_gap_of_the_visited_mixture() -> None:
    n, m, iters = 3, 3, 60
    table = random_payoffs(n, m, seed=9)
    cumulative, visited = reference_regret_matching(table, n, m, iters)
    solution = regret_matching(SymmetricGame(n, m, table), iters=iters)
    np.testing.assert_allclose(solution.sigma, np.mean(visited, axis=0), rtol=0, atol=1e-12)
    assert solution.extra["cce_gap"] == pytest.approx(cumulative.max() / iters, abs=1e-12)
    gap = cce_gap_of_visited_mixture(table, n, m, visited)
    assert solution.extra["cce_gap"] == pytest.approx(gap, abs=1e-12)
    assert solution.iterations == iters


# --- the independent verifier -------------------------------------------------------------------------------


def test_verify_regret_sampling_agrees_with_the_exact_value_within_its_standard_error() -> None:
    n, m = 5, 12  # 12**4 = 20736 ordered opponent tuples: just above the enumeration limit
    table = random_payoffs(n, m, seed=4)
    game = SymmetricGame(n, m, table)
    sigma = random_sigma(m, 6, zeros=(5,))
    sampled = verify_regret([(1.0, table, n)], m, sigma, samples=40_000, seed=3)
    assert sampled["method"] == "sampled"
    assert 0.0 < sampled["stderr"] < 0.05
    worst = int(np.argmax(sampled["regrets"]))
    exact_regret = regrets(game, sigma)[worst]
    assert abs(sampled["regrets"][worst] - exact_regret) < 5.0 * sampled["stderr"]
    assert np.abs(sampled["u"] - game.expected_payoffs(sigma)).max() < 0.05
    assert sampled["gain"] == pytest.approx(max(0.0, sampled["regrets"].max()), abs=1e-15)
    again = verify_regret([(1.0, table, n)], m, sigma, samples=40_000, seed=3)
    np.testing.assert_array_equal(again["u"], sampled["u"])
    exact = verify_regret([(1.0, table, n)], m, sigma, method="enumerate")
    assert exact["method"] == "enumerate"
    assert exact["stderr"] is None
    np.testing.assert_allclose(exact["u"], game.expected_payoffs(sigma), rtol=0, atol=1e-9)
    assert exact["mean"] == pytest.approx(mean_payoff(game, sigma), abs=1e-9)


def test_verify_regret_auto_enumerates_small_games_and_validates_input() -> None:
    table = random_payoffs(3, 3, seed=1)
    uniform = np.full(3, 1.0 / 3.0)
    assert verify_regret([(1.0, table, 3)], 3, uniform)["method"] == "enumerate"
    with pytest.raises(ValueError, match="probability vector"):
        verify_regret([(1.0, table, 3)], 3, [0.5, 0.2, 0.2])
    with pytest.raises(ValueError, match="method"):
        verify_regret([(1.0, table, 3)], 3, uniform, method="bogus")
    partial = {k: v for k, v in table.items() if k != (2, 2, 2)}
    with pytest.raises(ValueError, match="missing"):
        verify_regret([(1.0, partial, 3)], 3, uniform)
    unaffected = verify_regret([(1.0, partial, 3)], 3, [0.5, 0.5, 0.0])  # (2, 2, 2) is never needed
    assert np.isfinite(unaffected["u"]).all()


def test_verifier_uses_no_game_or_solver_code() -> None:
    """verify_regret and its helpers may touch only numpy, itertools, math and each other."""
    tree = ast.parse(inspect.getsource(mg))
    defined: set[str] = set()
    for node in tree.body:
        if isinstance(node, ast.FunctionDef | ast.ClassDef):
            defined.add(node.name)
        elif isinstance(node, ast.Assign):
            defined.update(t.id for t in node.targets if isinstance(t, ast.Name))
        elif isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name):
            defined.add(node.target.id)
    verifier = [
        node
        for node in tree.body
        if isinstance(node, ast.FunctionDef) and node.name.startswith(("verify_regret", "_verify_"))
    ]
    own = {node.name for node in verifier} | {"_VERIFY_ENUMERATE_LIMIT"}
    assert {"verify_regret", "_verify_vector", "_verify_enumerated", "_verify_drawn"} <= own
    used = {n.id for fn in verifier for n in ast.walk(fn) if isinstance(n, ast.Name)}
    assert used & defined <= own, sorted((used & defined) - own)
