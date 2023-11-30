cmake_minimum_required(VERSION 3.21)

project(example)

# 需先安装好pybind11(推荐使用vcpkg)
find_package(pybind11 CONFIG)

pybind11_add_module(example main.cpp)

set_target_properties(example
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)