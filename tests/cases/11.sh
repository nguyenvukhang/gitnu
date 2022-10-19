# xargs: cat range (2-5)

init 8
let i=1
while [ $i -le 8 ]; do
  echo "__content${i}__" >file_$i
  let i++
done
_gitnu status
save gitnu -c cat 2-5
