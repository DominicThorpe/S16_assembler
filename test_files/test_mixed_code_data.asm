.data:
    .word 700

.code:
    add ax bx
    push ax
    .byte 0x55
    pop bx