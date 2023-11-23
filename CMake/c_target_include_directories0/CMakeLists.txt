cmake_minimum_required(VERSION 3.26)
project(c_target_include_directories0)

set(CMAKE_CXX_STANDARD 17)

file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR} src/*.cpp)  # c_file0目录(不遍历子目录)下src目录下所有cpp文件
add_executable(c_target_include_directories0 ${OBJ_VAR} main.cpp)

#[[
Add the given directories to those the compiler uses to search for include files.

The include directories are added to the INCLUDE_DIRECTORIES directory property for the current CMakeLists file.
They are also added to the INCLUDE_DIRECTORIES target property for each target in the current CMakeLists file.
]]
#include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# Stores a property of directory scope in the named <variable>.
#get_directory_property(DIRS INCLUDE_DIRECTORIES)
#message("include_directories: ${DIRS}")

#[[
Specifies include directories to use when compiling a given target.
]]
target_include_directories(c_target_include_directories0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
# Get a property from a target.
get_target_property(DIRS c_target_include_directories0 INCLUDE_DIRECTORIES)
message("target_include_directories: ${DIRS}")


#[[
Note: Prefer the target_include_directories() command to add include directories to individual targets and optionally propagate/export them to dependents.
]]