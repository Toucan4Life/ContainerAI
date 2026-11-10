import random

from training.elo import ratings


def synth(strength: dict[str, float], games: int, seed: int = 0) -> list[dict]:
    rng = random.Random(seed)
    names = list(strength)
    out = []
    for _ in range(games):
        seats = [rng.choice(names) for _ in range(rng.choice([3, 4, 5]))]
        scores = [round(strength[s] + rng.gauss(0, 20), 2) for s in seats]
        out.append({"seats": seats, "scores": scores})
    return out


def test_order_and_anchor():
    r = ratings(synth({"random": 0, "a": 15, "b": 40}, 3000), "random")
    assert r["random"] == 0
    assert r["random"] < r["a"] < r["b"]


def test_recovers_known_gap():
    # Gap 28.3 with score noise sd 20 each: P(win) = Phi(1) = 0.841, so 400*log10(0.841/0.159) = 289.
    r = ratings(synth({"random": 0, "a": 28.3}, 6000), "random")
    assert 250 < r["a"] < 330


def test_ties_are_even():
    games = [{"seats": ["x", "y", "z"], "scores": [1, 1, 1]}] * 50
    r = ratings(games, "x")
    assert abs(r["y"]) < 1 and abs(r["z"]) < 1


def test_rate_against_known_anchor():
    from training.elo import rate_against

    # Winning 76% against a 0-rated bot is about +200.
    res = [("a", 1.0)] * 76 + [("a", 0.0)] * 24
    assert 170 < rate_against({"a": 0.0}, res) < 230
