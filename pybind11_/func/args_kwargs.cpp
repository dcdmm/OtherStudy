#include <pybind11/pybind11.h>
#include <iostream>
#include <string>

namespace py = pybind11;

PYBIND11_MODULE(args_kwargs, m) {
    // py::args derives from py::tuple
    m.def("args_function", [](py::args args) -> py::tuple {
        std::cout << py::cast<int>(args[0]) << std::endl;
        std::cout << py::cast<std::string>(args[1]) << std::endl;
        std::cout << py::cast<double>(args[2]) << std::endl;
        return args;
    });

    // py::kwargs derives from py::dict
    m.def("kwargs_function", [](py::kwargs kwargs) -> py::dict {
        std::cout << py::cast<int>(args[0]) << std::endl;
        std::cout << py::cast<std::string>(args[1]) << std::endl;
        std::cout << py::cast<double>(args[2]) << std::endl;
        return kwargs;
    });

//m.def("args_kwargs_function", [](const py::args &args, const py::kwargs &kwargs) {
//return py::make_tuple(args, kwargs);
//});
//
//// test_mixed_args_and_kwargs
//m.def("mixed_plus_args",
//[](int i, double j, const py::args &args) { return py::make_tuple(i, j, args); });
//m.def("mixed_plus_kwargs",
//[](int i, double j, const py::kwargs &kwargs) { return py::make_tuple(i, j, kwargs); });
//auto mixed_plus_both = [](int i, double j, const py::args &args, const py::kwargs &kwargs) {
//    return py::make_tuple(i, j, args, kwargs);
};