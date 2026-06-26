# Load two working values using $0 (always zero) as the source
ori  $1, $0, 0xF0F0
ori  $2, $0, 0x0FF0

# Register-to-register logical ops on $1 and $2
# and  $1, $1, $2
# or   $1, $1, $2
# xor  $1, $1, $2
nor  $1, $1, $2

# Immediate logical ops (reload $1 first so results are clean)
# ori  $1, $0, 0xF0F0
# andi $1, $1, 0x00FF
# ori  $1, $1, 0x000F
# xori $1, $1, 0xFFFF

# Shifts
# ori  $2, $0, 0x000F
# sll  $2, $2, 4
# srl  $2, $2, 4
