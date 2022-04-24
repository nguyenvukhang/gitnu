import subprocess
from . import cache


def is_range(string: str) -> bool:
    undash = string.replace("-", "", 1)
    # undash being only digits means it was originally
    # all digits and one dash.
    dash_end = not (string[-1] == "-")
    dash_start = not (string[0] == "-")
    return undash.isdigit() and dash_start and dash_end


def parse_ranges(args: list[str]) -> list[str]:
    result = []
    for arg in args:
        # handle number ranges
        if is_range(arg):
            split = list(map(int, arg.split('-')))
            split[1] += 1
            result.extend(map(str, range(*split)))
        # bypass for the rest
        else:
            result.append(arg)
    return result


# command_index is the index of the git command
# example: git -C /usr/share status -s
# command is "status", index is 3
def gitnu_use(args: list[str], command_index: int) -> None:
    # preserve everything before command
    pre_cmd = args[: command_index + 1]
    pre_cmd[0] = "git"

    # parse every after command
    post_cmd = parse_ranges(args[command_index + 1 :])

    # read from cache
    table = cache.get_table()

    if not table.is_empty():
        post_cmd = list(map(table.get_filename_by_index, post_cmd))

    subprocess.run(pre_cmd + post_cmd)
