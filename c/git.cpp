#include "git.h"
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

void remove_newline(std::string &s) {
  s.erase(std::remove(s.begin(), s.end(), '\n'), s.end());
}

std::string get_filename(std::string s) {
  if (s == "") {
    return s;
  }
  remove_newline(s);
  return s.substr(3);
}

// (in-place) get a string in between two substrings
// only if the both substrings are found
static inline void get_between_colors(std::string &s, const char start[6],
                                      const char end[5]) {
  int si = s.find(start);
  int ei = s.find(end);
  if (si >= 0 && ei >= 0) {
    s = s.substr(si + 5, ei - 6);
  }
}

// extract colored text from git status (red/green only)
std::string get_colored(std::string line) {
  const char red[6] = "\x1b[31m";
  const char green[6] = "\x1b[32m";
  const char reset[5] = "\x1b[m";
  const char newline[2] = "\n";
  get_between_colors(line, red, reset);
  get_between_colors(line, green, reset);
  return line;
}

namespace git {
void get_parallel(const char *cmd) {
  std::promise<std::queue<std::string>> p_pretty;
  std::future<std::queue<std::string>> f_pretty = p_pretty.get_future();

  std::promise<std::queue<std::string>> p_porcelain;
  std::future<std::queue<std::string>> f_porcelain = p_porcelain.get_future();

  std::thread t1(&get_stdout, "git -c status.color=always status",
                 std::move(p_pretty));
  std::thread t2(&get_stdout, "git status --porcelain", std::move(p_porcelain));
  t1.join();
  t2.join();
  std::queue<std::string> pretty = f_pretty.get();
  std::queue<std::string> porcelain = f_porcelain.get();
  int index = 1;
  while (!pretty.empty()) {
    std::string next = get_filename(porcelain.front());
    std::string colored = get_colored(pretty.front());
    std::cout << "|" << colored << "|" << std::endl;
    std::cout << "|" << next << "|" << std::endl;
    if (colored.find(next) >= 0) {
      std::cout << index << pretty.front();
      index++;
      porcelain.pop();
    } else {
      std::cout << pretty.front();
    }
    pretty.pop();
  }
  std::cout << "GOT HERE" << std::endl;
}
} // namespace git
