import os
from . import log
from . import shell
from . import cache

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
    cache_file = cwd + "/.git/gitnu.json"
    system(["rm", cache_file])


def title(string: str, end="\n"):
    log.gray("\n[ ", end="")
    log.cyan(string, end="")
    log.gray(" ]", end=end)


def run_tests(args: list[str], handle_arguments):
    gitnu = system(["which", "gitnu"])
    original_dir, margin = os.getcwd(), 24

    def test(string: str):
        return handle_arguments(args[0:1] + string.split(" ")[1:])

    # run test suite

    # WRITE OPERATIONS
    log.purple("\nWRITE TO CACHE")

    def pre(string: str):
        spaces = " " * (margin - len(string))
        log.gray(string, end=spaces)

    title("gitnu status")

    result = test("gitnu status")
    pre("command ran:")
    expect(result, [gitnu, "status"])

    pre("cache existence:")
    table = cache.get()
    expect(table.is_empty(), False)

    pre("cache correctness:")
    correct_table = cache.get(os.getcwd() + "/../root-gitnu.json")
    expect(table, correct_table)

    # change directory for next test

    os.chdir(os.getcwd() + "/some/thing")

    # gitnu status some/thing

    title("gitnu status", end=" ")
    log.gray("(from inside worktree)")

    result = test("gitnu status")
    pre("command ran:")
    expect(result, [gitnu, "status"])

    pre("cache existence:")
    table = cache.get()
    expect(table.is_empty(), False)

    pre("cache correctness:")
    correct_table = cache.get(original_dir + "/../some-thing-gitnu.json")
    expect(table, correct_table)

    # READ OPERATIONS
    log.purple("\nREAD FROM CACHE")

    # change directory for next test

    os.chdir(original_dir)

    title("gitnu add 1 2 3")

    result = test("gitnu add 1 2 3")
    pre("command ran:")
    expect(result, ["git", "add", "1", "2", "3"])

    title("gitnu add 1 2 4-7")

    result = test("gitnu add 1 2 4-7")
    pre("command ran:")
    expect(result, ["git", "add", "1", "2", "4", "5", "6", "7"])

    title("gitnu add 1-1 4-7")

    result = test("gitnu add 1-1 4-7")
    pre("command ran:")
    expect(result, ["git", "add", "1", "4", "5", "6", "7"])

    title("gitnu add alpha.py")

    result = test("gitnu add alpha.py")
    pre("command ran:")
    expect(result, ["git", "add", "alpha.py"])

    title("gitnu add alpha.py 1-3")

    result = test("gitnu add alpha.py 1-3")
    pre("command ran:")
    expect(result, ["git", "add", "alpha.py", "1", "2", "3"])


def debug(args: list[str], handle_arguments) -> list[str]:
    if len(args) == 2 and args[1] == "--test":
        run_tests(args, handle_arguments)
    # full bypass
    return args
