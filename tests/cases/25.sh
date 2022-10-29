# detached head

__gitnu__ init
touch file_1
__gitnu__ add file_1 && __gitnu__ commit -m 'add: file_1'

touch file_2
__gitnu__ add file_2 && __gitnu__ commit -m 'add: file_2'

__gitnu__ checkout HEAD~1

set_sha __gitnu__ rev-parse --short HEAD
save __gitnu__ status

# --------------------------------------------------------------------
# [31mHEAD detached at [m[:SHA:]
# nothing to commit, working tree clean
