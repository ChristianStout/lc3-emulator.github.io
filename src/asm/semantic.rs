use std::{collections::{HashMap, VecDeque}, hash::Hash};
use super::{asm_error::{AsmError, ErrorType}, asm_ins::OperandType, directive::{self, Directive}, token};
use super::token::*;
use super::file::AsmFile;

const CODE_INS_NO_OPERAND: &'static str = "SM000";
const CODE_RECEIVED_UNEXPECTED_INS: &'static str = "SM001";
const CODE_DIR_WRONG_OPERAND: &'static str = "SM002";
const CODE_RECEIVED_UNEXPECTED_LABEL: &'static str = "SM003";
const CODE_EXPECTED_NOTHING_RECEIVED_LABEL: &'static str = "SM004";
const CODE_REDEFINED_LABEL: &'static str = "SM005";
const CODE_RECEIVED_UNEXPECTED_NUMBER: &'static str = "SM006";
const CODE_EXPECTED_NOTHING_RECEIVED_NUMBER: &'static str = "SM007";
const CODE_RECEIVED_UNEXPECTED_REGISTER: &'static str = "SM008";
const CODE_EXPECTED_NOTHING_RECEIVED_REGISTER: &'static str = "SM009";
const CODE_RECEIVED_UNEXPECTED_STRING: &'static str = "SM010";
const CODE_EXPECTED_NOTHING_RECEIVED_STRING: &'static str = "SM011";
const CODE_NO_ORIG: &'static str = "SM012";
const CODE_NO_END: &'static str = "SM013";
const CODE_USED_UNDEFINED_LABEL: &'static str = "SM014";
const CODE_NUMBER_OUT_OF_BOUNDS: &'static str = "SM015";

#[allow(dead_code)]
pub struct SemanticChecker {
    pub symbol_table: HashMap<String, Token>,
    pub errors: Vec<AsmError>,
    original_file: AsmFile,
    used_labels: HashMap<String, Token>,
}

#[allow(dead_code)]
impl SemanticChecker {
    pub fn new() -> SemanticChecker {
        SemanticChecker {
            symbol_table: HashMap::new(),
            errors: vec![],
            original_file: AsmFile::new("".to_string()),
            used_labels: HashMap::new(),
        }
    }
    
    #[allow(unused_variables)]
    pub fn run(&mut self, tokens: &Vec<Token>, file: String) {
        self.original_file = AsmFile::new(file);
        let mut expected_operands: VecDeque<OperandType> = vec![].into_iter().collect();
        let mut curr_ins_token: &Token = &Token::get_useless_token();
        let mut end_encountered = false;
        
        if !self.orig_at_top(&tokens) {
            self.errors.push(AsmError::from(
                String::from(CODE_NO_ORIG),
                &self.original_file.get_line(tokens[0].line_num),
                tokens[0].clone(),
                ErrorType::LogicalError,
                "the `.ORIG` directive must be at the top of the file. To resolve this error, add `.ORIG x3000` at the top of the file.",
                
            ))
        }

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
                    if self.is_end(directive) {
                        end_encountered = true;
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
                    
                    match expected_operands.front().unwrap() {
                        OperandType::Label => { /* ... */ 
                            println!("LABEL MATCHED!");
                            self.used_labels.insert(token.original_match.clone(), token.clone());
                        },
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
                            String::from(CODE_EXPECTED_NOTHING_RECEIVED_NUMBER),
                            &self.original_file.get_line(token.line_num),
                            token.clone(),
                            ErrorType::OperandError,
                            &format!("no operands were expected, but received a number instead."),
                        ));
                        break 'number;
                    }
                    
                    let expected: &OperandType = expected_operands.front().unwrap();

