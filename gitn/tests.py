from . import log
from . import shell
from . import cache
import os

system = shell.system


def get_full_path(path: str, cwd: str) -> str:
    result = path
    if not path[0] == "/":
        result = cwd + "/" + path
    return result


def expect(tested, correct):
    if tested == correct:
        log.green("[PASSED]")
    else:
        log.red("[FAILED]", tested)
        log.green("[----->]", correct)


def test_remove_cache():
    # for pure debugging clutches
    cwd = os.getcwd()
    cache_file = cwd + "/.git/gitn.json"
    system(["rm", cache_file])


def TITLE(x, end="\n"):
    log.gray("\n[ ", end="")
    log.cyan(x, end="")
    log.gray(" ]", end=end)


def run_tests(args: list[str], handle_arguments):
    gitn = system(["which", "gitn"])
    original_dir = os.getcwd()

    margin = 24

    def test(x):
        return handle_arguments(args[0:1] + x.split(" ")[1:])

    # run test suite

    # WRITE OPERATIONS
    log.purple("\nWRITE TO CACHE")

    def pre(string: str):
        spaces = " " * (margin - len(string))
        log.gray(string, end=spaces)

    TITLE("gitn status")

    result = test("gitn status")
    pre("command ran:")
    expect(result, [gitn, "status"])

    pre("cache existence:")
    table_exists, table = cache.get_table()
    expect(table_exists, True)

    pre("cache correctness:")
    _, correct_table = cache.get_table(os.getcwd() + "/../root-gitn.json")
    expect(table, correct_table)

    # change directory for next test

    os.chdir(os.getcwd() + "/some/thing")
    # now_at = os.getcwd()
    # log.yellow('now at', now_at)

    # gitn status some/thing

    TITLE("gitn status", end=" ")
    log.gray("(from inside worktree)")

    result = test("gitn status")
    pre("command ran:")
    expect(result, [gitn, "status"])

    pre("cache existence:")
    table_exists, table = cache.get_table()
    expect(table_exists, True)

    pre("cache correctness:")
    _, correct_table = cache.get_table(original_dir + "/../some-thing-gitn.json")
    expect(table, correct_table)

    # READ OPERATIONS
    log.purple("\nREAD FROM CACHE")

    # change directory for next test

    os.chdir(original_dir)

    TITLE("gitn add 1 2 3")

    result = test("gitn add 1 2 3")
    pre("command ran:")
    expect(result, ["git", "add", "1", "2", "3"])

    TITLE("gitn add 1 2 4-7")

    result = test("gitn add 1 2 4-7")
    pre("command ran:")
    expect(result, ["git", "add", "1", "2", "4", "5", "6", "7"])

    TITLE("gitn add 1-1 4-7")

    result = test("gitn add 1-1 4-7")
    pre("command ran:")
    expect(result, ["git", "add", "1", "4", "5", "6", "7"])

    TITLE("gitn add alpha.py")

    result = test("gitn add alpha.py")
    pre("command ran:")
    expect(result, ["git", "add", "alpha.py"])

    TITLE("gitn add alpha.py 1-3")

    result = test("gitn add alpha.py 1-3")
    pre("command ran:")
    expect(result, ["git", "add", "alpha.py", "1", "2", "3"])


def debug(args: list[str], handle_arguments) -> list[str]:
    if len(args) == 2 and args[1] == "--test":
        run_tests(args, handle_arguments)
    # full bypass
    return args
