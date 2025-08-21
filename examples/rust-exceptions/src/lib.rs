// -----------------------------------------------------------------------------
// Rust-Python Exception Example (lib.rs)
// This file defines a Rust function that can be called from Python using PyO3.
// The function `divide` performs division and raises a Python ZeroDivisionError
// if the denominator is zero. The #[pymodule] section exposes this function as
// a Python module named `librust_exceptions`.
//
// Example usage:
//   make build
//   python3 ./pydiv.py
//
// This prints librust_exceptions.divide(10, 2) and
// raises a ZeroDivisionError when dividing by zero, e.g.
// librust_exceptions.divide(10, 0)
//
// -----------------------------------------------------------------------------

use pyo3::exceptions::PyZeroDivisionError;
use pyo3::prelude::*;

/// Divides a by b
/// Raises ZeroDivisionError if b is zero
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyZeroDivisionError::new_err("Exception: !Division by Zero"));
    }
    Ok(a / b)
}

/// A Python module implemented in Rust.
#[pymodule]
fn librust_exceptions(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}
