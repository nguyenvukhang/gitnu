# git handles -C correctly

mkdir one two three
cd one

__gitnu__ init
touch one_file
__gitnu__ status

cd ../two
__gitnu__ init
touch two_file
__gitnu__ status

cd ../three
__gitnu__ -C ../one add 1
save __gitnu__ -C ../one status

# --------------------------------------------------------------------
# On branch main
# 
# No commits yet
# 
# Changes to be committed:
# 1	[32mnew file:   one_file[m
# 
