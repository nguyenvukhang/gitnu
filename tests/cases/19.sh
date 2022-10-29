# merge conflict

init 3
__gitnu__ add --all
__gitnu__ commit -m "base commit"

# left branch
__gitnu__ branch -m LEFT
echo "left" >conflict_file
__gitnu__ add --all && __gitnu__ commit -m "left-commit"

# right branch
__gitnu__ checkout -b RIGHT
__gitnu__ reset --hard HEAD~1
echo "right" >conflict_file
__gitnu__ add --all && __gitnu__ commit -m "right-commit"

# merge
__gitnu__ merge LEFT

# make some files
touch fileA fileB fileC
__gitnu__ add fileA

# use gitnu index
__gitnu__ status
__gitnu__ add 3

save __gitnu__ status

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
