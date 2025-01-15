use super::asm_ins::OpcodeIns;


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
