from . import git
from . import write
from . import read
import sys


def run() -> None:
    args = sys.argv
    command, command_index = git.get_command(args)
    if len(args) <= 1 or command == "status":
        write.gitnu_status(args[command_index + 1 :])
    else:
        read.gitnu_use(args, command_index)


if __name__ == "__main__":
    run()
