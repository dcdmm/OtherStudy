cmake_minimum_required(VERSION 3.21)

project(example)

add_subdirectory(pybind11)

pybind11_add_module(example main.cpp)

set_target_properties(example
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)