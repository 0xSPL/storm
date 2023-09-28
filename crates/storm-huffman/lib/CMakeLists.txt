cmake_minimum_required(VERSION 3.27)

project(StormHuffman)

include(GNUInstallDirs)

set(HEADERS huffman.h)
set(SOURCES huffman.cpp)

option(HUFFMAN_BUILD_SHARED_LIB "Build dynamically linked library" ON)
option(HUFFMAN_BUILD_STATIC_LIB "Build static linked library" OFF) # TODO

if (HUFFMAN_BUILD_SHARED_LIB)
  message(STATUS "Build dynamically linked library")
  add_library(huffman SHARED ${SOURCES} ${HEADERS})

  if (APPLE)
    set_target_properties(huffman PROPERTIES PUBLIC_HEADER "huffman.h")
    set_target_properties(huffman PROPERTIES LINK_FLAGS "-framework Carbon")
  endif()

  if (UNIX)
    set_target_properties(huffman PROPERTIES VERSION 1.0.0)
    set_target_properties(huffman PROPERTIES SOVERSION 1)
  endif()

  install(
    TARGETS huffman
    EXPORT HuffmanTargets
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    FRAMEWORK DESTINATION /Library/Frameworks
  )

  target_include_directories(
    huffman
    PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
  )
endif()

install(FILES huffman.h DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
