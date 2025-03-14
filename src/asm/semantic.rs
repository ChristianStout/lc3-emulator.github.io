use std::collections::HashMap;
use super::{asm_error::{AsmError, ErrorType}, asm_ins::{OpcodeIns, OperandType}};
use super::token::*;
use super::file::AsmFile;


pub struct SemanticChecker {
    symbol_table: HashMap<String, Token>,
    errors: Vec<AsmError>,
    original_file: AsmFile,
}

impl SemanticChecker {
    pub fn new() -> SemanticChecker {
        SemanticChecker {
            symbol_table: HashMap::new(),
            errors: vec![],
            original_file: AsmFile::new("".to_string()),
        }
    }
    
    pub fn run(&mut self, tokens: &Vec<Token>, file: String) {
        self.original_file = AsmFile::new(file);
        let mut expected_operands: Vec<OperandType> = vec![];
        let mut prev_token: &Token = &Token::get_useless_token();
        let mut curr_ins_token: &Token = prev_token;

        // I am well aware of the spaghetti, thank you...
        for token in tokens {
            match &token.inner_token {
                TokenType::Instruction(op_ins) => {
                    if token.line_num == curr_ins_token.line_num {
                        self.errors.push(AsmError::from(
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            "an instruction cannot be an operand. Instructions MUST be separated by line.",
                        ));
                    }
 
                    if expected_operands.len() > 0 {
                        self.errors.push(AsmError::from(
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("{} was expected, but received an instruction instead.", expected_operands[0].as_string()),
                        ));
                    }                   
                    curr_ins_token = token;

                    expected_operands = op_ins.get_expected_operands();
                }
                TokenType::Label(label) => {
                    if expected_operands.len() == 0 {
                        self.define_label(label.clone(), token.clone());
                        continue;
                    }
                    
                    match token.inner_token {
                        TokenType::Label(_) => {},
                        _ => {
                            self.errors.push(AsmError::from(
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                &format!("{} was expected, but received a label instead.", expected_operands[0].as_string()),
                            ));
                        }
                    }
                },
                TokenType::Number(_) => {
                    
                }
                _ => {
                    // AsmError::new()
                }
            }
            prev_token = token;
        }
    }



    pub fn define_label(&mut self, label: String, token: Token) {
        if self.symbol_table.contains_key(&label) {
            let other = self.symbol_table.get(&label).unwrap();
            self.errors.push(AsmError::from(
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::LabelError,
                &format!("attempted to redefine a label that was already defined on line {}", other.line_num),
            ));
            return;
        }
        self.symbol_table.insert(label, token);
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::lexer::*, asm::asm_error::*};

    use super::SemanticChecker; 

    fn get_semantic_errors(file: &str) -> Vec<AsmError> {
        let mut lexer: Lexer = Lexer::new();
        if !lexer.syntax_checker.is_syntactically_valid_file(file) {
            panic!("COULD NOT SEMANTICALLY VERIFY FILE, BECAUSE IS ISN'T SYNTACTICALLY VALID!!!");
        }

        let tokens = lexer.run(file.to_string());
        if lexer.errors.len() > 0 {
            panic!("COULD NOT SEMANTICALLY VERIFY FILE, BECAUSE ERRORS OCCURRED WHILE GENERATING TOKENS!!!");
        }
        
        let mut semantic_checker = SemanticChecker::new();
        semantic_checker.run(&tokens, file.to_string());

        return semantic_checker.errors;
    }

    #[test]
    fn test_redefine_label() {
        let file = r#"
name ret
name ret
        "#;

        let errors = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
    }
}
