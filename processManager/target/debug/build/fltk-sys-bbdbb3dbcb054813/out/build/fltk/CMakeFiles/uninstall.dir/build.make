# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.10

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/nour/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.3.34/cfltk

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build

# Utility rule file for uninstall.

# Include the progress variables for this target.
include fltk/CMakeFiles/uninstall.dir/progress.make

fltk/CMakeFiles/uninstall:
	cd /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk && /usr/bin/cmake -P /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/cmake_uninstall.cmake

uninstall: fltk/CMakeFiles/uninstall
uninstall: fltk/CMakeFiles/uninstall.dir/build.make

.PHONY : uninstall

# Rule to build all files generated by this target.
fltk/CMakeFiles/uninstall.dir/build: uninstall

.PHONY : fltk/CMakeFiles/uninstall.dir/build

fltk/CMakeFiles/uninstall.dir/clean:
	cd /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk && $(CMAKE_COMMAND) -P CMakeFiles/uninstall.dir/cmake_clean.cmake
.PHONY : fltk/CMakeFiles/uninstall.dir/clean

fltk/CMakeFiles/uninstall.dir/depend:
	cd /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/nour/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.3.34/cfltk /home/nour/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fltk-sys-1.3.34/cfltk/fltk /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk /home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/CMakeFiles/uninstall.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : fltk/CMakeFiles/uninstall.dir/depend
