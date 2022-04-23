from _git import git
from os import getcwd, path
from log import log


class Entry:
    def __init__(self, index, line) -> None:
        self.state = ""
        self.action = ""
        self.filename = ""
        self.index = index
        self.line = line

    def get_state(self):
        return self.state

    def get_action(self):
        return self.action

    def get_filename(self):
        return self.filename

    def get_index(self):
        return self.index

    def get_line(self):
        return self.line

    def set_state(self, value):
        self.state = value

    def set_action(self, value):
        self.action = value

    def set_filename(self, value):
        self.filename = value

    def set_index(self, value):
        self.index = value

    def set_line(self, value):
        self.line = value

    def tuple(self):
        return (self.index, self.state, self.action, self.filename, self.line)

    def list(self):
        return [self.index, self.state, self.action, self.filename, self.line]

    def cache(self):
        return [self.index, self.filename]


class NumberedStatus:
    def __init__(self) -> None:
        self.data: list[Entry] = []

    def clear(self):
        self.data = []

    def push(self, element: Entry):
        self.data.append(element)

    def pop(self, element: Entry):
        self.data.remove(element)

    def get_data(self):
        return tuple(self.data)

    def get_tuples(self):
        return list(map(lambda x: x.tuple(), self.data))

    def cache(self):
        return list(map(lambda x: x.cache(), self.data))

    def is_empty(self):
        return len(self.data) == 0

    def print(self):
        log.yellow(self.get_tuples())

    def length(self):
        return len(self.data)

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


def fill_table(numbered_status: NumberedStatus) -> None:
    table = {}
    cwd = getcwd()
    state = "unset"  # 'staged' | 'unstaged' | 'untracked'
    data_list = numbered_status.get_data()
    for data in data_list:
        index = data.get_index()
        line = data.get_line()
        state = git.set_state.get(line, state)
        # these lines will not have filenames inside
        # and so do not need to remain indexed
        if line in git.set_state or state == "unset" or line == "":
            numbered_status.pop(data)
            continue

        # obtain action and filename from line
        action, filename = "", line.lstrip()

        for keyword, action_value in git.set_action.items():
            if line.startswith(keyword):
                action = action_value
                # too lazy to regex start-of-line,
                # so this too shall pass
                line = line[len(keyword) :]
                filename = path.join(cwd, line.lstrip())
                # use the first match only, so break after first if
                break
        entry = make_entry(state, action, filename)
        data.set_action(action)
        data.set_state(state)
        data.set_filename(filename)
        add_delta(index, entry, table)
