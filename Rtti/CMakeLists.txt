cmake_minimum_required(VERSION 3.5.0)
project(RTTI)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti")

enable_testing()
find_package(GTest)
add_executable(RTTI main.cpp)
target_link_libraries(RTTI gtest gtest_main)
include_directories(${GTEST_INCLUDE_DIR})