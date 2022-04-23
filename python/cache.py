import json
from shell import system


def get_filepath() -> str:
    cache_directory = system(["git", "rev-parse", "--git-dir"])
    return "%s/gitn.json" % (cache_directory)


def update(cache_filepath: str, table: dict):
    with open(cache_filepath, "w") as f:
        json.dump(table, f)


def get_table() -> tuple[bool, dict]:
    table = {}
    status = True
    try:
        with open(get_filepath(), "r") as f:
            table = json.load(f)
    except:
        status = False
    return (status, table)
