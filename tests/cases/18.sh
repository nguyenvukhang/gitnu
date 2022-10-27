# change cwd mid-way

init 3
# create and commit ./src
mkdir src
touch src/.gitkeep
$GITNU add src
$GITNU commit -m "src_dir"

# main test
pushd src >/dev/null
touch emerald sapphire ruby
$GITNU status # ran from /src
popd
$GITNU add 3-5 # ran from /
save $GITNU status

# --------------------------------------------------------------------
# On branch main
# Changes to be committed:
# 1	[32mnew file:   file_0003[m
# 2	[32mnew file:   src/emerald[m
# 3	[32mnew file:   src/ruby[m
# 
# Untracked files:
# 4	[31mfile_0001[m
# 5	[31mfile_0002[m
# 6	[31msrc/sapphire[m
# 
