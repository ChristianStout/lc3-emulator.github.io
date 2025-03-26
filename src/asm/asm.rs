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
        
        let mut binary_file: Vec<u16> = vec![];
        
        self.set_origin(&tokens);
        binary_file.push(self.memory_location as u16);

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


    pub fn handle_instruction(&mut self, instruction: &OpcodeIns, tokens: &Vec<Token>) -> u16 {
        match instruction {
            OpcodeIns::Trap(subroutine) => {
                let opcode = instruction.get_opcode_value();
                let ins = (opcode << 12) + subroutine;
                println!("TRAP: {:#018b}", ins);
                return ins;
            },
            _ => unimplemented!()
        }
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

#[cfg(test)]
mod tests {
    use crate::asm::token::*;
    use super::*;

    fn mk_token(t: TokenType) -> Token {
        Token {
            inner_token: t,
            to: 0, // this info id for errors, and errors shouldn't be possible in this step
            from: 0,
            file_relative_from: 0,
            file_relative_to:0,
            line_num: 0,
            original_match: "".to_string(),
        }
    }

    fn get_file(contents: Vec<TokenType>) -> Vec<Token> {
        let mut output = vec![
            mk_token(TokenType::Directive(Directive::ORIG)),
            mk_token(TokenType::Number(3000)),
        ];
        
        for token_t in contents {
            output.push(mk_token(token_t));
        }
        
        return output;
    }

    #[test]
    fn test_asm_directive_orig() {
        let mut asm = Asm::new();
        
        let stream = vec![
            mk_token(TokenType::Directive(Directive::ORIG)),
            mk_token(TokenType::Number(3000)),
        ];
        
        let bin = asm.assemble(stream);
        
        assert!(bin[0] == 3000);
        assert!(bin.len() == 1);

        let mut asm = Asm::new();
        
        let stream = vec![
            mk_token(TokenType::Directive(Directive::ORIG)),
            mk_token(TokenType::Number(42)),
        ];
        
        let bin = asm.assemble(stream);
        
        assert!(bin[0] == 42);
        assert!(bin.len() == 1);
    }

    #[test]
    fn test_asm_directive_fill() {
        let mut asm = Asm::new();
        
        let stream = get_file(vec![
            TokenType::Directive(Directive::FILL),
            TokenType::Number(10),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(1999),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert!(bin[1] == 0b0000_0000_0000_1010);
        assert!(bin[2] == 1999);
    }

    #[test]
    fn test_asm_directive_blkw() {
        let mut asm = Asm::new();
        
        let stream = get_file(vec![
            TokenType::Directive(Directive::BLKW),
            TokenType::Number(3),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert!(bin[1] == 0);
        assert!(bin[2] == 0);
        assert!(bin[3] == 0);
        assert!(bin.len() == 4);
    }

    #[test]
    fn test_asm_directive_stringz() {
        let mut asm = Asm::new();
        
        let stream = get_file(vec![
            TokenType::Directive(Directive::STRINGZ),
            TokenType::String(String::from("HELP ME!")),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert!(bin[1] as u8 == 'H' as u8);
        assert!(bin[2] as u8 == 'E' as u8);
        assert!(bin[3] as u8 == 'L' as u8);
        assert!(bin[4] as u8 == 'P' as u8);
        assert!(bin[5] as u8 == ' ' as u8);
        assert!(bin[6] as u8 == 'M' as u8);
        assert!(bin[7] as u8 == 'E' as u8);
        assert!(bin[8] as u8 == '!' as u8);
        
        assert!(bin.len() == 9);
    }

    #[test]
    fn test_trap() {
        let mut asm = Asm::new();
        
        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Trap(20)), // getc
            TokenType::Instruction(OpcodeIns::Trap(21)), // out
            TokenType::Instruction(OpcodeIns::Trap(22)), // puts
            TokenType::Instruction(OpcodeIns::Trap(23)), // in
            TokenType::Instruction(OpcodeIns::Trap(25)), // halt
            TokenType::Instruction(OpcodeIns::Trap(32)), // maybe some other instruction someday?
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b1111_0000_0001_0100);
        assert_eq!(bin[2], 0b1111_0000_0001_0101);
        assert_eq!(bin[3], 0b1111_0000_0001_0110);
        assert_eq!(bin[4], 0b1111_0000_0001_0111);
        assert_eq!(bin[5], 0b1111_0000_0001_1001);
        assert_eq!(bin[6], 0b1111_0000_0010_0000);
    }
}

