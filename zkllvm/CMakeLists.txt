cmake_minimum_required(VERSION 3.5 FATAL_ERROR)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake"
     "${CMAKE_CURRENT_LIST_DIR}/cmake/modules/share/modules/cmake"
     "/usr/share/zkllvm")

include(CMConfig)
include(CMSetupVersion)
include(CircuitCompile)

cm_workspace(crypto3)

macro(cm_find_package NAME)
    if(NOT "${NAME}" MATCHES "^${CMAKE_WORKSPACE_NAME}_.*$" AND NOT "${NAME}" STREQUAL CM)
        find_package(${ARGV})
    else()
        set(${ARGV0}_FOUND ON CACHE BOOL "")
    endif()
endmacro()

cm_setup_version(VERSION 0.1.0 PREFIX ${CMAKE_WORKSPACE_NAME})

add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/libs/crypto3")

#Example directories
add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/src")