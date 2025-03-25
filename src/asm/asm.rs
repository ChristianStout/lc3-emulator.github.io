use super::lexer::*;
use super::semantic::*;
use super::token::*;
use super::asm_ins::OpcodeIns;
use super::directive::Directive;

#[allow(dead_code)]
pub struct Asm {
    lexer: Lexer,
    semantic_checker: SemanticChecker,
    token_index: usize,
    memory_location: usize,
}

#[allow(dead_code)]
impl Asm {
    pub fn new() -> Asm {
        Asm {
            lexer: Lexer::new(),
            semantic_checker: SemanticChecker::new(),
            token_index: 0,
            memory_location: 0,
        }
    }

    pub fn run(&mut self, input_file: String) -> Vec<u16> {
        // 1. Verify that file is syntactically valid
        if !self.lexer.syntax_checker.is_syntactically_valid_file(&input_file) {
            return vec![];
        }
        
        // 2. Create token stream with Lexer
        let tokens = self.lexer.run(input_file.clone());
        
        if self.lexer.errors.len() > 0 {
            // let io = &Box::new(StdIO {});
            for error in self.lexer.errors.iter() {
                println!("{}", error.generate_msg());
            }
            return vec![];
        }
        
        // this is for debug purposes
        for (i, token) in tokens.iter().enumerate() {
            println!("{}\t: {:?}", i, token);
        }
        
        // 3. Verify that file is semantically valid
        self.semantic_checker.run(&tokens, input_file);
        
        if self.semantic_checker.errors.len() > 0 {
            for error in self.semantic_checker.errors.iter() {
                println!("{}", error.generate_msg());
            }
            return vec![];
        }
        
        // self.symbol_table = self.semantic_checker.symbol_table;
        
        // 4. Assemble Vec<Token> into binary Vec<u16> & Symbol Table
        return self.assemble(tokens);
    }

    pub fn assemble(&mut self, tokens: Vec<Token>) -> Vec<u16> {
        // Every token is already assumed completely semantically valid. Therefore, there
        // are no errors that should occur in this step. If we receive an instruction, it is
        // guaranteed to have all of its operands.
        
        self.set_origin(&tokens);

        let mut binary_file: Vec<u16> = vec![];

        while self.token_index < tokens.len() {
            if let TokenType::Label(_) = tokens[self.token_index].inner_token {
                self.token_index += 1;
            }

            match &tokens[self.token_index].inner_token {
                TokenType::Instruction(instruction) => {
                    self.token_index += 1;
                    binary_file.push(self.handle_instruction(instruction, &tokens));
                },
                TokenType::Directive(directive) => {
                    self.token_index += 1;
                    let memory_vec = self.handle_directive(directive, &tokens);
                    for value in memory_vec {
                        binary_file.push(value);
                    }
                },
                _ => {
                    unreachable!();
                }
            }
        }

        return binary_file;
    }

    pub fn set_origin(&mut self, tokens: &Vec<Token>) {
        if let TokenType::Label(_) = tokens[self.token_index].inner_token {
            self.token_index += 1;
        }

        self.token_index += 1; // skip .orig
        
        if let TokenType::Number(origin) = tokens[self.token_index].inner_token {
            self.memory_location = origin as usize;
            self.token_index += 1;
        } else {
            unreachable!();
        }
    }

    pub fn handle_instruction(&mut self, instruction: &OpcodeIns, tokens: &Vec<Token>) -> u16 {
        0
    }

    pub fn handle_directive(&mut self, directive: &Directive, tokens: &Vec<Token>) -> Vec<u16> {
        let mut output: Vec<u16> = vec![];

        match directive {
            Directive::END => return output,
            Directive::FILL => {
                if let TokenType::Number(value) = tokens[self.token_index].inner_token {
                    output.push(value as u16);
                } else {
                    unreachable!();
                }
            },
            Directive::BLKW => {
                if let TokenType::Number(count) = tokens[self.token_index].inner_token {
                    for _ in 0..count {
                        output.push(0);
                    }
                } else {
                    unreachable!();
                }
            },
            Directive::STRINGZ => {
                if let TokenType::String(string) = &tokens[self.token_index].inner_token {
                    for c in string.chars() {
                        output.push(c as u16);
                    }
                } else {
                    unreachable!();
                }
            }
            Directive::ORIG => unreachable!(),
        }
        
        self.token_index += 1;
        return output;
    }

    pub fn get_operands(&mut self, tokens: &Vec<Token>, count: i32) -> Vec<Token> {
        let mut output: Vec<Token> = vec![];

        for _ in 0..count {
            output.push(tokens[self.token_index].clone()); // TODO: remove clone()
            self.token_index += 1;
        }

        return output;
    }
}
