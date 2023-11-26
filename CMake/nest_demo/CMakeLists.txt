cmake_minimum_required(VERSION 3.26)

project(nest_demo)

set(LIB_PATH ${CMAKE_CURRENT_SOURCE_DIR}/lib)
set(EXEC_PATH ${CMAKE_CURRENT_SOURCE_DIR}/bin)
set(HEAD_PATH ${CMAKE_CURRENT_SOURCE_DIR}/include)

add_subdirectory(calc)
add_subdirectory(sort)
add_subdirectory(test_calc)
add_subdirectory(test_sort)
