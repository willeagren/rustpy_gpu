#[allow(unused_imports)]
use pyo3::prelude::*;
use numpy::{PyArrayDyn, dot};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn multiply(a: usize, b: usize) -> PyResult<usize> {
    Ok(a*b)
}


/// A Python module implemented in Rust.
#[pymodule]
fn rustpy_gpu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
