use crate::output;

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
                continue;
            }

            match &tokens[self.token_index].inner_token {
                TokenType::Instruction(instruction) => {
                    self.increment();
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

    pub fn increment(&mut self) {
        self.memory_location += 1;
        self.token_index += 1;
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
                    self.memory_location += 1;
                } else {
                    unreachable!();
                }
            },
            Directive::BLKW => {
                if let TokenType::Number(count) = tokens[self.token_index].inner_token {
                    for _ in 0..count {
                        output.push(0);
                        self.memory_location += 1;
                    }
                } else {
                    unreachable!();
                }
            },
            Directive::STRINGZ => {
                if let TokenType::String(string) = &tokens[self.token_index].inner_token {
                    for c in string.chars() {
                        output.push(c as u16);
                        self.memory_location += 1;
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
        let opcode = instruction.get_opcode_value() << 12;
        let output: u16;

        match instruction {
            OpcodeIns::Add | OpcodeIns::And => {
                return self.handle_reg_reg_ctrl_reg_or_imm5(opcode, tokens);
            },
            OpcodeIns::Br(n, z, p) => {
                return self.handle_br(*n, *z, *p, opcode, tokens);
            }
            OpcodeIns::Ld | OpcodeIns::Ldi | OpcodeIns::Lea | OpcodeIns::St | OpcodeIns::Sti => {
                return self.handle_reg_offset9(opcode, tokens);
            }
            OpcodeIns::Ldr | OpcodeIns::Str => {
                return self.handle_reg_reg_offset6(opcode, tokens);
            }
            OpcodeIns::Jsr => {
                return self.handle_jsr(opcode, tokens);
            }
            OpcodeIns::Jsrr => {
                return self.handle_jsrr(opcode, tokens);
            }
            OpcodeIns::Ret => {
                return 0b1100_000_111_000_000;
            }
            OpcodeIns::Rti => {
                return 0b1000_0000_0000_0000;
            }
            OpcodeIns::Trap(subroutine) => {
                let ins = opcode + subroutine;
                output = ins;
            },
            _ => unimplemented!()
        }

        
        println!("ins : {:#018b}", output);

        return output;
    }

    pub fn handle_reg_reg_ctrl_reg_or_imm5(&mut self, opcode: u16, tokens: &Vec<Token>) -> u16 {
        let reg1 = &tokens[self.token_index].inner_token;
        self.token_index += 1;
        let reg2 = &tokens[self.token_index].inner_token;
        self.token_index += 1;
        let reg3_or_imm5 = &tokens[self.token_index].inner_token;
        self.token_index += 1;

        let mut output_value = opcode;
    
        if let TokenType::Register(dr) = reg1 {
            output_value += dr << 9;
        } else {
            unreachable!();
        }

        if let TokenType::Register(sr1) = reg2 {
            output_value += sr1 << 6;
        } else {
            unreachable!();
        }
        
        match reg3_or_imm5 {
            TokenType::Register(sr2) => {
                return output_value + sr2;
            },
            TokenType::Number(imm5) => {
                output_value += 1 << 5; // control bit, tells the VM that this is an immediate value
                return self.add_imm(output_value, *imm5 as u16, 5);
            },
            _ => {
                unreachable!();
            }
        }
    }

    pub fn handle_br(&mut self, n: bool, z: bool, p: bool, opcode: u16, tokens: &Vec<Token>) -> u16 {
        let label = tokens[self.token_index].inner_token;
        self.token_index += 1;

        let mut output_value = opcode;

        if n {
            output_value += 1 << 11; // n is in the 11th position
        }
        if z {
            output_value += 1 << 10;
        }
        if p {
            output_value = 1 << 9;
        }

        if let TokenType::Label(l) = label {
            let (pcoffset9, _) = self.semantic_checker.symbol_table.get(&l)
                .expect(&format!("Expected that the label `{}` would be defined and verified in the semantic checker", l));
            output_value = self.add_imm(output_value, *pcoffset9 as u16, 9);
        } else {
            unreachable!();
        }

        return output_value;
    }

    pub fn handle_reg_offset9(&mut self, opcode: u16, tokens: &Vec<Token>) -> u16 {
        let imm_len = 9;
        let register = &tokens[self.token_index].inner_token;
        self.token_index += 1;

        let offset = &tokens[self.token_index].inner_token;
        self.token_index += 1;
        let mut output_value = opcode;

        if let TokenType::Register(dr) = register {
            output_value += dr << imm_len;
        } else {
            unreachable!();
        }

        if let TokenType::Label(label) = offset {
            let (label_loc, _) = self.semantic_checker.symbol_table.get(label)
                .expect(&format!("expected that the label `{label}` existed the symbol table existed"));
            let pcoffset9 = *label_loc - self.memory_location as i32;
            return self.add_imm(output_value, pcoffset9 as u16, imm_len);
        } else {
            unreachable!()
        }
    }

    pub fn handle_reg_reg_offset6(&mut self, opcode: u16, tokens: &Vec<Token>) -> u16 {
        0
    }
    
    pub fn handle_jsr(&mut self, opcode: u16, tokens: &Vec<Token>) -> u16 {
        0
    }
    
    pub fn handle_jsrr(&mut self, opcode: u16, tokens: &Vec<Token>) -> u16 {
        0
    }
    pub fn get_operands(&mut self, tokens: &Vec<Token>, count: i32) -> Vec<Token> {
        let mut output: Vec<Token> = vec![];

        for _ in 0..count {
            output.push(tokens[self.token_index].clone()); // TODO: remove clone()
            self.token_index += 1;
        }

        return output;
    }
    
    pub fn add_imm(&self, instruction: u16, immediate_value: u16, length: u16) -> u16 {
        if immediate_value as i16 >= 0 {
            return instruction + immediate_value;
        }

        let length_complement = 16 - length;
        let cut_imm = (immediate_value << length_complement) >> length_complement;
        println!("instruction \t= {:#018b}", instruction);
        println!("cut_imm \t= {:#018b}", cut_imm);
        return instruction + cut_imm;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_token(t: TokenType) -> Token {
        Token {
            inner_token: t,
            to: 0, // this info is for errors, and errors shouldn't be possible in this step
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
    fn test_add_immediate() {
        let mut asm = Asm::new();

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Add),
            TokenType::Register(1),
            TokenType::Register(1),
            TokenType::Number(10),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0001_001_001_1_01010);
    }

    #[test]
    fn test_add_register() {
        let mut asm = Asm::new();

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Add),
            TokenType::Register(1),
            TokenType::Register(1),
            TokenType::Register(7),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0001_001_001_0_00_111);
       
    }

    #[test]
    fn test_and_immediate() {
        let mut asm = Asm::new();

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::And),
            TokenType::Register(1),
            TokenType::Register(1),
            TokenType::Number(10),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0101_001_001_1_01010);
       
    }

    #[test]
    fn test_and_register() {
        let mut asm = Asm::new();

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::And),
            TokenType::Register(1),
            TokenType::Register(1),
            TokenType::Register(7),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0101_001_001_0_00_111);
       
    }

    #[test]
    fn test_ld() {
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("i"), (3001, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Ld),
            TokenType::Register(1),
            TokenType::Label(String::from("i")),
            
            TokenType::Label(String::from("i")),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(42),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0010_001_000000000);
    }

    #[test]
    fn test_ldi() {
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("i"), (3001, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Ldi),
            TokenType::Register(1),
            TokenType::Label(String::from("i")),
            
            TokenType::Label(String::from("i")),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(42),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b1010_001_000000000);
    }

    #[test]
    fn test_lea() {
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("i"), (3001, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Lea),
            TokenType::Register(1),
            TokenType::Label(String::from("i")),
            
            TokenType::Label(String::from("i")),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(42),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b1110_001_000000000);
    }

    #[test]
    fn test_st() {
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("i"), (3001, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::St),
            TokenType::Register(1),
            TokenType::Label(String::from("i")),
            
            TokenType::Label(String::from("i")),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(42),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b0011_001_000000000);
    }

    #[test]
    fn test_sti() {
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("i"), (3001, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Sti),
            TokenType::Register(1),
            TokenType::Label(String::from("i")),
            
            TokenType::Label(String::from("i")),
            TokenType::Directive(Directive::FILL),
            TokenType::Number(42),
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b1011_001_000000000);
    }

    #[test]
    fn test_pcoffset9() {
        // tests that the delta actually points in the correct signed direction
        let mut asm = Asm::new();
        
        asm.semantic_checker.symbol_table.insert(String::from("x"), (3000, mk_token(TokenType::Label(String::from("i")))));
        asm.semantic_checker.symbol_table.insert(String::from("y"), (3003, mk_token(TokenType::Label(String::from("i")))));

        let stream = get_file(vec![
            TokenType::Instruction(OpcodeIns::Sti),
            TokenType::Register(1),
            TokenType::Label(String::from("x")), // x -> 3000, pc == 3001 => -1

            TokenType::Instruction(OpcodeIns::Sti),
            TokenType::Register(1),
            TokenType::Label(String::from("y")), // y -> 3003, pc == 3002 => 1
        ]);
        
        let bin = asm.assemble(stream);
        
        assert_eq!(bin[1], 0b1011_001_111111111);
        assert_eq!(bin[2], 0b1011_001_000000001);
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

