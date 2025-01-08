
const POW_2_16: usize = 2_usize.pow(16);

pub struct Memory {
    inner: [u16; POW_2_16],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            inner: [0; POW_2_16],
        }
    }

    pub fn get(&self, loc: u16) -> u16 {
        return self.inner[loc as usize];
    }

    pub fn set(&mut self, loc: u16, val: u16) {
        self.inner[loc as usize] = val;
    }
}
