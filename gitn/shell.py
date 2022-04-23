import subprocess


def system(cmd: list[str]) -> str:
    return systemlist(cmd)[0]


def systemlist(cmd: list[str]) -> list[str]:
    result = []
    stdout = system_std(cmd)
    if not stdout:
        return []
    while stdout.readable():
        line = stdout.readline()
        if not line:
            break
        result.append(line.strip())
    if len(result) == 0:
        return [""]
    return result


def system_std(cmd: list[str]):
    # process = subprocess.Popen(
    #     cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True
    # )
    process = subprocess.Popen(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, universal_newlines=True
    )
    return process.stdout
