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

void loop_pretty(array<char, 128> buf_pr,
                 unique_ptr<FILE, decltype(&pclose)> pipe_pr,
                 queue<string> res_pr) {
  // loop `git status`
  while (fgets(buf_pr.data(), buf_pr.size(), pipe_pr.get()) != nullptr) {
    res_pr.push(buf_pr.data());
  }
}

void loop_porcelain(array<char, 128> buf_po,
                    unique_ptr<FILE, decltype(&pclose)> pipe_po,
                    queue<string> staged) {
  // loop `git status --porcelain`
  queue<string> unstaged, untracked;
  while (fgets(buf_po.data(), buf_po.size(), pipe_po.get()) != nullptr) {
    const string line = buf_po.data();
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
  // initialize `git status`
  queue<string> res_pr;
  array<char, 128> buf_pr;
  unique_ptr<FILE, decltype(&pclose)> pipe_pr(popen(cmd, "r"), pclose);
  if (!pipe_pr)
    throw runtime_error("popen() failed!");

  // initialize `git status --porcelain`
  array<char, 128> buf_po;
  unique_ptr<FILE, decltype(&pclose)> pipe_po(popen(cmd, "r"), pclose);
  queue<string> res_po;
  if (!pipe_po)
    throw runtime_error("popen() failed!");

  // loop `git status`
  thread t1(loop_pretty, buf_pr, pipe_pr, ref(res_pr));
  thread t2(loop_porcelain, buf_po, pipe_po, ref(res_po));

  // int index = 1;
  // while (!pretty.empty()) {
  //   if (porcelain.front() == "") {
  //     cout << pretty.front();
  //   } else if (pretty.front().find(porcelain.front()) != string::npos) {
  //     // gitnu goodness
  //     cout << index << pretty.front();
  //     index++;
  //     porcelain.pop();
  //   } else {
  //     // bypass
  //     cout << pretty.front();
  //   }
  //   pretty.pop();
  // }
}
} // namespace git
