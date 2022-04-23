import subprocess
from . import cache
from .git import git
from . import log
import os

def parse_ranges(args: list[str]) -> list[str]:
    result = []
    if len(args) == 0:
        return []
    for arg in args:
        # straight bypass for certain keywords
        if arg in git.commands:
            result.append(arg)
            continue
        # handle number ranges
        if "-" in arg:
            try:
                split = list(map(int, arg.split("-")))
            except:
                result.append(arg)
                continue
            if len(split) != 2 and not all(isinstance(x, int) for x in split):
                result.append(arg)
                continue
            result.extend(map(str, range(split[0], split[1] + 1)))
        # bypass for the rest
        else:
            result.append(arg)
    return result


def gitn_use(args: list[str], command_index: int) -> None:
    command_list = args[: command_index + 1]
    command_list[0] = "git"
    trailing = parse_ranges(args[command_index + 1 :])

    table_exists, table = cache.get_table()

    if table_exists:
        trailing = list(map(table.get_filename_by_index, trailing))

    cmd = command_list + trailing
    log.yellow(*cmd)
    # subprocess.run(cmd)
    subprocess.run(['echo', os.getcwd()])
