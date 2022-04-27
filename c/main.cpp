#include "git.h"
#include <iostream>
#include <string>

/* namespace gitnu {} // namespace gitnu */

int main() {
  /* gitnu_status("git -c status.color=always status"); */
  git::get_parallel();
  return 0;
}
