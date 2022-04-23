from _git import git
from os import getcwd

def make_entry(status, action, filename):
    return [status, action, filename]


def make_empty_entry():
    return ["", "", ""]


def get_status(elem):
    return elem[0]


def get_action(elem):
    return elem[1]


def get_filename(elem):
    return elem[2]

def add_delta(index, entry, table):
    table[index] = entry


def create_table(indexed_lines: list[tuple[str, str]]) -> dict:
    table = {}
    cwd = getcwd()
    state = "unset"  # 'staged' | 'unstaged' | 'untracked'
    for indexed_line in indexed_lines:
        index, line = indexed_line
        state = git.set_state.get(line, state)
        if line in git.set_state or state == "unset":
            continue
        action, filename = "", line.lstrip()
        for key, value in git.set_action.items():
            if line.startswith(key):
                action = value
                line = line[len(key) :]
                filename = "%s/%s" % (cwd, line.lstrip())
                break
        entry = make_entry(state, action, filename)
        add_delta(index, entry, table)
    return table


