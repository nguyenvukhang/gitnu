# detached head

git init
touch file_1
git add file_1 && git commit -m 'add: file_1'

touch file_2
git add file_2 && git commit -m 'add: file_2'

git checkout HEAD~1

set_sha git rev-parse --short HEAD
save $GITNU status

# --------------------------------------------------------------------
# [31mHEAD detached at [m[:SHA:]
# nothing to commit, working tree clean
