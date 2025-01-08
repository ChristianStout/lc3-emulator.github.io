use super::registers::Registers;

/*
Uses the command pattern to execute functions dynamically
*/

pub trait Instruction {
    fn exe(&self, value: u16, reg: &mut Registers);
}

pub struct Add;
pub struct And;
pub struct Br;
pub struct JmpRet;
pub struct Jsr;
pub struct Ld;
pub struct Ldi;
pub struct Lea;
pub struct Not;
pub struct Rti;
pub struct St;
pub struct Sti;
pub struct Str;
pub struct Trap;

impl Instruction for Add {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for And {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Br {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for JmpRet {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Jsr {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Ld {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Ldi {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Lea {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Not {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Rti {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for St {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Sti {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Str {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}

impl Instruction for Trap {
    fn exe(&self, value: u16, reg: &mut Registers) {

    }
}
