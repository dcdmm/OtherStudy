/*
A GIL-independent reference to an object allocated on the Python heap.

This type does not auto-dereference to the inner object because you must prove you hold the GIL to access it. Instead, call one of its methods to access the inner object:
    * Py::as_ref, to borrow a GIL-bound reference to the contained object.
    * Py::borrow, Py::try_borrow, Py::borrow_mut, or Py::try_borrow_mut, to get a (mutable) reference to a contained pyclass, using a scheme similar to std’s RefCell.
    * You can call methods directly on Py with Py::call_bound, Py::call_method_bound and friends. These require passing in the Python<'py> token but are otherwise similar to the corresponding methods on PyAny.
*/

// PyObject: An alias for Py<PyAny>

// # Trait Implementations
/*
impl<T> Drop for Py<T>
    Dropping a Py instance decrements the reference count on the object by 1.
*/

use pyo3::prelude::*;
use pyo3::types::*;

#[test]
fn Py_new_test() {
    // #[pyclass]
    // // error: #[pyclass] cannot have lifetime parameters. For an explanation, see https://pyo3.rs/latest/class.html#no-lifetime-parameters
    // struct Foo<'py> {
    //     inner: Bound<'py, PyDict>,
    // }
    // impl Foo {
    //     fn new() -> Foo {
    //         let foo = Python::with_gil(|py| {
    //             let dict: Bound<'_, PyDict> = PyDict::new_bound(py);

    //             Foo { inner: dict }
    //         });
    //         foo
    //     }
    // }
    // 修正如下所示:
    // *****************************************************************
    #[pyclass]
    struct Foo {
        inner: Py<PyDict>,
    }

    impl Foo {
        fn new() -> Foo {
            Python::with_gil(|py| {
                let dict: Py<PyDict> = PyDict::new_bound(py).unbind();
                Foo { inner: dict }
            })
        }
    }
    // *****************************************************************

    #[pyclass]
    struct Foo1 {/* fields omitted */}

    // Creates a new instance Py<T> of a #[pyclass] on the Python heap.
    Python::with_gil(|py| {
        let foo: Py<Foo1> = Py::new(py, Foo1 {}).unwrap();
    });
}

#[test]
fn Py_new_as_ref() {
    Python::with_gil(|py| {
        let list_Py: Py<PyList> = PyList::empty_bound(py).unbind();
        /*
        Borrows a GIL-bound reference to the contained T.

        For native types, this reference is &T. For pyclasses, this is &PyCell<T>.
         */
        let list_ref: &PyList = list_Py.as_ref(py);
    });
}

#[test]
fn Py_bind_borrow() {
    #[pyclass]
    struct Foo {
        inner: u8,
    }
    Python::with_gil(|py| {
        let foo: Py<Foo> = Py::new(py, Foo { inner: 73 }).unwrap();

        // bind: Attaches this Py to the given Python context, allowing access to further Python APIs.
        let b_ = foo.bind(py).borrow();
        /*
        Immutably borrows the value T.

        This borrow lasts while the returned PyRef exists. Multiple immutable borrows can be taken out at the same time.
         */
        let b = foo.borrow(py); // 与上等价
        let i: &u8 = &b.inner;
        println!("{:?}", *i);
    });
}

// Py<T> can be used to share ownership of a Python object, similar to std’s Rc<T>. As with Rc<T>, cloning it increases its reference count rather than duplicating the underlying object.
#[test]
fn Py_clone() {
    Python::with_gil(|py| {
        let first = PyDict::new_bound(py).unbind();
        
        // All of these are valid syntax
        let fourth = Py::clone(&first);
        let second = Py::clone_ref(&first, py);
        let third = first.clone_ref(py);
        let fifth = first.clone();

        // Disposing of our original `Py<PyDict>` just decrements the reference count.
        drop(first);

        // They all point to the same object
        println!("{}", second.is(&third)); // print->true
        println!("{}", second.is(&fourth)); // print->true
        println!("{}", second.is(&fifth)); // print->true
    });
}

