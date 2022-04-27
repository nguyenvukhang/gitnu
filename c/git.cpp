#include "git.h"
#include <array>
#include <future>
#include <iostream>
#include <memory>
#include <queue>
#include <string>
#include <thread>

void remove_newline(std::string &s) {
  s.erase(std::remove(s.begin(), s.end(), '\n'), s.end());
}

std::string get_filename(std::string s) {
  if (s == "") {
    return s;
  }
  remove_newline(s);
  if (s.front() == 'R') {
    // special parsing for renames
    // "R  pre-rename -> post-rename"
    int i = s.find(" -> ");
    return s.substr(i + 4);
  }
  if (s.length() > 3) {
    return s.substr(3);
  }
  return s;
}

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

void get_stdout_arr(const char *cmd,
                    std::promise<std::array<std::string, 128>> &&p) {
  std::array<std::string, 128> arr;
  std::array<char, 128> buffer;
  std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  int index = 0;
  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr &&
         index < 128) {
    arr[index] = buffer.data();
    index++;
  }
  p.set_value(arr);
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
  // QUEUES
  // std::promise<std::queue<std::string>> p_pretty;
  // std::future<std::queue<std::string>> f_pretty = p_pretty.get_future();
  //
  // std::promise<std::queue<std::string>> p_porcelain;
  // std::future<std::queue<std::string>> f_porcelain =
  // p_porcelain.get_future();

  // ARRAYS
  std::promise<std::array<std::string, 128>> p_pretty;
  std::future<std::array<std::string, 128>> f_pretty = p_pretty.get_future();

  std::promise<std::array<std::string, 128>> p_porcelain;
  std::future<std::array<std::string, 128>> f_porcelain =
      p_porcelain.get_future();

  std::thread t1(&get_stdout_arr, "git -c status.color=always status",
                 std::move(p_pretty));
  std::thread t2(&get_stdout_arr, "git status --porcelain",
                 std::move(p_porcelain));
  t1.join();
  t2.join();
  std::array<std::string, 128> pretty_arr = f_pretty.get();
  std::array<std::string, 128> porcelain_arr = f_porcelain.get();

  for (int i = 0; i < 128; i++) {
    porcelain_arr[i] = get_filename(porcelain_arr[i]);
  }

  int index = 1;
  for (const std::string &pretty_front : pretty_arr) {
    if (pretty_front == "") {
      break;
    }
    const std::string colored = get_colored(pretty_front);
    for (std::string &porcelain : porcelain_arr) {
      if (porcelain == "") {
        continue;
      }
      /* std::cout << "GOT HERE" << std::endl; */
      /* std::cout << porcelain << "|" << colored << std::endl; */
      if (colored.find(porcelain) != std::string::npos) {
        std::cout << index << pretty_front;
        porcelain = "";
        index++;
        break;
      }
    }
  }
  std::cout << "COMPLETED" << std::endl;
}
} // namespace git
