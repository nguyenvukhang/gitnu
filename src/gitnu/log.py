import warnings


GRAY = "\033[90m"
RED = "\033[91m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
BLUE = "\033[94m"
PURPLE = "\033[95m"
CYAN = "\033[96m"
ENDC = "\033[0m"
BOLD = "\033[1m"
UNDERLINE = "\033[4m"


def stringify(color):
    def inner(*args, end="\n"):
        text = " ".join(map(str, args))
        print(color + text + ENDC, end=end)

    return inner


def colored_warn(color):
    def inner(*args):
        text = " ".join(map(str, args))
        gitnu = (color + "\n" + text + ENDC, RuntimeWarning)
        stay_calm_and = warnings.warn
        stay_calm_and(*gitnu)

    return inner


gray = stringify(GRAY)
red = stringify(RED)
yellow = stringify(YELLOW)
green = stringify(GREEN)
blue = stringify(BLUE)
cyan = stringify(CYAN)
purple = stringify(PURPLE)
warn = colored_warn(YELLOW)
