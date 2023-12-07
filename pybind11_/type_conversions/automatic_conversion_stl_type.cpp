#include <pybind11/pybind11.h>

/*
 * When including the additional header file pybind11/stl.h, conversions between std::vector<>/std::deque<>/std::list<>/std::array<>/std::valarray<>, std::set<>/std::unordered_set<>, and std::map<>/std::unordered_map<> and the Python list, set and dict data structures are automatically enabled.
 * The major downside of these implicit conversions is that containers must be converted (i.e. copied) on every Python->C++ and C++->Python transition, which can have implications on the program semantics and performance.
 */
#include <pybind11/stl.h>

#include <iostream>
#include <vector>
#include <deque>
#include <list>
#include <array>
#include <valarray>
#include <map>
#include <set>

namespace py = pybind11;

// python:List[int] <===> c++:std::vector<int>/std::deque<int>/std::list<int>/std::array<int, len(List[int])>/std::valarray<int>
// python:Dict[str, int] <===> std::map<std::string, int>
// python:Set[int] <===> c++:std::set<int>
void print_vector(std::vector<int>& i) { // a fundamental limitation of this approach is that internal conversions between Python and C++ types involve a copy operation that prevents pass-by-reference semantics.
    std::cout << "std::vector<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 5

    i.push_back(-1); // 不会修改python端传入的list中的值
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 5 -1
    std::cout << std::endl;
}

void print_deque(std::deque<int>& i) {
    std::cout << "std::deque<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }

    i.push_back(-1);
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 5 -1
    std::cout << std::endl;
}

void print_list(std::list<int>& i) {
    std::cout << "std::list<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }

    i.push_back(-1);
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 5 -1
    std::cout << std::endl;
}

void print_array(std::array<int, 5>& i) {
    std::cout << "std::array<int, 5>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }

    i[4] = 9999;
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 9999
    std::cout << std::endl;
}

void print_valarray(std::valarray<int>& i) {
    std::cout << "std::vector<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }

    i[4] = 9999;
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->1 2 3 4 9999
    std::cout << std::endl;
}

void print_map(std::map<std::string, int>& i) {
    std::cout << "std::map<std::string, int>" << std::endl;
    for (const auto &pair: i)
        std::cout << pair.first << '\t' << pair.second << std::endl;

    i.insert(std::pair{"d", 4});
    std::cout << std::endl;

    for (const auto &pair: i)
        std::cout << pair.first << '\t' << pair.second << std::endl;
}

void print_set(std::set<int>& i) {
    std::cout << "std::set<int>" << std::endl;
    for (int value : i) {
        std::cout << value << " ";
    }

    i.insert(-1);
    std::cout << std::endl;

    for (int value : i) {
        std::cout << value << " ";
    } // print->-1 1 2 3 4
    std::cout << std::endl;
}

PYBIND11_MODULE(automatic_conversion_stl_type, m) {
    m.def("print_vector", &print_vector);
    m.def("print_deque", &print_deque);
    m.def("print_list", &print_list);
    m.def("print_array", &print_array);
    m.def("print_valarray", &print_valarray);

    m.def("print_map", &print_map);

    m.def("print_set", &print_set);
}