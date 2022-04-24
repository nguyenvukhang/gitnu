from .git import git


def has_red_or_green(string: str) -> bool:
    red, green = "\x1b[31m", "\x1b[32m"
    return red in string or green in string


class StdoutLine:
    def __init__(self, string: str) -> None:
        self.value: str = string
        self.has_filename: bool = has_red_or_green(string)

    def replace_list(self, src: list[str], dest: str):
        for i in src:
            self.value = self.value.replace(i, dest)

    def remove_ansi(self):
        self.replace_list(git.ansi_colors, "")

    def remove_git_suffix(self):
        self.replace_list(git.suffix_list, "\n")

    def remove_git_action(self):
        self.replace_list(git.actions, "\n")

    def get_filename(self):
        self.remove_ansi()
        self.remove_git_suffix()
        self.remove_git_action()
        return self.value.strip()
