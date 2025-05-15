use pyo3::prelude::*;

#[pyfunction]
fn fibonacci_rust(n: u64) -> PyResult<u64> {
    Ok(match n {
        0 => 0,
        1 => 1,
        n => fibonacci_rust(n - 1)? + fibonacci_rust(n - 2)?,
    })
}

#[pymodule]
fn rust_python_fib(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci_rust, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_rust(0).unwrap(), 0);
        assert_eq!(fibonacci_rust(1).unwrap(), 1);
        assert_eq!(fibonacci_rust(2).unwrap(), 1);
        assert_eq!(fibonacci_rust(3).unwrap(), 2);
        assert_eq!(fibonacci_rust(4).unwrap(), 3);
        assert_eq!(fibonacci_rust(5).unwrap(), 5);
    }
}
