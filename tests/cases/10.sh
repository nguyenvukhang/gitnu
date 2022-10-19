# xargs: cat

init 0
let i=1
while [ $i -le 20 ]; do
  printf -v p "%03d" $i
  echo "content__of__${p}" >"file_$p"
  let i++
done
_gitnu status
save gitnu -c cat 16-25
