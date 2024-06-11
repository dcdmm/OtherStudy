/*
Python                  Rust                    Rust(Python-native)
object                                          PyAny
str                     String/&str/char        PyString
bool                    bool                    PyBool
int                     i8/u8/i16/u16/i32/u32   PyLong
int                     i64/u64/i128/u128       PyLong
int                     issize/usize            PyLong
float                   f32/f64                 PyFloat
list[T]                 Vec<T>                  PyList
dict[K, V]              HashMap<K, V>           PyDict
tuple[T, U]             (T, U)/Vec<T>           PyTuple
set[T]                  HashSet<T>              PySet
*/

use pyo3::prelude::*;
use pyo3::types::*;
use std::collections::{HashMap, HashSet};

#[pyfunction]
pub fn int_test0(x: i32) {
    println!("{}", x);
}

#[pyfunction]
pub fn int_test1(x: isize) {
    println!("{}", x);
}

#[pyfunction]
pub fn int_test2(x: Bound<'_, PyLong>) {
    println!("{}", x);
}

#[pyfunction]
pub fn str_test0(mut x: String) {
    println!("{}", x); // print->hello world
    x.push('!');
    println!("{}", x); // print->hello wolrd!
}

#[pyfunction]
pub fn str_test1(x: Bound<'_, PyString>) {
    println!("{}", x); // print->hello world
}


#[pyfunction]
pub fn list_test0(mut x: Vec<i32>) {
    println!("{:?}", x); // print->[0, 1, 2]
    x.push(100); // 不会修改python端传入的list中的值
    println!("{:?}", x); // print->[0, 1, 2, 100]
}

#[pyfunction]
pub fn list_test1(x: Bound<'_, PyList>) {
    println!("{:?}", x); // print->[0, 1, 2]
    x.append("one");
    println!("{:?}", x); // print->[0, 1, 2, 'one']
}

#[pyfunction]
pub fn dict_test0(mut x: HashMap<String, i32>) {
    println!("{:?}", x); // print->{"a": 0, "b": 1, "c": 2}
    x.insert(String::from("d"), 4); // 不会修改python端传入的dict中的值
    println!("{:?}", x); // print->{"a": 0, "b": 1, "c": 2, "d": 4}
}

#[pyfunction]
pub fn dict_test1(x: Bound<'_, PyDict>) {
    println!("{:?}", x); // print->{'a': 0, 'b': 1, 'c': 2}
    x.set_item("d", 4);
    println!("{:?}", x); // print->{'a': 0, 'b': 1, 'c': 2, 'd': 4}
}

#[pyfunction]
pub fn set_test0(mut x: HashSet<i32>) {
    println!("{:?}", x); // print->{0, 2, 1}
    x.insert(3); // 不会修改python端传入的set中的值
    println!("{:?}", x); // print->{2, 1, 0, 3}
}

#[pyfunction]
pub fn set_test1(x: Bound<'_, PySet>) {
    println!("{:?}", x); // print->{0, 1, 2}
    x.add("three");
    println!("{:?}", x); // {0, 1, 2, 'three'}
}