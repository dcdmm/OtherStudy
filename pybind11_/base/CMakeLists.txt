cmake_minimum_required(VERSION 3.21)

project(base)

# This assumes that the pybind11 repository is located in a subdirectory named pybind11
# add_subdirectory(pybind11)

# find_package(pybind11) will only work correctly if pybind11 has been correctly installed on the system
find_package(pybind11 CONFIG) # 与上等价

pybind11_add_module(base base.cpp)

# 指定动态库生成路径
set_target_properties(base
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)