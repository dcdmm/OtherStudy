#include "iostream"
#include <pybind11/pybind11.h>

namespace py = pybind11;

using namespace std;

template<typename T>
class Beta {
private:
    template<class V>
    class Hold;

    Hold<T> q; // 基于T类型的Hold参数
    Hold<int> n; // 基于int类型的Hold对象
public:
    Beta(T t, int i) : q(t), n(i) {};

    // 类的成员函数也可以定义为模板
    template<typename U>
    U blab(U u, T t);

    void show() const {
        q.show();
        n.show();
    }
};

// 类外定义模板成员
template<typename T> // 类模板之外的成员函数必须以类模板模板前缀(本例为:template<typename T>)开头
template<typename V>
class Beta<T>::Hold {
private:
    V val;
public:
    explicit Hold(V v = 0) : val(v) {}

    void show() const { cout << val << endl; }

    V value() const { return val; }
};

template<typename T>
template<typename U>
U Beta<T>::blab(U u, T t) { return (n.value() + q.value()) * u / t; }

PYBIND11_MODULE(class_template, m) {
    // C++ templates may only be instantiated at compile time, so pybind11 can only wrap instantiated templated classes.
    py::class_<Beta<double>>(m, "Beta")
            .def(py::init<double, int>())
            // If your class methods have template parameters you can wrap those as well, but once again each instantiation must be explicitly specified
            .def("blab", &Beta<double>::blab<double>)

            .def("show", &Beta<double>::show);
}
