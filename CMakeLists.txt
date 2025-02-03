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

# Gather all source files automatically
file(GLOB_RECURSE SOURCES
    "${SRC_DIR}/*.cpp"
)

# Create executable
add_executable(OptiTrade ${SOURCES})

# Find Boost with necessary components
find_package(Boost 1.70 REQUIRED COMPONENTS system thread CONFIG)
if(Boost_FOUND)
    message(STATUS "Boost found at: ${Boost_INCLUDE_DIRS}")
    include_directories(${Boost_INCLUDE_DIRS})
    target_link_libraries(OptiTrade PRIVATE Boost::system Boost::thread)
else()
    message(FATAL_ERROR "Boost not found! Install it via Homebrew: `brew install boost`")
endif()

# Find WebSocket++ (installed via Homebrew or system paths)
find_path(WEBSOCKETPP_INCLUDE_DIR websocketpp/config/asio_client.hpp
    PATHS /usr/include /usr/local/include /opt/homebrew/include
    PATH_SUFFIXES websocketpp
)

if (WEBSOCKETPP_INCLUDE_DIR)
    message(STATUS "Found WebSocket++ in: ${WEBSOCKETPP_INCLUDE_DIR}")
    include_directories(${WEBSOCKETPP_INCLUDE_DIR})
else()
    message(FATAL_ERROR "WebSocket++ not found! Install it via Homebrew: `brew install websocketpp`")
endif()

find_package(OpenSSL REQUIRED)
if(OPENSSL_FOUND)
    message(STATUS "OpenSSL found at: ${OPENSSL_INCLUDE_DIR}")
    include_directories(${OPENSSL_INCLUDE_DIR})
    target_link_libraries(OptiTrade PRIVATE OpenSSL::SSL OpenSSL::Crypto)
else()
    message(FATAL_ERROR "OpenSSL not found! Install it via Homebrew: `brew install openssl`")
endif()

# WebSocket++ expects `io_service`, so force modern Boost detection
add_definitions(-DASIO_STANDALONE)

# Enable compiler optimizations for performance
if(CMAKE_BUILD_TYPE MATCHES "Release")
    message(STATUS "Compiling in Release mode with optimizations")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -march=native -flto -fstrict-aliasing")
endif()
