cmake_minimum_required(VERSION 3.21)
project(c_target_link_libraries0)

set(CMAKE_CXX_STANDARD 17)

add_executable(c_target_link_libraries0  main.cpp)

target_include_directories(c_target_link_libraries0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

#[[
Specify libraries or flags to use when linking a given target and/or its dependents.
Usage requirements from linked library targets will be propagated.

Usage requirements:https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html#target-usage-requirements
]]

set_target_properties(c_target_link_libraries0
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/dll_a)

# 可执行程序链接动态库
target_link_libraries(c_target_link_libraries0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/dll_a/libadd_sub_sh.so)

# 可执行程序链接静态库
# target_link_libraries(c_target_link_libraries0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/dll_a/libadd_sub_st.a)

# 动态库链接动态库参考:nest_demo1