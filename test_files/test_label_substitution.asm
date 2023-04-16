start: 
    add ax bx
    mul al ah

label_1:
    sub ah bh
    movi cx @label_2
    jump cx

label_2:
    movi ax @label_1
    
