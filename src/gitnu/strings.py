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


def handle_submodules(string: str) -> str:
    return string.replace("(new commits)\n", "\n")


def sanitize_line(string: str) -> str:
    return handle_submodules(remove_ansi(string)).strip()
