use pyo3::prelude::*;
use pyo3::types::*;

// Methods from Deref<Target = PyAny>
#[test]
fn PyString_test() {
    Python::with_gil(|py| {
        /*
        Represents a Python string (a Unicode string object).

        This type is immutable.
         */

        let s = PyString::new_bound(py, "hello world");
        println!("{}", s); // print->hello world

        // Converts the PyString into a Rust string.
        let s_t = s.to_string_lossy();
        println!("{}", s_t); // print->hello world
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyBool_test() {
    Python::with_gil(|py| {
        // Represents a Python bool.

        let b = PyBool::new_bound(py, false);
        println!("{:?}", b); // print->False

        // Gets whether this boolean is true.
        println!("{}", b.is_true()); // print->false
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyLong_test() {
    Python::with_gil(|py| {
        /*
        Represents a Python int object.

        You can usually avoid directly working with this type by using ToPyObject and extract with the primitive Rust integer types
         */

        let rust_num = 42;
        let py_any = rust_num.to_object(py);
        let py_long = py_any.extract::<&PyLong>(py);
        println!("{:?}", py_long); // print->Ok(42)
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyFloat_test() {
    Python::with_gil(|py| {
        /*
        Represents a Python float object.

        You can usually avoid directly working with this type by using ToPyObject and extract with f32/f64.
         */
        let rust_num = 3.14;
        let py_any = rust_num.to_object(py);
        let py_float = py_any.extract::<&PyFloat>(py);
        println!("{:?}", py_float); // print->Ok(3.14)
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyList_test() {
    Python::with_gil(|py| {
        // Represents a Python list.

        // Constructs a new empty list.
        let l0 = PyList::empty_bound(py);
        println!("{:?}", l0); // print->[]

        // Checks if the list is empty.
        println!("{}", l0.is_empty()); // print->true

        // Returns the length of the list.
        println!("{}", l0.len()); // print->0

        let elements: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        // Constructs a new list with the given elements.
        let l1 = PyList::new_bound(py, elements);
        println!("{:?}", l1); // print->[0, 1, 2, 3, 4, 5]

        // Appends an item to the list.
        l1.append("one");
        println!("{:?}", l1); // print->[0, 1, 2, 3, 4, 5, 'one']

        // Inserts an item at the specified index.
        l1.insert(1, false);
        println!("{:?}", l1); // print->[0, False, 1, 2, 3, 4, 5, 'one']

        // Returns the first index i for which self[i] == value.
        let i = l1.index(5);
        println!("{:?}", i); // print->Ok(6)
        println!("{:?}", l1.index(199)); // print->Err(PyErr { type: <class 'ValueError'>, value: ValueError('sequence.index(x): x not in sequence'), traceback: None })

        // Deletes the indexth element of self. This is equivalent to the Python statement del self[i].
        l1.del_item(0);
        println!("{:?}", l1); // print-> [False, 1, 2, 3, 4, 5, 'one']

        // Deletes the slice from low to high from self. This is equivalent to the Python statement del self[low:high].
        l1.del_slice(4, 6);
        println!("{:?}", l1); // print->[False, 1, 2, 3, 'one']

        // Gets the list item at the specified index. Undefined behavior on bad index. Use with caution.
        let gi_v = l1.get_item(1).unwrap();
        println!("{:?}", gi_v); // print->1

        // Sets the item at the specified index. Raises IndexError if the index is out of range.
        l1.set_item(1, -1);
        println!("{:?}", l1); // print->[False, -1, 2, 3, 'one']
        
        /*
        Determines if self contains value.

        This is equivalent to the Python expression value in self.
         */
        println!("{:?}", l1.contains(3)); // Ok(true)
        println!("{:?}", l1.contains("w")); // Ok(false)

        // Return a new tuple containing the contents of the list; equivalent to the Python expression tuple(list).
        let t = l1.to_tuple();
        println!("{:?}", t); // pirnt->(False, -1, 2, 3, 'one')

        for x in l1 {
            println!("{}", x);
        }

    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyTuple_test() {
    Python::with_gil(|py| {
        /*
        Represents a Python tuple object.

        This type is immutable.
         */

        // Constructs an empty tuple (on the Python side, a singleton object).
        let t0 = PyTuple::empty_bound(py);
        println!("{}", t0); // print->()

        // Checks if the tuple is empty.
        println!("{}", t0.is_empty()); // print->true

        // Gets the length of the tuple.
        println!("{}", t0.len());

        let elements: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        // Constructs a new tuple with the given elements.
        let t1 = PyTuple::new_bound(py, elements);
        println!("{}", t1); // print->(0, 1, 2, 3, 4, 5)

        let gi_v = t1.get_item(0).unwrap();
        println!("{:?}", gi_v); // print->0

        /*
        Determines if self contains value.

        This is equivalent to the Python expression value in self.
         */
        println!("{:?}", t1.contains(3)); // Ok(true)
        println!("{:?}", t1.contains("w")); // Ok(false)

        // Return a new list containing the contents of this tuple; equivalent to the Python expression list(tuple).
        let l = t1.to_list();
        println!("{:?}", l); // print->[0, 1, 2, 3, 4, 5]

        for x in t1 {
            println!("{}", x);
        }
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PyDict_test() {
    Python::with_gil(|py| {
        // Represents a Python dict.

        // Creates a new empty dictionary.
        let d = PyDict::new_bound(py);
        println!("{:?}", d); // print->{}

        // Checks if the dict is empty, i.e. len(self) == 0.
        println!("{}", d.is_empty()); // print->true

        // Return the number of items in the dictionary.
        println!("{}", d.len()); // print->0

        // Sets an item value. This is equivalent to the Python statement self[key] = value.
        d.set_item("a", 1);
        d.set_item("b", 2);
        d.set_item("c", 3);
        println!("{:?}", d); // {'a': 1, 'b': 2, 'c': 3}

        // Gets an item from the dictionary.
        println!("{:?}", d.get_item("a")); // print->Ok(Some(1))
        println!("{:?}", d.get_item("d")); // print->Ok(None)

        // Deletes an item. This is equivalent to the Python statement del self[key].
        d.del_item("a");
        println!("{}", d); // print->{'b': 2, 'c': 3}

        // Returns a list of dict keys. This is equivalent to the Python expression list(dict.keys()).
        let kv = d.keys();
        println!("{:?}", kv); // print->['b', 'c']

        // Returns a list of dict values. This is equivalent to the Python expression list(dict.values()).
        let vv = d.values();
        println!("{:?}", vv); // print->[2, 3]

        // Returns a list of dict items. This is equivalent to the Python expression list(dict.items()).
        let iv = d.items();
        println!("{:?}", iv); // print->[('b', 2), ('c', 3)]

        /*
        Determines if the dictionary contains the specified key.

        This is equivalent to the Python expression key in self.
         */
        println!("{:?}", d.contains("a")); // print->Ok(false)
        println!("{:?}", d.contains("b")); // print->Ok(true)

        for (k, v) in &d {
            println!("{:?}", k);
            println!("{:?}", v);
        }

        d.clear();
        println!("{:?}", d); // print->{}
    })
}

// Methods from Deref<Target = PyAny>
#[test]
fn PySet_test() {
    Python::with_gil(|py| {
        // Represents a Python set

        let elements: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        // Creates a new set with elements from the given slice.
        let s0 = PySet::new_bound(py, &elements).unwrap();
        println!("{:?}", s0); // print->{0, 1, 2, 3, 4, 5}

        // Returns the number of items in the set.
        println!("{:?}", s0.len());

        s0.add("one");
        s0.add("two");
        println!("{:?}", s0);

        // emoves the element from the set if it is present. Returns true if the element was present in the set.
        let d0 = s0.discard(1);
        println!("{:?}", d0); // print->Ok(true)
        let d1 = s0.discard("three");
        println!("{:?}", d1); // print->Ok(false)
        println!("{:?}", s0); // print->{0, 2, 3, 4, 5, 'one', 'two'}

        // Determines if the set contains the specified key. This is equivalent to the Python expression key in self.
        println!("{:?}", s0.contains(1)); // Ok(false)
        println!("{:?}", s0.contains(2)); // Ok(true)

        for v in &s0 {
            println!("{:?}", v);
        }

        s0.clear();
        println!("{:?}", s0); // print->set()
    })
}
