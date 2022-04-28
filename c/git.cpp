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

string parse_porcelain_line(string s) {
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

void get_pretty(const char *cmd, promise<queue<string>> &&p) {
  queue<string> result;
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    result.push(buffer.data());
  }
  p.set_value(result);
}

void get_porcelain(const char *cmd, promise<queue<string>> &&p) {
  queue<string> result;
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  queue<string> staged, unstaged, untracked;

  if (!pipe)
    throw runtime_error("popen() failed!");
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    const string line = buffer.data();
    const char first = line.at(0);
    const char second = line.at(1);
    const string filename = parse_porcelain_line(line);
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

namespace git {
void get_parallel(const char *cmd) {

  promise<queue<string>> p[2];
  future<queue<string>> f[2];
  for (int i = 0; i < 2; i++) {
    f[i] = p[i].get_future();
  }

  thread t1(&get_pretty, "git -c status.color=always status", move(p[0]));
  thread t2(&get_porcelain, "git status --porcelain", move(p[1]));
  t2.join();
  queue<string> porcelain = f[1].get();
  t1.join();
  queue<string> pretty = f[0].get();

  int index = 1;
  while (!pretty.empty()) {
    if (porcelain.empty() || porcelain.front() == "") {
      cout << pretty.front();
    } else if (pretty.front().find(porcelain.front()) != string::npos) {
      // gitnu goodness
      cout << index << pretty.front();
      index++;
      porcelain.pop();
    } else {
      // bypass
      cout << pretty.front();
    }
    pretty.pop();
  }
}
} // namespace git
