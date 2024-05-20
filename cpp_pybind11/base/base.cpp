#include <pybind11/pybind11.h>

// python模块名为base
PYBIND11_MODULE(base, m) {
    m.doc() = "base doc";  // 绑定python模块文档
}