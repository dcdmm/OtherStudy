cmake_minimum_required(VERSION 3.21)

aux_source_directory(./ SRC)
add_library(sort SHARED ${SRC})  # 动态库
target_include_directories(sort PUBLIC ${HEAD_PATH})
set_target_properties(
        sort
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY ${LIB_PATH})

# 动态库链接动态库
target_link_libraries(sort PUBLIC ${LIB_PATH}/libcalc.so)