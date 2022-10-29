# skip flags

init 10
__gitnu__ status
let i=1
while [ $i -le 10 ]; do
  __gitnu__ add $i && __gitnu__ commit -m "commit:$i"
  let i++
done

save __gitnu__ log -n 5 --pretty="%s" 6-8

# --------------------------------------------------------------------
# commit:8
# commit:7
# commit:6
