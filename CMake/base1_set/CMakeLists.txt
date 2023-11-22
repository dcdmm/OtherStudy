cmake_minimum_required(VERSION 3.26)
project(base1_set)

#[[
Set a normal, cache, or environment variable to a given value.
See the cmake-language(7) variables documentation for the scopes and interaction of normal variables and cache entries.

]]
set(CMAKE_CXX_STANDARD 17)  # 指定c++标准:98(C++98)/11(C++11)/14(C++14)/20(C++20)/23(C++23)/26(C++26)

set(OBJ_VAR main.cpp add.cpp sub.cpp)
#[[
A variable reference has the form ${<variable>} and is evaluated inside a Quoted Argument or an Unquoted Argument.
A variable reference is replaced by the value of the specified variable or cache entry, or if neither is set, by the empty string. Variable references can nest and are evaluated from the inside out, e.g. ${outer_${inner_variable}_variable}.
]]
add_executable(base1_set ${OBJ_VAR})
# add_executable(qwer main.cpp add.cpp sub.cpp) # 与上等价