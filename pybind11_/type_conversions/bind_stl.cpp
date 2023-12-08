#include <pybind11/pybind11.h>
#include <vector>
#include <string>
#include <pybind11/stl_bind.h>
#include <map>
#include <set>

namespace py = pybind11;

PYBIND11_MODULE(bind_stl, m) {
    auto vectorInt = py::bind_vector<std::vector<int>>(m, "VectorInt");
    vectorInt.def("pop_back", &std::vector<int>::pop_back);
}