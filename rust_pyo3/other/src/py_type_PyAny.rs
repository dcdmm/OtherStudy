use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::HashMap;

#[test]
fn PyAny0() -> PyResult<()> {
    Python::with_gil(|py| {
        /*
        Represents any Python object.

        It currently only appears as a reference, &PyAny, with a lifetime that represents the scope during which the GIL is held.
         */

        let py_dict = PyDict::new_bound(py);
        py_dict.set_item("a", 1);

        /*
        Checks whether this object is an instance of exactly type ty (not a subclass).

        This is equivalent to the Python expression type(self) is ty.
         */
        println!("{}", py_dict.is_instance_of::<PyAny>()); // print->true

        let py_any = py_dict.as_any();
        
        // Returns a GIL marker constrained to the lifetime of this type.
        let python_ = py_any.py();

        // Downcast this PyAny to a concrete Python type or pyclass.
        println!("{:?}", py_any.downcast::<PyDict>()); // print->Ok({})
        println!("{:?}", py_any.downcast::<PyList>()); // print->Err(DowncastError { from: {}, to: "PyList" })

        /*
        Extracts some type from the Python object.

        This is a wrapper function around FromPyObject::extract().
         */
        let rust_map0: HashMap<String, i32> = py_any.extract()?;
        println!("{:?}", rust_map0); // print->{"a": 1}
        let rust_map1 = py_any.extract::<HashMap<String, i32>>();
        println!("{:?}", rust_map1); // print->Ok({"a": 1})
        Ok(())
    })
}

#[test]
fn PyAny_call0_attr() -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<()> {
        let module = PyModule::import_bound(py, "builtins")?;

        /*
        Retrieves an attribute value.

        This is equivalent to the Python expression self.attr_name.
         */
        let help = module.getattr("help")?;

        // Calls the object without arguments. This is equivalent to the Python expression self().
        help.call0()?;

        Ok(())
    })
}

#[test]
fn PyAny_call1() -> PyResult<()> {
    const CODE: &str = r#"
def function(*args, **kwargs):
    assert args == ("hello",)
    assert kwargs == {}
    return "called with args"
"#;
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, CODE, "", "")?;
        let fun = module.getattr("function")?;
        let args = ("hello",);

        // Calls the object with only positional arguments. This is equivalent to the Python expression self(*args).
        let result = fun.call1(args)?;
        println!("{:?}", result.extract::<String>()); // print->k("called with args")

        Ok(())
    })
}

#[test]
fn PyAny_call_method0() -> PyResult<()> {
    const CODE: &str = r#"
class A:
    def method(self, *args, **kwargs):
        assert args == ()
        assert kwargs == {}
        return "called with no arguments"
a = A()
"#;
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, CODE, "", "")?;
        let instance = module.getattr("a")?;

        // Calls a method on the object without arguments. This is equivalent to the Python expression self.name().
        let result = instance.call_method0("method")?;
        println!("{:?}", result.extract::<String>()); // print->Ok("called with no arguments")

        Ok(())
    })
}

#[test]
fn PyAny_call_method1() -> PyResult<()> {
    const CODE: &str = r#"
class A:
    def method(self, *args, **kwargs):
        assert args == ("hello",)
        assert kwargs == {}
        return "called with args"
a = A()
"#;

    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, CODE, "", "")?;
        let instance = module.getattr("a")?;
        let args = ("hello",);

        // Calls a method on the object with only positional arguments. This is equivalent to the Python expression self.name(*args).
        let result = instance.call_method1("method", args)?;
        println!("{:?}", result.extract::<String>()); // print->Ok("called with args")

        Ok(())
    })
}
