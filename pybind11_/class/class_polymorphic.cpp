#include <pybind11/pybind11.h>
#include <string>

namespace py = pybind11;

struct Pet {
    Pet(const std::string &name) : name(name) { }
    std::string name;
};

// regular non-polymorphic types with an inheritance relationship.
struct Dog : Pet {
    Dog(const std::string &name) : Pet(name) { }
    std::string bark() const { return "woof!"; }
};

struct PolymorphicPet {
    virtual ~PolymorphicPet() = default; // 虚函数
};

//  In C++, a type is only considered polymorphic if it has at least one virtual function and pybind11 will automatically recognize this
struct PolymorphicDog : PolymorphicPet { //
    std::string bark() const { return "woof!"; }
};


PYBIND11_MODULE(class_polymorphic, m) {
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

    m.def("pet_store", []() { return std::unique_ptr<Pet>(new Dog("Molly")); });


    py::class_<PolymorphicPet>(m, "PolymorphicPet");
    py::class_<PolymorphicDog, PolymorphicPet>(m, "PolymorphicDog")
            .def(py::init<>())
            .def("bark", &PolymorphicDog::bark);

    // Given a pointer to a polymorphic base, pybind11 performs automatic downcasting to the actual derived type.(c++通过需要使用dynamic_cast来做这种转换,参考:CPPStudy\g_advanced\RTTI_dynamic_cast_typeid_.cpp)
    m.def("pet_store2", []() { return std::unique_ptr<PolymorphicPet>(new PolymorphicDog); });
}

