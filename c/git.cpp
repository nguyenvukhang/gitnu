#include "git.h"
#include <array>
#include <future>
#include <iostream>
#include <memory>
#include <queue>
#include <string>
#include <thread>
#include <functional>

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

void loop_pretty(queue<string> &pretty) {
  // initialize `git status`
  const char cmd[] = "git -c status.color=always status";
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw runtime_error("popen() failed!");
  // loop `git status`
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    pretty.push(buffer.data());
  }
}

void loop_porcelain(queue<string> &staged) {
  // initialize `git status --porcelain`
  const char cmd[] = "git status --porcelain";
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw runtime_error("popen() failed!");
  // loop `git status --porcelain`
  queue<string> unstaged, untracked;
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
}

namespace git {
void get_parallel(const char *cmd) {
  queue<string> pretty;
  queue<string> porcelain;

  // loop `git status`
  thread t1(loop_pretty, std::ref(pretty));
  thread t2(loop_porcelain, std::ref(porcelain));

  t1.join();
  t2.join();

  int index = 1;
  cout << pretty.size() << endl;
  cout << "GOT HERE" << endl;
  while (!pretty.empty()) {
    if (porcelain.front() == "") {
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
