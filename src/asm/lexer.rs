use super::asm_ins::*;
use super::directive::*;
use super::syntax::SyntaxChecker;
use super::asm_error::*;
use super::token::*;

const CODE_TOKEN_NO_CATEGORY: &'static str = "SX001";
const CODE_STRING_NOT_ENDED: &'static str = "SX002";
const CODE_INVALID_ESCAPE_CHAR: &'static str = "SX003";

pub struct Lexer {
    pub token_stream: Vec<Token>,
    pub errors: Vec<AsmError>,
    pub syntax_checker: SyntaxChecker,
    curr_file: String,
    file_as_chars: Vec<char>,
    curr_line_num: i32,
    file_length: usize,
    file_position: usize,
    line_position: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_stream: vec![],
            errors: vec![],
            syntax_checker: SyntaxChecker::new(),
            curr_file: String::new(),
            file_as_chars: vec![],
            curr_line_num: 1,
            file_length: 0,
            file_position: 0,
            line_position: 0,
        }
    }

    pub fn run(&mut self, mut input_file: String) -> Vec<Token> {
        input_file.push(' ');
        self.file_length = input_file.len();
        self.file_as_chars = input_file.chars().collect();
        self.curr_file = input_file;
        self.errors = vec![];

        let mut word_buffer: Vec<char> = vec![];
        let mut c: char;
            

        while self.file_position < self.file_as_chars.len() {
            c = self.next_char(); // iterates self.file_position, self.line_position

            if c == '\"' {
                let string = self.parse_string();
                self.token_stream.push(Token::new(
                    self.file_position,
                    self.line_position,
                    self.curr_line_num,
                    &format!(r#""{}""#, string),
                    TokenType::String(string),
                ));
                continue;
            }

            if (c.is_whitespace() || c == ';' || c == ',') && word_buffer.len() > 0 {
                self.parse_word(word_buffer.iter().collect());
                word_buffer.clear();
                if c == '\n' {
                    self.next_line();
                }
                if c == ';' {
                    self.skip_comment();
                }
                continue;
            }

            if c == '\n' {
                self.next_line();
            }

            if c == ';' {
                self.skip_comment();
                continue;
            }

            if c.is_whitespace() && word_buffer.len() == 0 {
                continue;
            }

            word_buffer.push(c);
        }

        let tokens = self.token_stream.clone(); // TODO: Remove clone()

        self.reset();

        return tokens;
    }

    fn next_char(&mut self) -> char {
        let c: char = self.file_as_chars[self.file_position];
        self.file_position += 1;
        self.line_position += 1;
        return c;
    }
    
    fn next_line(&mut self) {
        self.line_position = 0;
        // self.file_position += 1;
        self.curr_line_num += 1;
    }

    fn get_current_line(&mut self) -> String {
        let current_line_number = self.curr_line_num.clone() as usize;
        let split_file: Vec<&str> = self.curr_file.split_whitespace().collect();
        return String::from(split_file[current_line_number]);
    }

    fn skip_comment(&mut self) {
        while self.next_char() != '\n' {
            if self.file_position == self.file_length {
                return;
            }
        }
        self.next_line();
    }

    fn reset(&mut self) {
        self.token_stream = vec![];
        self.syntax_checker = SyntaxChecker::new();
        self.curr_file = String::new();
        self.file_as_chars = vec![];
        self.curr_line_num = 1;
        self.file_length = 0;
        self.file_position = 0;
        self.line_position = 0;
    }

    pub fn parse_word(&mut self, word: String) {
        // parse hierarchy
        let upper = word.to_ascii_uppercase();

        if self.syntax_checker.is_ignore(&upper) {
            return;
        }
        else if self.syntax_checker.is_instruction_name(&upper) {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word,
                TokenType::Instruction(OpcodeIns::from(&upper))
            ));
            return;
        }
        else if self.syntax_checker.is_directive_name(&upper) {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word,
                TokenType::Directive(Directive::from(&upper))
            ));
            return;
        }
        else if self.syntax_checker.is_valid_register(&upper) {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word,
                TokenType::Register(self.parse_register(&upper))
            ));
            return;
        }
        else if self.syntax_checker.is_valid_immediate_value(&word) {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word,
                TokenType::Number(self.parse_immediate_value(&word))
            ));
            return;
        }
        else if self.syntax_checker.is_valid_label(&word) {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word,
                TokenType::Label(word.to_string())
            ));
            return;
        }
        else {
            self.token_stream.push(Token::new(
                self.file_position,
                self.line_position,
                self.curr_line_num,
                &word.clone(),
                TokenType::INVALID(word)
            ));
            let line = self.get_current_line();
            self.errors.push(AsmError::new(
                String::from(CODE_TOKEN_NO_CATEGORY),
                &line,
                self.curr_line_num,
                ErrorType::SyntaxError,
                "a token could not be categorized"
            ))
        }
    }

    pub fn parse_register(&self, word: &str) -> u16 {
        let base = 10;

        let register_num = word
            .chars()
            .nth(1) // Always R1, R2, R3, ... , R7.
            .expect("Lexer::parse_register: Somehow a register was given without a number. This shouldn't be possible given the Regex.")
            .to_digit(base) // a base 10 number
            .expect(&format!("Lexer::parse_register: When converting the register value on line {}, could not convert value into base 10 number.", self.curr_line_num));
        
        return register_num as u16;
    }
    
    pub fn parse_immediate_value(&self, word: &str) -> i16 {
        match word.chars().nth(0).unwrap() {
            '#' => {
                return word[1..]
                    .parse()
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num));
            },
            'x' | 'X' => {
                let base = 16;
                return u16::from_str_radix(&word[1..], base)
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num)) as i16;
            },
            _ => unreachable!(),
        }
    }

    #[allow(while_true)]
    pub fn parse_string(&mut self) -> String {
        let mut str_buffer: Vec<char> = vec![];
        let mut is_escape = false;
        let mut c: char;

        while true {
            c = self.next_char();

            if is_escape {
                str_buffer.push(self.parse_escape(c));
                is_escape = false;
                continue;
            }

            if c == '\"' {
                break;
            }
            
            if c == '\\' {
                is_escape = true;
                continue;
            }

            if c == '\n' {
                let curr_line = self.get_current_line();
                self.errors.push(AsmError::new(
                    String::from(CODE_STRING_NOT_ENDED),
                    &curr_line,
                    self.curr_line_num,
                    ErrorType::SyntaxError,
                    "the given string was not terminated",
                ));
                break;
            }

            str_buffer.push(c);
        }

        return str_buffer.iter().collect();
    }

     pub fn parse_escape(&mut self, character: char) -> char {
        match character {
            '\\' | '\'' |'\"' => return character,
            'n' => return '\n',
            'r' => return '\r',
            't' => return '\t',
            '0' => return '\0',
            _ => {
                let line = self.get_current_line();
                let line_number = self.curr_line_num;
                self.errors.push(AsmError::new(
                    String::from(CODE_INVALID_ESCAPE_CHAR),
                    &line,
                    line_number,
                    ErrorType::SyntaxError,
                    &format!("the given escape character `\\{}` does not exist.", character)
                ));
            },
        }
        '\0'
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_br_nzp() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" BR "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(false, false, false))
        );
        assert_eq!(
            lexer.run(String::from(" BRn "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(true, false, false))
        );
        assert_eq!(
            lexer.run(String::from(" BRz "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(false, true, false))
        );
        assert_eq!(
            lexer.run(String::from(" BRp "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(false, false, true))
        );
        assert_eq!(
            lexer.run(String::from(" BRnz "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(true, true, false))
        );
        assert_eq!(
            lexer.run(String::from(" BRnp "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(true, false, true))
        );
        assert_eq!(
            lexer.run(String::from(" BRzp "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(false, true, true))
        );
        assert_eq!(
            lexer.run(String::from(" BRnzp "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Br(true, true, true))
        );
    }

    #[test]
    fn test_instructions() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" ADD "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Add)
        );
        assert_eq!(
            lexer.run(String::from(" AND "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::And)
        );
        assert_eq!(
            lexer.run(String::from(" JMP "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Jmp)
        );
        assert_eq!(
            lexer.run(String::from(" JSR "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Jsr)
        );
        assert_eq!(
            lexer.run(String::from(" Jsrr "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Jsrr)
        );
        assert_eq!(
            lexer.run(String::from(" LD "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Ld)
        );
        assert_eq!(
            lexer.run(String::from(" LDI "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Ldi)
        );
        assert_eq!(
            lexer.run(String::from(" LDR "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Ldr)
        );
        assert_eq!(
            lexer.run(String::from(" LEA "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Lea)
        );
        assert_eq!(
            lexer.run(String::from(" NOT "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Not)
        );
        assert_eq!(
            lexer.run(String::from(" RET "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Ret)
        );
        assert_eq!(
            lexer.run(String::from(" RTI "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Rti)
        );
        assert_eq!(
            lexer.run(String::from(" ST "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::St)
        );
        assert_eq!(
            lexer.run(String::from(" STI "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Sti)
        );
        assert_eq!(
            lexer.run(String::from(" STR "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Str)
        );
        assert_eq!(
            lexer.run(String::from(" GETC "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(20))
        );
        assert_eq!(
            lexer.run(String::from(" OUT "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(21))
        );
        assert_eq!(
            lexer.run(String::from(" PUTS "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(22))
        );
        assert_eq!(
            lexer.run(String::from(" IN "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(23))
        );
        assert_eq!(
            lexer.run(String::from(" HALT "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(25))
        );


        assert_ne!(
            lexer.run(String::from(" HALTS "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Trap(25))
        );
        assert_ne!(
            lexer.run(String::from(" ADDI "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Add)
        );
        assert_ne!(
            lexer.run(String::from(" ANDY "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::And)
        );
        assert_ne!(
            lexer.run(String::from(" JSRRR "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Jsrr)
        );
        assert_ne!(
            lexer.run(String::from(" badd "))[0].inner_token,
            TokenType::Instruction(OpcodeIns::Add)
        );
    }

    #[test]
    fn test_registers() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" R0 "))[0].inner_token,
            TokenType::Register(0)
        );
        assert_eq!(
            lexer.run(String::from(" R1 "))[0].inner_token,
            TokenType::Register(1)
        );
        assert_eq!(
            lexer.run(String::from(" R2 "))[0].inner_token,
            TokenType::Register(2)
        );
        assert_eq!(
            lexer.run(String::from(" R3 "))[0].inner_token,
            TokenType::Register(3)
        );
        assert_eq!(
            lexer.run(String::from(" R4 "))[0].inner_token,
            TokenType::Register(4)
        );
        assert_eq!(
            lexer.run(String::from(" R5 "))[0].inner_token,
            TokenType::Register(5)
        );
        assert_eq!(
            lexer.run(String::from(" R6 "))[0].inner_token,
            TokenType::Register(6)
        );
        assert_eq!(
            lexer.run(String::from(" R7 "))[0].inner_token,
            TokenType::Register(7)
        );


        assert_ne!(
            lexer.run(String::from(" R8 "))[0].inner_token,
            TokenType::Register(8)
        );
        assert_ne!(
            lexer.run(String::from(" RR1 "))[0].inner_token,
            TokenType::Register(1)
        );
        assert_ne!(
            lexer.run(String::from(" GO_TO_R1 "))[0].inner_token,
            TokenType::Register(1)
        );
        assert_ne!(
            lexer.run(String::from(" R1_FOREVER_IN_MY_HEART "))[0].inner_token,
            TokenType::Register(1)
        );
        assert_ne!(
            lexer.run(String::from(" okay_R1_your_right "))[0].inner_token,
            TokenType::Register(1)
        );
    }

    #[test]
    fn test_directives() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" .ORIG "))[0].inner_token,
            TokenType::Directive(Directive::ORIG)
        );
        assert_eq!(
            lexer.run(String::from(" .FILL "))[0].inner_token,
            TokenType::Directive(Directive::FILL)
        );
        assert_eq!(
            lexer.run(String::from(" .BLKW "))[0].inner_token,
            TokenType::Directive(Directive::BLKW)
        );
        assert_eq!(
            lexer.run(String::from(" .STRINGZ "))[0].inner_token,
            TokenType::Directive(Directive::STRINGZ)
        );
        assert_eq!(
            lexer.run(String::from(" .END "))[0].inner_token,
            TokenType::Directive(Directive::END)
        );
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" #1 "))[0].inner_token,
            TokenType::Number(1)
        );
        assert_eq!(
            lexer.run(String::from(" #2 "))[0].inner_token,
            TokenType::Number(2)
        );
        assert_eq!(
            lexer.run(String::from(" #3 "))[0].inner_token,
            TokenType::Number(3)
        );
        assert_eq!(
            lexer.run(String::from(" #128 "))[0].inner_token,
            TokenType::Number(128)
        );
        assert_eq!(
            lexer.run(String::from(" #-128 "))[0].inner_token,
            TokenType::Number(-128)
        );

        assert_eq!(
            lexer.run(String::from(" x0000 "))[0].inner_token,
            TokenType::Number(0)
        );
        assert_eq!(
            lexer.run(String::from(" x1 "))[0].inner_token,
            TokenType::Number(1)
        );
        assert_eq!(
            lexer.run(String::from(" x0001 "))[0].inner_token,
            TokenType::Number(1)
        );
        assert_eq!(
            lexer.run(String::from(" xFFFF "))[0].inner_token,
            TokenType::Number(-1)
        );
    }

    // #[test]
    // fn test_strings() {
    //     let file = String::from(r#".STRINGZ "Hello, World!"  "#);

    //     let mut lexer = Lexer::new();
    // }
    
    // #[test]
    // fn test_comments() {
    //     let mut lexer = Lexer::new();


    //     assert_eq!(
    //         lexer.run(String::from("LA;"))[0].inner_token,
    //         TokenType::Label(String::from("LA"))
    //     )
    // }
    
    #[test]
    fn test_labels() {
        // todo!();
    }

    #[test]
    fn test_case_sensitivity() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" this "))[0].inner_token,
            TokenType::Label("this".to_string())
        );
        assert_eq!(
            lexer.run(String::from(" THIS "))[0].inner_token,
            TokenType::Label("THIS".to_string())
        );
        assert_ne!(
            lexer.run(String::from(" this "))[0].inner_token,
            TokenType::Label("THIS".to_string())
        );


        assert_eq!(
            lexer.run(String::from(" halt "))[0].inner_token,
            lexer.run(String::from(" HALT "))[0].inner_token
        );
        assert_eq!(
            lexer.run(String::from(" and "))[0].inner_token,
            lexer.run(String::from(" AND "))[0].inner_token
        );
        assert_eq!(
            lexer.run(String::from(" jsrR "))[0].inner_token,
            lexer.run(String::from(" JSRr "))[0].inner_token
        );
        assert_eq!(
            lexer.run(String::from(" LEA "))[0].inner_token,
            lexer.run(String::from(" lea "))[0].inner_token
        );
        assert_eq!(
            lexer.run(String::from(" GeTc "))[0].inner_token,
            lexer.run(String::from(" gEtC "))[0].inner_token
        );


        assert_ne!(
            lexer.run(String::from(r#" "string" "#))[0].inner_token,
            lexer.run(String::from(r#" "STRING" "#))[0].inner_token
        );

        assert_eq!(
            lexer.run(String::from(" r1 "))[0].inner_token,
            lexer.run(String::from(" R1 "))[0].inner_token
        );

        assert_eq!(
            lexer.run(String::from(" .end "))[0].inner_token,
            lexer.run(String::from(" .END "))[0].inner_token
        );
    }

    #[test]
    fn test_hex_overflow() {
        // let file = String::from("NOT_TOO_BIG  .FILL   xFFF6 \n EVEN_THIS .FILL xFFFF");

        let mut lexer = Lexer::new();
        // let tokens = lexer.run(file);

        // assert!(tokens[2] == TokenType::Number(-10));
        // assert!(tokens[5] == TokenType::Number(-1));

        assert_eq!(
            lexer.run(String::from(" xFFF6 "))[0].inner_token,
            TokenType::Number(-10)
        );
        assert_eq!(
            lexer.run(String::from(" xFFFF "))[0].inner_token,
            TokenType::Number(-1)
        );
    }

    #[test]
    fn test_commas() {
        // todo!()
    }

    #[test]
    fn test_no_following_whitespace() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from("LA"))[0].inner_token,
            TokenType::Label(String::from("LA")),
        )
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from("LA;"))[0].inner_token,
            TokenType::Label(String::from("LA"))
        )
    }

    #[test]
    fn test_invalid_escape_char() {
        let mut lexer = Lexer::new();

        let _ = lexer.run(String::from(r#" "hello \." "#));
        
        assert_eq!(lexer.errors[0].code, String::from(CODE_INVALID_ESCAPE_CHAR));
    }
    
    #[test]
    fn test_token_after_comment_line_is_captured() {
        let mut lexer = Lexer::new();
        
        // importantly, the bug only occured when there was no whitespace 
        // before the next token after a comment
        let tokens = lexer.run(String::from(r#"
; This
MAYBE_HERE
        "#));

    assert!(tokens[0].inner_token == TokenType::Label(String::from("MAYBE_HERE")));

    }
}
