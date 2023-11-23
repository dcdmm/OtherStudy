cmake_minimum_required(VERSION 3.26)
project(c_set_target_properties0)

set(CMAKE_CXX_STANDARD 17)

file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR} src/*.cpp) # c_file0目录(不遍历子目录)下src目录下所有cpp文件

add_library(add_sub_sh SHARED ${OBJ_VAR})
add_library(add_sub_st STATIC ${OBJ_VAR})
add_executable(c_set_target_properties0 ${OBJ_VAR} main.cpp)


target_include_directories(add_sub_sh PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_include_directories(add_sub_st PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_include_directories(c_set_target_properties0 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

#[[
`set_target_properties(target1 target2 ...
                      PROPERTIES prop1 value1
                      prop2 value2 ...)`
Sets properties on targets. The syntax for the command is to list all the targets you want to change, and then provide the values you want to set next.
]]
set_target_properties(
        add_sub_st
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build/lib) # 设置静态库生成目录

set_target_properties(c_set_target_properties0
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build/run) # 设置可执行文件生成目录

# 设置动态库生成目录
if(WIN32)
    # Windows系统中,动态库(*.dll)生成目录由RUNTIME_OUTPUT_DIRECTORY控制
    set_target_properties(add_sub_sh
            PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build/dll)
else()
    set_target_properties(add_sub_sh
            PROPERTIES
            LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build/dll)
endif()


get_target_property(add_sub_p add_sub_st ARCHIVE_OUTPUT_DIRECTORY)
message("ARCHIVE_OUTPUT_DIRECTORY:" ${add_sub_p})
