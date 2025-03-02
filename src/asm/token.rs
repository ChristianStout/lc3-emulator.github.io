use super::asm_ins::*;
use super::directive::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
    String(String),
    Register(u16),
}
