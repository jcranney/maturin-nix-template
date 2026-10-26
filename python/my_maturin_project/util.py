from numpy.typing import NDArray
from my_maturin_project import PythonicClass


def one_shot_sum(array: NDArray) -> float | None:
    my_instance = PythonicClass(0.0)
    result = my_instance.numpy_array_add_one(array)
    return result


def an_untested_method() -> None:
    print("this method never gets tested, so should be flagged by pytest-cov")
