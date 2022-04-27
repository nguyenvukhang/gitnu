#include "git.h"
#include "shell.h"

#include <iostream>
#include <string>
#include <array>
#include <__memory/unique_ptr.h>

namespace git {
  void get_porcelain(const char *cmd) {
    std::array<char, 128> buffer;
    std::string result;
    // no idea what this does
    // but if I sub pipe() with a variable name, it fails
    // pipe is a given name
    // FILE is probably a typedef struct
    // popen, pclose seem to be methods of either FILE or unique_ptr
    std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  }
} // namespace git
