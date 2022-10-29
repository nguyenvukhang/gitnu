# porcelain format

init 4
__gitnu__ status --porcelain
__gitnu__ add 1-2
echo "_"=> file_1
save __gitnu__ status --porcelain

# --------------------------------------------------------------------
# 1  A  file_0001
# 2  A  file_0002
# 3  ?? file_0003
# 4  ?? file_0004
# 5  ?? file_1
