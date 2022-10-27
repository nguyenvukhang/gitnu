# skip flags

init 10
$GITNU status
let i=1
while [ $i -le 10 ]; do
  $GITNU add $i && git commit -m "commit:$i"
  let i++
done

save $GITNU log -n 5 --pretty="%s" 6-8

# --------------------------------------------------------------------
# commit:8
# commit:7
# commit:6
