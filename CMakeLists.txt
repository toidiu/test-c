cmake_minimum_required(VERSION 3.29)
project(mylib VERSION 1.0.1 DESCRIPTION "mylib description" LANGUAGES C)
include(GNUInstallDirs)

# place add_library in lib
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# place add_executable in bin
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# HEADERS
file(GLOB API_HEADERS "api/*.h")
file(GLOB SRC_HEADERS "src/*.h")
file(GLOB LIST_HEADERS
    ${API_HEADERS}
    ${SRC_HEADERS}
)

# SRC
file(GLOB SRC "src/*.c")
file(GLOB LIST_SRC
    ${SRC}
)

# ============ LIB

# add library target
add_library(${PROJECT_NAME} SHARED ${LIST_HEADERS} ${LIST_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
# add version to the library exec
set_target_properties(${PROJECT_NAME} PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(${PROJECT_NAME} PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})

# include the api header files
target_include_directories(${PROJECT_NAME} PUBLIC api)
target_include_directories(${PROJECT_NAME} PRIVATE src)


# ============ EXEC

# also create an executable to test that lib works
add_executable(exec_main bin/main.c)
# link the library to the executable
target_link_libraries(exec_main ${PROJECT_NAME})

