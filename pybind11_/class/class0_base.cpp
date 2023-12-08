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


bool is_cpp(Pet &p, std::string is_cpp) {
    if (p.name == is_cpp)
        return true;
    else
        return false;
}

struct Pet_dyn {
    Pet_dyn(const std::string &name) : name(name) { }
    std::string name;
};

PYBIND11_MODULE(class0_base, m) {
    py::class_<Pet>(m, "Pet")
            // init() is a convenience function that takes the types of a constructor’s parameters as template arguments and wraps the corresponding constructor (s
            .def(py::init<const std::string &>()) // 绑定python __init__函数

            /*
             * def setName(self, name_):
             *     self.name = name_
             */
            // 与上python类中的类方法等价
            .def("setName", &Pet::setName) // c++类成员函数:python中自动将实例本身作为hanshusetName的第一个参数(self)
            .def("getName", &Pet::getName) // 绑定python 类方法(即提供给python中调用的接口,更具实际情况选择绑定)

            /*
             * def is_cpp(self, is_cpp): # 第一个参数为self,self表示实例本身
             *     if self.name == is_cpp
             *         return True
             *     else:
             *         return False
             */
            // 与上python类中的类方法等价
            .def("is_cpp", &is_cpp) // c++独立函数:需指定Pet &p表示实例本身(即python类方法中的self)
            /*
             * def __repr__(self)
             *     return "<example.Pet named '" + self.name + "'>"
             */
            // 与上python类中的类方法等价
            .def("__repr__", [](const Pet &a) {return "<example.Pet named '" + a.name + "'>";}) // c++独立(lambda)函数:需指定const Pet &a表示实例本身(即python类方法中的self)

            .def_static("howAge", &Pet::howAge) // 绑定python 静态方法
            .def_readwrite_static("age", &Pet::age) // 绑定python 类属性(python类属性是静态变量)

            .def_readwrite("name", &Pet::name); // 绑定python 实例属性

    py::class_<Pet_dyn>(m, "Pet_dyn",
                        py::dynamic_attr())  // 启用动态绑定属性(默认不支持)
            .def(py::init<const std::string &>())
            .def_readwrite("name", &Pet_dyn::name);

    // 独立函数
    m.def("is_cpp", &is_cpp);
}

