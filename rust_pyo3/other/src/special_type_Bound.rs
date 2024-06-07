// A GIL-attached equivalent to Py.

use pyo3::prelude::*;

#[test]
fn t0() {
    #[pyclass]
    struct Foo {/* fields omitted */}
    Python::with_gil(|py| {
        // Creates a new instance Bound<T> of a #[pyclass] on the Python heap.
        let foo: Bound<'_, Foo> = Bound::new(py, Foo {}).unwrap();


        
    });
}
