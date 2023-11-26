cmake_minimum_required(VERSION 3.21)
project(c_add_library0)

set(CMAKE_CXX_STANDARD 17)

file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp) # c_add_library0目录(不递归遍历子目录)下所有.cpp文件

# Add a library to the project using the specified source files.
add_library(add_sub_sh SHARED ${OBJ_VAR})  # Linux/Windows系统动态库后缀:.so/.dll
add_library(add_sub_st STATIC ${OBJ_VAR})  # Linux/Windows系统静态库后缀:.a/.lib(cmake .. -G "MinGW Makefiles" + mingw32-make.exe编译生成的静态库后缀仍为.a)

target_include_directories(add_sub_sh PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_include_directories(add_sub_st PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)