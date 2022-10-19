# merge conflict

init 3
git add --all
git commit -m "base commit"

# left branch
git branch -m LEFT
echo "left" >conflict_file
git add --all && git commit -m "left-commit"

# right branch
git checkout -b RIGHT
git reset --hard HEAD~1
echo "right" >conflict_file
git add --all && git commit -m "right-commit"

# merge
git merge LEFT

# make some files
touch fileA fileB fileC
git add fileA

# use gitnu index
_gitnu status
gitnu add 3

save gitnu status
