if(NOT EXISTS "/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/install_manifest.txt")
   message(FATAL_ERROR
      "Cannot find install manifest: \"/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/install_manifest.txt\"")
endif(NOT EXISTS "/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/install_manifest.txt")

file(READ "/home/nour/OS-Project/processManager/target/debug/build/fltk-sys-bbdbb3dbcb054813/out/build/fltk/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")

foreach(file ${files})
message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
   exec_program("/usr/bin/cmake"
      ARGS "-E remove -f \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
   )
   if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
   endif(NOT "${rm_retval}" STREQUAL 0)
endforeach(file)
