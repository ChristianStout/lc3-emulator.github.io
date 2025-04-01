use super::{memory::Memory, registers::Registers};
use std::io::*;
pub struct Trap;

impl Trap {
    pub fn get_c(&self, reg: &mut Registers) {
        self.get_char(reg);
    }

    pub fn out(&self, reg: &mut Registers) {
        print!("{}", reg.get(0) as u8 as char);
    }

    pub fn put_s(&self, reg: &mut Registers, mem: &mut Memory) {
        self.print_string(reg, mem);
    }

    pub fn r#in(&self, reg: &mut Registers, mem: &mut Memory) {
        self.print_string(reg, mem);

        self.get_char(reg);
    }

    pub fn halt(&self, reg:&mut Registers) {
        reg.halt = true;
    }

    fn print_string(&self, reg: &mut Registers, mem: &mut Memory) {
        let mut i = reg.get(0);
        let mut c = mem.get(i) as u8 as char;

        while c != '\0' {
            print!("{c}");
            i += 1;
            c = mem.get(i) as u8 as char;
        }
    }

    fn get_char(&self, reg: &mut Registers) {
        let input: Option<i64> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as i64);

        // Since input is an Option<i64>, which is an enum, we have to consider it's cases: Some and None.
        match input {
            Some(input) => reg.set(0, input as u16),
            None => println!("Char: None"),
        }
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
