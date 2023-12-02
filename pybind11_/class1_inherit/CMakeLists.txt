cmake_minimum_required(VERSION 3.21)

project(class1_inherit)

find_package(pybind11 CONFIG)

pybind11_add_module(class1_inherit main.cpp)

set_target_properties(class1_inherit
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)