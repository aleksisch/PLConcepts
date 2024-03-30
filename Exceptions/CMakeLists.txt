cmake_minimum_required(VERSION 3.5.2)

project(exceptions)

set(CMAKE_CXX_STANDARD 20)

enable_testing()
find_package(GTest)
add_executable(exceptionTest main.cpp Exceptions.cpp)
target_link_libraries(exceptionTest gtest gtest_main)
include_directories(${GTEST_INCLUDE_DIR})
