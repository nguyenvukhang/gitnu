#!/usr/bin/env python3

import sys
from . import git
from . import tests
from . import write
from . import read

debug = tests.debug

# processes arguments, and returns the list of processed args
def handle_arguments(args: list[str]) -> list[str]:
    num_args = len(args)
    command, command_index = git.get_command(args)
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
