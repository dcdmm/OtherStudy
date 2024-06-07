// A GIL-attached equivalent to Py.

// # Trait Implementations
/*
impl<'py, T> Deref for Bound<'py, T> where T: DerefToPyAny,

impl<T> Drop for Bound<'_, T>
*/

use std::borrow::Borrow;

use pyo3::prelude::*;
use pyo3::types::*;

#[test]
fn t0() {
    #[pyclass]
    struct Foo {/* fields omitted */}
    Python::with_gil(|py| {
        // Creates a new instance Bound<T> of a #[pyclass] on the Python heap.
        let foo: Bound<'_, Foo> = Bound::new(py, Foo {}).unwrap();
    });
}

#[test]
fn t1() {
    Python::with_gil(|py| {
        let d: Bound<PyDict> = PyDict::new_bound(py);

        // Returns the GIL token associated with this object.
        let python_ = d.py();
        // Helper to cast to Bound<'py, PyAny>.
        let pyany_ = d.as_any();
        // Helper to cast to Bound<'py, PyAny>, transferring ownership.
        let borrowed_ = d.as_borrowed();
        // Removes the connection for this Bound<T> from the GIL, allowing it to cross thread boundaries.
        let py_ = d.unbind();
    })
}
