# skip flags

init 10
save gitnu status
let i=1
while [ $i -le 10 ]; do
  gitnu add $i && git commit -m "commit:$i"
  let i++
done

save gitnu log -n 5 --pretty="%s" 6-8

# --------------------------------------------------------------------
# commit:8
# commit:7
# commit:6
