# Types of Intructions
label?  instruction([nzp]?)  (reg (, reg|label (, reg|label)))? ingore_space

```asm
label? instruction                  ; no operands (includes traps)
label? instruction  reg             ; one register
label? instruction  label           ; one label
label? instruction  reg, reg        ; two registers
label? instruction  reg, label      ; one reg, one register
label? instruction  reg, reg, reg   ; three registers
label? instruction  reg, reg, label ; two registers, one label
label? instruction  reg, reg, imm   ; two registers, one immediate value
```
