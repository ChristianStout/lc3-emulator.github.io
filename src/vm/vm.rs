use super::instructions::*;
use std::collections::HashMap;

struct VM {
    instructions: HashMap<u8, Box<dyn Instruction>>
}
