use std::collections::{HashMap, VecDeque};
use super::{asm_error::{AsmError, ErrorType}, asm_ins::{OpcodeIns, OperandType}, directive::Directive, token};
use super::token::*;
use super::file::AsmFile;

const ARCH_LIMIT: i32 = 16;

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
const CODE_ORIG_NOT_GIVEN_NUMBER: &'static str = "SM016";
const CODE_FILE_NOT_VALID: &'static str = "SM017";
const CODE_FILE_EMPTY: &'static str = "SM018";

#[allow(dead_code)]
pub struct SemanticChecker {
    pub symbol_table: HashMap<String, (i32, Token)>,
    pub errors: Vec<AsmError>,
    original_file: AsmFile,
    used_labels: HashMap<String, Token>,
    memory_location: i32,
    in_blkw_directive: bool,

    // refactor items
    expected_operands: VecDeque<OperandType>,
    curr_ins_token: Token,
    end_encountered: bool,
}

#[allow(dead_code)]
impl SemanticChecker {
    pub fn new() -> SemanticChecker {
        SemanticChecker {
            symbol_table: HashMap::new(),
            errors: vec![],
            original_file: AsmFile::new("".to_string()),
            used_labels: HashMap::new(),
            memory_location: 0,
            in_blkw_directive: false,
            expected_operands: VecDeque::new(),
            curr_ins_token: Token::get_useless_token(),
            end_encountered: false,
        }
    }
    
    #[allow(unused_variables)]
    pub fn run(&mut self, tokens: &Vec<Token>, file: String) {
        self.original_file = AsmFile::new(file);
        
        if self.tokens_is_empty(tokens) {
            return;
        }
        
        self.handle_orig(tokens);

        for token in tokens {
            match &token.inner_token {
                TokenType::Instruction(instruction) => {
                    self.handle_instruction(token, instruction);
                }
                TokenType::Directive(directive) => {
                    self.handle_directive(token, directive);
                }
                TokenType::Label(label) => {
                    self.handle_label(token, label);
                },
                TokenType::Number(number) => {
                    self.handle_number(token, number);
                }
                TokenType::Register(_) => {
                    self.handle_register(token);
                }
                TokenType::String(string) => {
                    self.handle_string(token, string);
                }
                _ => {
                    // AsmError::new()
                }
            }
        }

        self.verify_all_used_labels_defined();

        if !self.end_encountered {
            self.errors.push(AsmError::new(
                String::from(CODE_NO_END),
                "",
                0,
                ErrorType::LogicalError,
                "the given file does not contain a `.END` directive. The easiest way to resolve this is to create a new line at the bottom of the file that only contains `.END`.",
            ))
        }
    }

    pub fn handle_instruction(&mut self, token: &Token, instruction: &OpcodeIns) {
        if token.line_num == self.curr_ins_token.line_num {
            self.errors.push(AsmError::from(
                String::from(CODE_INS_NO_OPERAND),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::OperandError,
                "an instruction cannot be an operand. Instructions MUST be separated by line.",
            ));
        }

        if self.expected_operands.len() > 0 {
            self.errors.push(AsmError::from(
                String::from(CODE_RECEIVED_UNEXPECTED_INS),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::OperandError,
                &format!("{} was expected, but received an instruction instead.", self.expected_operands[0].as_string()),
            ));
        }                   
        self.curr_ins_token = token.clone(); // These should be optimized out. In errors they are acceptable, but we should not take a performance hit to valid code.
        self.memory_location += 1;

        self.expected_operands = instruction.get_expected_operands();
    }
    
    pub fn handle_directive(&mut self, token: &Token, directive: &Directive) {
        if self.expected_operands.len() > 0 {
            // Syntactically, it is not possible to get a directive as an argument, so an instruction must have terminated early
            self.errors.push(AsmError::from(
                String::from(CODE_DIR_WRONG_OPERAND),
                &self.original_file.get_line(self.curr_ins_token.line_num),
                self.curr_ins_token.clone(),
                ErrorType::OperandError,
                &format!("{} was expected, was not provided.", self.expected_operands[0].as_string()),
            ));
        }
        if self.is_end(directive) {
            self.end_encountered = true;
        }
        self.curr_ins_token = token.clone();
        // TODO: figure how how I will increment memory for directives.
        // Some do, some don't, .STRINGZ could a little or a lot.
        self.move_memory_location_directive(directive);
        
        self.expected_operands = directive.get_expected_operands();
    }

