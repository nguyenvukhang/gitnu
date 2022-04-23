def get_git_command(args: list[str]) -> tuple[str, int]:
    for i in range(len(args)):
        word = args[i]
        if word in git.commands:
            return (word, i)
    return ("", 0)


class git:
    class cmd:
        status = ["git", "-c", "status.color=always", "status"]

    set_state = {
        "Changes to be committed:": "staged",
        "Changes not staged for commit:": "unstaged",
        "Untracked files:": "untracked",
        "": "unset",
    }
    ignore = ["no changes added to commit"]
    set_action = {
        "deleted:": "deleted",
        "new file:": "newfile",
        "modified:": "modified",
        "renamed:": "renamed",
    }
    set_state_keys = list(set_state.keys())
    commands = ("status", "add", "reset", "diff", "checkout", "--")
    get_commmand = lambda x: get_git_command(x)
