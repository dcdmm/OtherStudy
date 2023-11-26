cmake_minimum_required(VERSION 3.26)

aux_source_directory(./ SRC)
add_library(sort SHARED ${SRC})  # 动态库
target_include_directories(sort PUBLIC ${HEAD_PATH})
set_target_properties(
        sort
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY ${LIB_PATH})