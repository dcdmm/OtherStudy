#include <pybind11/pybind11.h>
#include <string>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name) : name(name) { }
    void setName(const std::string &name_) { name = name_; }
    const std::string &getName() const { return name; }

    static int howAge() {return age;}

    std::string name;
    static int age;
};

int Pet::age = 100;

class People {
public:
    People(const std::string &name) : name(name) { }

    void setName(const std::string &name_) { name = name_; }
    const std::string &getName() const { return name; }
private:
    std::string name;  // private成员(只能类的成员函数/友元使用)
};

PYBIND11_MODULE(class0, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &>()) // 绑定python __init__函数

            .def("setName", &Pet::setName) // 绑定python 类方法
            .def("getName", &Pet::getName)

            .def("__repr__", [](const Pet &a) {return "<example.Pet named '" + a.name + "'>";}) // 非成员函数(lambda函数)

            .def_static("howAge", &Pet::howAge) // 绑定python 静态方法
            .def_readwrite_static("age", &Pet::age) // 绑定python 类属性(python类属性是静态变量)

            .def_readwrite("name", &Pet::name); // 绑定python 实例属性

    py::class_<People>(m, "People")
            .def(py::init<const std::string &>())
            .def_property("name",
                          &People::getName, // 绑定python property装饰器@property属性
                          &People::setName); // 绑定python property装饰器@name.setter属性
}

