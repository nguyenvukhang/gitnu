# many files (add 25-75)

init 0
let i=1
while [ $i -le 100 ]; do
  printf -v padded "%04d" $i
  touch "file_$padded"
  let i++
done
_gitnu status
_gitnu add 25-75
log gitnu status
