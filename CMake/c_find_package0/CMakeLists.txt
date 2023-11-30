cmake_minimum_required(VERSION 3.21)
project(c_find_package0)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_PREFIX_PATH ${CMAKE_SOURCE_DIR}/pybind11/include)

#[[
The REQUIRED option stops processing with an error message if the package cannot be found.
]]
find_package(pybind11 REQUIRED)

if (pybind11_FOUND)
    message(STATUS "Found pybind11: ${pybind11_INCLUDE_DIRS}")
else ()
    message(FATAL_ERROR "pybind11 not found. Please install it or specify its location.")
endif ()

