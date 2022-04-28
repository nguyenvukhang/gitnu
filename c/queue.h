#pragma once

#include <algorithm>
#include <iostream>
#include <mutex>
#include <optional>
#include <queue>
#include <string>
#include <thread>

class non_empty_queue : public std::exception {
  std::string what_;

public:
  explicit non_empty_queue(std::string msg);
  const char *what() const noexcept override;
};

template <typename T> class ThreadsafeQueue {
  std::queue<T> queue_;
  mutable std::mutex mutex_;
  [[nodiscard]] bool empty() const;

public:
  ThreadsafeQueue() = default;
  ThreadsafeQueue(const ThreadsafeQueue<T> &) = delete;
  ThreadsafeQueue &operator=(const ThreadsafeQueue<T> &) = delete;
  ThreadsafeQueue(ThreadsafeQueue<T> &&other) noexcept(false);
  virtual ~ThreadsafeQueue() noexcept(false) {
    std::lock_guard<std::mutex> lock(mutex_);
    if (!empty()) {
      throw non_empty_queue("Destroying a non-empty queue"s);
    }
  }
  [[nodiscard]] unsigned long size() const;
  std::optional<T> pop();
  void push(const T &item);
};
