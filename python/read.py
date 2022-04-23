import subprocess
import cache
from strings import expand_ranges
from log import log

def gitn_use(args: list[str], command_index: int) -> None:
    trailing = expand_ranges(args[command_index + 1 :])
    command_list = args[: command_index + 1]
    command_list[0] = "git"
    table_exists, table = cache.get_table()
    if table_exists:
        trailing = list(filter(None, map(table.get_filename_by_index, trailing)))
    cmd = command_list + trailing
    log.gray(*cmd)
    subprocess.run(cmd)
