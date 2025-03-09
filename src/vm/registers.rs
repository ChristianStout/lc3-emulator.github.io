
pub struct Registers {
    pub r: [u16; 8],
    pub pc: u16,
    pub n: bool,
    pub z: bool,
    pub p: bool,
    pub halt: bool,
}

#[allow(dead_code)]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            r: [0; 8],
            pc: 0,
            n: false,
            z: false,
            p: false,
            halt: false,
        }
    }

    pub fn get(&self, reg_value: usize) -> u16 {
        // TODO: Add error handling that will (gracefully) shutdown
        //      entire VM if this get hit.
        if reg_value >= 8 {
            println!("Well, no. Don't give me {} as a register value. There are only 8 registers...", reg_value);
            return 1;
        }

        return self.r[reg_value];
    }

    pub fn set(&mut self, reg_value: usize, new_value: u16) {
        // TODO: Add error handling that will (gracefully) shutdown
        //      entire VM if this get hit.
        if reg_value >= 8 {
            println!("Well, no. Don't give me {} as a register value. There are only 8 registers...", reg_value);
            return;
        }

        self.r[reg_value] = new_value;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_set() {
        let mut reg = Registers::new();

        assert!(reg.r[0] == 0);

        reg.set(0, 256);

        assert!(reg.r[0] != 0);
        assert!(reg.r[0] == 256);
    }

    #[test]
    fn test_get() {
        let mut reg = Registers::new();

        assert!(reg.get(0) == 0);

        reg.r[7] = 1000;
        assert!(reg.get(7) == 1000);

        reg.r[3] = 712;
        assert!(reg.get(3) == 712);
    }
}