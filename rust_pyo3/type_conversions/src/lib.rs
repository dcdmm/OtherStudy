#![allow(non_snake_case)]
#![allow(warnings)]

use pyo3::prelude::*;

mod argument_types;

#[pymodule]
fn type_conversions(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(argument_types::str_test0, m)?)?;
    m.add_function(wrap_pyfunction!(argument_types::str_test1, m)?)?;

    m.add_function(wrap_pyfunction!(argument_types::list_test0, m)?)?;
    m.add_function(wrap_pyfunction!(argument_types::list_test1, m)?)?;

    m.add_function(wrap_pyfunction!(argument_types::dict_test0, m)?)?;
    m.add_function(wrap_pyfunction!(argument_types::dict_test1, m)?)?;
    
    m.add_function(wrap_pyfunction!(argument_types::set_test0, m)?)?;
    m.add_function(wrap_pyfunction!(argument_types::set_test1, m)?)?;

    Ok(())
}
