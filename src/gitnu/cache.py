import json

from . import log
from .git import git
from .shell import system
from .data_structure import NumberedStatus, Entry


def get_filepath() -> str:
    cache_directory = system(git.cmd.get_repo)
    if cache_directory == "":
        log.warn("get_filepath: cache_directory empty.")
    return "%s/gitnu.json" % (cache_directory)


def write(numbered_status: NumberedStatus):
    with open(get_filepath(), "w") as f:
        json.dump(numbered_status.json(), f)


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
        table.push(Entry(i[0], i[1]))
    return (status, table)
