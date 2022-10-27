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
$GITNU status
$GITNU add 3

save $GITNU status

# --------------------------------------------------------------------
# On branch RIGHT
# You have unmerged paths.
# 
# Changes to be committed:
# 1	[32mnew file:   fileA[m
# 2	[32mnew file:   fileB[m
# 
# Unmerged paths:
# 3	[31mboth added:      conflict_file[m
# 
# Untracked files:
# 4	[31mfileC[m
# 
