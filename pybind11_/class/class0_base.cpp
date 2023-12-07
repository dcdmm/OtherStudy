#include <pybind11/pybind11.h>
#include <string>
#include <iostream>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name) : name(name) { }
    void setName(const std::string &name_) { name = name_; }
    const std::string &getName() const { return name; }

    static int howAge() {return age;}

    void show() {
        std::cout << "wang wang wang!\n";
    }

    std::string name;
    static int age;
};

int Pet::age = 100;

struct Pet_dyn {
    Pet_dyn(const std::string &name) : name(name) { }
    std::string name;
};

PYBIND11_MODULE(class0_base, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &>()) // 绑定python __init__函数

            .def("setName", &Pet::setName) // 绑定python 类方法(即提供给python中调用的接口,更具实际情况选择绑定)
            .def("getName", &Pet::getName)

            .def("__repr__", [](const Pet &a) {return "<example.Pet named '" + a.name + "'>";}) // 非成员函数(lambda函数)

            .def_static("howAge", &Pet::howAge) // 绑定python 静态方法
            .def_readwrite_static("age", &Pet::age) // 绑定python 类属性(python类属性是静态变量)

            .def_readwrite("name", &Pet::name); // 绑定python 实例属性

    py::class_<Pet_dyn>(m, "Pet_dyn",
                        py::dynamic_attr())  // 启用动态绑定属性(默认不支持)
            .def(py::init<const std::string &>())
            .def_readwrite("name", &Pet_dyn::name);
}

