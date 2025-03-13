use std::{collections::HashMap};

use super::{asm_error::{AsmError, ErrorType}, asm_ins::{OpcodeIns, OperandType}};
use super::token::*;


pub struct SemanticChecker {
    symbol_table: HashMap<String, Token>,
    errors: Vec<AsmError>
}

impl SemanticChecker {
    pub fn new() -> SemanticChecker {
        SemanticChecker {
            symbol_table: HashMap::new(),
            errors: vec![],
        }
    }
    
    pub fn run(&mut self, tokens: &Vec<Token>) {
        let mut expected_operands: Vec<OperandType> = vec![];

        for token in tokens {
            match &token.inner_token {
                TokenType::Label(label) => {
                    if expected_operands.len() == 0 {
                        self.define_label(label.clone(), token.clone())
                        continue;
                    }
                },
                _ => {
                    // AsmError::new()
                }
            }
        }
    }


    pub fn define_label(&mut self, label: String, token: Token) {
        if self.symbol_table.contains_key(&label) {
            let other = self.symbol_table.get(&label).unwrap();
            self.errors.push(AsmError::new(
                "",
                token.line_num as i32,
                ErrorType::LabelError,
                &format!("attempted to redefine a label that already defined on line {}", other.line_num),
            ));
            return;
        }
        self.symbol_table.insert(label, token);
    }
}
