cmake_minimum_required(VERSION 3.21)

project(base)

# This assumes that the pybind11 repository is located in a subdirectory named pybind11
# add_subdirectory(pybind11)

# find_package(pybind11) will only work correctly if pybind11 has been correctly installed on the system
find_package(pybind11 CONFIG) # 与上等价

pybind11_add_module(base base.cpp)

# base.cpython-310-x86_64-linux-gnu.so
# * base:python中模块名称(应与pybind11定义的模块名称相同)
# * cpython-310:为CPython(Python3.10版本)编译的
# * x86_64:指令集架构,即基于x86架构的64位拓展
# * linux-gnu:在基于GNU工具链和库的Linux系统上编译的
# * .so:动态库后缀

# 指定动态库生成路径
set_target_properties(base PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)

pybind11_add_module(benchmark benchmark.cpp)
set_target_properties(benchmark PROPERTIES LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/pytest)