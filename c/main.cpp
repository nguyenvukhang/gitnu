#include "entry.h"
#include "git.h"
#include "shell.h"
#include <iostream>
#include <string>

/* namespace gitnu {} // namespace gitnu */

int main() {
  /* gitnu_status("git -c status.color=always status"); */
  git::get_porcelain("git status");
  std::string output;
  /* output = shell::line("git add"); */
  std::cout << output;
  return 0;
}
