# xargs: cat (from -s)

init 0
let i=1
while [ $i -le 20 ]; do
  printf -v p "%03d" $i
  echo "content__of__${p}" >"file_$p"
  let i++
done
gitnu status -s
save gitnu -x cat 16-25

# --------------------------------------------------------------------
# content__of__016
# content__of__017
# content__of__018
# content__of__019
# content__of__020
