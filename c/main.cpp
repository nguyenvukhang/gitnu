#include "entry.h"
#include "shell.h"
#include <array>
#include <cstring>
#include <fstream>
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
  if (!pipe)
    throw std::runtime_error("popen() failed!");

  // open write stream to cache file
  std::ofstream cache_file;
  const char cache_filename[12] = "/gitnu.txt";
  const std::string cache_path =
      shell::line("git rev-parse --git-dir").append(cache_filename);
  /* std::cout << "cache file target --------- |" << target << "|" << std::endl;
   */
  cache_file.open(cache_path);

  if (!cache_file.is_open())
    throw std::runtime_error("failed to open cache file!");

  int index = 1;
  bool had_filename;
  // iterates through the output as long as there is still output
  // note that this operates in real-time, concurrently as stdout
  // received output line by line
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    had_filename = printer(index, buffer.data());
    if (had_filename)
      Entry entry(index, buffer.data()); // TODO: remove hasf from entry, since
                                         // it's done here already
    index += had_filename;
    result += buffer.data();
  }
  cache_file.close();
  return result;
}

namespace gitnu {} // namespace gitnu

int main() {
  gitnu_status("git -c status.color=always status");
  std::string output;
  /* output = shell::line("git add"); */
  std::cout << output;
  return 0;
}
