# git handles -C correctly

mkdir one two three
cd one

git init
touch one_file
$GITNU status

cd ../two
git init
touch two_file
$GITNU status

cd ../three
$GITNU -C ../one add 1
save $GITNU -C ../one status

# --------------------------------------------------------------------
# On branch main
# 
# No commits yet
# 
# Changes to be committed:
# 1	[32mnew file:   one_file[m
# 
