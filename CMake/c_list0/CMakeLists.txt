cmake_minimum_required(VERSION 3.26)
project(c_list0)

set(CMAKE_CXX_STANDARD 17)

#[[
Operations on semicolon-separated lists.

semicolon-separated lists:https://cmake.org/cmake/help/latest/manual/cmake-language.7.html#cmake-language-lists
]]
list(APPEND L0 "a" "b" "c")  # APPEND:Appends elements to the list.
message("L0:" ${L0})  # a;b;c

file(GLOB SRC ${PROJECT_SOURCE_DIR} src/*.cpp)
message("SRC:" ${SRC})

