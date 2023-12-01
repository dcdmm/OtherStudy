#include <pybind11/pybind11.h>
#include <string>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name) : name(name) { }
    void setName(const std::string &name_) { name = name_; }
    const std::string &getName() const { return name; }

    std::string name;
};

PYBIND11_MODULE(class0, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &>()) // 构造函数
            .def("setName", &Pet::setName)
            .def("getName", &Pet::getName)
            .def("__repr__", [](const Pet &a) {return "<example.Pet named '" + a.name + "'>";}); // lambda函数
}

