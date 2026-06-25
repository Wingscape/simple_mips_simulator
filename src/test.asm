LOAD 0x0, $1
LOAD 60, $2
LOAD 60, $3
climb:
ADD $1, 0x14
BNEQ $1, $2, climb
BEQ $1, $3, match
ADD $1, 0x3E7
match:
ADD $1, 1
