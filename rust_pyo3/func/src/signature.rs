use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

#[pyfunction]
#[pyo3(signature = (num0, num1=-1))] // 默认参数num1=-1
pub fn default_para(num0: i32, num1: i32) -> i32 {
    num0 + num1
}

#[pyfunction]
#[pyo3(signature = (num0, *py_args))]
pub fn args_para(num0: i32, py_args: &Bound<'_, PyTuple>) {
    println!("一般参数num0: {}", num0);
    for args in py_args {
        println!("可变参数*args: {}", args);
    }
}

#[pyfunction]
#[pyo3(signature = (num0, **py_kwargs))]
pub fn kwargs_prara(num0: i32, py_kwargs: Option<&Bound<'_, PyDict>>) {
    println!("一般参数num0: {}", num0);
    if let Some(py_kwargs) = py_kwargs {
        for (k, v) in py_kwargs {
            println!("可变参数**kwargs: {:?} {:?}", k, v);
        }
    }
}
