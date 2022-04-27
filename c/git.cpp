#include "git.h"
#include <array>
#include <functional>
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

void loop_pretty(queue<string> &pretty, bool &done) {
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
  done = true;
}

void loop_porcelain(queue<string> &staged, bool &done) {
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
  done = true;
}

void loop_print(queue<string> &pretty, queue<string> &porcelain,
                bool &pretty_done, bool &porcelain_done) {
  int index = 1;

  // when a queue is truly done:
  // is when queue.empty() AND queue_done

  // while pretty is not truly done
  while (!(pretty.empty() && pretty_done)) {
    // 1: pretty is empty, pretty is done => end
    if (pretty.empty() && pretty_done) {
      break;
    } else if (pretty.empty() && !pretty_done) {
      continue;
    } else {
      // execute
      if (porcelain.empty() && porcelain_done) {
        // bypass
        cout << pretty.front();
        pretty.pop();
      } else if (porcelain.empty() && !porcelain_done) {
        continue;
      } else {
        // check for match
        const bool has_filename =
            pretty.front().find(porcelain.front()) != string::npos;
        if (has_filename) {
          cout << index << pretty.front();
          index++;
          pretty.pop();
          porcelain.pop();
        } else {
          // no filename, bypass
          cout << pretty.front();
          pretty.pop();
        }
      }
    }
  }
}

// 1: pretty is     empty, pretty is     done => end
// 2: pretty is     empty, pretty is not done => wait
// 3: pretty is not empty, pretty is     done => execute
// 4: pretty is not empty, pretty is not done => execute
// 5: porcelain is     empty, porcelain is     done => bypass
// 6: porcelain is     empty, porcelain is not done => wait
// 7: porcelain is not empty, porcelain is     done => compare
// 8: porcelain is not empty, porcelain is not done => compare

namespace git {
void get_parallel(const char *cmd) {
  queue<string> pretty;
  queue<string> porcelain;
  bool pretty_done = false;
  bool porcelain_done = false;

  // loop `git status`
  thread t1(loop_pretty, ref(pretty), ref(pretty_done));
  thread t2(loop_porcelain, ref(porcelain), ref(porcelain_done));
  thread t3(loop_print, ref(pretty), ref(porcelain), ref(pretty_done),
            ref(porcelain_done));

  t1.join();
  t2.join();
  t3.join();
}
} // namespace git
