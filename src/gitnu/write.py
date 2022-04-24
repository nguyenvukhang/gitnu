from .data_structure import Entry, NumberedStatus
from .shell import system_std
from .strings import StdoutLine
from .git import git
from . import cache
# from . import log


def read_stdout(stdout) -> NumberedStatus:
    numbered_status = NumberedStatus()
    if not stdout:
        return numbered_status

    index = 1
    stdout_lines = stdout.readlines()
    if len(stdout_lines) == 0:
        return numbered_status

    for stdout_line in stdout_lines:
        line = StdoutLine(stdout_line)

        if line.has_filename:
            entry = Entry(index, line.get_filename())
            numbered_status.push(entry)
            print(index, stdout_line, end="")
            index += 1
        else:
            print(stdout_line, end="")

    return numbered_status


# print and extract git status
# enumerate git status
# write cache
def gitnu_status(args):
    stdout, returncode = system_std(git.cmd.status + args)
    # if there is an error running the git command,
    # stderr (bypass) will be printed to terminal
    if returncode != 0:
        return
    numbered_status = read_stdout(stdout)
    stdout.close()
    # numbered_status.clean()
    if numbered_status.is_empty():
        return
    # fill_table(numbered_status)
    cache.write(cache.get_filepath(), numbered_status)
