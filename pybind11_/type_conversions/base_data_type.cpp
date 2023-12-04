#include <pybind11/pybind11.h>
#include <iostream>

namespace py = pybind11;

// c++:bool ===> python:bool
// c++:short/int/long long ===> python:int
// c++:float/double ===> python:float
// c++:char */std::string ===> python:str
// c++:std::nullptr_t ===> None
void bool_(bool i) {
    std::cout << "bool " << i << std::endl;
}

void short_(short i) {
    std::cout << "short " << i << std::endl;
}

void int_(int i) {
    std::cout << "int " << i << std::endl;
}

void long_(long i) {
    std::cout << "long " << i << std::endl;
}

void long_long_(long long i) {
    std::cout << "long long " << i << std::endl;
}

void float_(float i) {
    std::cout << "float " << i << std::endl;
}

void double_(double i) {
    std::cout << "double " << i << std::endl;
}

void c_string_(char *i) {
    std::cout << "char * " << i << std::endl;
}

void std_string_(std::string i) {
    std::cout << "std::string " << i << std::endl;
}

void nullptr_(std::nullptr_t) {
   std::cout << "nullptr " << std::endl;
}

PYBIND11_MODULE(base_data_type, m) {
    m.def("bool_", &bool_);
    m.def("short_", &short_);
    m.def("int_", &int_);
    m.def("long_", &long_);
    m.def("long_long_", &long_long_);
    m.def("float_", &float_);
    m.def("double_", &double_);
    m.def("c_string_", &c_string_);
    m.def("std_string_", &std_string_);
    m.def("nullptr_", &nullptr_);
}