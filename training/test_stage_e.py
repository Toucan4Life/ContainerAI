import numpy as np

from training import egta, stage_e


def test_null_variants_reproduce_the_baseline_and_a_price_change_moves_only_the_deviating_games(tmp_path):
    base = tmp_path / "base"
    egta.run(base, seeds=2, log=lambda *_: None)
    stage_e.run(base, tmp_path / "out", log=lambda *_: None)  # raises unless both null variants equal the baseline game for game
    with np.load(tmp_path / "out" / "variants.npz") as z:
        # the deviating Producer seat is a Producer seat of the baseline game: only the price lots differ
        assert np.array_equal(z["v0_pos"], z["b0_pos"])
        assert z["v0_score"].shape == z["v1_score"].shape
        assert not np.array_equal(z["v1_produced"], z["b1_produced"], equal_nan=True)
        assert np.array_equal(z["b1_produced"], z["b0_produced"], equal_nan=True)  # the baseline side does not depend on the variant
    text = stage_e.render({k: v for k, v in np.load(tmp_path / "out" / "variants.npz").items()}, base)
    assert "Summary against the pre-registered rule" in text
