#!/usr/bin/env python3

import sys
from _git import git
from tests import debug
import write
import read


# processes arguments, and returns the list of processed args
def handle_arguments(args: list[str]) -> list[str]:
    num_args = len(args)
    command, command_index = git.get_commmand(args)
    if num_args <= 1 or command == "status":
        write.gitn_status(args[command_index + 1 :])
    # from here on there are at least two args
    # if gitn command doesn't include status
    # it's essentially a full bypass with the numbers as file aliases
    else:
        read.gitn_use(args, command_index)
    return args


def main():
    args = debug(sys.argv, handle_arguments)
    handle_arguments(args)


if __name__ == "__main__":
    main()