    pub fn handle_label(&mut self, token: &Token, label: &String) {
        if self.expected_operands.len() == 0 {
            if token.line_num == self.curr_ins_token.line_num {
                self.errors.push(AsmError::from(
                    String::from(CODE_EXPECTED_NOTHING_RECEIVED_LABEL),
                    &self.original_file.get_line(token.line_num),
                    token.clone(),
                    ErrorType::OperandError,
                    "no operands were expected, but received a label instead."
                ));
                return;
            }
            self.define_label(label.clone(), token.clone());
            return;
        }
        
        let expected = self.expected_operands.pop_front().unwrap();

        match expected {
            OperandType::Label => { /* ... */ 
                self.used_labels.insert(token.original_match.clone(), token.clone());
            },
            _ => {
                self.errors.push(AsmError::from(
                    String::from(CODE_RECEIVED_UNEXPECTED_LABEL),
                    &self.original_file.get_line(token.line_num),
                    token.clone(),
                    ErrorType::OperandError,
                    &format!("{} was expected, but received a label instead.", expected.as_string()),
                ));
            }
        }
    }
    
    pub fn handle_number(&mut self, token: &Token, number: &i16) {
            if self.expected_operands.len() == 0 {
            self.errors.push(AsmError::from(
                String::from(CODE_EXPECTED_NOTHING_RECEIVED_NUMBER),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::OperandError,
                &format!("no operands were expected, but received a number instead."),
            ));
            return;
        }
        
        let expected: OperandType = self.expected_operands.pop_front().unwrap();

        match expected {
            OperandType::Imm | OperandType::RegOrImm => {
                self.verify_immediate_value_in_range(token);

                if self.in_blkw_directive {
                    self.in_blkw_directive = false;
                    self.memory_location += *number as i32;
                }
            },
            _ => {
                self.errors.push(AsmError::from(
                    String::from(CODE_RECEIVED_UNEXPECTED_NUMBER),
                    &self.original_file.get_line(token.line_num),
                    token.clone(),
                    ErrorType::OperandError,
                    &format!("{} was expected, but received a number instead.", expected.as_string()),
                ));
            }
        }       
    }

    pub fn handle_register(&mut self, token: &Token) {
            if self.expected_operands.len() == 0 {
            self.errors.push(AsmError::from(
                String::from(CODE_EXPECTED_NOTHING_RECEIVED_REGISTER),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::OperandError,
                &format!("no operands were expected, but received a register instead."),
            ));
            return;
        }
        
        let expected = self.expected_operands.pop_front().unwrap();
        match expected {
            OperandType::Reg | OperandType::RegOrImm => { /* ... */ },
            _ => {
                self.errors.push(AsmError::from(
                    String::from(CODE_RECEIVED_UNEXPECTED_REGISTER),
                    &self.original_file.get_line(token.line_num),
                    token.clone(),
                    ErrorType::OperandError,
                    &format!("{} was expected, but received a register instead.", expected.as_string()),
                ));
            }
        }
    }

    pub fn handle_string(&mut self, token: &Token, string: &String) {
        if self.expected_operands.len() == 0 {
            self.errors.push(AsmError::from(
                String::from(CODE_EXPECTED_NOTHING_RECEIVED_STRING),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::OperandError,
                &format!("no operands were expected, but received a string instead."),
            ));
            return;
        }
        
        let expected = self.expected_operands.pop_front().unwrap();

        match expected {
            OperandType::String => {
                self.memory_location += string.len() as i32;
            },
            _ => {
                self.errors.push(AsmError::from(
                    String::from(CODE_RECEIVED_UNEXPECTED_STRING),
                    &self.original_file.get_line(token.line_num),
                    token.clone(),
                    ErrorType::OperandError,
                    &format!("{} was expected, but received a string instead.", expected.as_string()),
                ));
            }
        }
    }

    pub fn tokens_is_empty(&mut self, tokens: &Vec<Token>) -> bool {
        if tokens.len() == 0 {
            self.errors.push(AsmError::new(
                String::from(CODE_FILE_EMPTY),
                "",
                0,
                ErrorType::LogicalError,
                "The provided file was empty.",
            ));
            return true;
        }

        return false;
    }

