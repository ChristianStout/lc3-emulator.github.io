
use super::asm_ins::*;
use super::lexer::*;

pub struct Asm {
    lexer: Lexer,
}

impl Asm {
    pub fn new() -> Asm {
        Asm {
            lexer: Lexer::new(),
        }
    }
}
