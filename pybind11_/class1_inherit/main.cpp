#include <pybind11/pybind11.h>
#include <string>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name) : name(name) { }
    std::string name;
};

struct Dog : Pet {
    Dog(const std::string &name) : Pet(name) { }
    std::string bark() const { return "woof!"; }
};


PYBIND11_MODULE(class1_inherit, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &>())
            .def_readwrite("name", &Pet::name);

    // Method 1:the first specifies the C++ base class as an extra template parameter of the class_
    py::class_<Dog, Pet>(m, "Dog")
            .def(py::init<const std::string &>())
            .def("bark", &Dog::bark);

    // Method 2:assign a name to the previously bound Pet class_ object and reference it when binding the Dog class
//    py::class_<Dog>(m, "Dog", pet)
//            .def(py::init<const std::string &>())
//            .def("bark", &Dog::bark);

    // Method 1与Method 2等价
}

