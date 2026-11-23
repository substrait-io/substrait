# SPDX-License-Identifier: Apache-2.0
cmake_minimum_required(VERSION 3.24)
project(SubstraitAntlrCheck LANGUAGES CXX)

set(PARSER_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../gen/antlr/cpp")
file(STRINGS "${PARSER_SOURCE_DIR}/substraittype/SubstraitTypeParser.cpp"
     ANTLR_GENERATOR_HEADER LIMIT_COUNT 1 REGEX "by ANTLR")
string(REGEX MATCH "[0-9]+\\.[0-9]+\\.[0-9]+"
       ANTLR_VERSION "${ANTLR_GENERATOR_HEADER}")

include(FetchContent)
set(ANTLR_BUILD_CPP_TESTS OFF CACHE BOOL "" FORCE)
set(ANTLR_BUILD_SHARED OFF CACHE BOOL "" FORCE)
set(ANTLR4_INSTALL OFF CACHE BOOL "" FORCE)
FetchContent_Declare(
  antlr4_runtime
  GIT_REPOSITORY https://github.com/antlr/antlr4.git
  GIT_TAG ${ANTLR_VERSION}
  GIT_SHALLOW TRUE
  SOURCE_SUBDIR runtime/Cpp)
FetchContent_MakeAvailable(antlr4_runtime)

file(GLOB PARSER_SOURCES CONFIGURE_DEPENDS
     "${PARSER_SOURCE_DIR}/substraittype/*.cpp"
     "${PARSER_SOURCE_DIR}/functestcase/*.cpp")
add_library(antlrcheck STATIC ${PARSER_SOURCES})
target_compile_features(antlrcheck PRIVATE cxx_std_17)
target_include_directories(antlrcheck PRIVATE
                           "${PARSER_SOURCE_DIR}/substraittype"
                           "${PARSER_SOURCE_DIR}/functestcase")
target_link_libraries(antlrcheck PRIVATE antlr4_static)
