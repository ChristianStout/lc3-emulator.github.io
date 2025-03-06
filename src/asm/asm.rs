use super::lexer::*;
// use super::semantic::*;

pub struct Asm {
    lexer: Lexer,
}

impl Asm {
    pub fn new() -> Asm {
        Asm {
            lexer: Lexer::new(),
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

        let tokens = self.lexer.run(input_file);
        
        if self.lexer.errors.len() > 0 {
            for error in self.lexer.errors.iter() {
                error.print();
            }
        }

        // println!("TOKENS: {:?}", tokens);
        for (i, token) in tokens.iter().enumerate() {
            println!("{}\t: {:?}", i, token);
        }

        vec![]
    }
}
