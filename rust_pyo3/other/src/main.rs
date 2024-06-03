use pyo3::prelude::*;
use pyo3::types::PyString;
use std::env;

// fn main() -> PyResult<()> {

//     // env::set_var("PYTHONHOME", "C:\\Users\\duanm\\anaconda3");
//     // env::set_var("PYTHONPATH", "C:\\Users\\duanm\\anaconda3\\Lib");
    
//     pyo3::prepare_freethreaded_python();
//     Python::with_gil(|py| {
//         // 在这个代码块中，`py` 是 Python 的 GIL 句柄

//         // 创建一个新的 PyString 对象
//         // let s = PyString::new_bound(py, "Hello, world!");

//         // // 使用正确的错误处理方式打印 Python 字符串
//         // println!("Created Python string: {}", s.to_str()?);

//         Ok(())
//     })
// }

use pyo3::types::PyTuple;

fn main() -> PyResult<()> {
    let arg1 = "arg1";
    let arg2 = "arg2";
    let arg3 = "arg3";

    // env::set_var("PYTHONHOME", "C:\\Users\\duanm\\anaconda3");
    // env::set_var("PYTHONPATH","C:\\Users\\duanm\\anaconda3\\Lib");
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code_bound(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?
        .getattr("example")?
        .into();

        // call object without any arguments
        fun.call0(py)?;

        // pass object with Rust tuple of positional arguments
        let args = (arg1, arg2, arg3);
        fun.call1(py, args)?;

        // call object with Python tuple of positional arguments
        let args = PyTuple::new_bound(py, &[arg1, arg2, arg3]);
        fun.call1(py, args)?;
        Ok(())
    })
}