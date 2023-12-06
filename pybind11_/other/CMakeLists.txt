cmake_minimum_required(VERSION 3.21)

project(other)

find_package(pybind11 CONFIG)

pybind11_add_module(xx xx.cpp)
set_target_properties(xx PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/xx)