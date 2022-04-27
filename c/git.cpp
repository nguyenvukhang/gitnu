#include "git.h"
#include "shell.h"
#include <__memory/unique_ptr.h>
#include <array>
#include <iostream>
#include <string>
#include <queue>

namespace git {
void get_parallel(const char *cmd) {
  std::array<char, 128> buffer;
  std::queue<std::string> status_list;
  std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    status_list.push(buffer.data());
  }
  while (!status_list.empty()) {
    std::cout << status_list.front();
    status_list.pop();
  }
}
} // namespace git
