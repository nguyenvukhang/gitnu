class bcolors:
    PURPLE = "\033[95m"
    BLUE = "\033[94m"
    CYAN = "\033[96m"
    GREEN = "\033[92m"
    YELLOW = "\033[93m"
    RED = "\033[91m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"


def stringify(color):
    def fn(*args):
        text = " ".join(args)
        print(color + text + bcolors.ENDC)

    return fn


class log:
    red = stringify(bcolors.RED)
    yellow = stringify(bcolors.YELLOW)
    green = stringify(bcolors.GREEN)
    blue = stringify(bcolors.BLUE)
    cyan = stringify(bcolors.CYAN)
    purple = stringify(bcolors.PURPLE)
