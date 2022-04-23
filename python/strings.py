from _git import git


def remove_ansi(string: str) -> str:
    ansi = [
        "\x1b[31m",
        "\x1b[32m",
        "\x1b[33m",
        "\x1b[34m",
        "\x1b[35m",
        "\x1b[36m",
        "\x1b[37m",
        "\x1b[m",
    ]
    for i in ansi:
        string = string.replace(i, "")
    return string


def sanitize(string: str) -> str:
    return remove_ansi(string.strip())


def expand_ranges(args: list[str]) -> list[str]:
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
