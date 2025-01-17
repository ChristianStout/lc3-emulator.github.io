use super::asm_ins::OpcodeIns;
use regex::Regex;


pub enum Token {
    Instruction(OpcodeIns),
    Directive,
    Number(i16),
}



pub struct Lexer {
    
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {

        }
    }


}
