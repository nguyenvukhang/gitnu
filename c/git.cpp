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

std::string get_filename(std::string s) { return s; }

namespace git {
void get_parallel(const char *cmd) {
  std::promise<std::queue<std::string>> p_pretty;
  std::future<std::queue<std::string>> f_pretty = p_pretty.get_future();

  std::promise<std::queue<std::string>> p_porcelain;
  std::future<std::queue<std::string>> f_porcelain = p_porcelain.get_future();

  std::thread t1(&get_stdout, "git -c status.color=always status", std::move(p_pretty));
  std::thread t2(&get_stdout, "git status --porcelain", std::move(p_porcelain));
  t1.join();
  t2.join();
  std::queue<std::string> pretty = f_pretty.get();
  std::queue<std::string> porcelain = f_porcelain.get();
  while (!pretty.empty()) {
    std::cout << pretty.front();
    pretty.pop();
  }
}
} // namespace git
