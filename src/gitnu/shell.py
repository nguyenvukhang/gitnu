import subprocess
from typing import IO, TextIO
from . import log
import io

# actually used a lot
def system(cmd: list[str]) -> str:
    l = systemlist(cmd)
    if len(l) == 0:
        log.warn("system: empty output.")
        return ""
    return l[0]


# not used anywhere
def systemlist(cmd: list[str]) -> list[str]:
    result = []
    stdout, _ = system_std(cmd)
    if not stdout:
        log.warn("systemlist: no stdout.")
        return [""]
    stdout_lines = stdout.readlines()
    for line in stdout_lines:
        result.append(line.strip())
    return result

# used once in write
def system_std(cmd: list[str]) -> tuple[IO[str], int]:
    p = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        universal_newlines=True,
        # keep the line below commented to let stderr text
        # bypass straight to user's terminal as stderr
        # stderr=subprocess.PIPE,
    )
    p.wait()
    if not p.stdout:
        return (TextIO(), p.returncode)
    return (p.stdout, p.returncode)
