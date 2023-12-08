#include <pybind11/pybind11.h>
#include <vector>
#include <string>
#include <pybind11/stl_bind.h>
#include <map>
#include <set>

namespace py = pybind11;

PYBIND11_MODULE(bind_stl, m) {
    auto vectorInt = py::bind_vector<std::vector<int>>(m, "VectorInt");
vectorInt.def("modify_and_pop_back", [](std::vector<int> &v, int value) {
// 在这里可以根据需要修改向量
for (auto &item : v) {
item += value;
}

// 检查向量是否为空，然后调用 pop_back
if (!v.empty()) {
v.pop_back();
}
});
}