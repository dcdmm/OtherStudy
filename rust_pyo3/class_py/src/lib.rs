#![allow(non_snake_case)]
#![allow(warnings)]

mod base;
mod class_as_function_arguments;
mod magic_methods;
mod method_arguments;
mod pyclass_para;

use pyo3::{class, prelude::*};

#[pymodule]
fn class_py(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<base::Number>()?;
    m.add_class::<base::Nonzero>()?;
    m.add_class::<method_arguments::MyClass>()?;
    m.add_class::<class_as_function_arguments::MC>()?;
    m.add_function(wrap_pyfunction!(
        class_as_function_arguments::rust_struct,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(class_as_function_arguments::pyref_, m)?)?;
    m.add_function(wrap_pyfunction!(class_as_function_arguments::pyrefmut_, m)?)?;
    m.add_function(wrap_pyfunction!(
        class_as_function_arguments::print_refcnt,
        m
    )?)?;
    m.add_class::<class_as_function_arguments::MC1>()?;
    m.add_function(wrap_pyfunction!(
        class_as_function_arguments::dissamble_clone,
        m
    )?)?;
    m.add_class::<magic_methods::SimpleIterator>()?;
    m.add_class::<pyclass_para::PyPeople>()?;
    m.add_class::<pyclass_para::PyStudy>()?;

    Ok(())
}
