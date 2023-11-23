cmake_minimum_required(VERSION 3.26)
project(c_message0)

set(CMAKE_CXX_STANDARD 17)

# Record the specified message text in the log.
message(STATUS ${CMAKE_CURRENT_SOURCE_DIR})

# The main interesting messages that project users might be interested in. Id
message(STATUS ${PROJECT_SOURCE_DIR})

# CMake Warning, continue processing.
message(WARNING "===>WARNING")

# CMake Error, continue processing, but skip generation.
message(SEND_ERROR "===>SEND_ERROR")

# CMake Error, stop processing and generation.
message(FATAL_ERROR "===>FATAL_ERROR")


