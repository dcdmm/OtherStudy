/*
Represents a Python module object.

As with all other Python objects, modules are first class citizens. This means they can be passed to or returned from functions, created dynamically, assigned to variables and so forth.
*/

use pyo3::prelude::*;

// Methods from Deref<Target = PyAny>
#[test]
fn t0() {
    Python::with_gil(|py| {
        // Creates a new module object with the __name__ attribute set to name.
        let module = PyModule::new_bound(py, "my_module").unwrap();
        println!("{:?}", module.name()); // print->Ok('my_module')

        // This is equivalent to the following Python expression: `import antigravity`
        let module1 = PyModule::import_bound(py, "antigravity").unwrap();
        println!("{:?}", module1.name()); // print->Ok('antigravity')

        const CODE: &str = r#"
def function(x):
    return -x
"#;
        // Creates and loads a module named module_name, containing the Python code passed to code and pretending to live at file_name
        let module2 = PyModule::from_code_bound(py, CODE, "example.py", "my_module2").unwrap();
        println!("{:?}", module2.name()); // print->Ok('my_module2')
    })
}

/// # add
///
/// Adds an attribute to the module.
///
/// For adding classes, functions or modules, prefer to use PyModule::add_class, PyModule::add_function or PyModule::add_submodule instead, respectively.
///
/// ## Examples
/// ```rust
/// use pyo3::prelude::*;
/// #[pymodule]
/// fn my_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
/// module.add("c", 299_792_458)?;
/// Ok(())
/// }
/// ```
///
/// Python code can then do the following:
///
/// ```python
/// from my_module import c
///
/// print("c is", c) // print->c is 299792458
/// ```
///
///
/// # add_class(详情见GitHubProjects\OtherStudy\rust_pyo3\class_)
///
/// Adds a new class to the module.
///
///
/// # add_function(详情见GitHubProjects\OtherStudy\rust_pyo3\func)
///
/// Add a function to a module.
///
///
/// # add_module
///
/// Adds a submodule to a module.
///
/// This is especially useful for creating module hierarchies.
///
/// Note that this doesn’t define a package, so this won’t allow Python code to directly import submodules by using from my_module import submodule.
///
/// ## Example
/// ```rust
/// use pyo3::prelude::*;
///
/// #[pymodule]
/// fn my_module(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
///     let submodule = PyModule::new_bound(py, "submodule")?;
///     submodule.add("super_useful_constant", "important")?;
///
///     module.add_submodule(&submodule)?;
///     Ok(())
/// }
/// ```
///
/// Python code can then do the following:
///
/// ```python
/// import my_module
///
/// print("super_useful_constant is", my_module.submodule.super_useful_constant) // print->super_useful_constant is important
/// ```
///
/// # add_wrapped
///
/// Adds a function or a (sub)module to a module, using the functions name as name.
///
/// Prefer to use PyModule::add_function and/or PyModule::add_submodule instead.
fn module_add() {}
