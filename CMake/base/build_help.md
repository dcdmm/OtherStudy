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


