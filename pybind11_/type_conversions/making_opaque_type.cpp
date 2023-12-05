#include <pybind11/pybind11.h>
#include <vector>
#include <string>

namespace py = pybind11;

PYBIND11_MAKE_OPAQUE(std::vector<std::string>);
using StringList = std::vector<std::string>;

PYBIND11_MODULE(making_opaque_type, m) {
    py::class_<StringList>(m, "StringList")
            .def(py::init<>())
            .def("pop_back", &StringList::pop_back)
            .def("push_back", (void(StringList::*)(const std::string &)) & StringList::push_back)
            .def("back", (std::string & (StringList::*) ()) & StringList::back)
            .def("__len__", [](const StringList &v) { return v.size(); });

    m.def("print_opaque_list", [](const StringList &l) {
        std::string ret = "Opaque list: [";
        bool first = true;
        for (const auto &entry : l) {
            if (!first) {
                ret += ", ";
            }
            ret += entry;
            first = false;
        }
        return ret + "]";
    });
}