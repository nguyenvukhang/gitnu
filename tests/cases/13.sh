# many files (add 25-75)

init 0
let i=1
while [ $i -le 100 ]; do
  printf -v padded "%04d" $i
  touch "file_$padded"
  let i++
done
$GITNU status
$GITNU add 25-75
save $GITNU status

# --------------------------------------------------------------------
# On branch main
# 
# No commits yet
# 
# Changes to be committed:
# 1	[32mnew file:   file_0025[m
# 2	[32mnew file:   file_0026[m
# 3	[32mnew file:   file_0027[m
# 4	[32mnew file:   file_0028[m
# 5	[32mnew file:   file_0029[m
# 6	[32mnew file:   file_0030[m
# 7	[32mnew file:   file_0031[m
# 8	[32mnew file:   file_0032[m
# 9	[32mnew file:   file_0033[m
# 10	[32mnew file:   file_0034[m
# 11	[32mnew file:   file_0035[m
# 12	[32mnew file:   file_0036[m
# 13	[32mnew file:   file_0037[m
# 14	[32mnew file:   file_0038[m
# 15	[32mnew file:   file_0039[m
# 16	[32mnew file:   file_0040[m
# 17	[32mnew file:   file_0041[m
# 18	[32mnew file:   file_0042[m
# 19	[32mnew file:   file_0043[m
# 20	[32mnew file:   file_0044[m
# 21	[32mnew file:   file_0045[m
# 22	[32mnew file:   file_0046[m
# 23	[32mnew file:   file_0047[m
# 24	[32mnew file:   file_0048[m
# 25	[32mnew file:   file_0049[m
# 26	[32mnew file:   file_0050[m
# 27	[32mnew file:   file_0051[m
# 28	[32mnew file:   file_0052[m
# 29	[32mnew file:   file_0053[m
# 30	[32mnew file:   file_0054[m
# 31	[32mnew file:   file_0055[m
# 32	[32mnew file:   file_0056[m
# 33	[32mnew file:   file_0057[m
# 34	[32mnew file:   file_0058[m
# 35	[32mnew file:   file_0059[m
# 36	[32mnew file:   file_0060[m
# 37	[32mnew file:   file_0061[m
# 38	[32mnew file:   file_0062[m
# 39	[32mnew file:   file_0063[m
# 40	[32mnew file:   file_0064[m
# 41	[32mnew file:   file_0065[m
# 42	[32mnew file:   file_0066[m
# 43	[32mnew file:   file_0067[m
# 44	[32mnew file:   file_0068[m
# 45	[32mnew file:   file_0069[m
# 46	[32mnew file:   file_0070[m
# 47	[32mnew file:   file_0071[m
# 48	[32mnew file:   file_0072[m
# 49	[32mnew file:   file_0073[m
# 50	[32mnew file:   file_0074[m
# 51	[32mnew file:   file_0075[m
# 
# Untracked files:
# 52	[31mfile_0001[m
# 53	[31mfile_0002[m
# 54	[31mfile_0003[m
# 55	[31mfile_0004[m
# 56	[31mfile_0005[m
# 57	[31mfile_0006[m
# 58	[31mfile_0007[m
# 59	[31mfile_0008[m
# 60	[31mfile_0009[m
# 61	[31mfile_0010[m
# 62	[31mfile_0011[m
# 63	[31mfile_0012[m
# 64	[31mfile_0013[m
# 65	[31mfile_0014[m
# 66	[31mfile_0015[m
# 67	[31mfile_0016[m
# 68	[31mfile_0017[m
# 69	[31mfile_0018[m
# 70	[31mfile_0019[m
# 71	[31mfile_0020[m
# 72	[31mfile_0021[m
# 73	[31mfile_0022[m
# 74	[31mfile_0023[m
# 75	[31mfile_0024[m
# 76	[31mfile_0076[m
# 77	[31mfile_0077[m
# 78	[31mfile_0078[m
# 79	[31mfile_0079[m
# 80	[31mfile_0080[m
# 81	[31mfile_0081[m
# 82	[31mfile_0082[m
# 83	[31mfile_0083[m
# 84	[31mfile_0084[m
# 85	[31mfile_0085[m
# 86	[31mfile_0086[m
# 87	[31mfile_0087[m
# 88	[31mfile_0088[m
# 89	[31mfile_0089[m
# 90	[31mfile_0090[m
# 91	[31mfile_0091[m
# 92	[31mfile_0092[m
# 93	[31mfile_0093[m
# 94	[31mfile_0094[m
# 95	[31mfile_0095[m
# 96	[31mfile_0096[m
# 97	[31mfile_0097[m
# 98	[31mfile_0098[m
# 99	[31mfile_0099[m
# 100	[31mfile_0100[m
# 
