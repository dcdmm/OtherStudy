#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <iostream>
#include <vector>
#include <deque>
#include <list>
#include <array>
#include <valarray>

void print_vector(const std::vector<int>& i) {
    std::cout << "std::vector<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }
    std::cout << std::endl << std::endl;
}

void print_deque(const std::deque<int>& i) {
    std::cout << "std::deque<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }
    std::cout << std::endl << std::endl;
}

void print_list(const std::list<int>& i) {
    std::cout << "std::list<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }
    std::cout << std::endl << std::endl;
}

void print_array(const std::array<int, 5>& i) {
    std::cout << "std::array<int, 5>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }
    std::cout << std::endl << std::endl;
}

void print_valarray(const std::valarray<int>& i) {
    std::cout << "std::vector<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }
    std::cout << std::endl << std::endl;
}


PYBIND11_MODULE(stl_type, m) {
    m.def("print_vector", &print_vector);
    m.def("print_deque", &print_deque);
    m.def("print_list", &print_list);
    m.def("print_array", &print_array);
    m.def("print_valarray", &print_valarray);
}