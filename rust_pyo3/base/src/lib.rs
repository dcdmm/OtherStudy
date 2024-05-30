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

/// Rust函数文档注释
#[pymodule]
// 函数名称对应该rust项目包名(即base,见Cargo.toml [package] name)
// python模块名为该函数名(即base)
// 模块文档字符串为base函数文档注释(即"Rust函数文档注释")
fn base(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_rust, m)?)?;
    Ok(())
}
