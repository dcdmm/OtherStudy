cmake_minimum_required(VERSION 3.21)

project(test_calc)
aux_source_directory(./ SRC)
add_executable(test_calc ${SRC})

target_include_directories(test_calc PUBLIC ${HEAD_PATH})
target_link_libraries(test_calc PUBLIC ${LIB_PATH}/libcalc.a)
set_target_properties(
        test_calc
        PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${EXEC_PATH})

