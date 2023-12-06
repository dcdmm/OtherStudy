#include <pybind11/pybind11.h>
#include <iostream>

namespace py = pybind11;

void process_custom_class(py::object point) {
    int count = point.attr("count").cast<int>(); // python对象属性
    int x = point.attr("put_x")().cast<int>(); // python对象方法
    double y = point.attr("put_y")(100).cast<double>(); // python对象方法
    int count_s = point.attr("put_count")().cast<int>(); // python对象方法
    std::cout << count << '\t' << '\t' << x << '\t' << y << '\t' << count_s << std::endl;
}

void process_list(py::list lst0, py::list lst1) {
    py::print("init lst0", lst0);
    lst0.attr("append")(-1);
    py::print("append lst0", lst0);
    lst0.attr("extend")(lst1);
    py::print("extend lst0", lst0);
    lst0.attr("pop")();
    py::print("pop lst0", lst0);
    py::print("len", lst0.attr("__len__")());

    py::list lst2 = lst0.attr("__mul__")(2);
    py::print("lst2", lst2);

    py::list lst3 = lst0.attr("__add__")(lst1);
    py::print("lst3", lst3);
}

PYBIND11_MODULE(cpp_access_python_c, m) {
    m.def("process_custom_class", &process_custom_class);
    m.def("process_list", &process_list);
}