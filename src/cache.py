import json
from shell import system
from data_structure import NumberedStatus, Entry


def get_filepath() -> str:
    cache_directory = system(["git", "rev-parse", "--git-dir"])
    return "%s/gitn.json" % (cache_directory)


def write(cache_filepath: str, table: NumberedStatus):
    with open(cache_filepath, "w") as f:
        json.dump(table.cache(), f)


def get_table(path: str = "") -> tuple[bool, NumberedStatus]:
    if path == "":
        path = get_filepath()
    table = NumberedStatus()
    status = True
    cache_table = []
    try:
        with open(path, "r") as f:
            cache_table = json.load(f)
    except:
        status = False
    for i in cache_table:
        table.push(Entry(i[0], "", i[1]))
    return (status, table)
