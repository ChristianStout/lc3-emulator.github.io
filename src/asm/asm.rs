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
        // Split string into Vec<&str>
        // Hand split file into lexer, turn into Vec<Token>
        // Assemble Vec<Token> into binary Vec<u16>

        let split_file: Vec<&str> = input_file.split('\n').collect();
        let tokens = self.lexer.run(split_file);
        
        if self.lexer.errors.len() > 0 {
            for error in self.lexer.errors.iter() {
                error.print();
            }
        }

        println!("TOKENS: {:?}", tokens);

        vec![]
    }
}
