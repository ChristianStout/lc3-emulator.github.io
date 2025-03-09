
const POW_2_16: usize = 2_usize.pow(16);

pub struct Memory {
    inner: [u16; POW_2_16],
}

#[allow(dead_code)]
impl Memory {
    pub fn new() -> Memory {
        Memory {
            inner: [0; POW_2_16],
        }
    }

    pub fn get(&self, loc: u16) -> u16 {
        if loc < 8 {
            return self.inner[loc as usize];
        }
        panic!("Only 8 registers exist (0-7). Therefore, register {} does not exist.", loc);
    }

    pub fn set(&mut self, loc: u16, val: u16) {
        if loc < 8 {
            self.inner[loc as usize] = val;
            return;
        }
        panic!("Only 8 registers exist (0-7). Therefore, register {} does not exist.", loc);
    }
}
