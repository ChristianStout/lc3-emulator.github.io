
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

    pub fn load_file(&mut self, file: Vec<u16>) {
        let mut mem_i = file[0] as usize; // origin
        let mut vec_i = 1;

        while vec_i < file.len() {
            self.inner[mem_i] = file[vec_i];
            
            vec_i += 1;
            mem_i += 1;
        }
    }

    pub fn get(&self, loc: u16) -> u16 {
        return self.inner[loc as usize];
    }

    pub fn set(&mut self, loc: u16, val: u16) {
        self.inner[loc as usize] = val;
    }
}
