cmake_minimum_required(VERSION 3.21)
project(c_aux_source_directory0)

set(CMAKE_CXX_STANDARD 17)

#[[
Find all source files in a directory

`aux_source_directory(<dir> <variable>)`

Collects the names of all the source files in the specified directory and stores the list in the <variable> provided.
]]
aux_source_directory(${PROJECT_SOURCE_DIR} OBJ_VAR)

add_executable(c_aux_source_directory0 ${OBJ_VAR})