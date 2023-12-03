cmake_minimum_required(VERSION 3.21)

project(class)

find_package(pybind11 CONFIG)

pybind11_add_module(class0 main.cpp)

set_target_properties(class0
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)