                    match expected {
                        OperandType::Imm | OperandType::RegOrImm => { /* ... */ 
                            println!("NUMBER MATCHED!");
                            self.verify_immediate_value_in_range(curr_ins_token, token);
                        },
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
                        OperandType::Reg | OperandType::RegOrImm => { /* ... */ 
                            println!("REGISTER MATCHED!");
                    },
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
                    
                    let expected = expected_operands.front().unwrap();

                    match expected {
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

        self.verify_all_used_labels_defined();

        if !end_encountered {
            self.errors.push(AsmError::new(
                String::from(CODE_NO_END),
                "",
                0,
                ErrorType::LogicalError,
                "the given file does not contain a `.END` directive. The easiest way to resolve this is to create a new line at the bottom of the file that only contains `.END`.",
            ))
        }
    }

    pub fn orig_at_top(&self, tokens: &Vec<Token>) -> bool {
        match &tokens[0].inner_token {
            TokenType::Directive(directive) => {
                match directive {
                    Directive::ORIG => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_end(&self, directive: &Directive) -> bool {
        match directive {
            Directive::END => true,
            _ => false,
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

    fn verify_all_used_labels_defined(&mut self) {
        for label in self.used_labels.keys() {
            if !self.symbol_table.contains_key(label) {
                self.errors.push(AsmError::from(
                    String::from(CODE_USED_UNDEFINED_LABEL),
                    &self.original_file.get_line(self.used_labels.get(label).unwrap().line_num),
                    self.used_labels.get(label).unwrap().clone(),
                    ErrorType::LabelError,
                    &format!("the label `{}` was never defined within the file.", label),
                ))
            }
        }
    }
    
    fn verify_immediate_value_in_range(&mut self, current_instruction: &Token, value: &Token) {
        let mut maybe_width: Option<i32> = None;

        match &current_instruction.inner_token {
            TokenType::Instruction(opcode_ins) => {
                maybe_width = opcode_ins.get_immediate_value_width();
            },
            TokenType::Directive(_) => {
                return;
            }
            _ => { },
        }
        
        let width: i32 = maybe_width
            .expect("Somehow we are trying to verify that a value is within range when the instruction does not take in a value. THIS SHOULD NOT BE POSSIBLE!");
        
        let mut in_range = false;

        match &value.inner_token {
            TokenType::Number(number) => {
                let number = *number as i32;
                let (lower, upper) = self.get_twos_complement_range(width);
                
                if number < lower || number > upper {
                    self.errors.push(AsmError::from(
                        String::from(CODE_NUMBER_OUT_OF_BOUNDS),
                        &self.original_file.get_line(value.line_num),
                        value.clone(),
                        ErrorType::BoundError,
                        &format!(
                            "the number `{}` (or `{}`) is out of the bounds of `{}`, which takes a(n) {}-bit immediate value. Therefore, the accepted range is `[{}, {}]`
                            REMEMBER: The LC-3 takes only accepts 2's complement values as immediate values.",
                            value.original_match,
                            number,
                            current_instruction.original_match,
                            width,
                            lower,
                            upper,
                        )
                    ));
                }
            },
            TokenType::Label(_label) => {

            },
            _ => {
                unreachable!();
            }
        }
    }

    fn get_twos_complement_range(&self, width: i32) -> (i32, i32) {
        let upper = (width - 1).pow(2);
        let lower = -((width - 1).pow(2) + 1);
        return (lower, upper);
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::{asm_error::*, lexer::*};
    use super::*;

    fn get_semantic_errors(file: &str) -> Vec<AsmError> {
        let mut lexer: Lexer = Lexer::new();
        if !lexer.syntax_checker.is_syntactically_valid_file(file) {
            panic!("COULD NOT SEMANTICALLY VERIFY FILE, BECAUSE IS ISN'T SYNTACTICALLY VALID!!!");
        }

        let tokens = lexer.run(file.to_string());
        if lexer.errors.len() > 0 {
            panic!("COULD NOT SEMANTICALLY VERIFY FILE, BECAUSE ERRORS OCCURRED WHILE GENERATING TOKENS!!!");
        }
        
        for (i, token) in tokens.iter().enumerate() {
            println!("{}\t: {:?}", i, token);
        }
        
        let mut semantic_checker = SemanticChecker::new();
        semantic_checker.run(&tokens, file.to_string());

        return semantic_checker.errors;
    }

    #[test]
    fn test_redefine_label() {
        let file = r#"
.ORIG x3000
name ret
name ret
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_REDEFINED_LABEL);
    }

    #[test]
    fn test_use_undefined_label() {
        let file = r#"
        .ORIG x3000
        LEA R0, undefined_lol
        .END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_USED_UNDEFINED_LABEL);
    }

    #[test]
    fn test_instruction_on_same_line() {
        let file = r#"
.ORIG x3000
ADD R1, R2, RET
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_INS_NO_OPERAND);
    }

    #[test]
    fn test_unexpected_label() {
        let file = r#"
.ORIG x3000
Hello RET
ADD R1, R2, Hello
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_LABEL);
    }
    
    #[test]
    fn test_expected_nothing_but_received_label() {
        let file = r#"
.ORIG x3000
RET hello
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_LABEL);
    }
    
    
    #[test]
    fn test_expected_nothing_but_received_number() {
        let file = r#"
.ORIG x3000
RET #1
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_NUMBER);
    }
    
    #[test]
    fn test_received_unexpected_number() {
        let file = r#"
.ORIG x3000
JSR #1
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_NUMBER);
    }
    
    #[test]
    fn test_expected_nothing_but_received_register() {
        let file = r#"
.ORIG x3000
RET r1
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_REGISTER);
        
        let file = r#"
.ORIG x3000
        BR          begin, r1    ; comment
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_REGISTER);
    }
    
    #[test]
    fn test_received_unexpected_register() {
        let file = r#"
.ORIG x3000
JSR r1
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
            assert_eq!(err.code, String::from(CODE_RECEIVED_UNEXPECTED_REGISTER));
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_REGISTER);
    
    }

    #[test]
    fn test_expected_nothing_but_received_string() {
        let file = r#"
.ORIG x3000
.END "This"
        "#; // strings can only SYNTACTICALLY be given to a directive

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
            assert_eq!(err.code, String::from(CODE_EXPECTED_NOTHING_RECEIVED_STRING));
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_STRING);
    }
    
    #[test]
    fn test_received_unexpected_string() {
        let file = r#"
.ORIG x3000
.FILL "This"
.END
        "#; // strings can only SYNTACTICALLY be given to a directive

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_STRING);
    }
 
    #[test]
    fn test_value_out_bounds_for_add() {
        let file = r#"
.ORIG x3000
ADD R0, R2, x3000
.END
        "#; // strings can only SYNTACTICALLY be given to a directive

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(get_semantic_errors(file).len() > 0);
        assert_eq!(errors[0].code, CODE_NUMBER_OUT_OF_BOUNDS);
    }
}
