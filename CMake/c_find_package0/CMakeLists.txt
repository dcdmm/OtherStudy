cmake_minimum_required(VERSION 3.21)
project(c_find_package0)

set(CMAKE_CXX_STANDARD 17)

#[[
Find a package (usually provided by something external to the project), and load its package-specific details

The CONFIG option, the synonymous NO_MODULE option, or the use of options not specified in the basic signature all enforce pure Config mode.
In pure Config mode, the command skips Module mode search and proceeds at once with Config mode search.

The REQUIRED option stops processing with an error message if the package cannot be found.
]]
find_package(pybind11 CONFIG)

if (pybind11_FOUND)
    message(STATUS "Found pybind11: ${pybind11_INCLUDE_DIRS}")
else ()
    message(FATAL_ERROR "pybind11 not found. Please install it or specify its location.")
endif ()

