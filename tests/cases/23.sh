# skip flags

init 10
save gitnu status
let i=1
while [ $i -le 10 ]; do
  gitnu add $i && git commit -m "commit:$i"
  let i++
done

# save gitnu log -n 5 \
#   --pretty=format:%s \
#   file_0006 file_0007 file_0008

save gitnu log -n 5 \
  --pretty=format:%s 6-8
