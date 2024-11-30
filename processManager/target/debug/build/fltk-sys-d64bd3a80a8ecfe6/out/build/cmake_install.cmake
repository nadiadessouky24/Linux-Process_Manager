# Install script for directory: /home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/lib/libcfltk.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/lib" TYPE STATIC_LIBRARY FILES "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/libcfltk.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_box.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_browser.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_button.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_dialog.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_draw.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_enums.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_group.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_image.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_input.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_lock.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_macros.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_menu.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_misc.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_prefs.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_printer.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_surface.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_table.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_text.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_tree.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_utils.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_valuator.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_widget.h;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_widget.hpp;/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/include/cfltk" TYPE FILE FILES
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_box.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_browser.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_button.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_dialog.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_draw.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_enums.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_group.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_image.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_input.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_lock.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_macros.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_menu.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_misc.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_prefs.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_printer.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_surface.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_table.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_text.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_tree.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_utils.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_valuator.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_widget.h"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_widget.hpp"
    "/home/moh04/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.4.36/cfltk/include/cfl_window.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake"
         "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/CMakeFiles/Export/_home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk" TYPE FILE FILES "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/CMakeFiles/Export/_home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig-debug.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk" TYPE FILE FILES "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/CMakeFiles/Export/_home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfig-debug.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/cmake/cfltk" TYPE FILE FILES "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/cfltkConfigVersion.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/pkgconfig/cfltk.pc")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/share/pkgconfig" TYPE FILE FILES "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/cfltk.pc")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/moh04/my_code/demo/OS-Project/processManager/target/debug/build/fltk-sys-d64bd3a80a8ecfe6/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
