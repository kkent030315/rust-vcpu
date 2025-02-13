mov r0, offsetof key
mov r1, %KEYLEN%
mov r2, offsetof sbox
mov r8, offsetof buf
mov r9, %BUFLEN%

rc4_init:
; key = r0
; len = r1
; sbox = r2
    xor r3, r3                      ; r3 = i = 0

loop1_s:
    ; Initialize sbox with 0,1,2,...,255
    mov byte [r2 + r3], r3
    inc r3
    cmp r3, 256
    jnz loop1_s

    xor r3, r3 ; r3 = i = 0
    xor r5, r5 ; r5 = j = 0
    xor r6, r6 ; r6 = i mod key_len = 0

loop2_s:
; j += s[i]
	mov r4, byte [r2 + r3]
	add r5, r4
	mov r7, r4

; j += key[i mod L]
	mov r4, byte [r0 + r6]
	add r5, r4

; j mod 256
	xor r4, r4
	mov r4, r5
    and r4, 0FFh
    and r5, 0FFh
	xchg r5, r4

; Swap s[i] with s[j]
	mov r4, byte [r2 + r5]
	mov byte [r2 + r3], r4
	mov r4, r7
	mov byte [r2 + r5], r4

; i mod L
	inc r6
	xor r4, r4
	cmp r1, r6
    jnz no_reset
	mov r6, r4
no_reset:
; i++
    inc r3
    cmp r3, 256
    jnz loop2_s

rc4_apply_stream:
; buf_in = r8
; len = r9
; sbox = r2
; free regs: [r0, r1, r3, r4, r5, r6, r7, r10, r11, r12, r13, r14, r15]

; cleanup regs
    xor r0, r0
    xor r1, r1
    xor r3, r3
    xor r4, r4
    xor r5, r5
    xor r6, r6
    xor r7, r7
    xor r10, r10
    xor r11, r11
    xor r12, r12
    xor r13, r13
    xor r14, r14
    xor r15, r15

	mov r1, r3
	mov r1, r4
loop3_s:
	cmp r0, r9
	jz end
; i := (i + 1) mod 256
	mov r1, r3
	inc r1
	xor r10, r10
    and r1, 0FFh
	mov r10, r1
    and r10, 0FFh
	mov r3, r10
; Load s[i]
	mov r1, byte [r2 + r10]
; j := (j + s[i]) mod 256
	mov r10, r4
	mov r12, r1
	add r1, r10
	xor r10, r10
    and r1, 0FFh
	mov r10, r1
	mov r4, r10
; Load s[j] and save it in s[i]
	mov r1, byte [r2 + r10]
    and r1, 0FFh
	mov r13, r1
	mov r10, r3
	mov byte [r2 + r10], r1
; Save s[i] in s[j]
	mov r10, r4
	mov r1, r12
	mov byte [r2 + r10], r1
; (s[i] + s[j]) mod 256
	mov r10, r13
	add r1, r10
	xor r10, r10
    and r1, 0FFh
	mov r10, r1
    and r10, 0FFh
; Load s[(s[i] + s[j]) mod 256]
	mov r1, byte [r2 + r10]
; Load buf_in[n]
	mov r10, byte [r8 + r0]
; Encrypt
	xor r1, r10
; Save encrypted value
    and r1, 0FFh
	mov byte [r8 + r0], r1
; Next n
	inc r0
	jmp loop3_s
end:
	exit

sbox:
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000

    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000

    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000

    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000
    dq 0000000000000000

; key:
; buf:
