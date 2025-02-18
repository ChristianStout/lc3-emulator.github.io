use super::asm_ins::OpcodeIns;
use regex::Regex;


pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(String),
    Number(i16),
}



pub struct Lexer {
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
            ins_regex: ins_regex,
            dir_regex: dir_regex,
            ignore_regex: ignore_regex,
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::Lexer;

    #[test]
    fn test_br_regex() {
    }
}
