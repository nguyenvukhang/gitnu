# xargs: cat range (2-5)

init 0
let i=1
while [ $i -le 8 ]; do
  echo "__content${i}__" >file_$i
  let i++
done
gitnu status
save gitnu -x cat 2-5

# --------------------------------------------------------------------
# __content2__
# __content3__
# __content4__
# __content5__
