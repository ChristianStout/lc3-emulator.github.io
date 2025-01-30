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
    // instuction_regex: Regex,
}

impl Lexer {
    pub fn new() -> Lexer {
        let br_regex: Regex = Regex::new(r#"^BR[n]?[z]?[p]?"#).unwrap();
        
        Lexer {
            br_regex: br_regex,
            // instuction_regex:
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn test_br_regex() {
        let lexer = Lexer::new();

        assert!(lexer.br_regex.is_match("BR"));
        assert!(lexer.br_regex.is_match("BRn"));
        assert!(lexer.br_regex.is_match("BRz"));
        assert!(lexer.br_regex.is_match("BRnz"));
        assert!(lexer.br_regex.is_match("BRnp"));
        assert!(lexer.br_regex.is_match("BRpz"));
        assert!(lexer.br_regex.is_match("BRnzp"));
        
        // assert!(!lexer.br_regex.is_match("BRx"));
        assert!(!lexer.br_regex.is_match("B R"));
        // assert!(!lexer.br_regex.is_match("BRR"));
    }
}
