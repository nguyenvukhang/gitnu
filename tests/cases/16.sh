# porcelain format

init 4
$GITNU status --porcelain
$GITNU add 1-2
echo "_"=> file_1
save $GITNU status --porcelain

# --------------------------------------------------------------------
# 1  A  file_0001
# 2  A  file_0002
# 3  ?? file_0003
# 4  ?? file_0004
# 5  ?? file_1
