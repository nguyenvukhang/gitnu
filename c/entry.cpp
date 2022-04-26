#include "entry.h"
#include <iostream>
#include <string>

#include <algorithm>
#include <cctype>
#include <locale>

// gets a substring before first occurence of a string
std::string strbf(std::string str, std::string query) {
  std::string::size_type pos = str.find(query);
  if (pos != std::string::npos) {
    return str.substr(0, pos);
  } else {
    return str;
  }
}

// (in-place) get a string in between two substrings
// only if the both substrings are found
static inline bool getbtw(std::string &s, char start[], char end[]) {
  int si = s.find(start);
  int ei = s.find(end);
  if (si >= 0 && ei >= 0) {
    s = s.substr(si + strlen(start), ei - strlen(end));
    return true;
  }
  return false;
}

// naively extract filename, given a line from git status
// that is guaranteed to have a filename inside
static inline void exgitfn(std::string &s) {
  char action[6][10] = {
      "modified:", "added:   ", "deleted: ", "renamed: ", "copied:  ", "new file:"};
  for (int i = 0; i < 6; i++) {
    int m = s.rfind(action[i]);
    if (m == 0) {
      s = s.substr(m + 12);
      break;
    }
  }
}

// extract filename from a line of git status
std::string exfn(std::string line) {
  std::string result = "";
  char red[6] = "\x1b[31m";
  char green[6] = "\x1b[32m";
  char reset[5] = "\x1b[m";
  bool hasf = false;
  hasf = getbtw(line, red, reset) || hasf;
  hasf = getbtw(line, green, reset) || hasf;
  if (hasf)
    exgitfn(line);
  std::cout << "end result:|" << line << "|" << std::endl;
  return result;
}

Entry::Entry(int index, std::string filename) {
  this->index = index;
  this->filename = exfn(filename);
}
void Entry::display() { std::cout << "-> fn: " << filename << std::endl; }
