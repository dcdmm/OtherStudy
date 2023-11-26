cmake_minimum_required(VERSION 3.21)
project(c_target_include_directories0)

set(CMAKE_CXX_STANDARD 17)

file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)  # c_target_include_directories0/src目录(不递归遍历子目录)下所有.cpp文件
add_executable(c_target_include_directories0 ${OBJ_VAR} main.cpp)

#[[
Specifies include directories to use when compiling a given target.
]]
target_include_directories(c_target_include_directories0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

get_target_property(DIRS c_target_include_directories0 INCLUDE_DIRECTORIES)
message("target_include_directories: ${DIRS}")