use super::registers::Registers;
use super::memory::Memory;
use super::trap::Trap;

/*
Uses the command pattern to execute functions dynamically
*/

#[allow(dead_code)]
pub trait Instruction {
    /*
    value is the raw instruction interpreted from the asm,
    *excluding* the opcode.
     ^^^^^^^^^
    This is because we already had to obtain that information
    in order to dynamically call the correct instruction. 
    */
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory);
}

#[allow(dead_code, unused_variables)]
pub struct Add;
pub struct And;
pub struct Br;
pub struct JmpRet;
pub struct Jsr;
pub struct Ld;
pub struct Ldi;
pub struct Ldr;
pub struct Lea;
pub struct Not;
pub struct Rti;
pub struct St;
pub struct Sti;
pub struct Str;

impl Instruction for Add {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        ADD - | 0001 000 000 000 000 |
              | ---- --- --- --- --- |
              | op   dr  sr1 --- sr2 |
              +----------------------+
        ADD - | 0001 000 000 1 00000 |
              | ---- --- --- - ----- |
              | op   dr  sr1 - imm   |
        */
        let mut i = value;

        let dr = i >> 9;
        i -= dr << 9;

        let sr1 = i >> 6;
        i -= sr1 << 6;

        let new_value: u16;
        let code = get_bit_index(value, 5);

        match code {
            0 => {
                let sr2 = i;

                let v1 = reg.get(sr1 as usize);
                let v2 = reg.get(sr2 as usize);
        
                new_value = v1 + v2;
            },
            1 => {
                let reg_val = reg.get(sr1 as usize);
                let imm_val = get_offset(value, 5);
                new_value = (reg_val as i16 + imm_val as i16) as u16;
            }
            _ => unreachable!()
        }

        reg.set(dr as usize, new_value);

        set_nzp(reg, new_value);
    }
}

impl Instruction for And {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        AND - | 0101 000 000 000 000 |
              | ---- --- --- --- --- |
              | op   dr  sr1 --- sr2 |
              +----------------------+
        AND - | 0101 000 000 1 00000 |
              | ---- --- --- - ----- |
              | op   dr  sr1 - imm   |
        */
        let mut i = value;

        let dr = i >> 9;
        i -= dr << 9;

        let sr1 = i >> 6;
        let x = sr1 << 6;
        i -= x;

        let code = get_bit_index(value, 5);

        let new_value: u16;

        match code {
            0 => {
                let sr2 = i;

                let v1 = reg.get(sr1 as usize);
                let v2 = reg.get(sr2 as usize);
                new_value = v1 & v2;
            },
            1 => {
                i -= code >> 5;
                let reg_val = reg.get(sr1 as usize);
                let imm_val = i;
                new_value = reg_val & imm_val;
            },
            _ => {
                unreachable!();
            },
        }   
        reg.set(dr as usize, new_value);

        set_nzp(reg, new_value)
    }
}

impl Instruction for Br {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        BR  - | 0000 000 000000000 |
              | ---- --- --------- |
              | op   nzp pcoffset9 |
        */
        let n = get_bit_index(value, 4);
        let z = get_bit_index(value, 5);
        let p = get_bit_index(value, 6);

        // TODO: An unconditional branch is also if ALL ARE NEGATIVE!!!

        if (n == 1 && reg.n) || (z == 1 && reg.z) || (p == 1 && reg.p) {
            reg.pc += get_offset(value, 9);
        }
    }
}

impl Instruction for JmpRet {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        JMP - | 1100 000 000 000000 |
              | ---- --- --- ------ |
              | op       baser      |
              +---------------------+
        JMP - | 1100 000 111 000000 |
              | ---- --- --- ------ |
              | op       r7         |
        */
        let base_reg = value >> 6;

        reg.pc = reg.get(base_reg as usize);
    }
}

impl Instruction for Jsr {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        JSR - | 0100 1 00000000000   |
              | ---- - -----------   |
              | op   c pcoffset11    |
              +----------------------+
        JSRR- | 0100 0 00 000 000000 |
              | ---- - -- --- ------ |
              | op   c -- br  ------ |
        */
        // let code = get_bit_index(value, 12);
        let code = value >> 11;
        let inc_pc = reg.pc;


        match code {
            0 => {
                let offset_reg = value >> 6;
                let offset = reg.r[offset_reg as usize];
                reg.pc += offset;
            },
            1 => {
                let offset = get_offset(value, 11);
                reg.pc = offset;
            },
            _ => unreachable!(),
        }

