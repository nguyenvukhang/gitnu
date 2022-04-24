from .git import git

def replace_all(string: str, src: list[str], dest: str) -> str:
    for i in src:
        string = string.replace(i, dest)
    return string


class ansi:
    red = "\x1b[31m"
    green = "\x1b[32m"


def remove_ansi(string: str) -> str:
    codes = [
        ansi.red,
        ansi.green,
        "\x1b[33m",
        "\x1b[34m",
        "\x1b[35m",
        "\x1b[36m",
        "\x1b[37m",
        "\x1b[m",
    ]
    return replace_all(string, codes, "")

def has_red_or_green(string: str) -> bool:
    return ansi.red in string or ansi.green in string


def remove_git_suffix(string: str) -> str:
    suffixes = [
        "(new commits)\n",
        "(modified content)\n",
    ]
    return replace_all(string, suffixes, "\n")

def remove_git_action(string: str) -> str:
    return replace_all(string, git.actions, "\n")




def sanitize_line(string: str) -> str:
    return remove_git_suffix(remove_ansi(string)).strip()

def extract_filename(string: str) -> str:
    return remove_git_action(remove_git_suffix(remove_ansi(string))).strip()
