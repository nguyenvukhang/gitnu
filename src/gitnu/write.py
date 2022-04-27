from typing import IO
from .data_structure import Entry, NumberedStatus
from .shell import system_std
from .strings import StdoutLine
from . import git
from . import cache


def read_stdout(stdout: IO[str]) -> NumberedStatus:
    numbered_status = NumberedStatus()
    index = 1
    stdout_lines = stdout.readlines()
    for std_line in stdout_lines:
        line = StdoutLine(std_line)
        if line.has_filename:
            entry = Entry(index, line.get_filename())
            numbered_status.push(entry)
            print(index, std_line, end="")
            index += 1
        else:
            print(std_line, end="")

    return numbered_status


# print, enumerate, and extract git status, then write cache
def gitnu_status(args):
    stdout, returncode = system_std(git.status + args)
    # if there is an error running the git command,
    # stderr (bypass) will be printed to terminal
    if returncode != 0:
        return
    numbered_status = read_stdout(stdout)
    stdout.close()
    if numbered_status.is_empty():
        return
    cache.write(numbered_status)
