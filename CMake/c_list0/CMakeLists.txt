cmake_minimum_required(VERSION 3.26)
project(c_list0)

set(CMAKE_CXX_STANDARD 17)

list(APPEND L0 "a" "b" "c")
message(STATUS ${L0})