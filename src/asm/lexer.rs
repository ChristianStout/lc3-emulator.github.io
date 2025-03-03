use crate::asm::token;

use super::asm_ins::*;
use super::directive::*;
use super::syntax::SyntaxChecker;
use super::asm_error::*;
use super::token::*;

pub struct Lexer {
    pub token_stream: Vec<Token>,
    pub errors: Vec<AsmError>,
    syntax_checker: SyntaxChecker,
    curr_file: String,
    file_as_chars: Vec<char>,
    curr_line_num: i32,
    position: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_stream: vec![],
            errors: vec![],
            syntax_checker: SyntaxChecker::new(),
            curr_file: String::new(),
            file_as_chars: vec![],
            curr_line_num: 0,
            position: 0,
        }
    }

    pub fn run(&mut self, input_file: String) -> Vec<Token> {
        self.file_as_chars = input_file.chars().collect();
        self.curr_file = input_file;

        let mut word_buffer: Vec<char> = vec![];
        let mut c: char;
            

        while self.position < self.file_as_chars.len() {
            c = self.next_char(); // iterates self.position

            if c == '\"' {
                let string = self.parse_string();
                self.token_stream.push(Token::String(string));
                continue;
            }

            if c == '\n' {
                self.curr_line_num += 1;
            }

            if c == ';' {
                self.skip_comment();
                continue;
            }

            if (c.is_whitespace() || c == ';') && word_buffer.len() > 0 {
                self.parse_word(word_buffer.iter().collect());
                word_buffer.clear();
                continue;
            }

            if c.is_whitespace() && word_buffer.len() == 0 {
                continue;
            }

            word_buffer.push(c);
            println!("word_buffer: {}", word_buffer.iter().collect::<String>());
        }

        return self.token_stream.clone(); // TODO: Remove clone()
    }

    fn next_char(&mut self) -> char {
        let c: char = self.file_as_chars[self.position];
        self.position += 1;
        return c;
    }

    fn get_current_line(&mut self) -> String {
        let current_line_number = self.curr_line_num.clone() as usize;
        let split_file: Vec<&str> = self.curr_file.split_whitespace().collect();
        return String::from(split_file[current_line_number]);
    }

    fn skip_comment(&mut self) {
        while self.next_char() != '\n' {
            // ...
        }
    }

    pub fn parse_word(&mut self, word: String) {
        // parse hierarchy
        let upper = word.to_uppercase();

        println!("{}, len = {}", upper, upper.len());

        if self.syntax_checker.is_ignore(&upper) {
            return;
        }
        else if self.syntax_checker.is_instruction_name(&word) {
            self.token_stream.push(Token::Instruction(OpcodeIns::from(&upper)));
            return;
        }
        else if self.syntax_checker.is_directive_name(&upper) {
            self.token_stream.push(Token::Directive(Directive::from(&upper)));
            return;
        }
        else if self.syntax_checker.is_valid_register(&upper) {
            self.token_stream.push(Token::Register(self.parse_register(&upper)));
            return;
        }
        else if self.syntax_checker.is_valid_immediate_value(&upper) {
            self.token_stream.push(Token::Number(self.parse_immediate_value(&upper)));
            return;
        }
        else if self.syntax_checker.is_valid_label(&word) {
            self.token_stream.push(Token::Label(word.to_string()));
            return;
        }
        else {
            self.token_stream.push(Token::INVALID(word));
            let line = self.get_current_line();
            self.errors.push(AsmError::new(
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
            .expect(&format!("Lexer::parse_register: When converting the register value on line {}, could not conver value into base 10 number.", self.curr_line_num));
        
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
        println!("Reached here");
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
    fn test_hex_overflow() {
        let file = String::from("NOT_TOO_BIG  .FILL   xFFF6\nEVEN_THIS .FILL xFFFF");

        let mut lexer = Lexer::new();
        let tokens = lexer.run(file.split_ascii_whitespace().collect());

        assert!(tokens[2] == Token::Number(65526_u16 as i16));
        assert!(tokens[5] == Token::Number(65535_u16 as i16));
    }

    #[test]
    fn test_strings() {
        let file = String::from(r#".STRINGZ "Hello, World!"  "#);

        let mut lexer = Lexer::new();
        let tokens = lexer.run(file.split_ascii_whitespace().collect());

        println!("TOKENS: {:?}", tokens);

        assert!(tokens == vec![Token::Directive(Directive::STRINGZ), Token::String("Hello, World!".to_string())]);
        // assert!(tokens[5] == Token::Number(65535_u16 as i16));
    }
}
