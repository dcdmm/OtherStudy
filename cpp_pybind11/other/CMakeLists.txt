cmake_minimum_required(VERSION 3.21)

project(other)

find_package(pybind11 CONFIG)

pybind11_add_module(py_type py_type.cpp)
set_target_properties(py_type PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/py_type)

pybind11_add_module(py_type_cast py_type_cast.cpp)
set_target_properties(py_type_cast PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/py_type_cast)

pybind11_add_module(cpp_access_python_f cpp_access_python_f.cpp)
set_target_properties(cpp_access_python_f PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/cpp_access_python_f)

pybind11_add_module(cpp_access_python_c cpp_access_python_attr.cpp)
set_target_properties(cpp_access_python_c PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/cpp_access_python_attr)
