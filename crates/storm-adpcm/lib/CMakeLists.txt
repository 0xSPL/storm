cmake_minimum_required(VERSION 3.27)

project(StormADPCM)

include(GNUInstallDirs)

set(HEADERS adpcm.h)
set(SOURCES adpcm.cpp)

option(ADPCM_BUILD_SHARED_LIB "Build dynamically linked library" ON)
option(ADPCM_BUILD_STATIC_LIB "Build static linked library" OFF) # TODO

if (ADPCM_BUILD_SHARED_LIB)
  message(STATUS "Build dynamically linked library")
  add_library(adpcm SHARED ${SOURCES} ${HEADERS})

  if (APPLE)
    set_target_properties(adpcm PROPERTIES PUBLIC_HEADER "adpcm.h")
    set_target_properties(adpcm PROPERTIES LINK_FLAGS "-framework Carbon")
  endif()

  if (UNIX)
    set_target_properties(adpcm PROPERTIES VERSION 1.0.0)
    set_target_properties(adpcm PROPERTIES SOVERSION 1)
  endif()

  install(
    TARGETS adpcm
    EXPORT HuffmanTargets
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    INCLUDES DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
    FRAMEWORK DESTINATION /Library/Frameworks
  )

  target_include_directories(
    adpcm
    PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
  )
endif()

install(FILES adpcm.h DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
