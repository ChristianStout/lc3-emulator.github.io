use std::collections::{HashMap, VecDeque};
use super::{asm_error::{AsmError, ErrorType}, asm_ins::OperandType};
use super::token::*;
use super::file::AsmFile;

const CODE_INS_NO_OPERAND: &'static str = "SM000";
const CODE_RECEIVED_UNEXPECTED_INS: &'static str = "SM001";
const CODE_DIR_WRONG_OPERAND: &'static str = "SM002";
const CODE_RECEIVED_UNEXPECTED_LABEL: &'static str = "SM003";
const CODE_EXPECTED_NOTHING_RECEIVED_LABEL: &'static str = "SM004";
const CODE_REDEFINED_LABEL: &'static str = "SM005";
const CODE_RECEIVED_UNEXPECTED_NUMBER: &'static str = "SM006";
const CODE_EXPECTED_NOTHING_RECEIVED_REGISTER: &'static str = "SM007";
const CODE_RECEIVED_UNEXPECTED_REGISTER: &'static str = "SM008";
const CODE_EXPECTED_NOTHING_RECEIVED_STRING: &'static str = "SM009";
const CODE_RECEIVED_UNEXPECTED_STRING: &'static str = "SM010";

#[allow(dead_code)]
pub struct SemanticChecker {
    pub symbol_table: HashMap<String, Token>,
    pub errors: Vec<AsmError>,
    original_file: AsmFile,
}

#[allow(dead_code)]
impl SemanticChecker {
    pub fn new() -> SemanticChecker {
        SemanticChecker {
            symbol_table: HashMap::new(),
            errors: vec![],
            original_file: AsmFile::new("".to_string()),
        }
    }
    
