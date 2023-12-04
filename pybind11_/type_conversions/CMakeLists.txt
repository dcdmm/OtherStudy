cmake_minimum_required(VERSION 3.21)

project(type_conversions)

find_package(pybind11 CONFIG)

pybind11_add_module(base_data_type base_data_type.cpp)
set_target_properties(base_data_type PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/base_data_type)