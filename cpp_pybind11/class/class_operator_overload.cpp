#include <string>
#include <pybind11/operators.h>

namespace py = pybind11;

class Vector2 {
public:
    Vector2(float x, float y) : x(x), y(y) {}

    Vector2 operator+(const Vector2 &v) const { return Vector2(x + v.x, y + v.y); }

    Vector2 operator*(float value) const { return Vector2(x * value, y * value); }

    Vector2 &operator+=(const Vector2 &v) {
        x += v.x;
        y += v.y;
        return *this;
    }

    Vector2 &operator*=(float v) {
        x *= v;
        y *= v;
        return *this;
    }

    friend Vector2 operator*(float f, const Vector2 &v) {
        return Vector2(f * v.x, f * v.y);
    }

    std::string toString() const {
        return "[" + std::to_string(x) + ", " + std::to_string(y) + "]";
    }

private:
    float x, y;
};

PYBIND11_MODULE(class_operator_overload, m) {
    py::class_<Vector2>(m, "Vector2")
            .def(py::init<float, float>())

            // 运算符重载
            .def(py::self + py::self)
            .def(py::self += py::self)
            .def(py::self *= float())
            // The py::is_operator flag marker is needed to inform pybind11 that this is an operator, which returns NotImplemented when invoked with incompatible arguments rather than throwing a type error.
            // .def("__mul__", [](const Vector2 &a, float b) { return a * b;}, py::is_operator())  // 绑定python __mul__函数
            .def(float() * py::self) // 简写形式;与上等价
            .def(py::self * float())

            .def("__repr__", &Vector2::toString);
}