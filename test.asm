        lea r0, start
        jmp r0
max     .fill xFFFF

end     ld r0, max
        halt

start   ld r1, max
        lea r7, end
        jmp r7