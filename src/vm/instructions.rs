
/*
Uses the command pattern to execute functions dynamically
*/

pub trait Instruction {
    fn exe(&self, value: u16);
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
    fn exe(&self, value: u16) {

    }
}

impl Instruction for And {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Br {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for JmpRet {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Jsr {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Ld {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Ldi {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Lea {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Not {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Rti {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for St {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Sti {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Str {
    fn exe(&self, value: u16) {

    }
}

impl Instruction for Trap {
    fn exe(&self, value: u16) {

    }
}
