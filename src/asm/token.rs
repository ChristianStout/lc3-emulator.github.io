use super::asm_ins::*;

#[derive(Debug, Clone)]
pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
    String(String),
    Register(u16),
}
