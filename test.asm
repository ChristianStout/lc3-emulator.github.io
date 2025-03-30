
        .orig           x0

        lea             r0, hi
        puts
        halt
hi      .stringz        "Hello, World from the LC-3 Assembler!\n"

        .end
