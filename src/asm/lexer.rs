use super::asm_ins::*;
use super::syntax::SyntaxChecker;
use super::asm_error::*;
use super::token::*;
use std::i16;

pub struct Lexer {
    pub token_stream: Vec<Token>,
    pub errors: Vec<AsmError>,
    syntax_checker: SyntaxChecker,
    curr_line_num: i32,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_stream: vec![],
            errors: vec![],
            syntax_checker: SyntaxChecker::new(),
            curr_line_num: 0,
        }
    }

    pub fn run(&mut self, input_file: Vec<&str>) -> Vec<Token> {
        for line in input_file {
            self.curr_line_num += 1;

            if self.syntax_checker.is_ins(line) {
                self.parse_instruction(line);
                continue;
            }
            if self.syntax_checker.is_dir(line) {
                self.parse_directive(line);
                continue;

            }
            if !self.syntax_checker.is_ignore(line) {
                // TODO: Add Error struct with a `discover_error` function
                //      that takes in any known information. This should also
                //      be able to display EVERY current error, so a vector.
                panic!("SYNTAX ERROR: On line {}, neither an instruction nor directive was given.", self.curr_line_num);
            }
        }

        // TODO: Rid yourself of this HORRID thing D8< (the clone)
        return self.token_stream.clone();
    }

    pub fn parse_instruction(&mut self, line: &str) {
        // This is also doing some semantic checking on the instruction's operands.
        // If it fails, there should be a list of errors given.
        let split_line: Vec<&str> = line.split_whitespace().collect();

        let mut instruction = OpcodeIns::from(split_line[0]);
        let mut i = 1;

        if instruction == OpcodeIns::INVALID {
            self.token_stream.push(Token::Label(String::from(split_line[0])));
            instruction = OpcodeIns::from(split_line[1]);
            i = 2;
        }

        

        let operand_types = instruction.get_type();
        self.token_stream.push(Token::Instruction(instruction));

        for ot in operand_types {
            let mut curr_operand = split_line[i];

            // Delete commas
            if let Some(',') = curr_operand.chars().nth(curr_operand.len() - 1) {
                curr_operand =  &curr_operand[..curr_operand.len()-1] 
            }

            match ot {
                OperandType::Reg => self.parse_reg(curr_operand, line),
                OperandType::Label => self.parse_label(curr_operand),
                OperandType::Imm => self.parse_imm(curr_operand, line),
                OperandType::RegOrImm => self.parse_reg_or_imm(curr_operand, line),
            }
            i += 1;
        }
    }

    pub fn parse_directive(&mut self, line: &str) {
        let _split_line: Vec<&str> = line.split_whitespace().collect();
    }

    fn parse_reg(&mut self, reg: &str, line: &str) {
        // let num_str = "1";
        let num_str = &reg[1..];

        let num = num_str
            .parse::<u16>()
            .expect("In Lexer::parse_reg(), a register somehow was not given a register number. This should't possible based on syntax");

        if num <= 7 {
            self.token_stream.push(Token::Register(num));
            return;
        }

        self.errors.push(AsmError::new(
            line,
            self.curr_line_num,
            ErrorType::OperandError,
            &format!("only regitsers 0-7 exist, but regitser {num} was given.")
        ));
    }

    fn parse_label(&mut self, label: &str) {
        self.token_stream.push(Token::Label(String::from(label)));
    }

    fn parse_imm(&mut self, imm: &str, line: &str) {
        let t = imm.chars().nth(0).expect("in parse_imm: Shouldn't be possible, a value of length 0 may have been given.");
        let str_value = &imm[1..];

        if t == '#' {
            match str_value.parse::<i16>() {
                Ok(num_value) => self.token_stream.push(Token::Number(num_value)),
                Err(e) => {
                    self.errors.push(AsmError::new(
                        line,
                        self.curr_line_num,
                        ErrorType::OperandError,
                        &format!("{e}"),
                    ));
                }
            }
        }

        if t == 'x' || t == 'X' {
            match i16::from_str_radix(str_value, 4) {
                Ok(num_value) => self.token_stream.push(Token::Number(num_value)),
                Err(e) => {
                    self.errors.push(AsmError::new(
                        line,
                        self.curr_line_num,
                        ErrorType::OperandError,
                        &format!("{e}"),
                    ));
                }
            }
        }
    }

    fn parse_reg_or_imm(&mut self, reg_or_imm: &str, line: &str) {
        let c = reg_or_imm
            .chars()
            .nth(0)
            .expect("This shouldn't be possible given how we got here, yet here we are...");

        if c == 'R' || c == 'r' {
            self.parse_reg(reg_or_imm, line);
            return;
        }
        if c == 'x' || c == 'X' || c == '#' {
            self.parse_imm(reg_or_imm, line);
            return;
        }

        self.errors.push(AsmError::new(
            line, 
            self.curr_line_num, 
            ErrorType::OperandError, 
            &format!("a regitser or immediate value was supposed to be given, but instead `{reg_or_imm}` was given." )   
        ))
    }
}

#[cfg(test)]
mod tests {
    // use super::Lexer;
}
