cmake_minimum_required(VERSION 3.29)
project(mylib VERSION 1.0.1 DESCRIPTION "mylib description" LANGUAGES C)
include(GNUInstallDirs)

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

add_library(mylib SHARED ${LIST_HEADERS} ${LIST_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES LINKER_LANGUAGE C)
# set_target_properties(mylib PROPERTIES VERSION ${PROJECT_VERSION})
# set_target_properties(mylib PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})


target_include_directories(mylib PUBLIC include)
# target_include_directories(mylib PRIVATE src)

install(FILES ${LIST_HEADERS} DESTINATION "include/" COMPONENT Development)



add_executable(exec_main bin/main.c)
target_link_libraries(exec_main ${PROJECT_NAME})
# target_compile_options(exec_main PRIVATE -std=gnu99)

