# every possible state

init 0

files=(A B C _D E F G H _I)
for i in ${files[@]}; do
  echo $i >$i
done

# [x] A: index_new
# [x] B: index_modified
# [x] C: index_deleted
# [x] D: index_renamed
# [x] E: index_typechange
# [x] F: wt_new
# [x] G: wt_modified
# [x] H: wt_typechange
# [?] I: wt_renamed

git add B C _D E G H _I
git commit -m "pre"

echo "_" >B
echo "_" >G
rm C
mv _D D
mv _I I

ln -sf . E
ln -sf . H
git add A B C D _D E

save gitnu status
