#include <pybind11/pybind11.h>

int add(int i, int j) {
    return i + j;
}

/*
 * This macro creates the entry point that will be invoked when the Python interpreter imports an extension module.
 * The module name is given as the first argument and it should not be in quotes.
 * The second macro argument defines a variable of type py::module_ which can be used to initialize the module.
 */
PYBIND11_MODULE(example, m) {
    // Get or set the object’s docstring, i.e. obj.__doc__.
    m.doc() = "pybind11 example plugin";

    /*
     * Create Python binding for a new function within the module scope.
     * Func can be a plain C++ function, a function pointer, or a lambda function.
     * For details on the Extra&& ... extra argument, see section Passing extra arguments to def or class_.
     * Passing extra arguments to def or class_:https://pybind11.readthedocs.io/en/stable/reference.html#extras
     */
    m.def("add", &add, "A function that adds two numbers");
}