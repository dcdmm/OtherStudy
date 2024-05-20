#include <pybind11/pybind11.h>
#include <string>

namespace py = pybind11;

class People {
public:
    People(const std::string &name) : name(name) { }

    void setName(const std::string &name_) { name = name_; }
    const std::string &getName() const { return name; }
private:
    std::string name;  // private成员(只能类的成员函数/友元使用)
};

PYBIND11_MODULE(class_get_set, m) {
    py::class_<People>(m, "People")
            .def(py::init<const std::string &>())
            .def_property("name",
                          &People::getName, // 绑定python property装饰器@property属性
                          &People::setName); // 绑定python property装饰器@name.setter属性
}