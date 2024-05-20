#include <pybind11/pybind11.h>
#include <vector>
#include <string>
#include <pybind11/stl_bind.h>
#include <map>
#include <algorithm>

namespace py = pybind11;

template<class Type>
class MultValue {
private:
    Type Factor;
public:
    MultValue(const Type &value) : Factor(value) {}

    Type operator( )(Type &elem) const {
        return elem * Factor;
    }
};

void multiply_elements(std::vector<int>& v, int factor) {
    std::transform(v.begin(), v.end(), v.begin(),MultValue<int>(factor)); // stl算法
}

PYBIND11_MODULE(bind_stl, m) {
    // 绑定std::vector<int>为python VectorInt类,并模仿python list中常见的方法(底层为c++实现)
    auto vectorInt = py::bind_vector<std::vector<int>>(m, "VectorInt");
    // 额外自定义绑定的方法
    vectorInt.def("pop_back", &std::vector<int>::pop_back);
    vectorInt.def("multiply_elements", &multiply_elements); // c++独立函数:第一个参数必须为std::vector<int>& v表示实例本身(即python类方法中的self)

    // 绑定std::map<std::string, double>为python MapStringDoubl类,并模仿python dict中常见的方法(底层为c++实现)
    py::bind_map<std::map<std::string, double>>(m, "MapStringDouble");
}