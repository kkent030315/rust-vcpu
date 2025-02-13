; input = r1
; output = r0
    mov     r5, 1
    cmp     r1, r5
    jle     base_case     ; if (n <= 1) return n

    xor     r2, r2        ; a = 0
    mov     r4, 1         ; b = 1
    mov     r3, 2         ; i = 2

loop_start:
    cmp     r3, r1
    jg      done         ; if (i > n) break

    add     r2, r4       ; temp = a + b
    mov     r5, r4       ; r5 = old_b
    mov     r4, r2       ; b = temp
    mov     r2, r5       ; a = old_b

    inc     r3           ; i++
    jmp     loop_start

done:
    mov     r0, r4       ; return b
    exit

base_case:
    mov     r0, r1       ; return n (r0 = n)
    exit
