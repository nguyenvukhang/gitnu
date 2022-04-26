#include "entry.h"
#include <string>
#include <iostream>

Entry::Entry(int index, std::string filename) {
  this->index = index;
  this->filename = filename;
}
void Entry::display() { std::cout << index << ": " << filename << std::endl; }
