cmake_minimum_required(VERSION 3.21)
project(c_add_compile_definitions0)

#[[
`add_compile_definitions(<definition> ...)`

The preprocessor definitions are added to the COMPILE_DEFINITIONS directory property for the current CMakeLists file.
They are also added to the COMPILE_DEFINITIONS target property for each target in the current CMakeLists file.
]]
add_compile_definitions(DEBUG)

get_directory_property(DCD COMPILE_DEFINITIONS)
message(STATUS "Compile definitions for current directory: ${DCD}")

add_executable(c_add_compile_definitions0 main.cpp)
