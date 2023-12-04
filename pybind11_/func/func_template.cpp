#include <pybind11/pybind11.h>

namespace py = pybind11;

template<typename T, typename U>
auto add(T &a, U &b) {
    return a + b;
}

template<typename T, typename U>
auto sub(T &a, U &b) {
    return a - b;
}

PYBIND11_MODULE(func_template, m) {
    // must bind each instantiated function template separately.
    // You may bind each instantiation with the same name, which will be treated the same as an overloaded function
    m.def("sub", &sub<int, long>, "int - long");
    m.def("sub", &sub<int, double>, "int - double"); // 函数重载

    // Sometimes it’s more clear to bind them with separate names, which is also an option:
    m.def("add_long", &add<int, long>, "int + long");
    m.def("add_double", &add<int, double>, "int + double"); // 与函数重载相比,更加清晰
}