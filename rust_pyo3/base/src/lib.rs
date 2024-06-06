mod other;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pyfunction]
fn sum_rust(n: i64) -> i64 {
    let mut result: i64 = 0;
    for _i in 0..n {
        for _j in 0..n {
            result += 1;
        }
    }
    result
}

// python模块名为该函数名(即base)
// 模块文档字符串为该函数文档注释(即"Rust函数文档注释")
/// Rust函数文档注释
#[pymodule]
// 函数名称对应该rust项目包名(即base,见Cargo.toml [package] name)
/*
Type Alias pyo3::PyResult

`pub type PyResult<T> = Result<T, PyErr>;`

sents the result of a Python call.
*/
fn base(_py: Python, m: &Bound<PyModule>) -> PyResult<()> { 
    m.add_function(wrap_pyfunction!(sum_rust, m)?)?;
    m.add_function(wrap_pyfunction!(other::triple, m)?)?;

    m.add_wrapped(wrap_pymodule!(other::child_module))?; // 添加子模块
    Ok(())
}