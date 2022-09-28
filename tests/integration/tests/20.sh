# complex short status

init 100
_gitnu status -s
_gitnu add 25-75 # effectively adds files 25-50 inclusive
_gitnu status -s
_gitnu reset 10-20 # resets files 34-44 inclusive
log gitnu status -s
