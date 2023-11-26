cmake_minimum_required(VERSION 3.26)

aux_source_directory(./ SRC)
add_library(calc STATIC ${SRC})  # 静态库
target_include_directories(calc PUBLIC ${HEAD_PATH})
set_target_properties(
        calc
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY ${LIB_PATH})