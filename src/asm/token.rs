use super::asm_ins::*;
use super::directive::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
    String(String),
    Register(u16),
    INVALID(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub inner_token: TokenType,
    pub to: usize,
    pub from: usize,
    pub line_num: usize,
    pub original_match: String,
}

impl Token {
    pub fn new(index: usize, line_num: i32, string: &str, token: TokenType) -> Token {
        let original_match = string.to_string();
        let to = index - 1; // Because it only matches on the whitespace after the match
        let from = to - original_match.len();

        Token {
            inner_token: token,
            to: to,
            from: from,
            line_num: line_num as usize,
            original_match: original_match,
        }
    }
}
