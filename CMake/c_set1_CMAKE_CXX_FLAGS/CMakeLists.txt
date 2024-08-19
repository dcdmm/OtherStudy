cmake_minimum_required(VERSION 3.21)
project(c_set1_CMAKE_CXX_FLAGS)

# Add default compilation flags to be used when compiling CXX (C++) files.
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra") # 全局编译选项

# Debug版本构建:cmake -DCMAKE_BUILD_TYPE=Debug ..
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -O0") # Debug编译选项(优先级高于全局编译选项)

# Release版本构建:cmake -DCMAKE_BUILD_TYPE=Release ..
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -w -O3")  # Release编译选项(优先级高于全局编译选项)

add_executable(c_set1_CMAKE_CXX_FLAGS main.cpp)
