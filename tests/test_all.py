import pytest


def test_basic_usage():
    import numpy as np
    from my_maturin_project.util import one_shot_sum

    x = np.arange(20, dtype=np.float64).reshape([5, 4])
    y = one_shot_sum(x)
    assert y == pytest.approx(190.0)
