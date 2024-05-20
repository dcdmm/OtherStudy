#include <pybind11/pybind11.h>
#include <iostream>

namespace py = pybind11;

void process_custom_class(py::object point) {
    // 操作的是python对象的引用(会修改python对象)
    int count = point.attr("count").cast<int>(); // 调用python对象属性
    int x = point.attr("put_x")().cast<int>(); // 调用python对象方法
    double y = point.attr("put_y")(100).cast<double>(); // 调用python对象方法
    int count_s = point.attr("put_count")().cast<int>(); // 调用python对象方法
    std::cout << count << '\t' << '\t' << x << '\t' << y << '\t' << count_s << std::endl;
}

void process_list(py::list lst0, py::list lst1) {
    py::print("init lst0", lst0); // 打印pyton对象,等价于python端print(lst0)
    lst0.attr("append")(-1); // list内置方法
    py::print("append lst0", lst0);
    lst0.attr("extend")(lst1);
    py::print("extend lst0", lst0);
    lst0.attr("pop")();
    py::print("pop lst0", lst0);
    py::print("len", lst0.attr("__len__")()); // 调用python对象调用魔法方法

    py::list lst2 = lst0.attr("__mul__")(2); // 即lst0 * 2
    py::print("lst2", lst2);

    py::list lst3 = lst0.attr("__add__")(lst1); // 即lst0 + lst1
    py::print("lst3", lst3);
}

PYBIND11_MODULE(cpp_access_python_c, m) {
    m.def("process_custom_class", &process_custom_class);
    m.def("process_list", &process_list);
}