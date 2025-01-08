use super::instructions::{
    Instruction, Add, And, Br, JmpRet, Jsr, Ld,
    Ldi, Lea, Not, Rti, St, Sti, Trap,
};
use super::registers::Registers;
use std::collections::HashMap;

pub struct VM {
    instructions: HashMap<u8, Box<dyn Instruction>>,
    registers: Registers,
}

impl VM {
    pub fn new() -> VM {
        let mut ins: HashMap<u8, Box<dyn Instruction>> = HashMap::new();

        ins.insert(1, Box::new(Add {}));
        ins.insert(5, Box::new(And {}));
        ins.insert(0, Box::new(Br {}));
        ins.insert(12, Box::new(JmpRet {}));
        ins.insert(4, Box::new(Jsr {}));
        ins.insert(2, Box::new(Ld {}));
        ins.insert(10, Box::new(Ldi {}));
        ins.insert(14, Box::new(Lea {}));
        ins.insert(9, Box::new(Not {}));
        ins.insert(8, Box::new(Rti {}));
        ins.insert(3, Box::new(St {}));
        ins.insert(7, Box::new(Sti {}));
        ins.insert(15, Box::new(Trap {}));

        VM {
            instructions: ins,
            registers: Registers::new()
        }
    }
}
