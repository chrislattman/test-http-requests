cmake_minimum_required(VERSION 3.22)
project(test-http-requests C)

find_package(CURL REQUIRED COMPONENTS libcurl)

add_executable(request request.c)
target_link_libraries(request PRIVATE CURL::libcurl)

add_custom_target(run-request
    COMMAND request
    DEPENDS request
    WORKING_DIRECTORY "${PROJECT_SOURCE_DIR}"
)
