#include <pybind11/pybind11.h>

#include <iostream>
#include <string>

namespace py = pybind11;

class Example {
public:
    int age = 28;
    int rank = 1;
    double salary = 99999;
    std::string name = "dmm";
public:
    explicit Example(int i1, int i2, double s, std::string n) :
            age(i1),
            rank(i2), salary(s),
            name(std::move(n)) {};

    explicit Example(double s) : salary(s) {};

    Example(int i1, int i2) : age(i1), rank(i2) {};

    explicit Example(std::string n) : name(std::move(n)) {};

    void show() {
        std::cout << age << '\t' << rank << '\t' <<
        salary << '\t' << name << std::endl;
    }
};

PYBIND11_MODULE(class_custom_init, m) {
    py::class_<Example>(m, "Example")
        // Bind a lambda function returning a pointer wrapped in a holder:
        .def(py::init([](std::string arg) { return std::unique_ptr<Example>(new Example(arg)); }))
        // Return a raw pointer:
        .def(py::init([](int a, int b) { return new Example(a, b); }))

        // You can mix the above with regular C++ constructor bindings as well:
        .def(py::init<double>())
        .def(py::init<int, int, double, std::string &>())

        .def("show", &Example::show);
}