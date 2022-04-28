#include "git.h"
#include <array>
#include <functional>
#include <future>
#include <iostream>
#include <memory>
#include <queue>
#include <string>
#include <thread>

#include <algorithm>
#include <mutex>
#include <optional>

using namespace std;

class non_empty_queue : public std::exception {
  std::string what_;

public:
  explicit non_empty_queue(std::string msg) { what_ = std::move(msg); }
  const char *what() const noexcept override { return what_.c_str(); }
};

template <typename T> class ThreadsafeQueue {
  std::queue<T> queue_;
  mutable std::mutex mutex_;

  // Moved out of public interface to prevent races between this
  // and pop().
  [[nodiscard]] bool empty() const { return queue_.empty(); }

public:
  ThreadsafeQueue() = default;
  ThreadsafeQueue(const ThreadsafeQueue<T> &) = delete;
  ThreadsafeQueue &operator=(const ThreadsafeQueue<T> &) = delete;

  ThreadsafeQueue(ThreadsafeQueue<T> &&other) noexcept(false) {
    std::lock_guard<std::mutex> lock(mutex_);
    if (!empty()) {
      throw non_empty_queue("Moving into a non-empty queue"s);
    }
    queue_ = std::move(other.queue_);
  }

  virtual ~ThreadsafeQueue() noexcept(false) {
    std::lock_guard<std::mutex> lock(mutex_);
    if (!empty()) {
      throw non_empty_queue("Destroying a non-empty queue"s);
    }
  }

  [[nodiscard]] unsigned long size() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return queue_.size();
  }

  std::optional<T> pop() {
    std::lock_guard<std::mutex> lock(mutex_);
    if (queue_.empty()) {
      return {};
    }
    T tmp = queue_.front();
    queue_.pop();
    return tmp;
  }

  void push(const T &item) {
    std::lock_guard<std::mutex> lock(mutex_);
    queue_.push(item);
  }
};

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

void loop_pretty(ThreadsafeQueue<string> &pretty) {
  cout << "STARTED PRETTY" << endl;
  // initialize `git status`
  const char cmd[34] = "git -c status.color=always status";
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw runtime_error("popen() failed!");
  // loop `git status`
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    pretty.push(buffer.data());
  }
  cout << "PRETTY DONE" << endl;
}

void loop_porcelain(ThreadsafeQueue<string> &staged) {
  cout << "STARTED PORCELAIN" << endl;
  // initialize `git status --porcelain`
  const char cmd[23] = "git status --porcelain";
  array<char, 128> buffer;
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  if (!pipe)
    throw runtime_error("popen() failed!");
  // loop `git status --porcelain`
  queue<string> unstaged, untracked;
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    const string line = buffer.data();
    const char first = line[0];
    const char second = line[1];
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
    // sleep(1);
    staged.push(unstaged.front());
    unstaged.pop();
  }
  while (!untracked.empty()) {
    // sleep(1);
    staged.push(untracked.front());
    untracked.pop();
  }
  cout << "PORCELAIN DONE" << endl;
}

void loop_print(ThreadsafeQueue<string> &pretty,
                ThreadsafeQueue<string> &porcelain, bool *pretty_done,
                bool *porcelain_done) {
  std::this_thread::sleep_for(100us);
  cout << "STARTED LOOP" << endl;
  int index = 1;
  optional<string> _pretty = pretty.pop();
  optional<string> _porcelain = porcelain.pop();
  while (_pretty) {
    string j = *_pretty;
    cout << j;
    _pretty = pretty.pop();
    if (!_pretty) {
      std::this_thread::sleep_for(100us);
      _pretty = pretty.pop();
    }
  }
  cout << "EXITED PRINT LOOP" << pretty.size() << ", " << porcelain.size()
       << endl;
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
void get_parallel() {
  cout << "started execution" << endl;
  ThreadsafeQueue<string> pretty;
  ThreadsafeQueue<string> porcelain;
  bool pretty_done = false;
  bool porcelain_done = false;

  // loop `git status`
  thread t1(loop_pretty, ref(pretty));
  thread t2(loop_porcelain, ref(porcelain));
  thread t3(loop_print, ref(pretty), ref(porcelain), &pretty_done,
            &porcelain_done);

  t1.join();
  pretty_done = true;
  t2.join();
  porcelain_done = true;
  t3.join();
  cout << "Completed execution." << endl;
}
} // namespace git
