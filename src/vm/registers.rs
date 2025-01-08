
pub struct Registers {
    r: [u16; 8],
    pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r: [0; 8],
            pc: 0,
        }
    }
}
