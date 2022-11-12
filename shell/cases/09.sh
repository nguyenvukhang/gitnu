# not from git root

init 3
# create and commit ./src
mkdir src
touch src/.gitkeep
__gitnu__ add src
__gitnu__ commit -m "src_dir"

cd src
touch emerald sapphire ruby
__gitnu__ status
__gitnu__ add 3-5
save __gitnu__ status

# --------------------------------------------------------------------
# On branch main
# Changes to be committed:
# 1	[32mnew file:   ../file_0003[m
# 2	[32mnew file:   emerald[m
# 3	[32mnew file:   ruby[m
# 
# Untracked files:
# 4	[31m../file_0001[m
# 5	[31m../file_0002[m
# 6	[31msapphire[m
# 
