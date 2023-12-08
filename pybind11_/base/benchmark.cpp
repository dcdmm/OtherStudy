#include <pybind11/pybind11.h>


long long sum_cpp(int n) {
    long long result = 0;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            result += 1;
        }
    }
    return result;
}

PYBIND11_MODULE(benchmark, m) {
    m.def("sum_cpp", &sum_cpp);
}