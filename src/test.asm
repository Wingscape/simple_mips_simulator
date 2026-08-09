// or $25, $0, $0
// ori $11, $0, 0x3528
// ori $12, $0, 0x4A

// sll $11, $11, 16
// or $25, $11, $0

// or $25, $25, $12

// ori $11, $0, 255
// ori $12, $0, 255

// add $13, $12, $11

// ori $7, $0, 146
// ori $8, $0, 82

// nor $8, $8, $0
// ori $9, $0, 1
// addu $8, $8, $9

// addiu $10, $7, -82

// ori $8, $0, 12
// ori $9, $0, 5
// mult $9, $8
// mflo $9
// addiu $9, $9, -74

// ori $8, $0, 8
// ori $9, $0, 36

// addu $10, $9, $8
// subu $11, $9, $8

// div $10, $11
// mflo $10
// mfhi $11

// ori $1, $0, 260
// ori $2, $0, 0x10
// sw $1, 4($2)
// lw $3, 4($2)
// addi $3, $3, 10

// lui $9, 0x1234
// ori $9, $9, 0x5678
// ori $8, $0, 0x10

// big endian
// sb $9, 0x3($8)
// srl $9, $9, 8
// sb $9, 0x2($8)
// srl $9, $9, 8
// sb $9, 0x1($8)
// srl $9, $9, 8
// sb $9, 0x0($8)

// dangerous endless loop, but it works
// main:
// sll $0, $0, 0
// sll $0, $0, 0
// sll $0, $0, 0
// sll $0, $0, 0
// j main
// addiu $8, $8, 1

// lui $10, 0x0

// store the number
// addi $1, $0, -1
// sw $1, 0($10)

// load the number
// lw $8, 0($10)
// sll $0, $0, 0

// is it negative?
// srl $9, $8, 31
// beq $0, $9, done
// sll $0, $0, 0

// it's negative
// so we turn it into positive
// subu $8, $0, $8
// sw $8, 0($10)

// it's positive
// sll $0, $0, 0

//ori $3, $0, 1

//ori $2, $0, 40
//ori $4, $0, 29

//sltiu $6, $2, 56
//sltu $7, $4, $2

//and $8, $6, $7
//beq $8, $0, false
//sll $0, $0, 0

//j fine
//sll $0, $0, 0

//false:
//ori $3, $0, 0

//fine:
//sll $0, $0, 0

ori $10, $0, 0
ori $8, $0, 0

test:
sltiu $9, $8, 10
beq $9, $0, endLp
sll $0, $0, 0

addu $10, $10, $8

addiu $8, $8, 1
j test
sll $0, $0, 0

endLp:
sll $0, $0, 0
