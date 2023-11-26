cmake_minimum_required(VERSION 3.21)

project(nest_demo0)

set(LIB_PATH ${CMAKE_CURRENT_SOURCE_DIR}/soa_lib)  # 库文件路径
set(EXEC_PATH ${CMAKE_CURRENT_SOURCE_DIR}/exe)  # 可执行文件路径
set(HEAD_PATH ${CMAKE_CURRENT_SOURCE_DIR}/include)  # 可执行文件路径

#[[
Adds a subdirectory to the build.
]]
add_subdirectory(calc)
add_subdirectory(sort)
add_subdirectory(test_calc)
add_subdirectory(test_sort)
