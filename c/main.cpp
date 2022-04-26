#include <array>
#include <cstring>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>

bool printer(int index, char *line) {
  char red[6] = "\x1b[31m";
  char green[6] = "\x1b[32m";
  char *has_red, *has_green = NULL;
  has_red = strstr(line, red);
  has_green = strstr(line, green);
  if (has_red || has_green) {
    std::cout << index << line;
    index++;
    return true;
  } else {
    std::cout << line;
    return false;
  }
}

// source:
// https://stackoverflow.com/questions/478898/how-do-i-execute-a-command-and-get-the-output-of-the-command-within-c-using-po
std::string gitnu_status(const char *cmd) {
  // and array of characters, length 128
  std::array<char, 128> buffer;
  std::string result;
  // no idea what this does
  // but if I sub pipe() with a variable name, it fails
  // pipe is a given name
  // FILE is probably a typedef struct
  // popen, pclose seem to be methods of either FILE or unique_ptr
  std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  // if no pipe, throw error: subprocess failed to start
  if (!pipe) {
    throw std::runtime_error("popen() failed!");
  }
  int index = 1;
  bool had_filename;
  // iterates through the output as long as there is still output
  // note that this operates in real-time, concurrently as stdout
  // received output line by line
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    had_filename = printer(index, buffer.data());
    index += had_filename;
    result += buffer.data();
  }
  return result;
}

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

namespace gitnu {} // namespace gitnu

int main() {
  gitnu_status("git -c status.color=always status");
  std::string output;
  output = shell::line("printf \"hello world\\nsecond line\"");
  std::cout << output;
  output = shell::line("git add");
  std::cout << output;
  return 0;
}
