cmake_minimum_required(VERSION 3.5.0)
project(polymorphism)

set(CMAKE_CXX_STANDARD 20)

enable_testing()
find_package(GTest)
add_executable(polymorphism main.cpp)
target_link_libraries(polymorphism gtest gtest_main)
include_directories(${GTEST_INCLUDE_DIR})