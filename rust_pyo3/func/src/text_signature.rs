use pyo3::prelude::*;

#[pyclass]
pub struct MyStruct {
    value: i32,
}

#[pymethods]
impl MyStruct {
    #[new]
    fn new(value: i32) -> Self {
        MyStruct { value }
    }

    /*
    The function signature is exposed to Python via the __text_signature__ attribute. PyO3 automatically generates this for every #[pyfunction] and all #[pymethods] directly from the Rust function, taking into account any override done with the #[pyo3(signature = (...))] option.

    This automatic generation can only display the value of default arguments for strings, integers, boolean types, and None. Any other default arguments will be displayed as ....(.pyi type stub files commonly also use ... for default arguments in the same way.)
     */
    #[pyo3(signature = (x, z, y=vec![1, 2]))]
    fn add(&self, x: i32, z: &str, y: Vec<i32>) -> i32 {
        println!("{}", z);
        self.value + x + y[0] + y[1]
    }

    #[pyo3(signature = (x, z, y=vec![1, 2]))]
    // The #[pyo3(text_signature = "(<some signature>)")] attribute can be used to override the default generated signature.
    #[pyo3(text_signature = "(self, x, sentence, y=[1, 2])")] // 用于增强生成的python文档的可读性和易用性
    fn sub(&self, x: i32, z: &str, y: Vec<i32>) -> i32 {
        println!("{}", z);
        self.value - x - y[0] - y[1]
    }

    #[pyo3(signature = (x, z, y=vec![1, 2]))]
    // If no signature is wanted at all, #[pyo3(text_signature = None)] will disable the built-in signature.
    #[pyo3(text_signature = None)]
    fn mul(&self, x: i32, z: &str, y: Vec<i32>) -> i32 {
        self.value * x * y[0] * y[1]
    }
}