    #[allow(unused_variables)]
    pub fn run(&mut self, tokens: &Vec<Token>, file: String) {
        self.original_file = AsmFile::new(file);
        let mut expected_operands: VecDeque<OperandType> = vec![].into_iter().collect();
        let mut curr_ins_token: &Token = &Token::get_useless_token();

        // I am well aware of the spaghetti, thank you...
        for token in tokens {
            match &token.inner_token {
                TokenType::Instruction(op_ins) => {
                    if token.line_num == curr_ins_token.line_num {
                        self.errors.push(AsmError::from(
                            String::from(CODE_INS_NO_OPERAND),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            "an instruction cannot be an operand. Instructions MUST be separated by line.",
                        ));
                    }
 
                    if expected_operands.len() > 0 {
                        self.errors.push(AsmError::from(
                            String::from(CODE_RECEIVED_UNEXPECTED_INS),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("{} was expected, but received an instruction instead.", expected_operands[0].as_string()),
                        ));
                    }                   
                    curr_ins_token = token;

                    expected_operands = op_ins.get_expected_operands();
                    continue;
                }
                TokenType::Directive(directive) => {
                    if expected_operands.len() > 0 {
                        // Syntactically, it is not possible to get a directive as an argument, so an instruction must have terminated early
                        self.errors.push(AsmError::from(
                            String::from(CODE_DIR_WRONG_OPERAND),
                            &self.original_file.get_line(curr_ins_token.line_num),
                            curr_ins_token.clone(),
                            ErrorType::OperandError,
                            &format!("{} was expected, was not provided.", expected_operands[0].as_string()),
                        ));
                    }                   
                    curr_ins_token = token;

                    expected_operands = directive.get_expected_operands();
                    continue
                }
                TokenType::Label(label) => {
                    if expected_operands.len() == 0 {
                        if token.line_num == curr_ins_token.line_num {
                            self.errors.push(AsmError::from(
                                String::from(CODE_EXPECTED_NOTHING_RECEIVED_LABEL),
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                "no operands were expected, but received a label instead."
                            ));
                            continue;
                        }
                        self.define_label(label.clone(), token.clone());
                        continue;
                    }
                    
                    match token.inner_token {
                        TokenType::Label(_) => { /* ... */ },
                        _ => {
                            self.errors.push(AsmError::from(
                                String::from(CODE_RECEIVED_UNEXPECTED_LABEL),
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                &format!("{} was expected, but received a label instead.", expected_operands[0].as_string()),
                            ));
                        }
                    }
                },
                TokenType::Number(_) => 'number: {
                    if expected_operands.len() == 0 {
                        self.errors.push(AsmError::from(
                            String::from(CODE_EXPECTED_NOTHING_RECEIVED_LABEL),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("no operands were expected, but received a number instead."),
                        ));
                        break 'number;
                    }
                    
                    match expected_operands.front().unwrap() {
                        OperandType::Imm | OperandType::RegOrImm => { /* ... */ },
                        _ => {
                            self.errors.push(AsmError::from(
                                String::from(CODE_RECEIVED_UNEXPECTED_NUMBER),
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                &format!("{} was expected, but received a number instead.", expected_operands[0].as_string()),
                            ));
                        }
                    }
                }
                TokenType::Register(_) => 'number: {
                    if expected_operands.len() == 0 {
                        self.errors.push(AsmError::from(
                            String::from(CODE_EXPECTED_NOTHING_RECEIVED_REGISTER),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("no operands were expected, but received a register instead."),
                        ));
                        break 'number;
                    }
                    
                    match expected_operands.front().unwrap() {
                        OperandType::Imm | OperandType::RegOrImm => { /* ... */ },
                        _ => {
                            self.errors.push(AsmError::from(
                                String::from(CODE_RECEIVED_UNEXPECTED_REGISTER),
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                &format!("{} was expected, but received a register instead.", expected_operands[0].as_string()),
                            ));
                        }
                    }
                }
                TokenType::String(_) => 'string: {
                    if expected_operands.len() == 0 {
                        self.errors.push(AsmError::from(
                            String::from(CODE_EXPECTED_NOTHING_RECEIVED_STRING),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("no operands were expected, but received a string instead."),
                        ));
                        break 'string;
                    }
                    
                    match expected_operands.front().unwrap() {
                        OperandType::String => { /* ... */ },
                        _ => {
                            self.errors.push(AsmError::from(
                                String::from(CODE_RECEIVED_UNEXPECTED_STRING),
                                &self.original_file.get_line(token.line_num),
                                token.clone(),
                                ErrorType::OperandError,
                                &format!("{} was expected, but received a string instead.", expected_operands[0].as_string()),
                            ));
                        }
                    }
                }
                _ => {
                    // AsmError::new()
                }
            }

            if expected_operands.len() > 0 {
                expected_operands.pop_front();
            }
        }
    }



    pub fn define_label(&mut self, label: String, token: Token) {
        if self.symbol_table.contains_key(&label) {
            let other = self.symbol_table.get(&label).unwrap();
            self.errors.push(AsmError::from(
                String::from(CODE_REDEFINED_LABEL),
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

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
    }

    #[test]
    fn test_instruction_on_same_line() {
        let file = r#"
ADD R1, R2, RET
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
    }

    #[test]
    fn test_unexpected_label() {
        let file = r#"
Hello RET
ADD R1, R2, Hello
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
    
    #[test]
    fn test_expected_nothing_but_received_label() {
        let file = r#"
RET hello
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
    }
    
    
    #[test]
    fn test_expected_nothing_but_received_number() {
        let file = r#"
RET #1
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
    
    #[test]
    fn test_received_unexpected_number() {
        let file = r#"
JSR #1
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
    
    #[test]
    fn test_expected_nothing_but_received_register() {
        let file = r#"
RET r1
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
    
    #[test]
    fn test_received_unexpected_register() {
        let file = r#"
JSR r1
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    
    }

    #[test]
    fn test_expected_nothing_but_received_string() {
        let file = r#"
.END "This"
        "#; // strings can only SYNTACTICALLY be given to a directive

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
    
    #[test]
    fn test_received_unexpected_string() {
        let file = r#"
.FILL "This"
        "#; // strings can only SYNTACTICALLY be given to a directive

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0)
    }
 
}
