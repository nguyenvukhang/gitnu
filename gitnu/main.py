#!/usr/bin/env python3

import sys

from . import git
from . import write
from . import read
# from . import log

# processes arguments, and returns the list of processed args
def handle_arguments(args: list[str]) -> list[str]:
    num_args = len(args)
    command, command_index = git.get_command(args)
    if num_args <= 1 or command == "status":
        write.gitnu_status(args[command_index + 1 :])
    # from here on there are at least two args
    # if gitnu command doesn't include status
    # it's essentially a full bypass with the numbers as file aliases
    else:
        read.gitnu_use(args, command_index)
    return args


def main():
    handle_arguments(sys.argv)
