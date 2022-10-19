# unstaged modified file (number)

init 3
echo "_" >file_3
_gitnu status
_gitnu add 3 # use number on file_3
_gitnu commit -m "first"
echo "_" >>file_3
_gitnu status
_gitnu add 1 # use number on file_3 again
save gitnu status
