cmake_minimum_required(VERSION 3.22)
project(test-http-requests C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

find_package(CURL REQUIRED COMPONENTS libcurl)

add_executable(request request.c)
target_link_libraries(request PRIVATE CURL::libcurl)

add_custom_target(run-request
    COMMAND request
    DEPENDS request
    WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}"
)
