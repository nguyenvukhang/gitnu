import re, subprocess


def run_sh(*cmd):
    return subprocess.check_output(cmd).decode("utf-8").split("\n")


output = run_sh("git", "--help", "--all")
NOT_WORD = re.compile("\W")

for line in filter(lambda x: len(x) > 0 and NOT_WORD.match(x[0]), output):
    command = line.strip().split(" ", 1)[0]
    print(command)
