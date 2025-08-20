import pytest

"""
# test_calc.py
# pytest /workspaces/rust-with-python/pycalc-cli/test_calc.py
"""

from libpycalc_cli import (
    sum_as_string,
    subtract_as_string,
    multiply_as_string,
    divide_as_string,
    sqrt_as_string,
    power_as_string,
)

def test_add():
    assert sum_as_string(2, 3) == "5"

def test_subtract():
    assert subtract_as_string(5, 2) == "3"

def test_multiply():
    assert multiply_as_string(4, 3) == "12"

def test_divide():
    assert divide_as_string(10, 2) == "5"
    with pytest.raises(Exception):
        divide_as_string(10, 0)

def test_sqrt():
    assert sqrt_as_string(9) == "3"
    with pytest.raises(Exception):
        sqrt_as_string(-1)

def test_power():
    assert power_as_string(2, 3) == "8"
    assert power_as_string(9, 0.5) == "3"