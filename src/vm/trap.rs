use super::{memory::Memory, registers::Registers};
// use std::io::{Read, Write};
pub struct Trap;

impl Trap {
    pub fn get_c(&self, reg: &mut Registers) {

    }

    pub fn out(&self, reg: &mut Registers) {
        print!("{}", reg.get(0) as u8 as char);
    }

    pub fn put_s(&self, reg: &mut Registers, mem: &mut Memory) {

    }

    pub fn r#in(&self, reg:&mut Registers) {
        
    }

    pub fn halt(&self, reg:&mut Registers) {
        reg.halt = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::registers::Registers;
    use super::*;

    #[test]
    fn test_out() {
        let mut reg = Registers::new();
        let trap = Trap {};

        reg.set(0, 'a' as u16);

        trap.out(&mut reg);

        reg.set(0, 'p' as u16);

        trap.out(&mut reg);
        trap.out(&mut reg);
    }
}
