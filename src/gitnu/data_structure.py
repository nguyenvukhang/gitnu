from . import log


class Entry:
    def __init__(self, index, filename) -> None:
        self.filename = filename
        self.index = index

    def get_filename(self):
        return self.filename

    def get_index(self):
        return self.index

    def json(self):
        return [self.index, self.filename]

    def print(self):
        log.cyan(self.json())


class NumberedStatus:
    def __init__(self) -> None:
        self.data: list[Entry] = []

    def push(self, entry: Entry):
        self.data.append(entry)

    def remove(self, entry: Entry):
        self.data.remove(entry)

    def get_filename(self, index: str):
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

    def json(self):
        return list(map(lambda x: x.json(), self.data))

    def is_empty(self):
        return len(self.data) == 0

    def length(self):
        return len(self.data)

    def print(self):
        log.yellow(self.json())
