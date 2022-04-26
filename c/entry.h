#pragma once

#include <string>

class Entry {
  private: 
    int index;
    std::string filename;
  public:
    bool hasf;
    Entry(int index, std::string filename);
    void display();
    std::string cache();
};
