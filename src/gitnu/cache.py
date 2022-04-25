import json

from . import log
from . import git
from .shell import system
from .data_structure import NumberedStatus, Entry


def get_filepath() -> str:
    cache_directory = system(git.get_repo)
    if cache_directory == "":
        log.warn("get_filepath: cache_directory empty.")
    return f"{cache_directory}/gitnu.json"


def write(numbered_status: NumberedStatus):
    with open(get_filepath(), "w", encoding="UTF-8") as file:
        json.dump(numbered_status.json(), file)


def get(path: str = "") -> NumberedStatus:
    if path == "":
        path = get_filepath()
    table = NumberedStatus()
    try:
        with open(path, "r", encoding="UTF-8") as file:
            for i in json.load(file):
                table.push(Entry(i[0], i[1]))
    except FileNotFoundError:
        log.warn("Cache doesn't exist yet. Run gitnu first.")
    return table
