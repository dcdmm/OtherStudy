#include <pybind11/pybind11.h>
#include <string>
#include <iostream>

namespace py = pybind11;

class Pet {
private:
    std::string name;
    int age;
public:
    Pet(const std::string &name, int age) : name(name), age(age) {}

    std::string getName() {
        return name;
    }

    int getAge() {
        return age;
    }

    void show() {
        std::cout << "wang wang wang!" << std::endl;
    }
};

void print(std::string attr) {
    std::cout << "attr:" << attr << std::endl;
}

struct People {
    People(const std::string &name, Pet p) : name(name), p(p) {}

    std::string getPetName() {
        print("name");
        return p.getName();
    }

    int getPetAge() {
        print("age");
        return p.getAge();
    }

    std::string name;
    Pet p;  // 组合:在新类中产生产生现有类的对象
};

PYBIND11_MODULE(class_has_a_combination, m) {
    py::class_<Pet>(m, "Pet")
            .def(py::init<const std::string &, int>())
            .def("show", &Pet::show);

    py::class_<People>(m, "People")
            .def(py::init<const std::string &, Pet>())
            .def("getPetName", &People::getPetName)
            .def("getPetAge", &People::getPetAge)
            .def_readwrite("p", &People::p)
            .def_readwrite("name", &People::name);
}