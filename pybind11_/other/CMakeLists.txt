cmake_minimum_required(VERSION 3.21)

project(other)

find_package(pybind11 CONFIG)

pybind11_add_module(py_type_ldst py_type_ldst.cpp)
set_target_properties(py_type_ldst PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/py_type_ldst)