#pragma once

#include <string>

class Entry {
  public:
    int index;
    std::string filename;
    bool hasf;
    Entry(int index, std::string filename);
    void display();
    std::string cache();
};
