#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "cfltk::cfltk" for configuration "Release"
set_property(TARGET cfltk::cfltk APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(cfltk::cfltk PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/lib/libcfltk.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS cfltk::cfltk )
list(APPEND _IMPORT_CHECK_FILES_FOR_cfltk::cfltk "/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/lib/libcfltk.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
