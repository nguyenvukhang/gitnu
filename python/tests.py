from log import log
from shell import system
import json
import cache
from shell import system
import os


def expect(tested, correct):
    if tested == correct:
        log.green("[PASSED]")
    else:
        log.red('[FAILED]', *tested)
        log.green('[----->]', *correct)

def test_remove_cache():
    # for pure debugging clutches
    cwd = os.getcwd()
    cache_file = cwd + "/.git/gitn.json"
    system(["rm", cache_file])


def run_tests(args: list[str], handle_arguments):
    which = system(["which", "gitn"])
    gitn = args[0]
    cwd = os.getcwd()

    def test(x):
        print(x)
        return handle_arguments([gitn] + x.split(" ")[1:])

    # run test suite

    # WRITE OPERATIONS
    log.purple("\nwrite to cache")

    # gitn status
    expect(test("gitn status"), [which, "status"])
    table_exists, table = cache.get_table()
    _, correct_table = cache.get_table(cwd + '/../root-gitn.json')
    log.gray('hello world', end="")
    expect(table, correct_table)

    if table_exists:
        if table == correct_table:
            log.green("cache contents are correct.")
        else:
            log.red("cache contents are wrong.")
    else:
        log.red("cache doesn't exist after gitn status")

    # gitn status some/thing
    expect(test("gitn status some/thing"), [which, "status", "some/thing"])

    # READ OPERATIONS
    log.purple("\nread from cache")

    # gitn add 1 2 3
    expect(test("gitn add 1 2 3"), ["git", "add", "1", "2", "3"])

    # gitn add 1 2 4-7
    expect(test("gitn add 1 2 4-7"), ["git", "add", "1", "2", "4", "5", "6", "7"])

    # gitn add 1-1 4-7
    expect(test("gitn add 1-1 4-7"), ["git", "add", "1", "4", "5", "6", "7"])


def debug(args: list[str], handle_arguments) -> list[str]:
    if len(args) == 2 and args[1] == "--test":
        run_tests(args, handle_arguments)
    # full bypass
    return args
