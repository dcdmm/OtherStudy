cmake_minimum_required(VERSION 3.21)
project(c_set0)

#[[
Set a normal, cache, or environment variable to a given value.
See the cmake-language(7) variables documentation for the scopes and interaction of normal variables and cache entries.

cmake-language(7) variables:https://cmake.org/cmake/help/latest/manual/cmake-language.7.html#cmake-language-variables
]]
set(CMAKE_CXX_STANDARD 17)  # 指定c++标准为C++17

set(OBJ_VAR
        main.cpp
        add.cpp
        sub.cpp)

message("***********************")
# 参考:https://cmake.org/cmake/help/latest/manual/cmake-language.7.html#cmake-language-lists
set(srcs a.c b.c c.c) # sets "srcs" to "a.c;b.c;c.c"
message(${srcs})

message(${OBJ_VAR})
message("***********************")

#[[
A variable reference has the form ${<variable>} and is evaluated inside a Quoted Argument or an Unquoted Argument.
A variable reference is replaced by the value of the specified variable or cache entry, or if neither is set, by the empty string.
Variable references can nest and are evaluated from the inside out, e.g. ${outer_${inner_variable}_variable}.
]]
add_executable(c_set0 ${OBJ_VAR})
# add_executable(command_set0 main.cpp add.cpp sub.cpp) # 与上等价