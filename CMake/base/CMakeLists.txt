# 注释
# Sets the minimum required version of cmake for a project.
cmake_minimum_required(VERSION 3.26)
# Sets the name of the project, and stores it in the variable PROJECT_NAME.
project(base)
#[[
多行注释
]]
#[[
Adds an executable target called <name> to be built from the source files listed in the command invocation.
The <name> corresponds to the logical target name and must be globally unique within a project.
]]
add_executable(base main.cpp)

#[[
# 构建base项目

# Build dir
mkdir build
cd build

# Windows
# cmake .. -G "MinGW Makefiles"
# or
# Linux
cmake

# Windows
# mingw32-make.exe
# or
# Linux
make
]]