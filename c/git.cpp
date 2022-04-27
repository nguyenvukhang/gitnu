#include "git.h"
#include <array>
#include <future>
#include <iostream>
#include <memory>
#include <queue>
#include <string>
#include <thread>

using namespace std;

void remove_newline(string &s) {
  s.erase(remove(s.begin(), s.end(), '\n'), s.end());
}

string get_filename(string s) {
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

void get_stdout(const char *cmd, promise<queue<string>> &&p) {
  queue<string> result;
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    result.push(buffer.data());
  }
  p.set_value(result);
}

void get_porcelain_stdout(const char *cmd, promise<queue<string>> &&p) {
  queue<string> result;
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  queue<string> staged, unstaged, untracked;

  if (!pipe)
    throw std::runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    const string line = buffer.data();
    const char first = line.at(0);
    const char second = line.at(1);
    const string filename = get_filename(line);
    if (first != ' ' && first != '?') {
      staged.push(filename);
    }
    if (second != ' ' && second != '?') {
      unstaged.push(filename);
    }
    if (first == '?' && second == '?') {
      untracked.push(filename);
    }
  }
  while (!unstaged.empty()) {
    staged.push(unstaged.front());
    unstaged.pop();
  }
  while (!untracked.empty()) {
    staged.push(untracked.front());
    untracked.pop();
  }
  p.set_value(staged);
}

// void get_stdout_arr(const char *cmd,
//                     promise<array<string, 128>> &&p) {
//   array<string, 128> arr;
//   array<char, 128> buffer;
//   unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
//   int index = 0;
//   if (!pipe)
//     throw runtime_error("popen() failed!");
//   while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr &&
//          index < 128) {
//     arr[index] = buffer.data();
//     index++;
//   }
//   p.set_value(arr);
// }

// (in-place) get a string in between two substrings
// only if the both substrings are found
static inline void get_between_colors(string &s, const char start[6],
                                      const char end[5]) {
  int si = s.find(start);
  int ei = s.find(end);
  if (si >= 0 && ei >= 0) {
    s = s.substr(si + 5, ei - 6);
  }
}

// extract colored text from git status (red/green only)
string get_colored(string line) {
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
  // promise<queue<string>> p_pretty;
  // future<queue<string>> f_pretty = p_pretty.get_future();
  //
  // promise<queue<string>> p_porcelain;
  // future<queue<string>> f_porcelain =
  // p_porcelain.get_future();

  // ARRAYS
  promise<queue<string>> p_pretty;
  future<queue<string>> f_pretty = p_pretty.get_future();

  promise<queue<string>> p_porcelain;
  future<queue<string>> f_porcelain =
      p_porcelain.get_future();

  thread t1(&get_stdout, "git -c status.color=always status",
                 move(p_pretty));
  thread t2(&get_porcelain_stdout, "git status --porcelain",
                 move(p_porcelain));
  t1.join();
  t2.join();
  queue<string> pretty = f_pretty.get();
  queue<string> porcelain = f_porcelain.get();

  int index = 1;
  while (!pretty.empty()) {
    std::string next = porcelain.front();
    std::string colored = get_colored(pretty.front());
    // std::cout << "|" << colored << "|" << std::endl;
    // std::cout << "|" << next << "|" << std::endl;
    if (next == "") {
      std::cout << pretty.front();
    } else if (colored.find(next) != string::npos) {
      std::cout << index << pretty.front();
      index++;
      porcelain.pop();
    } else {
      std::cout << pretty.front();
    }
    pretty.pop();
  }

  cout << "COMPLETED" << endl;
}
} // namespace git
