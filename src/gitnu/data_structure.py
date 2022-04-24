from .strings import extract_filename
from . import log


class Entry:
    def __init__(self, index, line="", filename="") -> None:
        self.state = ""
        self.action = ""
        self.filename = filename
        self.index = index
        self.line = line

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

    def cache(self):
        return [self.index, self.filename]


class NumberedStatus:
    def __init__(self) -> None:
        self.data: list[Entry] = []

    def push(self, entry: Entry):
        self.data.append(entry)

    def remove(self, entry: Entry):
        self.data.remove(entry)

    def get_filename_by_index(self, index: str):
        # return if not a number
        # this is gitnu's promise to be nothing but an alias for numbers
        if not index.isdigit():
            return index
        i = int(index)
        if i in range(1, self.length() + 1):
            return self.data[i - 1].get_filename()
        return index

    def copy(self):
        return tuple(self.data)

    def cache(self):
        return list(map(lambda x: x.cache(), self.data))

    def is_empty(self):
        return len(self.data) == 0

    def length(self):
        return len(self.data)

    def print(self):
        log.yellow(self.cache())

    def clean(self):
        # new_numbered_status = NumberedStatus()
        new_data: list[Entry] = []
        for entry in self.data:
            index = entry.get_index()
            line = entry.get_line()
            entry.set_filename(extract_filename(line))
            # length = new_numbered_status.length()
            length = len(new_data)
            if index == length + 1:
                new_data.append(entry)
            elif index <= length:
                new_data[index - 1] = entry
            else:
                log.perma.yellow('NumberedStatus.clean(): index went beyond current max.')
        self.data = new_data
