# staged file (number)

init 2
touch file:gold
__gitnu__ status
__gitnu__ add 1-2 # use number
__gitnu__ status
__gitnu__ reset 1 # use number again, on file:gold
save __gitnu__ status

# --------------------------------------------------------------------
# On branch main
#
# No commits yet
#
# Changes to be committed:
# 1	[32mnew file:   file_0001[m
#
# Untracked files:
# 2	[31mfile:gold[m
# 3	[31mfile_0002[m
#
