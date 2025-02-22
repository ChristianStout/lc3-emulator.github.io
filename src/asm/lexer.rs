use super::asm_ins::*;
use regex::Regex;


pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(Directive),
    Number(i16),
}



pub struct Lexer {
    token_buffer: Vec<Token>,
    ins_regex: Regex,
    dir_regex: Regex,
    ignore_regex: Regex,
}

impl Lexer {
    pub fn new() -> Lexer {
        let ins_regex: Regex = Regex::new(r#"([A-Za-z_][A-Za-z0-9_]*\s)?(\s)*[A-Z]+(\s)*(\s([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC))?)?)?(\s)*(;.*)?[\n|\r|\n\r]"#).unwrap();
        let dir_regex: Regex = Regex::new(r#"([A-Za-z][A-Za-z0-9]*\s)?(\s)*[.][A-Za-z0-9]*(\s)+(x[0-9]+|["].+["]|)?(\s)?(;.*)?[\n|\r|\n\r]"#).unwrap();
        let ignore_regex: Regex = Regex::new(r#"(\s)*(;.*)?[\n|\r|\n\r]"#).unwrap();

        Lexer {
            token_buffer: vec![],
            ins_regex: ins_regex,
            dir_regex: dir_regex,
            ignore_regex: ignore_regex,
        }
    }

    pub fn run(&mut self, input_file: Vec<&str>) -> Vec<Token>{
        for (num, line) in input_file.into_iter().enumerate() {
            if self.ins_regex.is_match(line) {
                self.parse_instruction(line);
            }
            if self.dir_regex.is_match(line) {
                self.parse_directive(line);
            }
            if !self.ignore_regex.is_match(line) {
                panic!("Panic in Lexer::run(). Somehow a line was not an intruction, directive, or ignorable.")
            }
        }
        vec![]
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
