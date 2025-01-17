use super::asm_ins::OpcodeIns;
use regex::Regex;


pub enum Token {
    Label(String),
    Instruction(OpcodeIns),
    Directive(String),
    Number(i16),
}



pub struct Lexer {
    br_regex: Regex,
}

impl Lexer {
    pub fn new() -> Lexer {
        let br_regex: Regex = Regex::new(r#"BR[nzp][nzp][nzp]"#).unwrap();
        
        Lexer {
            br_regex: br_regex,
        }
    }


}
