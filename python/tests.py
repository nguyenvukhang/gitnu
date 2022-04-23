from log import log
from shell import system
import json
import cache
from shell import system
import os

def get_full_path(path: str, cwd: str) -> str:
    result = path
    if not path[0] == '/':
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


def TITLE(x):
    log.gray("\n[ ", end="")
    log.cyan(x, end="")
    log.gray(" ]")


def run_tests(args: list[str], handle_arguments):
    gitn = system(["which", "gitn"])
    original_dir = os.getcwd()
    repo_root = system(["git", "rev-parse", "--show-toplevel"])
    def repo_dir():
        return system(['git', 'rev-parse', '--git-dir'])
    margin = 24

    def test(x):
        return handle_arguments(args[0:1] + x.split(" ")[1:])

    # run test suite

    # WRITE OPERATIONS
    log.purple("\nwrite to cache")

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

    os.chdir(os.getcwd() + '/some/thing')
    # now_at = os.getcwd()
    # log.yellow('now at', now_at)

    # gitn status some/thing

    TITLE('gitn status, from a nested directory')

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
    log.purple("\nread from cache")

    # gitn add 1 2 3
    # expect(test("gitn add 1 2 3"), ["git", "add", "1", "2", "3"])

    # gitn add 1 2 4-7
    # expect(test("gitn add 1 2 4-7"), ["git", "add", "1", "2", "4", "5", "6", "7"])

    # gitn add 1-1 4-7
    # expect(test("gitn add 1-1 4-7"), ["git", "add", "1", "4", "5", "6", "7"])


def debug(args: list[str], handle_arguments) -> list[str]:
    if len(args) == 2 and args[1] == "--test":
        run_tests(args, handle_arguments)
    # full bypass
    return args
