data:
    some_data: .array 0x50 0x27 0x92 0xAC
    one_byte: .byte 0xBC

code:
    start: 
        add ax bx
        mul al al

    label_1:
        sub ah bh
        movi cx @label_2
        jump cx

    label_2:
        movi ax @label_1
        movi bx @one_byte
        load cx, bx
        store cx bx
    
