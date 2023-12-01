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

PYBIND11_MODULE(func0, m) {
    m.doc() = "func0 doc";  // 模块文档

    m.def("add", &add,
          "func0 add doc");  // 函数文档

    m.def("sub", &sub, "func0 sub doc",
          py::arg("i"), py::arg("j")); // 关键字参数

    m.def("multi", &multi, "func0 multi doc",
          py::arg("i") = 1, py::arg("j") = 1); // 默认参数
}