    pub fn handle_orig(&mut self, tokens: &Vec<Token>) {
        // TODO: handle if orig contains a label

        if !self.orig_at_top(&tokens) {
            self.errors.push(AsmError::from(
                String::from(CODE_NO_ORIG),
                &self.original_file.get_line(tokens[0].line_num),
                tokens[0].clone(),
                ErrorType::LogicalError,
                "the `.ORIG` directive must be at the top of the file. To resolve this error, add `.ORIG x3000` at the top of the file.",
            ));
            return;
        } else if tokens.len() > 1 {
            self.set_memory_orig(&tokens);
        } else {
            self.errors.push(AsmError::new(
                String::from(CODE_FILE_NOT_VALID),
                &self.original_file.get_line(tokens[0].line_num),
                tokens[0].line_num as i32,
                ErrorType::LogicalError,
                "The provided file is not valid, because it only contains a `.ORIG` directive without arguments, and no `.END` directive",
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

    pub fn set_memory_orig(&mut self, tokens: &Vec<Token>) {
        match tokens[1].inner_token {
            TokenType::Number(location) => {
                self.memory_location = location as i32;
            },
            _ => {
                AsmError::from(
                    String::from(CODE_ORIG_NOT_GIVEN_NUMBER),
                    &self.original_file.get_line(tokens[1].line_num),
                    tokens[1].clone(),
                    ErrorType::OperandError,
                    &format!("{} must be given a number as an immediate value", tokens[1].original_match),
                );
            }
        }
        
    }

    pub fn is_end(&self, directive: &Directive) -> bool {
        match directive {
            Directive::END => true,
            _ => false,
        }
    }

    pub fn move_memory_location_directive(&mut self, directive: &Directive) {
        match directive {
            Directive::FILL => {
                self.memory_location += 1;
            },
            Directive::BLKW => {
                // Unfortunately, the number token will have to handle this, since we cannot have clairvoyance.
                self.in_blkw_directive = true;
            },
            Directive::STRINGZ => {
                // Strings can handle themselves, since this is the only syntactically valid position
                // for a string in LC-3 assembly
            }
            _ => {

            }
        }
    }

    pub fn define_label(&mut self, label: String, token: Token) {
        if self.symbol_table.contains_key(&label) {
            let (_, other) = self.symbol_table.get(&label).unwrap();
            self.errors.push(AsmError::from(
                String::from(CODE_REDEFINED_LABEL),
                &self.original_file.get_line(token.line_num),
                token.clone(),
                ErrorType::LabelError,
                &format!("attempted to redefine a label that was already defined on line {}", other.line_num),
            ));
            return;
        }
        self.symbol_table.insert(label, (self.memory_location, token));
    }

    fn verify_all_used_labels_defined(&mut self) {
        println!("USED LABELS = {:?}", self.used_labels);
        println!("\n\nDEFINED LABELS = {:?}", self.symbol_table);
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
    
    fn verify_immediate_value_in_range(&mut self, value: &Token) {
        let width: i32;

        match &self.curr_ins_token.inner_token {
            TokenType::Instruction(opcode_ins) => {
                width = opcode_ins.get_immediate_value_width()
                    .expect("Somehow we are trying to verify that a value is within range when the instruction does not take in a value. THIS SHOULD NOT BE POSSIBLE!");
            },
            TokenType::Directive(_) => {
                width = ARCH_LIMIT; // This is because directives only store information in memory. They don't have limits, other than architecture.
            }
            _ => {
                panic!("semantic::SemanticChecker::verify_value_in_range(): A non-instruction/directive was given as a token that can take an immediate value");
            },
        }
         
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
                            self.curr_ins_token.original_match,
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
        let upper = 2_i32.pow(width as u32 - 1) - 1;
        let lower = -(2_i32.pow(width as u32 - 1));
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

    fn get_symbol_table(file: &str) -> HashMap<String, (i32, Token)> {
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

        if semantic_checker.errors.len() > 0 {
            panic!("COULD NOT PROVIDE A RELIABLE SYMBOL TABLE, BECAUSE THE FILE PROVIDED IS NOT SEMANTICALLY VALID!!!");
        }

        return semantic_checker.symbol_table;
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
Hello .FILL #0
RET hello
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
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

        assert!(errors.len() > 0);
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

        assert!(errors.len() > 0);
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

        assert!(errors.len() > 0);
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

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_REGISTER);
    
    }

    #[test]
    fn test_expected_nothing_but_received_string() {
        let file = r#"
.ORIG x3000
.END "This"
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
            assert_eq!(err.code, String::from(CODE_EXPECTED_NOTHING_RECEIVED_STRING));
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_EXPECTED_NOTHING_RECEIVED_STRING);
    }
    
    #[test]
    fn test_received_unexpected_string() {
        let file = r#"
.ORIG x3000
.FILL "This"
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_RECEIVED_UNEXPECTED_STRING);
    }
 
    #[test]
    fn test_value_out_bounds_for_add() {
        let file = r#"
.ORIG x3000
ADD R0, R2, x3000
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_NUMBER_OUT_OF_BOUNDS);
    let file = r#"
.ORIG x3000
ADD R0, R2, #16
.END
        "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_NUMBER_OUT_OF_BOUNDS)
    }

    #[test]
    fn test_get_twos_complement_range() {
        let sm = SemanticChecker::new();

        assert_eq!(sm.get_twos_complement_range(16), (i16::MIN as i32, i16::MAX as i32));
    }

    #[test]
    fn test_empty_tokens() {
        let file = r#"
; I'M JUST EXISTING OKAY! WHAT'S YOUR PROBLEM!!!??? D:<
            "#;

        let errors: Vec<AsmError> = get_semantic_errors(file);

        for err in errors.iter() {
            println!("{}", err.generate_msg());
        }

        assert!(errors.len() > 0);
        assert_eq!(errors[0].code, CODE_FILE_EMPTY);
    }

    #[test]
    fn test_label_symbol_table_only_labels() {
        let file = r#"
.orig #3000
start   add r1, r1, r1
other   add r1, r1, r1
maybe   add r1, r1, r1
.end
            "#;

        let st: HashMap<String, (i32, Token)> = get_symbol_table(file);
        
        assert_eq!(st.len(), 3);
        
        let (location, _) = st.get("start").unwrap();
        assert_eq!(*location, 3000);
        
        let (location, _) = st.get("other").unwrap();
        assert_eq!(*location, 3001);
        
        let (location, _) = st.get("maybe").unwrap();
        assert_eq!(*location, 3002);
    }

    #[test]
    fn test_label_symbol_table_with_string() {
        let file = r#"
.orig #3000
start       add r1, r1, r1

.stringz    "length=8"

other       add r1, r1, r1

hello .stringz "hello" ; length = 5

maybe       add r1, r1, r1
.end
            "#;

        let st: HashMap<String, (i32, Token)> = get_symbol_table(file);
        
        assert_eq!(st.len(), 4);
        
        let (location, _) = st.get("start").unwrap();
        assert_eq!(*location, 3000);
        
        let (location, _) = st.get("other").unwrap();
        assert_eq!(*location, 3009); // it should account for the 8-long string between `start` and `other`
        
        let (location, _) = st.get("hello").unwrap();
        assert_eq!(*location, 3010); // the difference is only one because the string starts in the following memory location
        
        let (location, _) = st.get("maybe").unwrap();
        assert_eq!(*location, 3015);
    }
    
    #[test]
    fn test_label_symbol_table_with_blkw() {
        let file = r#"
.orig #3000
start       add r1, r1, r1

.blkw       #8

other       add r1, r1, r1

hello       .blkw   #5

maybe       add r1, r1, r1
.end
            "#;

        let st: HashMap<String, (i32, Token)> = get_symbol_table(file);
        
        assert_eq!(st.len(), 4);
        
        let (location, _) = st.get("start").unwrap();
        assert_eq!(*location, 3000);
        
        let (location, _) = st.get("other").unwrap();
        assert_eq!(*location, 3009); // it should account for the 8-long block between `start` and `other`
        
        let (location, _) = st.get("hello").unwrap();
        assert_eq!(*location, 3010); // the difference is only one because the block starts in the following memory location
        
        let (location, _) = st.get("maybe").unwrap();
        assert_eq!(*location, 3015);
    }
    
    #[test]
    fn test_label_symbol_table_with_fill() {
        let file = r#"
.orig #3000
start       add r1, r1, r1

.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042

other       add r1, r1, r1

hello .fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042
.fill       x0042

maybe       add r1, r1, r1
.end
            "#;

        let st: HashMap<String, (i32, Token)> = get_symbol_table(file);
        
        assert_eq!(st.len(), 4);
        
        let (location, _) = st.get("start").unwrap();
        assert_eq!(*location, 3000);
        
        let (location, _) = st.get("other").unwrap();
        assert_eq!(*location, 3009); // it should account for the 8-long block between `start` and `other`
        
        let (location, _) = st.get("hello").unwrap();
        assert_eq!(*location, 3010); // the difference is only one because the block starts in the following memory location
        
        let (location, _) = st.get("maybe").unwrap();
        assert_eq!(*location, 3015);
    }
}
