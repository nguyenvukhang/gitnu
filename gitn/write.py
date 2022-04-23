from .data_structure import fill_table, Entry, NumberedStatus
from .shell import system_std
from .strings import sanitize
from .git import git
from . import cache


def read_stdout(stdout) -> NumberedStatus:
    numbered_status = NumberedStatus()
    if not stdout:
        return numbered_status

    checkpoints = git.set_state_keys
    result, index, counting = [], 1, False
    stdout_lines = stdout.readlines()
    if len(stdout_lines) == 0:
        return numbered_status

    for line in stdout_lines:
        stripped = sanitize(line)
        result.append((index, stripped))
        entry = Entry(index, stripped)
        numbered_status.push(entry)
        if counting:
            if stripped == "":
                counting = False
                print(line, end="")
            elif stripped in checkpoints:
                print(line, end="")
            else:
                print(index, line, end="")
                index += 1
        else:
            if stripped in checkpoints:
                counting = True
            print(line, end="")
    return numbered_status


# print and extract git status
# enumerate git status
# write cache
def gitn_status(args):
    stdout = system_std(git.cmd.status + args)
    numbered_status = read_stdout(stdout)
    if numbered_status.is_empty():
        return
    fill_table(numbered_status)
    cache.write(cache.get_filepath(), numbered_status)
