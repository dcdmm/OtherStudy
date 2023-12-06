cmake_minimum_required(VERSION 3.21)

project(func)

find_package(pybind11 CONFIG)

pybind11_add_module(func0_base func0_base.cpp)
set_target_properties(func0_base PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/func0_base)

pybind11_add_module(func_template_overload func_template_overload.cpp)
set_target_properties(func_template_overload PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/func_template_overload)

pybind11_add_module(args_kwargs args_kwargs.cpp)
set_target_properties(args_kwargs PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest/args_kwargs)
