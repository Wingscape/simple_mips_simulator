LOAD 0, #1
LOAD 60, #2
LOAD 60, #3
climb:
ADD #1, 20
BNEQ #1, #2, climb
BEQ #1, #3, match
ADD #1, 999
match:
ADD #1, 1
