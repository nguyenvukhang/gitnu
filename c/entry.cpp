#include "entry.h"
#include <iostream>
#include <string>

#include <algorithm>
#include <cctype>
#include <locale>

const std::string WHITESPACE = " \n\r\t\f\v";


std::string cltrim(const std::string &s) {
  size_t start = s.find_first_not_of(WHITESPACE);
  return (start == std::string::npos) ? "" : s.substr(start);
}

std::string crtrim(const std::string &s) {
  size_t end = s.find_last_not_of(WHITESPACE);
  return (end == std::string::npos) ? "" : s.substr(0, end + 1);
}

std::string ctrim(const std::string &s) { return crtrim(cltrim(s)); }

void itrim(std::string &s) { s = crtrim(cltrim(s)); }
void iltrim(std::string &s) { s = cltrim(s); }
void irtrim(std::string &s) { s = crtrim(s); }

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
static inline bool getbtw(std::string &s, const char start[], const char end[]) {
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
      "modified:", "added:", "deleted:", "renamed:", "copied:", "new file:"};
  for (int i = 0; i < 6; i++) {
    int m = s.rfind(action[i]);
    if (m == 0) {
      s = s.substr(strlen(action[i]));
      iltrim(s);
      break;
    }
  }
}

// extract filename from a line of git status
std::string exfn(std::string line, bool &hasf) {
  const char red[6] = "\x1b[31m";
  const char green[6] = "\x1b[32m";
  const char reset[5] = "\x1b[m";
  const char newline[2] = "\n";
  hasf = getbtw(line, red, reset) || hasf;
  hasf = getbtw(line, green, reset) || hasf;
  if (hasf)
    exgitfn(line);
  /* std::cout << "end result:|" << line << "|" << std::endl; */
  return line;
}

Entry::Entry(int index, std::string filename) {
  this->index = index;
  bool hasf = false;
  this->filename = exfn(filename, hasf);
  this->hasf = hasf;
}
void Entry::display() { std::cout << "entry::filename [" << filename << "]" << std::endl; }
