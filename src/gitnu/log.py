import warnings

class bcolors:
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
    def fn(*args, end="\n"):
        text = " ".join(map(str, args))
        print(color + text + bcolors.ENDC, end=end)

    return fn

def colored_warn(color):
    def fn(*args):
        text = " ".join(map(str, args))
        warnings.warn(color + '\n' + text + bcolors.ENDC)

    return fn

gray = stringify(bcolors.GRAY)
red = stringify(bcolors.RED)
yellow = stringify(bcolors.YELLOW)
green = stringify(bcolors.GREEN)
blue = stringify(bcolors.BLUE)
cyan = stringify(bcolors.CYAN)
purple = stringify(bcolors.PURPLE)
warn = colored_warn(bcolors.YELLOW)

class perma:
    gray = stringify(bcolors.GRAY)
    red = stringify(bcolors.RED)
    yellow = stringify(bcolors.YELLOW)
    green = stringify(bcolors.GREEN)
    blue = stringify(bcolors.BLUE)
    cyan = stringify(bcolors.CYAN)
    purple = stringify(bcolors.PURPLE)

class temp:
    gray = stringify(bcolors.GRAY)
    red = stringify(bcolors.RED)
    yellow = stringify(bcolors.YELLOW)
    green = stringify(bcolors.GREEN)
    blue = stringify(bcolors.BLUE)
    cyan = stringify(bcolors.CYAN)
    purple = stringify(bcolors.PURPLE)
