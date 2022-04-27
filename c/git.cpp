#include "git.h"
#include "shell.h"
#include <array>
#include <future>
#include <iostream>
#include <memory>
#include <queue>
#include <string>
#include <thread>

void get_stdout(const char *cmd, std::promise<std::queue<std::string>> &&p) {
  std::queue<std::string> result;
  std::array<char, 128> buffer;
  std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    result.push(buffer.data());
  }
  p.set_value(result);
}

namespace git {
void get_parallel(const char *cmd) {
  std::promise<std::queue<std::string>> p_pretty;
  std::future<std::queue<std::string>> f_pretty = p_pretty.get_future();
  std::thread t1(&get_stdout, "git status", std::move(p_pretty));
  t1.join();
  std::queue<std::string> result = f_pretty.get();
  while (!result.empty()) {
    std::cout << result.front() << std::endl;
    result.pop();
  }
}
} // namespace git
