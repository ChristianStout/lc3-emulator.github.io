use super::instructions::{
    Instruction, Add, And, Br, JmpRet, Jsr, Ld,
    Ldi, Lea, Not, Rti, St, Sti, Trap, Ldr
};
use super::registers::Registers;
use super::memory::Memory;
use std::collections::HashMap;

const CMD_SIZE: u8 = 16;
const OPCODE_SIZE: u8 = 4;
const OPCODE_DELTA: u8 = CMD_SIZE - OPCODE_SIZE;

pub struct VM {
    instructions: HashMap<u8, Box<dyn Instruction>>,
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new() -> VM {
        let mut ins: HashMap<u8, Box<dyn Instruction>> = HashMap::new();

        ins.insert(0, Box::new(Br {}));
        ins.insert(1, Box::new(Add {}));
        ins.insert(2, Box::new(Ld {}));
        ins.insert(3, Box::new(St {}));
        ins.insert(4, Box::new(Jsr {}));
        ins.insert(5, Box::new(And {}));
        ins.insert(6, Box::new(Ldr {}));
        ins.insert(7, Box::new(Sti {}));
        ins.insert(8, Box::new(Rti {}));
        ins.insert(9, Box::new(Not {}));
        ins.insert(10, Box::new(Ldi {}));
        ins.insert(11, Box::new(Sti {}));
        ins.insert(12, Box::new(JmpRet {}));
        // ins.insert(13, Box::new(Reserved {}));
        ins.insert(14, Box::new(Lea {}));
        ins.insert(15, Box::new(Trap {}));

        VM {
            instructions: ins,
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn run_single_command(&mut self, cmd: u16) {
        let opcode: u16 = cmd >> OPCODE_DELTA;
        let value: u16 = cmd - (opcode << OPCODE_DELTA);
        self.instructions[&(opcode as u8)]
            .exe(value, &mut self.registers, &mut self.memory);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
