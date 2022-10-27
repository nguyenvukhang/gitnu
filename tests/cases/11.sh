# number out of range

init 3
$GITNU status
$GITNU add 2-5
save $GITNU status

# nothing should happen, because this is the same as running git
# add with some files invalid

# --------------------------------------------------------------------
# On branch main
#
# No commits yet
#
# Untracked files:
# 1	[31mfile_0001[m
# 2	[31mfile_0002[m
# 3	[31mfile_0003[m
#
# nothing added to commit but untracked files present
