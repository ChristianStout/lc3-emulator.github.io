use super::lexer::*;
use super::semantic::*;
use crate::output::*;

#[allow(dead_code)]
pub struct Asm {
    lexer: Lexer,
    semantic_checker: SemanticChecker,
}

#[allow(dead_code)]
impl Asm {
    pub fn new() -> Asm {
        Asm {
            lexer: Lexer::new(),
            semantic_checker: SemanticChecker::new(),
        }
    }

    pub fn run(&mut self, input_file: String) -> Vec<u16> {
        // 1. Verify that file is syntactically valid
        // 2. Create token stream with Lexer
        // 3. Verify that file is semantically valid
        // 4. Assemble Vec<Token> into binary Vec<u16> & Symbol Table

        if !self.lexer.syntax_checker.is_syntactically_valid_file(&input_file) {
            return vec![];
        }

        let tokens = self.lexer.run(input_file.clone());
        
        if self.lexer.errors.len() > 0 {
            // let io = &Box::new(StdIO {});
            for error in self.lexer.errors.iter() {
                println!("{}", error.generate_msg());
            }
            return vec![];
        }

        for (i, token) in tokens.iter().enumerate() {
            println!("{}\t: {:?}", i, token);
        }
        
        self.semantic_checker.run(&tokens, input_file);
        
        if self.semantic_checker.errors.len() > 0 {
            for error in self.semantic_checker.errors.iter() {
                println!("{}", error.generate_msg());
            }
            return vec![];
        }

        vec![]
    }
}
