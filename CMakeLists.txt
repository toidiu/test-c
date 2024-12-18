cmake_minimum_required(VERSION 3.29)
project(mylib VERSION 1.0.1 DESCRIPTION "mylib description" LANGUAGES C)
include(GNUInstallDirs)

# place add_library in lib
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# place add_executable in bin
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# headers
file(GLOB API_HEADERS "include/*.h")
file(GLOB SRC_HEADERS "src/*.h")
file(GLOB LIST_HEADERS
    ${API_HEADERS}
    ${SRC_HEADERS}
)
# src
file(GLOB SRC "src/*.c")
file(GLOB LIST_SRC
    ${SRC}
)

# add library target
add_library(mylib SHARED ${LIST_HEADERS} ${LIST_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
# add version to the library exec
# set_target_properties(mylib PROPERTIES VERSION ${PROJECT_VERSION})
# set_target_properties(mylib PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})

# add the api header source
target_include_directories(mylib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

install(FILES ${LIST_HEADERS} DESTINATION "include/" COMPONENT Development)


# also create an executable to test that lib works
add_executable(exec_main bin/main.c)
# link the library to the executable
target_link_libraries(exec_main ${PROJECT_NAME})

