
cmake_minimum_required(VERSION 3.22)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD {{ cxx_version }})
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project({{ project_name }})

add_executable({{ bin_name }}
    src/main.cpp
)
# file(GLOB_RECURSE ALL_SOURCES "src/*.cpp" "src/*.hpp" "src/*.c" "src/*.h")
# target_sources(main PRIVATE ${ALL_SOURCES})

target_link_libraries(main PRIVATE
  stdc++
)

