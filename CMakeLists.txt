cmake_minimum_required(VERSION 3.29)
project(libdlist C)

set(INSTALL_CMAKE_DIR lib/cmake CACHE PATH "Installation directory for cmake files") 


# add_executable(linklist src/main.c)

# headers
file(GLOB API_HEADERS "include/*.h")
file(GLOB SRC_HEADERS "src/*.c")
file(GLOB LIST_HEADERS
    ${API_HEADERS}
    ${SRC_HEADERS}
)

# src
file(GLOB SRC "src/*.c")
file(GLOB LIST_SRC
    ${SRC}
)

add_library(${PROJET_NAME} SHARED ${LIST_HEADERS} ${LIST_SRC})
# set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)


