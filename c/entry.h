#pragma once

#include <string>

class Entry {
  private: 
    int index;
    std::string filename;
  public:
    Entry(int index, std::string filename);
    void display();
};
