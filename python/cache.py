import json
from shell import system
from data_structure import NumberedStatus, Entry
from log import log


def get_filepath() -> str:
    cache_directory = system(["git", "rev-parse", "--git-dir"])
    return "%s/gitn.json" % (cache_directory)


def update(cache_filepath: str, table: NumberedStatus):
    data = table.cache()
    log.red(data)
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
            log.purple(cache_table, path)
    except:
        status = False
    for i in cache_table:
        table.push(Entry(i[0], "", i[1]))
    table.print()
    return (status, table)
