#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <iostream>
#include <vector>
#include <deque>
#include <list>
#include <map>
#include <set>
#include <array>
#include <valarray>

namespace py = pybind11;

std::vector<int> py_list(py::list i) {
    /*
     * template<typename T>
     *     T cast() const
     *
     *  Attempt to cast the Python object into the given C++ type. A cast_error will be throw upon failure.
     */
    std::vector<int> v = py::cast<std::vector<int>>(i); // copy operation
    for (int vi: v)
        std::cout << vi << '\t';
    std::cout << std::endl;
    v.push_back(100); // 不会修改python端传入的list中的值
    auto d = py::cast<std::deque<int>>(i);
    auto l = py::cast<std::list<int>>(i);
    auto a = py::cast<std::array<int, 4>>(i);
    auto vala = py::cast<std::valarray<int>>(i);

    return v;
}

std::map<std::string, int> py_dict(py::dict i) {
    std::map<std::string, int> m = py::cast<std::map<std::string, int>>(i); // copy operation
    m.insert(std::pair{"e", 5});  // 不会修改python端传入的dict中的值
    for (const auto &pair: m)
        std::cout << pair.first << '\t' << pair.second << std::endl;

    return m;
}

std::set<std::string> py_set(py::set i) {
    std::set<std::string> s = py::cast<std::set<std::string>>(i); // copy operation
    s.insert("e"); // 不会修改python端传入的set中的值
    for (std::string value : s) {
        std::cout << value << '\n';
    }

    return s;
}

py::tuple py_tuple(py::tuple i) {
    for (auto item: i){
        int item_cpp = py::cast<int>(item);
        std::cout << item_cpp << std::endl;
    }

    return i;
};

PYBIND11_MODULE(py_type, m) {
    m.def("py_list", &py_list);
    m.def("py_dict", &py_dict);
    m.def("py_set", &py_set);
    m.def("py_tuple", &py_tuple);
}