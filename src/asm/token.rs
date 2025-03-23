use super::asm_ins::*;
use super::directive::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Tsify, Serialize,Deserialize)]
pub enum TokenType {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
    String(String),
    Register(u16),
    INVALID(String),
}

#[derive(Debug, Clone, PartialEq, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Token {
    pub inner_token: TokenType,
    pub to: usize,
    pub from: usize,
    pub file_relative_to: usize,
    pub file_relative_from: usize,
    pub line_num: usize,
    pub original_match: String,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenCollection {
    pub tokens: Vec<Token>,
}

impl Token {
    pub fn new(file_index: usize, line_index: usize, line_num: i32, string: &str, token: TokenType) -> Token {
        let original_match = string.to_string();
        let to = line_index - 1; // Because it only matches on the whitespace after the match
        let from = to - original_match.len();
        let file_relative_to = file_index - 1;
        let file_relative_from = file_relative_to - original_match.len();

        Token {
            inner_token: token,
            to: to - 1,
            from: from,
            file_relative_to: file_relative_to - 1,
            file_relative_from: file_relative_from,
            line_num: line_num as usize,
            original_match: original_match,
        }
    }
    
    pub fn get_useless_token() -> Token {
        Token {
            inner_token: TokenType::INVALID("".to_string()),
            from: 0,
            to: 0,
            file_relative_from: 0,
            file_relative_to: 0,
            line_num: 0,
            original_match: "".to_string(),
        }
    }
}
