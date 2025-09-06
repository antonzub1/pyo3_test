import math

import pytest

from stat_functions import mean

def test_mean_ok():
    values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert(mean(values)) == 5.5


def test_mean_empty():
    with pytest.raises(ValueError, match="Cannot compute mean value of empty array"):
        mean([])


def test_mean_nan():
    values = [1, 2, 3, 4, float("nan"), 6, 7, 8, 9, 10];
    assert math.isnan(mean(values))


def test_mean_inf():
    values = [1, 2, 3, 4, float("+inf"), 6, 7, 8, 9, 10];
    assert mean(values) == float("+inf")


def test_mean_neg_inf():
    values = [1, 2, 3, 4, float("-inf"), 6, 7, 8, 9, 10];
    assert mean(values) == float("-inf")
