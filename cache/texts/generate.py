import random
import string
import sys

letters = string.ascii_lowercase
rand = lambda n: "".join(random.choice(letters) for _ in range(n))


for _ in range(int(sys.argv[1])):
    print(rand(80))
