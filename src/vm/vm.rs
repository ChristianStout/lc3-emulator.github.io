use super::instructions::{
    Instruction, Add, And, Br, JmpRet, Jsr, Ld,
    Ldi, Lea, Not, Rti, St, Sti, Str, Ldr,
};
use super::trap::Trap;
use super::registers::Registers;
use super::memory::Memory;
use std::collections::HashMap;

use std::process::Command;

const CMD_SIZE: u8 = 16;
const OPCODE_SIZE: u8 = 4;
const OPCODE_DELTA: u8 = CMD_SIZE - OPCODE_SIZE;

pub struct VM {
    instructions: HashMap<u8, Box<dyn Instruction>>,
    registers: Registers,
    memory: Memory,
}

#[allow(dead_code)]
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
        ins.insert(7, Box::new(Str {}));
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

    pub fn run(&mut self, file: Vec<u16>) {
        self.registers.pc = file[0];
        
        self.memory.load_file(file);

        while self.registers.halt != true {
            print!("\n{:#06x}\t : ", self.registers.pc);
            /* let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _ = child.wait().unwrap(); */
            self.run_single_command();
        }
    }

    pub fn run_single_command(&mut self) {
        if self.registers.halt == true {
            return;
        }
        let cmd = self.memory.get(self.registers.pc);
        self.registers.pc += 1;

        let opcode: u16 = cmd >> OPCODE_DELTA;
        let value: u16 = cmd - (opcode << OPCODE_DELTA);
        self.instructions[&(opcode as u8)]
            .exe(value, &mut self.registers, &mut self.memory);
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::asm::Asm;
    use super::*;

    fn run_vm(file: &str) -> VM {
        let file = format!(".orig x0000
        
        {file}
    
        halt
        .end"
    );
        let mut asm = Asm::new();

        let binary_file = asm.run(file.to_string());
        
        if binary_file.len() == 0 {
            panic!("Errors occurred during the assembly process, so the VM could not be run");
        }
        
        println!("\nBinary file:");
        for (i, two_bytes) in binary_file.iter().enumerate() {
            println!("{i}:\t{:#018b}", two_bytes);
        }
        
        let mut vm = VM::new();

        vm.run(binary_file);

        return vm;
    }

    #[test]
    fn test_add() {
        let vm = run_vm("
        add r1, r1, #10 ; since every register should be set to 0 by default, this should always just put 10 in r1
        add r2, r2, #5
        add r3, r1, r2  ; r3 == 15
        add r4, r3, r3  ; r4 == 30
        ");

        assert_eq!(vm.registers.r[1], 10);
        assert_eq!(vm.registers.r[2], 5);
        assert_eq!(vm.registers.r[3], 15);
        assert_eq!(vm.registers.r[4], 30);
    }

    #[test]
    fn test_and() {
        let vm = run_vm("
        add r1, r1, #15 ; since every register should be set to 0 by default, this should always just put 10 in r1
        add r2, r2, #5
        and r3, r1, r2  ; r3 == 5
        add r4, r4, #6  ; r4 == 6
        and r5, r4, r3  ; r4 == 4
        ");

        assert_eq!(vm.registers.r[3], 5);
        assert_eq!(vm.registers.r[5], 4);
    }

    #[test]
    fn test_not() {
        let vm = run_vm("
        br      start
num     .fill   #5300
start   add r1, r1, #15 ; since every register should be set to 0 by default, this should always just put 10 in r1
        add r2, r2, #6  ; r2 == 6
        
        not r1, r1
        not r2, r2

        ld  r3, num
        not r3, r3
        ");

        assert_eq!(vm.registers.r[1], !15);
        assert_eq!(vm.registers.r[2], !6);
        assert_eq!(vm.registers.r[3], !5300);
    }

    #[test]
    fn test_jmp() {
        let vm = run_vm("
        lea r0, start
        jmp r0
max     .fill xFFFF

end     ld r0, max
        halt

start   ld r1, max
        lea r7, end
        jmp r7
       ");

        assert_eq!(vm.registers.r[0], u16::MAX);
        assert_eq!(vm.registers.r[1], u16::MAX);
       
    }

    #[test]
    fn test_lea() {
        let vm = run_vm(r#"
        lea r0, start
        lea r1, max
        lea r2, string
        lea r3, end
        lea r4, num_16
        halt

max     .fill xFFFF     ; address = 6

end     ld r0, max      ; address = 7
        halt

string .stringz "len=5" ; address = 9

start   ld r1, max      ; address = 14
        lea r7, end
        jmp r7
num_16  .fill   #16     ; address = 17
        "#);

        assert_eq!(vm.registers.r[0], 14);
        assert_eq!(vm.registers.r[1], 6); 
        assert_eq!(vm.registers.r[2], 9); 
        assert_eq!(vm.registers.r[3], 7); 
        assert_eq!(vm.registers.r[4], 17); 
    }
}
