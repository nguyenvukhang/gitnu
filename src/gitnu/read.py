import subprocess
from . import cache


def is_range(string: str) -> bool:
    if "-" not in string:
        return False
    if string[-1] == "-" or string[0] == "-":
        return False
    undash = string.replace("-", "", 1)
    # undash being only digits means it was originally
    # all digits and one dash.
    return undash.isdigit()


def parse_ranges(args: list[str]) -> list[str]:
    result = []
    for arg in args:
        # handle number ranges
        if is_range(arg):
            split = list(map(int, arg.split("-")))
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
    # parse everything after command
    post_cmd = parse_ranges(args[command_index + 1 :])
    # read from cache
    table = cache.get()
    if not table.is_empty():
        post_cmd = list(map(table.get_filename, post_cmd))
    subprocess.run(pre_cmd + post_cmd, check=False)
