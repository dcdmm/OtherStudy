#include <pybind11/pybind11.h>
#include <pybind11/functional.h>
#include <iostream>

namespace py = pybind11;

void process_function(py::function func, py::list py_list) {
    // 调用python函数
    py::object result = func(py_list, 100);
    double f_result_cpp = result.cast<double>();
    std::cout << f_result_cpp << std::endl;
}

PYBIND11_MODULE(cpp_access_python_f, m) {
    m.def("process_function", &process_function);
}