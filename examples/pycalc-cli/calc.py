#!/usr/bin/env python3
"""
PyCalc CLI using Python Fire
"""
import fire

from libpycalc_cli import (
    sum_as_string,
    subtract_as_string,
    multiply_as_string,
    divide_as_string,
    sqrt_as_string,
    power_as_string,
)
import math

class Calculator(object):
    """Rust Calculator Class

    Methods:
        sum(num1, num2): Add two numbers.
        subtract(num1, num2): Subtract two numbers.
        multiply(num1, num2): Multiply two numbers.
        divide(num1, num2): Divide two numbers.
        sqrt(num): Square root of a number.
        power(base, exp): Exponentiation (base^exp).

    Usage:
        $ python calc.py sum 2 3
        $ python calc.py subtract 5 2
        $ python calc.py multiply 4 3
        $ python calc.py divide 10 2
        $ python calc.py sqrt 9
        $ python calc.py power 2 3
        $ python calc.py power 9 0.5
    """

    def _validate_number(self, value, name="number"):
        try:
            return float(value)
        except ValueError:
            raise ValueError(f"Invalid {name}: {value}. Must be a number.")

    def sum(self, num1, num2):
        """Add two numbers"""
        num1 = self._validate_number(num1, "num1")
        num2 = self._validate_number(num2, "num2")
        return sum_as_string(num1, num2)

    def subtract(self, num1, num2):
        """Subtract two numbers"""
        num1 = self._validate_number(num1, "num1")
        num2 = self._validate_number(num2, "num2")
        return subtract_as_string(num1, num2)

    def multiply(self, num1, num2):
        """Multiply two numbers"""
        num1 = self._validate_number(num1, "num1")
        num2 = self._validate_number(num2, "num2")
        return multiply_as_string(num1, num2)

    def divide(self, num1, num2):
        """Divide two numbers"""
        num1 = self._validate_number(num1, "num1")
        num2 = self._validate_number(num2, "num2")
        if num2 == 0:
            raise ZeroDivisionError("Division by zero is not allowed.")
        return divide_as_string(num1, num2)

    def sqrt(self, num):
        """Square root of a number"""
        num = self._validate_number(num)
        if num < 0:
            raise ValueError("Cannot take square root of negative number.")
        return sqrt_as_string(num)

    def power(self, base, exp):
        """Exponentiation (base^exp)"""
        base = self._validate_number(base, "base")
        exp = self._validate_number(exp, "exp")
        return power_as_string(base, exp)

if __name__ == "__main__":
    fire.Fire(Calculator)