## cmake(1)

* -S <path-to-source>
    * Path to root directory of the CMake project to build.

* -B <path-to-build>
    * Path to directory which CMake will use as the root of build directory.
    * If the directory doesn't already exist CMake will make it. If the directory doesn't already exist CMake will make
      it.

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
      particular build system. Possible generator names are specified in
      the [cmake-generators(7)](https://cmake.org/cmake/help/latest/manual/cmake-generators.7.html#manual:cmake-generators(7))
      manual.
    * If not specified, CMake checks
      the [CMAKE_GENERATOR](https://cmake.org/cmake/help/latest/envvar/CMAKE_GENERATOR.html#envvar:CMAKE_GENERATOR)
      environment variable and otherwise falls back to a builtin default selection.

## cmake-variables(7)

* PROJECT_NAME
    * Name of the project given to the project command.

* CMAKE_CXX_STANDARD
    * Default value
      for [CXX_STANDARD](https://cmake.org/cmake/help/latest/prop_tgt/CXX_STANDARD.html#prop_tgt:CXX_STANDARD) target
      property if set when a target is created.
    * See
      the [cmake-compile-features(7)](https://cmake.org/cmake/help/latest/manual/cmake-compile-features.7.html#manual:cmake-compile-features(7))
      manual for information on compile features and a list of supported compilers.

* PROJECT_SOURCE_DIR
    * This is the source directory of the last call to the project() command made in the current directory scope or one
      of its parents.

* PROJECT_BINARY_DIR
    * Full path to build directory for project.

* CMAKE_CURRENT_SOURCE_DIR
    * This is the full path to the source directory that is currently being processed by cmake.

* CMAKE_CURRENT_BINARY_DIR
    * This is the full path to the build directory that is currently being processed by cmake.