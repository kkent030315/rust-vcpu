; r6=array
mov r6, offsetof array

xor r0, r0
xor r1, r1
xor r2, r2
xor r3, r3
xor r4, r4

mov r5, 0
mov r0, byte [r6 + r5]

mov r5, 1
mov r1, byte [r6 + r5]

mov r5, 2
mov r2, byte [r6 + r5]

mov r5, 3
mov r3, byte [r6 + r5]

mov r5, 4
mov r4, byte [r6 + r5]

exit
