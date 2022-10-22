# don't create cache if git failed to run

touch one_file # no git repo initialized
gitnu status   # should not create a stray gitnu.txt
save ls

# --------------------------------------------------------------------
# one_file
