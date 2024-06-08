// PyRef<T>: A #[pyclass] borrowed immutably.
// PyRefMut<T>: A #[pyclass] borrowed mutably.

use pyo3::prelude::*;
use pyo3::types::*;

#[pyclass]
struct Foo {
    inner: u8,
}

#[test]
fn t0() {
    Python::with_gil(|py| {
        let foo: Bound<'_, Foo> = Bound::new(py, Foo { inner: 73 }).unwrap();
        // Immutably borrows the value T.
        let foo_b0 = foo.borrow();
        let foo_b1 = foo.borrow();

        println!("{}", foo_b0.inner); // print->73
        println!("{}", foo_b1.inner); // print->73
    });
}

#[test]
fn t1() {
    Python::with_gil(|py| {
        let foo: Bound<'_, Foo> = Bound::new(py, Foo { inner: 73 }).unwrap();

        // Mutably borrows the value T.
        let mut foo_bm = foo.borrow_mut();
        foo_bm.inner = 35;

        // assert_eq!(foo.borrow().inner, 35);
    });
}