        // link back to the instruction after Jsr by putting PC in R7
        reg.r[7] = inc_pc;
    }
}

impl Instruction for Ld {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        LD  - | 0010 000 000000000 |
              | ---- --- --------- |
              | op   dr  pcoffset9 |
        */
        let dr = value >> 9;
        let offset = get_offset(value, 9);

        let new_value = mem.get(offset);
        reg.set(dr as usize, new_value);
    }
}

impl Instruction for Ldi {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        LDI - | 1010 000 000000000 |
              | ---- --- --------- |
              | op   dr  pcoffset9 |
        */
        let dr = value << 9;
        let ptr = get_offset(value, 9);

        let address = mem.get(ptr);
        let new_value = mem.get(address);
        reg.set(dr as usize, new_value);
    }
}

impl Instruction for Ldr {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        LDR - | 0110 000 000 000000 |
              | ---- --- --- ------ |
              | op   dr  br  offset6|
        */
        let mut buffer = value;
        let dr = buffer >> 9;
        buffer -= dr << 9;
        let base_r = buffer >> 6;
        let offset = get_offset(value, 6);

        let address = mem.get(base_r) + offset;
        let new_value = mem.get(address);
        reg.set(dr as usize, new_value);
    }
}

impl Instruction for Lea {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        LEA - | 1110 000 000000000 |
              | ---- --- --------- |
              | op   dr  label     |
        
        Loads memory location of the label into memory
        */
        let dr = value >> 9;
        let ptr = get_offset(value, 9);

        let address = reg.pc + ptr;
        reg.set(dr as usize, address);
    }
}

impl Instruction for Not {
    fn exe(&self, value: u16, reg: &mut Registers, _mem: &mut Memory) {
        /*
        NOT - | 1001 000 000 111111 |
              | ---- --- --- ------ |
              | op   dr  sr         |
        */
        let mut i = value;
        let dr = i >> 9;
        i -= dr << 9;
        let sr = i >> 6;

        let old_val = reg.get(sr as usize);
        let not_val = !old_val;

        reg.set(dr as usize, not_val);

        set_nzp(reg, not_val);
    }
}

impl Instruction for Rti {
    fn exe(&self, _value: u16, _reg: &mut Registers, _mem: &mut Memory) {
        /*
        RTI - | 1000 000000000000 |
              | ---- ------------ |
              | op                |
        */
    }
}

impl Instruction for St {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        ST  - | 0011 000 000000000 |
              | ---- --- --------- |
              | op   sr  pcoffset9 |
        */
        let sr = value >> 9;
        let pcoffset9 = get_offset(value, 9);

        mem.set(reg.pc + pcoffset9, reg.get(sr as usize));
    }
}

impl Instruction for Sti {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        STI - | 1011 000 000000000 |
              | ---- --- --------- |
              | op   sr  pcoffset9 |
        */
        let sr = value >> 9;
        let pcoffset9 = get_offset(value, 9);
        let indirect = mem.get(reg.pc + pcoffset9);

        mem.set(indirect, reg.get(sr as usize));
    }
}

impl Instruction for Str {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        STR - | 0111 000 000 000000  |
              | ---- --- --- ------  |
              | op   sr      offset6 |
                         base_r
        */
        let mut buffer = value;
        let sr = buffer >> 9;
        buffer -= sr << 9;
        let base_r = buffer >> 6;
        let offset6 = get_offset(buffer, 6);

        mem.set(reg.get(base_r as usize) + offset6, reg.get(sr as usize));
    }
}

impl Instruction for Trap {
    fn exe(&self, value: u16, reg: &mut Registers, mem: &mut Memory) {
        /*
        TRAP - | 1111 0000 00000000 |
               | ---- ---- -------- |
               | op        trapvec8 |
        */
        let code = get_offset(value, 8);

        match code {
            20 => self.get_c(reg),
            21 => self.out(reg),
            22 => self.put_s(reg, mem),
            23 => self.r#in(reg, mem),
            25 => self.halt(reg),
            _ => unreachable!(),
        }
    }
}

fn get_offset(mut value: u16, num_bits: i32) -> u16 {
    /*
    Every number passed here is a 2's complement signed integer.
    Therefore, we need to check if the right-most bit is a `1`.
    if true, exend entire number with ones.
    */
    let mut pos: u32 = 1;
    let mut buf: u16 = 0;
    let mut bit = 0;

    for _ in 0..num_bits {
        bit = (value % 2) * pos as u16;
        buf += bit;
        pos *= 2;
        value = value >> 1;
    }

    // value is negative if the last bit was not zero
    if bit > 1 {
        let remaining_bits = 16 - num_bits;

        for _ in 0..remaining_bits {
            buf += pos as u16;
            pos *= 2;
        }
    }

    return buf;
}

