cmake_minimum_required(VERSION 3.26)
project(c_list0)

set(CMAKE_CXX_STANDARD 17)

#[[
Operations on semicolon-separated lists.

semicolon-separated lists:https://cmake.org/cmake/help/latest/manual/cmake-language.7.html#cmake-language-lists

list常见操作
* APPEND
* REMOVE_ITEM
* LENGTH
* GET
* JOIN
* FIND
* INSERT
* POP_FRONT
* POP_BACK
]]

list(APPEND L0 a b c d e f)  # APPEND:Appends elements to the list.
message("L0:" ${L0})  # print->a;b;c;d;e;f

list(REMOVE_ITEM L0 a)  # REMOVE_ITEM:Removes all instances of the given items from the list.
message("remove a after L0:" ${L0})  # print->b;c;d;e;f

file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp ${CMAKE_CURRENT_SOURCE_DIR}/src/*.h)
message("OBJ_VAR:" ${OBJ_VAR})
list(REMOVE_ITEM OBJ_VAR ${PROJECT_SOURCE_DIR}/src/head.h)
message("remove after OBJ_VAR:" ${OBJ_VAR})

list(LENGTH L0 L0_LENGTH)  # LENGTH:Returns the list's length.
message("L0_LENGTH:" ${L0_LENGTH}) # print->5

list(GET L0 0 3 2 L0_GET) # GET:Returns the list of elements specified by indices from the list.
message("L0_GET:" ${L0_GET}) # print->b;e;d

#[[
`list(JOIN <list> <glue> <output variable>)`

Returns a string joining all list's elements using the glue string. T
]]
list(JOIN L0 - L0_JOIN)
message("L0_JOIN:" ${L0_JOIN})  # print->b-c-d-e-f;

#[[
`list(FIND <list> <value> <output variable>)`

Returns the index of the element specified in the list or -1 if it wasn't found.
]]
list(FIND L0 d L0_FIND_d)
message("L0_FIND_d:" ${L0_FIND_d}) # print->2
list(FIND L0 z L0_FIND_z)
message("L0_FIND_z:" ${L0_FIND_z}) # print->-1

#[[
`list(INSERT <list> <element_index> <element> [<element> ...])`

Inserts elements to the list to the specified index.
]]
list(INSERT L0 0 a)
message("insert a after L0:" ${L0}) # print->a;b;c;d;e;f

#[[
`list(POP_FRONT <list> [<out-var>...])`

If no variable name is given, removes exactly one element.
Otherwise, with N variable names provided, assign the first N elements' values to the given variables and then remove the first N values from <list>.
]]
list(POP_FRONT L0)
#[[
`list(POP_BACK <list> [<out-var>...])`

If no variable name is given, removes exactly one element.
Otherwise, with N variable names provided, assign the last N elements' values to the given variables and then remove the last N values from <list>.
]]
list(POP_BACK L0)
message("pop front pop back after L0:" ${L0})  # print->b;c;d;e