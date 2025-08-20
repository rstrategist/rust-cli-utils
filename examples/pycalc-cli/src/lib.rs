/*
Calculator functions to import into a Python Fire CLI.
*/

use pyo3::exceptions;
use pyo3::prelude::*;

/// Adds two numbers and returns the result as a string.
#[pyfunction]
fn sum_as_string(num1: f64, num2: f64) -> PyResult<String> {
    Ok((num1 + num2).to_string())
}

/// Subtracts num2 from num1 and returns the result as a string.
#[pyfunction]
fn subtract_as_string(num1: f64, num2: f64) -> PyResult<String> {
    Ok((num1 - num2).to_string())
}

/// Multiplies two numbers and returns the result as a string.
#[pyfunction]
fn multiply_as_string(num1: f64, num2: f64) -> PyResult<String> {
    Ok((num1 * num2).to_string())
}

/// Divides num1 by num2 and returns the result as a string.
/// Returns an error if division by zero is attempted.
#[pyfunction]
fn divide_as_string(num1: f64, num2: f64) -> PyResult<String> {
    if num2 == 0.0 {
        Err(PyErr::new::<exceptions::PyZeroDivisionError, _>(
            "Division by zero is not allowed.",
        ))
    } else {
        Ok((num1 / num2).to_string())
    }
}

/// Returns the square root of a number as a string.
/// Returns an error if the input is negative.
#[pyfunction]
fn sqrt_as_string(num: f64) -> PyResult<String> {
    if num < 0.0 {
        Err(PyErr::new::<exceptions::PyValueError, _>(
            "Cannot take square root of a negative number.",
        ))
    } else {
        Ok(num.sqrt().to_string())
    }
}

/// Raises base to the power of exp and returns the result as a string.
#[pyfunction]
fn power_as_string(base: f64, exp: f64) -> PyResult<String> {
    Ok(base.powf(exp).to_string())
}

/// Python module definition
#[pymodule]
fn libpycalc_cli(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(power_as_string, m)?)?;
    Ok(())
}
