cmake_minimum_required(VERSION 3.21)

project(test_sort)
aux_source_directory(./ SRC)
add_executable(test_sort ${SRC})

target_include_directories(test_sort PUBLIC ${HEAD_PATH})
target_link_libraries(test_sort PUBLIC ${LIB_PATH}/libsort.so)
set_target_properties(
        test_sort
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${EXEC_PATH})

