#!/usr/bin/env python3

import time
import subprocess

limit = 20

results = []

def benchmark(name: str, cmd_list: list[str]):
    s = time.time()
    for _ in range(limit):
        with subprocess.Popen(cmd_list):
            pass
    e = time.time()
    results.append((name, (e-s)/limit))

    # print("%s --- %s seconds ---" % (name, s - e))


for i in range(limit):
    with subprocess.Popen(["gitnu"]):
        pass


benchmark("PYTHON", ["gitnu"])
benchmark("C++", ["cgit"])
benchmark("git", ["git", "status"])

for i in results:
    print("%s --- %s seconds ---" % i)
