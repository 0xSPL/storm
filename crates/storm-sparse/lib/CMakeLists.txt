cmake_minimum_required(VERSION 3.27)

project(StormSparse)

include(GNUInstallDirs)

set(HEADERS sparse.h)
set(SOURCES sparse.cpp)

option(SPARSE_BUILD_SHARED_LIB "Build dynamically linked library" ON)
option(SPARSE_BUILD_STATIC_LIB "Build static linked library" OFF) # TODO

if (SPARSE_BUILD_SHARED_LIB)
  message(STATUS "Build dynamically linked library")
  add_library(sparse SHARED ${SOURCES} ${HEADERS})

  if (APPLE)
    set_target_properties(sparse PROPERTIES PUBLIC_HEADER "sparse.h")
    set_target_properties(sparse PROPERTIES LINK_FLAGS "-framework Carbon")
  endif()

  if (UNIX)
    set_target_properties(sparse PROPERTIES VERSION 1.0.0)
    set_target_properties(sparse PROPERTIES SOVERSION 1)
  endif()

  install(
    TARGETS sparse
    EXPORT HuffmanTargets
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    FRAMEWORK DESTINATION /Library/Frameworks
  )

  target_include_directories(
    sparse
    PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
  )
endif()

install(FILES sparse.h DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
