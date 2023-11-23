cmake_minimum_required(VERSION 3.26)
project(c_target_link_libraries0)

set(CMAKE_CXX_STANDARD 17)

add_executable(c_target_link_libraries0  main.cpp)

target_include_directories(c_target_link_libraries0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)


#[[
Specify libraries or flags to use when linking a given target and/or its dependents.
Usage requirements from linked library targets will be propagated.

Usage requirements:https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html#target-usage-requirements
]]
target_link_libraries(c_target_link_libraries0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/dll_a)