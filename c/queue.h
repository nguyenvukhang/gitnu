#pragma once

#include <algorithm>
#include <iostream>
#include <mutex>
#include <optional>
#include <queue>
#include <string>
#include <thread>


using namespace std;

class non_empty_queue : public std::exception {};

template<typename T>
class ThreadsafeQueue {};
