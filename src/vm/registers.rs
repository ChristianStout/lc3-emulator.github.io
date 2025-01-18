
pub struct Registers {
    pub r: [u16; 8],
    pub pc: u16,
    pub n: bool,
    pub z: bool,
    pub p: bool,
    pub halt: bool,
}

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
