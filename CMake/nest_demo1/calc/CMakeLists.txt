cmake_minimum_required(VERSION 3.21)

aux_source_directory(./ SRC)
add_library(calc SHARED ${SRC})  # 动态库
target_include_directories(calc PUBLIC ${HEAD_PATH})
set_target_properties(
        calc
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${LIB_PATH})