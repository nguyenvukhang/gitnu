status = ["git", "-c", "status.color=always", "status"]
get_repo = ["git", "rev-parse", "--git-dir"]


commands = ["status", "add", "reset", "diff", "checkout"]

actions = ["deleted:", "new file:", "modified:", "renamed:", "both modified:"]

suffix_list = [
    "(new commits)\n",
    "(modified content)\n",
    "(untracked content)\n",
]

ansi_colors = [
    "\x1b[31m",
    "\x1b[32m",
    "\x1b[33m",
    "\x1b[34m",
    "\x1b[35m",
    "\x1b[36m",
    "\x1b[37m",
    "\x1b[m",
]


def get_command(args: list[str]) -> tuple[str, int]:
    for index, word in enumerate(args):
        if word in commands:
            return (word, index)
    return ("", 0)
