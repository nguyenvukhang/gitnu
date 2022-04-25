import subprocess
from typing import IO, TextIO
from . import log

# actually used a lot
def system(cmd: list[str]) -> str:
    lst = systemlist(cmd)
    if len(lst) == 0:
        log.warn("system: empty output.")
        return ""
    return lst[0]


# not used anywhere
def systemlist(cmd: list[str]) -> list[str]:
    result = []
    stdout, _ = system_std(cmd)
    stdout_lines = stdout.readlines()
    stdout.close()
    for line in stdout_lines:
        result.append(line.strip())
    return result


# used once in write
# TODO: implement this with `with`
# possible solution: pass in a function to execute per-line
def system_std(cmd: list[str]) -> tuple[IO[str], int]:
    proc = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        universal_newlines=True,
        # keep the line below commented to let stderr text
        # bypass straight to user's terminal as stderr
        # stderr=subprocess.PIPE,
    )
    proc.wait()
    if not proc.stdout:
        # return an empty TextIO object, so it still can close
        return (TextIO(), proc.returncode)
    return (proc.stdout, proc.returncode)
