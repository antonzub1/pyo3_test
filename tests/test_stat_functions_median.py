import math
import statistics

import pytest

from stat_functions import median


@pytest.mark.parametrize("numbers, expected", [
    ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5.5),
    ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 6),
], ids=["even count", "odd count"])
def test_median_ok(numbers, expected):
    assert median(numbers) == expected

def test_median_empty():
    with pytest.raises(ValueError, match="Cannot compute median value of empty array"):
        median([])

def test_median_nan():
    values = [1, 2, 3, 4, float("nan"), 6, 7, 8, 9, 10];
    assert math.isnan(median(values))

def test_median_inf():
    values = [1, 2, 3, 4, float("+inf"), 6, 7, 8, 9, 10];
    assert median(values) == statistics.median(values)

def test_median_neg_inf():
    values = [1, 2, 3, 4, float("-inf"), 6, 7, 8, 9, 10];
    assert median(values) == statistics.median(values)

