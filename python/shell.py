import subprocess


def system(cmd: list[str]) -> str:
    return systemlist(cmd)[0]


def systemlist(cmd: list[str]) -> list[str]:
    result = []
    process = subprocess.Popen(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True
    )
    if not process.stdout:
        return []
    while process.stdout.readable():
        line = process.stdout.readline()
        if not line:
            break
        result.append(line.strip())
    if len(result) == 0:
        return [""]
    return result
