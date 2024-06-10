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
use std::collections::HashMap;

#[pyfunction]
pub fn str_test0(mut x: String) {
    println!("{}", x);
    x.push('!');
    println!("{}", x);
}

#[pyfunction]
pub fn str_test1(x: Bound<'_, PyString>) {
    println!("{}", x);
}

#[pyfunction]
pub fn list_test0(mut x: Vec<i32>) {
    println!("{:?}", x);
    x.push(100); // 不会修改python端传入的list中的值
    println!("{:?}", x);
}

#[pyfunction]
pub fn list_test1(x: Bound<'_, PyList>) {
    println!("{:?}", x);
    x.append("one");
    println!("{:?}", x);
}

#[pyfunction]
pub fn dict_test0(mut x: HashMap<String, i32>) {
    println!("{:?}", x);
    x.insert(String::from("d"), 4);
    println!("{:?}", x);
}

#[pyfunction]
pub fn dict_test1(x: Bound<'_, PyDict>) {
    println!("{:?}", x);
    x.set_item("d", 4);
    println!("{:?}", x);
}
