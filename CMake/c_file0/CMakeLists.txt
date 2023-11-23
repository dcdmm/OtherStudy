cmake_minimum_required(VERSION 3.26)
project(c_file0)

set(CMAKE_CXX_STANDARD 17)

#[[
File manipulation command.
This command is dedicated to file and path manipulation requiring access to the filesystem.

`file(GLOB <variable> [LIST_DIRECTORIES true|false] [RELATIVE <path>] [CONFIGURE_DEPENDS] [<globbing-expressions>...])`

Generate a list of files that match the <globbing-expressions> and store it into the <variable>.


`file(GLOB_RECURSE <variable> [FOLLOW_SYMLINKS] [LIST_DIRECTORIES true|false] [RELATIVE <path>] [CONFIGURE_DEPENDS] [<globbing-expressions>...])`

The GLOB_RECURSE mode will traverse all the subdirectories of the matched directory and match the files.

Examples of globbing expressions include:
*.cxx       match all files with extension cxx
*.vt?       match all files with extension vta, ..., vtz
f[3-5].txt  match files f3.txt, f4.txt, f5.txt
]]
file(GLOB OBJ_VAR ${CMAKE_CURRENT_SOURCE_DIR} *.cpp) # c_file0目录(不遍历子目录)下所有cpp文件
add_executable(c_file0 ${OBJ_VAR})