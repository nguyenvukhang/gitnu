#include <array>
#include <cstring>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>

using namespace std;

bool printer(int index, char *line) {
  char red[6] = "\x1b[31m";
  char green[6] = "\x1b[32m";
  char *has_red, *has_green = NULL;
  has_red = strstr(line, red);
  has_green = strstr(line, green);
  if (has_red || has_green) {
    cout << index << line;
    index++;
    return true;
  } else {
    cout << line;
    return false;
  }
}

// source:
// https://stackoverflow.com/questions/478898/how-do-i-execute-a-command-and-get-the-output-of-the-command-within-c-using-po
string exec(const char *cmd) {
  // and array of characters, length 128
  array<char, 128> buffer;
  string result;
  // no idea what this does
  // but if I sub pipe() with a variable name, it fails
  // pipe is a given name
  // FILE is probably a typedef struct
  // popen, pclose seem to be methods of either FILE or unique_ptr
  unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
  // if no pipe, throw error: subprocess failed to start
  if (!pipe) {
    throw runtime_error("popen() failed!");
  }
  int index = 1;
  bool had_filename;
  // iterates through the output as long as there is still output
  // note that this operates in real-time, concurrently as stdout
  // received output line by line
  while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
    had_filename = printer(index, buffer.data());
    index += had_filename;
    result += buffer.data();
  }
  return result;
}

int main() {
  exec("git -c status.color=always status");
  return 0;
}
