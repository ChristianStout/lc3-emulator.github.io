use super::asm_ins::*;
use super::syntax::SyntaxChecker;
use regex::Regex;



pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
}



pub struct Lexer {
    token_buffer: Vec<Token>,
    syntax_checker: SyntaxChecker,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_buffer: vec![],
            syntax_checker: SyntaxChecker::new(),
        }
    }

    pub fn run(&mut self, input_file: Vec<&str>) -> Vec<Token> {
        for (num, line) in input_file.into_iter().enumerate() {
            if self.syntax_checker.is_ins(line) {
                self.parse_instruction(line);
            }
            if self.syntax_checker.is_dir(line) {
                self.parse_directive(line);
            }
            if !self.syntax_checker.is_ignore(line) {
                // TODO: Add Error struct with a `discover_error` function
                //      that takes in any known information. This should also
                //      be able to display EVERY current error, so a vector.
                panic!("SYNTAX ERROR: On line {num}, neither an instruction nor directive was given.")
            }
        }
        return vec![];
    }

    pub fn parse_instruction(&mut self, line: &str) {
        let split_line: Vec<&str> = line.split_whitespace().collect();
    }

    pub fn parse_directive(&mut self, line: &str) {
        let split_line: Vec<&str> = line.split_whitespace().collect();
    }
}

#[cfg(test)]
mod tests {
    // use super::Lexer;

    #[test]
    fn test_br_regex() {
    }
}
