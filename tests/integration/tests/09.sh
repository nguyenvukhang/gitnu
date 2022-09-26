# not from git root

init 3
# create and commit ./src
mkdir src
touch src/.gitkeep
_gitnu add src
_gitnu commit -m "src_dir"

cd src
touch emerald sapphire ruby
_gitnu status
_gitnu add 3-5
log gitnu status
