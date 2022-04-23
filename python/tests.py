from log import log
from shell import system

def expect(tested, correct):
    if tested == correct:
        log.green("ok", *tested)
    else:
        log.red(*tested)
        log.green(*correct)


def debug(args: list[str], handle_arguments) -> list[str]:
    which = system(["which", "gitn"])
    gitn = args[0]
    if len(args) == 2 and args[1] == "--test":

        def test(x):
            print(x)
            return handle_arguments([gitn] + x.split(" ")[1:])

        # run test suite
        log.purple('\nwrite to cache')
        expect(test("gitn status"), [which, "status"])
        expect(test("gitn status some/thing"), [which, "status", "some/thing"])
        log.purple('\nread from cache')
        expect(test("gitn add 1 2 3"), ["git", "add", "1", "2", "3"])
        expect(test("gitn add 1 2 4-7"), ["git", "add", "1", "2", "4", "5", "6", "7"])
        expect(test("gitn add 1-1 4-7"), ["git", "add", "1", "4", "5", "6", "7"])
        pass
    # full bypass
    return args
