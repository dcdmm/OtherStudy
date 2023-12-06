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

/*
 * pybind11 exposes all major Python types using thin C++ wrapper classes.
 * These wrapper classes can also be used as parameters of functions in bindings, which makes it possible to directly work with native Python types on the C++ side.
 *
 * Available types include:
 * * handle(pytypes.h)
 * * object(pytypes.h)
 * * bool_(pytypes.h)
 * * int_(pytypes.h)
 * * float_(pytypes.h)
 * * str(pytypes.h)
 * * bytes(pytypes.h)
 * * tuple(pytypes.h)
 * * list(pytypes.h)
 * * dict(pytypes.h)
 * * slice(pytypes.h)
 * * none(pytypes.h)
 * * capsule(pytypes.h)
 * * iterable(pytypes.h)
 * * iterator(pytypes.h)
 * * function(pytypes.h)
 * * buffer(pytypes.h)
 * * array(numpy.h)
 * * array_t(numpy.h)
 */

/*
 * The Python list is not converted in any way – it’s just wrapped in a C++ py::list class.
 * At its core it’s still a Python object. Copying a py::list will do the usual reference-counting like in Python.
 * Returning the object to Python will just remove the thin wrapper.
 */
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

    // py::list常见操作
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    i.append(-1);
    i[0] = 999; // 修改
    std::cout << py::cast<int>(i[0]) << std::endl; // 索引
    i.insert(0, -1111); // 插入
    std::cout << "**************************************\n";
    for (auto item: i){
        int item_cpp = py::cast<int>(item);  //
        std::cout << item_cpp << std::endl;
    }
    std::cout << "**************************************\n";

    return v;
}

std::map<std::string, int> py_dict(py::dict i) {
    std::map<std::string, int> m = py::cast<std::map<std::string, int>>(i); // copy operation
    m.insert(std::pair{"e", 5});  // 不会修改python端传入的dict中的值
    for (const auto &pair: m)
        std::cout << pair.first << '\t' << pair.second << std::endl;

    // py::dict常见操作
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    std::cout << i.contains("e") << std::endl;
    i["e"] = -1; // 修改
    std::cout << py::cast<int>(i["e"]) << std::endl; // 索引
    i["a"] = 999; // 更新
    std::cout << "**************************************\n";
    for (auto item: i){
        std::string key = py::cast<std::string>(item.first); // 转换键
        int value = py::cast<int>(item.second); // 转换值
        std::cout << "Key: " << key << ", Value: " << value << std::endl;
    }
    std::cout << "**************************************\n";

    return m;
}

std::set<std::string> py_set(py::set i) {
    std::set<std::string> s = py::cast<std::set<std::string>>(i); // copy operation
    s.insert("e"); // 不会修改python端传入的set中的值
    for (std::string value : s) {
        std::cout << value << " ";
    }
    std::cout << std::endl;

    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    std::cout << i.contains("e") << std::endl;
    i.add("e"); // 添加
    std::cout << "**************************************\n";
    for (auto item : i) {
        std::string item_cpp = py::cast<std::string>(item);
        std::cout << item_cpp << std::endl;
    }
    std::cout << "**************************************\n";

    return s;
}

void py_tuple(py::tuple i) {
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    std::cout << py::cast<int>(i[0]) << std::endl; // 索引(不能修改元组元素值)
    std::cout << "**************************************\n";
    for (auto item: i){
        int item_cpp = py::cast<int>(item);
        std::cout << item_cpp << std::endl;
    }
    std::cout << "**************************************\n";
};

PYBIND11_MODULE(py_type, m) {
    m.def("py_list", &py_list);
    m.def("py_dict", &py_dict);
    m.def("py_set", &py_set);
    m.def("py_tuple", &py_tuple);
}