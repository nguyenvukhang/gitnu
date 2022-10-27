# unstaged modified file (filename)

init 2
echo "_" >gold
$GITNU add gold # use filename
$GITNU commit -m "first"
echo "_" >>gold
save $GITNU status

# --------------------------------------------------------------------
# On branch main
# Changes not staged for commit:
# 1	[31mmodified:   gold[m
#
# Untracked files:
# 2	[31mfile_0001[m
# 3	[31mfile_0002[m
#
# no changes added to commit
