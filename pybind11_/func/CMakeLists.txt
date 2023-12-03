cmake_minimum_required(VERSION 3.21)

project(func)

find_package(pybind11 CONFIG)

pybind11_add_module(func0_base func0_base.cpp)
set_target_properties(func0_base PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)