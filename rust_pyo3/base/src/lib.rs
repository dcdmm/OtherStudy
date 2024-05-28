use pyo3::prelude::*;

#[pyfunction]
fn sum_rust(n: i64) -> PyResult<i64> {
    let mut result: i64 = 0;
    for _i in 0..n {
        for _j in 0..n {
            result += 1;
        }
    }
    Ok(result)
}

/// base doc
#[pymodule]
fn base(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_rust, m)?)?;
    Ok(())
}