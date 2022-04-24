def replace_all(string: str, src: list[str], dest: str) -> str:
    for i in src:
        string = string.replace(i, dest)
    return string


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
    return replace_all(string, ansi, "")


def handle_submodules(string: str) -> str:
    suffixes = [
        "(new commits)\n",
        "(modified content)\n",
    ]
    return replace_all(string, suffixes, "\n")


def sanitize_line(string: str) -> str:
    return handle_submodules(remove_ansi(string)).strip()
