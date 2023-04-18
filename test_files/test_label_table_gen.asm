data:
    my_byte: .byte 0x55
    my_word: .word 7000
    my_long: .long 7000000
    my_array: .array 20 21 22 23 24
    my_ascii: .asciiz `Hello world!`  


code:
    start:
        add ax bx
        sub ax bx

    label_2: sll ax cx



    label_3: add cl dl
        movi ax 700
    label_4:


        sub ch dh
