#include <pybind11/pybind11.h>
#include <iostream>

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
void py_list(py::list i) {
    // py::list常见操作
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    i.append(-1);
    i[0] = 999; // 更新
}

void py_dict(py::dict i) {
    // py::dict常见操作
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    std::cout << i.contains("e") << std::endl;
    i["e"] = -1; // 添加
    i["a"] = 999; // 更新
}

void py_set(py::set i) {
    std::cout << i.size() << std::endl;
    std::cout << i.empty() << std::endl;
    std::cout << i.contains("e") << std::endl;
    i.add("e");
}

PYBIND11_MODULE(py_type, m) {
    m.def("py_list", &py_list);
    m.def("py_dict", &py_dict);
    m.def("py_set", &py_set);
}