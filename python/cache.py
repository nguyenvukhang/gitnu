import json
from shell import system
from data_structure import NumberedStatus


def get_filepath() -> str:
    cache_directory = system(["git", "rev-parse", "--git-dir"])
    return "%s/gitn.json" % (cache_directory)


def update(cache_filepath: str, table: NumberedStatus):
    with open(cache_filepath, "w") as f:
        json.dump(table.cache(), f)


def get_table(path: str = "") -> tuple[bool, dict]:
    if path == "":
        path = get_filepath()
    table = {}
    status = True
    try:
        with open(path, "r") as f:
            table = json.load(f)
    except:
        status = False
    return (status, table)
