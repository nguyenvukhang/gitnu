class keyphrases:
    set_state = {
        "Changes to be committed:": "staged",
        "Changes not staged for commit:": "unstaged",
        "Untracked files:": "untracked",
        "no changes added to commit": "",
        "": "none",
    }
    set_action = {
        "deleted:": "deleted",
        "new file:": "newfile",
        "modified:": "modified",
        "renamed:": "renamed",
    }
    set_state_keys = list(set_state.keys())
