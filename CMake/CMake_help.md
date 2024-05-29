## PUBLIC/PRIVATE/INTERFACE

| Include Inheritance | Description                                                                                                                                                                                                                                              |
|---------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PUBLIC`            | All the directories following `PUBLIC` will be used for the current target and the other targets that have dependencies on the current target, i.e., appending the directories to `INCLUDE_DIRECTORIES` and `INTERFACE_INCLUDE_DIRECTORIES`.             |
| `PRIVATE`           | All the include directories following `PRIVATE` will be used for the current target only, i.e., appending the directories to `INCLUDE_DIRECTORIES`.                                                                                                      |
| `INTERFACE`         | All the include directories following `INTERFACE` will NOT be used for the current target but will be accessible for the other targets that have dependencies on the current target, i.e., appending the directories to `INTERFACE_INCLUDE_DIRECTORIES`. |

| Link Type   | Description                                                                                                                                                                      |
|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PUBLIC`    | All the objects following `PUBLIC` will be used for linking to the current target and providing the interface to the other targets that have dependencies on the current target. |
| `PRIVATE`   | All the objects following `PRIVATE` will only be used for linking to the current target.                                                                                         |
| `INTERFACE` | All the objects following `INTERFACE` will only be used for providing the interface to the other targets that have dependencies on the current target.                           |

    * 使用参考:c_target_include_directories0/c_target_link_libraries0

## cmake(1)

* -S <path-to-source>
    * Path to root directory of the CMake project to build.

* -B <path-to-build>
    * Path to directory which CMake will use as the root of build directory.
    * If the directory doesn't already exist CMake will make it. If the directory doesn't already exist CMake will make it.

    | Command Line             | Source Dir | Build Dir |
    |:-------------------------|:-----------|:----------|
    | `cmake -B build`         | cwd        | `build`   |
    | `cmake -B build src`     | `src`      | `build`   |
    | `cmake -B build -S src`  | `src`      | `build`   |
    | `cmake src`              | `src`      | cwd       |
    | `cmake build` (existing) | loaded     | `build`   |
    | `cmake -S src`           | `src`      | cwd       |
    | `cmake -S src build`     | `src`      | `build`   |
    | `cmake -S src -B build`  | `src`      | `build`   |

* -G <generator-name>
    * Specify a build system generator.
    * CMake may support multiple native build systems on certain platforms. A generator is responsible for generating a
      particular build system. Possible generator names are specified in the [cmake-generators(7)](https://cmake.org/cmake/help/latest/manual/cmake-generators.7.html#manual:cmake-generators(7))
      manual.
    * If not specified, CMake checks the [CMAKE_GENERATOR](https://cmake.org/cmake/help/latest/envvar/CMAKE_GENERATOR.html#envvar:CMAKE_GENERATOR) environment variable and otherwise falls back to a builtin default selection.

## cmake-variables(7)

* PROJECT_NAME
    * Name of the project given to the project command.

* CMAKE_CXX_STANDARD
    * Default value for [CXX_STANDARD](https://cmake.org/cmake/help/latest/prop_tgt/CXX_STANDARD.html#prop_tgt:CXX_STANDARD) target property if set when a target is created.
    * See the [cmake-compile-features(7)](https://cmake.org/cmake/help/latest/manual/cmake-compile-features.7.html#manual:cmake-compile-features(7)) manual for information on compile features and a list of supported compilers.

* PROJECT_SOURCE_DIR
    * This is the source directory of the last call to the project() command made in the current directory scope or one of its parents.

* PROJECT_BINARY_DIR
    * Full path to build directory for project.

* CMAKE_CURRENT_SOURCE_DIR
    * This is the full path to the source directory that is currently being processed by cmake.

* CMAKE_CURRENT_BINARY_DIR
    * This is the full path to the build directory that is currently being processed by cmake.

* CMAKE_PREFIX_PATH
    * Semicolon-separated list of directories specifying installation prefixes to be searched by the find_package(), find_program(), find_library(), find_file(), and find_path() commands.

* CMAKE_MODULE_PATH
    * Semicolon-separated list of directories, represented using forward slashes, specifying a search path for CMake modules to be loaded by the include() or find_package() commands before checking the default modules that come with CMake. 
    * By default it is empty. It is intended to be set by the project.
* CMAKE_TOOLCHAIN_FILE
    * Path to toolchain file supplied to cmake(1).
    * This variable is specified on the command line when cross-compiling with CMake. 
    * It is the path to a file which is read early in the CMake run and which specifies locations for compilers and toolchain utilities, and other target platform and compiler related information.

## cmake-properties(7)

* INCLUDE_DIRECTORIES
    * List of preprocessor include file search directories.

* INTERFACE_INCLUDE_DIRECTORIES
    * List of public include directories requirements for a library.
    * Targets may populate this property to publish the include directories required to compile against the headers for the target.
    * The [target_include_directories()](https://cmake.org/cmake/help/latest/command/target_include_directories.html#command:target_include_directories) command populates this property with values given to the PUBLIC and INTERFACE keywords.

* ARCHIVE_OUTPUT_DIRECTORY
    * Output directory in which to build [ARCHIVE](https://cmake.org/cmake/help/latest/prop_tgt/ARCHIVE_OUTPUT_DIRECTORY.html#prop_tgt:ARCHIVE_OUTPUT_DIRECTORY) target files.
    * 使用参考:c_set_target_properties0

* LIBRARY_OUTPUT_DIRECTORY
    * Output directory in which to build [LIBRARY](https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html#library-output-artifacts) target files.
    * 使用参考:c_set_target_properties0

* RUNTIME_OUTPUT_DIRECTORY
    * Output directory in which to build [RUNTIME](https://cmake.org/cmake/help/latest/manual/cmake-buildsystem.7.html#runtime-output-artifacts) target files.
    * 使用参考:c_set_target_properties0

* COMPILE_DEFINITIONS
  * Preprocessor definitions for compiling a directory's sources.
  * 使用参考:c_add_compile_definitions0

## cmake-commands(7)

* get_target_property
    ```markdown
    Get a property from a target.
    
    `get_target_property(<VAR> target property)`
    
    Get a property from a target. The value of the property is stored in the variable <VAR>.
    ```
    * 使用参考:c_target_include_directories0


* get_directory_property
    ```markdown
    Get a property of DIRECTORY scope.

    `get_directory_property(<variable> [DIRECTORY <dir>] <prop-name>)`

    Stores a property of directory scope in the named <variable>.

    The DIRECTORY argument specifies another directory from which to retrieve the property value instead of the current directory. 
    Relative paths are treated as relative to the current source directory. 
    ```
    * 使用参考:c_target_compile_definition0