
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
