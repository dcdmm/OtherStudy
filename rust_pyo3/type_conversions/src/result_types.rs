/*
Rust type                       Resulting Python Type
String                          str
&str                            str
bool                            bool
i8/u8/i16/u16/i32/u32           int
i64/u64/i128/u128               int
issize/usize                    int
f32/f64                         float
Option<T>                       Optional[T]
(T, U)                          Tuple[T, U]
Vec<T>                          List[T]
HashMap<K, V>                   Dict[K, V]
HashSet<T>                      Set[T]
Py<T>                           T
Bound<T>                        T
PyRef<T: PyClass>               T
PyRefMut<T: PyClass>            T
*/

use pyo3::prelude::*;
use pyo3::types::*;

#[pyfunction]
pub fn str_t0() -> String {
    String::from("hello world")
}

#[pyfunction]
pub fn str_t1() -> &'static str {
    "hello java"
}

#[pyfunction]
pub fn Option_t0() -> Option<i32> {
    let some_number: Option<i32> = Some(5);
    some_number
}

#[pyfunction]
pub fn list_t0() -> Vec<i32> {
    vec![1, 2, 3, 4]
}

#[pyfunction]
pub fn T_t0(py: Python) -> Bound<PyDict> {
    let d: Bound<PyDict> = PyDict::new_bound(py);
    d
}
