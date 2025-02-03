cmake_minimum_required(VERSION 3.14)
project(OptiTrade)

# Set C++ standard to C++23
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source directories
set(SRC_DIR "${CMAKE_SOURCE_DIR}/src")
set(INCLUDE_DIR "${CMAKE_SOURCE_DIR}/include")

# Include directories
include_directories(${INCLUDE_DIR})

# Gather all source files
file(GLOB_RECURSE SOURCES
    "${SRC_DIR}/*.cpp"
)

# Create executable
add_executable(OptiTrade ${SOURCES})

# Link required libraries (for WebSocket, Math, and Threads)
find_package(Threads REQUIRED)

# Dynamically locate WebSocket++ (installed via Homebrew or system paths)
find_path(WEBSOCKETPP_INCLUDE_DIR websocketpp/config/asio_client.hpp
    PATHS /usr/include /usr/local/include /opt/homebrew/include
    PATH_SUFFIXES websocketpp
)

if (WEBSOCKETPP_INCLUDE_DIR)
    message(STATUS "Found WebSocket++ in: ${WEBSOCKETPP_INCLUDE_DIR}")
    include_directories(${WEBSOCKETPP_INCLUDE_DIR})
else()
    message(FATAL_ERROR "WebSocket++ not found! Ensure it is installed via Homebrew: `brew install websocketpp`")
endif()

target_link_libraries(OptiTrade PRIVATE Threads::Threads)

# Enable compiler optimizations for performance
if(CMAKE_BUILD_TYPE MATCHES "Release")
    message("Compiling in Release mode with optimizations")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -march=native -flto -fstrict-aliasing")
endif()
