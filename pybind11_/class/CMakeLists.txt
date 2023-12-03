cmake_minimum_required(VERSION 3.21)

project(class)

find_package(pybind11 CONFIG)

pybind11_add_module(class0_base class0_base.cpp)

set_target_properties(class0_base
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)