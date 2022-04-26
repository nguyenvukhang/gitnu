#include <array>
#include <cstring>
#include <iostream>
#include <memory>
#include <stdexcept>
#include "shell.h"
/* #include <string> */

namespace shell {
/**
 * first line of output of a shell command
 * @param cmd the shell command to be run
 * @return the first line of output as a string
 */
std::string line(const char *cmd) {
  std::array<char, 128> buffer;
  std::string result = "";
  std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  bool written = false;
  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (!feof(pipe.get()) && !written) {
    if (fgets(buffer.data(), 128, pipe.get()) != nullptr) {
      written = true;
      result += buffer.data();
    }
  }
  int rc = pclose(pipe.get());
  if (rc == EXIT_SUCCESS) {        // == 0
  } else if (rc == EXIT_FAILURE) { // EXIT_FAILURE is not used by all programs,
  }
  return result;
}
} // namespace shell
