use pyo3::prelude::*;

// name: Sets the name that Python sees this class as. Defaults to the name of the Rust struct.
// subclass: Allows other Python classes and #[pyclass] to inherit from this class. Enums cannot be subclassed.
#[pyclass(name = "People", subclass)]
pub struct PyPeople {
    #[pyo3(get, set)]
    age: i32,
    #[pyo3(get, set)]
    is_man: bool,
}

#[pymethods]
impl PyPeople {
    #[new]
    fn new(age: i32, is_man: bool) -> Self {
        PyPeople { age, is_man }
    }

    fn print_age(&self) {
        println!("People age: {}", self.age);
    }

    fn print_is_man(&self, py: Python) {
        println!("People is_man: {}", self.is_man);
    }
}

// extends: Use a custom baseclass. Defaults to PyAny
#[pyclass(extends=PyPeople, name = "Study")]
pub struct PyStudy {
    #[pyo3(get, set)]
    name: String,
}

#[pymethods]
impl PyStudy {
    #[new]
    // 构造函数返回值类型为: (Self, 父类类型)
    pub fn new(age: i32, is_man: bool, name: String) -> (Self, PyPeople) {
        (PyStudy { name }, PyPeople::new(age, is_man))
    }

    // 新的方法
    fn print_name(&self) {
        println!("Study name: {}", self.name);
    }

    // 子类重写父类方法
    // 参考:class_as_function_arguments.rs
    fn print_is_man(slf: &Bound<'_, Self>) {
        let b_any = slf.as_any(); // 转换为PyAny类型
        let b_pp = b_any.downcast::<PyPeople>().unwrap(); // 向下转型为PyPeople类型
        let pyref_pp = b_pp.borrow();
        let is_man = pyref_pp.is_man; // 获取父类中的属性
        println!("Study is_man: {}", is_man);
    }
}
