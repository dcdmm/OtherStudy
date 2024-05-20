#include <pybind11/pybind11.h>

namespace py = pybind11;

int add(int i, int j) {
    return i + j;
}

int sub(int i, int j) {
    return i - j;
}

int multi(int i = 1, int j = 1) {
    return i * j;
}

PYBIND11_MODULE(func0_base, m) {
    m.def("add", &add,
          "func0 add doc");  // 对应python函数文档

    m.def("sub", &sub, "func0 sub doc",
          py::arg("i"), py::arg("j")); // 对应python函数关键字参数

    m.def("multi", &multi, "func0 multi doc",
          py::arg("i") = 1, py::arg("j") = 1); // 对应python函数默认参数

    m.def("div",
          [](double i, double j) {return i / j;},  // lambda函数
          "func0 div doc",
          py::arg("i") = 1.0, py::arg("j") = 1.0);
}