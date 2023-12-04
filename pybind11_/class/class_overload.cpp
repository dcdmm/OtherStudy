#include <pybind11/pybind11.h>
#include <string>
#include <iostream>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name, int age) : name(name), age(age) { }

    void set(int age_) {
        std::cout << "call void set(int age_)" << std::endl;
        age = age_;
    }
    void set(const std::string &name_) {
        std::cout << "call void set(const std::string &name_)" << std::endl;
        name = name_;
    }

    std::string name;
    int age;
};

PYBIND11_MODULE(class_overload, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &, int>())
            // 函数重载
            .def("set", py::overload_cast<int>(&Pet::set), "Set the pet's age")
            .def("set", py::overload_cast<const std::string &>(&Pet::set), "Set the pet's name")

            .def_readwrite("name", &Pet::name)
            .def_readwrite("age", &Pet::age);
}