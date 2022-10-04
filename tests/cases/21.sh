# change cwd mid-way

init 3
# create and commit ./src
mkdir src
touch src/.gitkeep
_gitnu add src
_gitnu commit -m "src_dir"

# main test
pushd src >/dev/null
touch emerald sapphire ruby
_gitnu status # ran from /src
popd
_gitnu add 3-5 # ran from /
log gitnu status
