use super::registers::Registers;
pub struct Trap;

impl Trap {
    pub fn get_c(&self, reg: &mut Registers) {

    }

    pub fn out(&self, reg: &mut Registers) {

    }

    pub fn put_s(&self, reg:&mut Registers) {

    }

    pub fn r#in(&self, reg:&mut Registers) {
        
    }

    pub fn halt(&self, reg:&mut Registers) {
        reg.halt = true;
    }
}
