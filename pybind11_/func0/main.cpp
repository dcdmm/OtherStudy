#include <pybind11/pybind11.h>

namespace py = pybind11;

int add(int i, int j) {
    return i + j;
}

int sub(int i = 1, int j = 1) {
    return i - j;
}

PYBIND11_MODULE(func0, m) {
    m.doc() = "func0 doc";

    m.def("add", &add, "func0 add doc");

    m.def("sub", &sub, "func0 sub doc",
          py::arg("i") = 1, py::arg("j") = 1); // 指定默认参数的值
}