use pyo3::prelude::*;

#[pyclass]
pub struct MC {
    #[pyo3(get, set)]
    my_field: i32,
}

#[pymethods]
impl MC {
    #[new]
    fn new(value: i32) -> Self {
        MC { my_field: value }
    }
}

#[pyfunction]
pub fn rust_struct(mc: &mut MC) {
    mc.my_field += 1;
    println!("rust_struct: {}", mc.my_field);
}

#[pyfunction]
pub fn pyref_(mc: PyRef<'_, MC>) {
    println!("pyref_: {}", mc.my_field);
}

#[pyfunction]
pub fn pyrefmut_(mc: &Bound<'_, MC>) {
    let mut prm: PyRefMut<MC> = mc.borrow_mut();
    prm.my_field += 1;

    println!("pyrefmut_: {}", prm.my_field);
}

#[pyfunction]
pub fn print_refcnt(mc: Py<MC>, py: Python<'_>) {
    println!("print_refcnt: {}", mc.get_refcnt(py));
}

#[pyclass]
#[derive(Clone)]
pub struct MC1 {
    my_field: Box<i32>,
}

// Classes can also be passed by value if they can be cloned, i.e. they automatically implement FromPyObject if they implement Clone, e.g. via #[derive(Clone)]:
#[pymethods]
impl MC1 {
    #[new]
    fn new(value: i32) -> Self {
        MC1 {
            my_field: Box::new(value),
        }
    }
}

#[pyfunction]
pub fn dissamble_clone(my_class: MC1) {
    let MC1 { mut my_field } = my_class;
    *my_field += 1;
    println!("dissamble_clone: {}", my_field);
}