fn get_bit_index(value: u16, index: i32) -> u16 {
    return value >> index & 1;
}

fn set_nzp(reg: &mut Registers, value: u16) {
    reg.n = false;
    reg.z = false;
    reg.p = false;

    let signed = value as i16;

    if signed < 0 {
        reg.n = true;
    }
    if signed == 0 {
        reg.z = true;
    }
    if signed > 0 {
        reg.p = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let add = super::Add {};

        reg.set(0, 2);
        reg.set(1, 8);

        let ins: u16 = 0b0000_010_001_0_00_000;
        add.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) == 10);

        let ins: u16 = 0b0000_010_001_1_00011; // 3
        add.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) == 11);
        // TODO: Account for NZP bits

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        let ins: u16 = 0b0000_010_001_1_11000; // -8
        add.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) == 0);

        assert!(reg.n == false);
        assert!(reg.z == true);
        assert!(reg.p == false);

        let ins: u16 = 0b0000_010_000_1_11000; // R0 with -8
        add.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) as i16 == -6);

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);
    }

    #[test]
    fn test_and() {
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let and = super::And {};

        reg.set(0, 3);
        reg.set(1, 9);

        let mut ins: u16 = 0b0000_010_001_0_00_000;
        and.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) == 1);

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        ins = 0b0000_010_001_1_11001;
        and.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(2) == 9);

        // TODO: Account for NZP bits
    }


    // #[test]
    // fn test_br() {
    //     unimplemented!();
    // }

    #[test]
    fn test_jmp() {
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let jmp = super::JmpRet {};

        let ins: u16 = 0b0000_000_001_000000;
        reg.pc = 16;
        reg.set(1, 4000);

        jmp.exe(ins, &mut reg, &mut mem);

        assert!(reg.pc != 16);
        assert!(reg.pc == 4000);

        let ins: u16 = 0b0000_000_011_000000;
        reg.set(3, 2048);

        jmp.exe(ins, &mut reg, &mut mem);

        assert!(reg.pc != 4000);
        assert!(reg.pc == 2048);
    }

    #[test]
    fn test_ret() {
        let mut mem = Memory::new();
        let mut reg = super::Registers::new();
        let ret = super::JmpRet {};

        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let jmp = super::JmpRet {};

        let ins: u16 = 0b0000_000_111_000000; // CAN NEVER CHANGE. RET is a completely static instruction
        reg.pc = 16;
        reg.set(7, 999);

        jmp.exe(ins, &mut reg, &mut mem);

        assert!(reg.pc != 16);
        assert!(reg.pc == 999);

        reg.set(7, 2190);
        jmp.exe(ins, &mut reg, &mut mem);

        assert!(reg.pc != 999);
        assert!(reg.pc == 2190);
    }

    // #[test]
    // fn test_jsr() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_jsrr() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_ld() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_ldi() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_ldr() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_lea() {
    //     unimplemented!();
    // }

    #[test]
    fn test_not() {
        let mut mem = super::Memory::new();
        let mut reg = super::Registers::new();
        let not = super::Not {};

        reg.set(1, 0b0000_0101_0000_1111);

        let ins: u16 = 0b0000_000_001_111111;
        not.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);

        reg.set(1, 0b0000_1111_0101_1010);

        let ins: u16 = 0b0000_000_001_111111;
        not.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == true);
        assert!(reg.z == false);
        assert!(reg.p == false);

        reg.set(1, 0b1101_1011_1111_1110);
        not.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == false);
        assert!(reg.z == false);
        assert!(reg.p == true);

        reg.set(1, 0b1111_1111_1111_1111);
        not.exe(ins, &mut reg, &mut mem);

        assert!(reg.get(0) != reg.get(1));
        assert!(reg.get(0) == !reg.get(1));

        assert!(reg.n == false);
        assert!(reg.z == true);
        assert!(reg.p == false);
    }

    // #[test]
    // fn test_rti() {
    //     // unimplemented!(); going to implement in later version.
    // }

    // #[test]
    // fn test_st() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_sti() {
    //     unimplemented!();
    // }

    // #[test]
    // fn test_str() {
    //     unimplemented!();
    // }
}
