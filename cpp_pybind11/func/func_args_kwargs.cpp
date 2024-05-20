#include <pybind11/pybind11.h>
#include <iostream>
#include <string>

namespace py = pybind11;

PYBIND11_MODULE(func_args_kwargs, m) {
    // py::args derives from py::tuple
    m.def("args_function", [](py::args args) -> py::tuple {
        py::print(args);

        std::cout << py::cast<int>(args[0]) << std::endl;
        std::cout << py::cast<std::string>(args[1]) << std::endl;
        std::cout << py::cast<double>(args[2]) << std::endl;
        return args;
    });

    // py::kwargs derives from py::dict
    m.def("kwargs_function", [](py::kwargs kwargs) -> py::dict {
        py::print(kwargs);

        py::list keys = kwargs.attr("keys")();
        py::list values = kwargs.attr("values")();
        py::list items = kwargs.attr("items")();
        py::print(keys);
        py::print(values);
        py::print(items);
        return kwargs;
    });

    m.def("mixed_plus_args", [](int i, double j, const py::args &args) {
        std::cout << i << std::endl;
        std::cout << j << std::endl;
        py::print(args);
        py::print();
    });

    m.def("mixed_plus_kwargs", [](int i, double j, const py::kwargs &kwargs) {
        std::cout << i << std::endl;
        std::cout << j << std::endl;
        py::print(kwargs);
        py::print();
    });

    // py::kwargs只能放到参数列表末尾
    // py::args只能放到参数列表末尾或py::kwargs之前(若有)
    auto mixed_plus_both = [](int i, double j, const py::args &args, const py::kwargs &kwargs) {
        std::cout << i << std::endl;
        std::cout << j << std::endl;
        py::print(args);
        py::print(kwargs);
        py::print();
    };

    m.def("mixed_plus_args_kwargs", mixed_plus_both);
};