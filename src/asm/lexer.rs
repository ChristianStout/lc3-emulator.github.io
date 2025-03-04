use super::asm_ins::*;
use super::directive::*;
use super::syntax::SyntaxChecker;
use super::asm_error::*;
use super::token::*;

pub struct Lexer {
    pub token_stream: Vec<Token>,
    pub errors: Vec<AsmError>,
    syntax_checker: SyntaxChecker,
    curr_file: String,
    file_as_chars: Vec<char>,
    curr_line_num: i32,
    position: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            token_stream: vec![],
            errors: vec![],
            syntax_checker: SyntaxChecker::new(),
            curr_file: String::new(),
            file_as_chars: vec![],
            curr_line_num: 0,
            position: 0,
        }
    }

    pub fn run(&mut self, input_file: String) -> Vec<Token> {
        self.file_as_chars = input_file.chars().collect();
        self.curr_file = input_file;

        let mut word_buffer: Vec<char> = vec![];
        let mut c: char;
            

        while self.position < self.file_as_chars.len() {
            c = self.next_char(); // iterates self.position

            if c == '\"' {
                let string = self.parse_string();
                self.token_stream.push(Token::String(string));
                continue;
            }

            if c == '\n' {
                self.curr_line_num += 1;
            }

            if c == ';' {
                self.skip_comment();
                continue;
            }

            if (c.is_whitespace() || c == ';' || c == ',') && word_buffer.len() > 0 {
                self.parse_word(word_buffer.iter().collect());
                word_buffer.clear();
                continue;
            }

            if c.is_whitespace() && word_buffer.len() == 0 {
                continue;
            }

            word_buffer.push(c);
            println!("word_buffer: {}", word_buffer.iter().collect::<String>());
        }

        let tokens = self.token_stream.clone();

        self.reset();

        return tokens; // TODO: Remove clone()
    }

    fn next_char(&mut self) -> char {
        let c: char = self.file_as_chars[self.position];
        self.position += 1;
        return c;
    }

    fn get_current_line(&mut self) -> String {
        let current_line_number = self.curr_line_num.clone() as usize;
        let split_file: Vec<&str> = self.curr_file.split_whitespace().collect();
        return String::from(split_file[current_line_number]);
    }

    fn skip_comment(&mut self) {
        while self.next_char() != '\n' {
            // ...
        }
    }

    fn reset(&mut self) {
        self.token_stream = vec![];
        self.errors = vec![];
        self.syntax_checker = SyntaxChecker::new();
        self.curr_file = String::new();
        self.file_as_chars = vec![];
        self.curr_line_num = 0;
        self.position = 0;
    }

    pub fn parse_word(&mut self, word: String) {
        // parse hierarchy
        let upper = word.to_ascii_uppercase();

        println!("{}, len = {}", upper, upper.len());

        if self.syntax_checker.is_ignore(&upper) {
            return;
        }
        else if self.syntax_checker.is_instruction_name(&upper) {
            self.token_stream.push(Token::Instruction(OpcodeIns::from(&upper)));
            return;
        }
        else if self.syntax_checker.is_directive_name(&upper) {
            self.token_stream.push(Token::Directive(Directive::from(&upper)));
            return;
        }
        else if self.syntax_checker.is_valid_register(&upper) {
            self.token_stream.push(Token::Register(self.parse_register(&upper)));
            return;
        }
        else if self.syntax_checker.is_valid_immediate_value(&word) {
            self.token_stream.push(Token::Number(self.parse_immediate_value(&word)));
            return;
        }
        else if self.syntax_checker.is_valid_label(&word) {
            self.token_stream.push(Token::Label(word.to_string()));
            return;
        }
        else {
            self.token_stream.push(Token::INVALID(word));
            let line = self.get_current_line();
            self.errors.push(AsmError::new(
                &line,
                self.curr_line_num,
                ErrorType::SyntaxError,
                "a token could not be categorized"
            ))
        }
    }

    pub fn parse_register(&self, word: &str) -> u16 {
        let base = 10;

        let register_num = word
            .chars()
            .nth(1) // Always R1, R2, R3, ... , R7.
            .expect("Lexer::parse_register: Somehow a register was given without a number. This shouldn't be possible given the Regex.")
            .to_digit(base) // a base 10 number
            .expect(&format!("Lexer::parse_register: When converting the register value on line {}, could not conver value into base 10 number.", self.curr_line_num));
        
        return register_num as u16;
    }
    
    pub fn parse_immediate_value(&self, word: &str) -> i16 {
        match word.chars().nth(0).unwrap() {
            '#' => {
                return word[1..]
                    .parse()
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num));
            },
            'x' | 'X' => {
                let base = 16;
                return u16::from_str_radix(&word[1..], base)
                    .expect(&format!("Lexer::parse_immediate_value: The given number on line {} is not valid", self.curr_line_num)) as i16;
            },
            _ => unreachable!(),
        }
    }

    #[allow(while_true)]
    pub fn parse_string(&mut self) -> String {
        let mut str_buffer: Vec<char> = vec![];
        let mut is_escape = false;
        let mut c: char;

        while true {
            c = self.next_char();

            if is_escape {
                str_buffer.push(self.parse_escape(c));
                is_escape = false;
                continue;
            }

            if c == '\"' {
                break;
            }
            
            if c == '\\' {
                is_escape = true;
                continue;
            }

            if c == '\n' {
                let curr_line = self.get_current_line();
                self.errors.push(AsmError::new(
                    &curr_line,
                    self.curr_line_num,
                    ErrorType::SyntaxError,
                    "the given string was not terminated",
                ));
                break;
            }

            str_buffer.push(c);
        }

        return str_buffer.iter().collect();
    }

     pub fn parse_escape(&mut self, character: char) -> char {
        println!("Reached here");
        match character {
            '\\' | '\'' |'\"' => return character,
            'n' => return '\n',
            'r' => return '\r',
            't' => return '\t',
            '0' => return '\0',
            _ => {
                let line = self.get_current_line();
                let line_number = self.curr_line_num;
                self.errors.push(AsmError::new(
                    &line,
                    line_number,
                    ErrorType::SyntaxError,
                    &format!("the given escape character `\\{}` does not exist.", character)
                ));
            },
        }
        '\0'
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_br_nzp() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" BR ")),
            vec![Token::Instruction(OpcodeIns::Br(false, false, false))]
        );
        assert_eq!(
            lexer.run(String::from(" BRn ")),
            vec![Token::Instruction(OpcodeIns::Br(true, false, false))]
        );
        assert_eq!(
            lexer.run(String::from(" BRz ")),
            vec![Token::Instruction(OpcodeIns::Br(false, true, false))]
        );
        assert_eq!(
            lexer.run(String::from(" BRp ")),
            vec![Token::Instruction(OpcodeIns::Br(false, false, true))]
        );
        assert_eq!(
            lexer.run(String::from(" BRnz ")),
            vec![Token::Instruction(OpcodeIns::Br(true, true, false))]
        );
        assert_eq!(
            lexer.run(String::from(" BRnp ")),
            vec![Token::Instruction(OpcodeIns::Br(true, false, true))]
        );
        assert_eq!(
            lexer.run(String::from(" BRzp ")),
            vec![Token::Instruction(OpcodeIns::Br(false, true, true))]
        );
        assert_eq!(
            lexer.run(String::from(" BRnzp ")),
            vec![Token::Instruction(OpcodeIns::Br(true, true, true))]
        );
    }

    #[test]
    fn test_instructions() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" ADD ")),
            vec![Token::Instruction(OpcodeIns::Add)]
        );
        assert_eq!(
            lexer.run(String::from(" AND ")),
            vec![Token::Instruction(OpcodeIns::And)]
        );
        assert_eq!(
            lexer.run(String::from(" JMP ")),
            vec![Token::Instruction(OpcodeIns::Jmp)]
        );
        assert_eq!(
            lexer.run(String::from(" JSR ")),
            vec![Token::Instruction(OpcodeIns::Jsr)]
        );
        assert_eq!(
            lexer.run(String::from(" Jsrr ")),
            vec![Token::Instruction(OpcodeIns::Jsrr)]
        );
        assert_eq!(
            lexer.run(String::from(" LD ")),
            vec![Token::Instruction(OpcodeIns::Ld)]
        );
        assert_eq!(
            lexer.run(String::from(" LDI ")),
            vec![Token::Instruction(OpcodeIns::Ldi)]
        );
        assert_eq!(
            lexer.run(String::from(" LDR ")),
            vec![Token::Instruction(OpcodeIns::Ldr)]
        );
        assert_eq!(
            lexer.run(String::from(" LEA ")),
            vec![Token::Instruction(OpcodeIns::Lea)]
        );
        assert_eq!(
            lexer.run(String::from(" NOT ")),
            vec![Token::Instruction(OpcodeIns::Not)]
        );
        assert_eq!(
            lexer.run(String::from(" RET ")),
            vec![Token::Instruction(OpcodeIns::Ret)]
        );
        assert_eq!(
            lexer.run(String::from(" RTI ")),
            vec![Token::Instruction(OpcodeIns::Rti)]
        );
        assert_eq!(
            lexer.run(String::from(" ST ")),
            vec![Token::Instruction(OpcodeIns::St)]
        );
        assert_eq!(
            lexer.run(String::from(" STI ")),
            vec![Token::Instruction(OpcodeIns::Sti)]
        );
        assert_eq!(
            lexer.run(String::from(" STR ")),
            vec![Token::Instruction(OpcodeIns::Str)]
        );
        assert_eq!(
            lexer.run(String::from(" GETC ")),
            vec![Token::Instruction(OpcodeIns::Trap(20))]
        );
        assert_eq!(
            lexer.run(String::from(" OUT ")),
            vec![Token::Instruction(OpcodeIns::Trap(21))]
        );
        assert_eq!(
            lexer.run(String::from(" PUTS ")),
            vec![Token::Instruction(OpcodeIns::Trap(22))]
        );
        assert_eq!(
            lexer.run(String::from(" IN ")),
            vec![Token::Instruction(OpcodeIns::Trap(23))]
        );
        assert_eq!(
            lexer.run(String::from(" HALT ")),
            vec![Token::Instruction(OpcodeIns::Trap(25))]
        );


        assert_ne!(
            lexer.run(String::from(" HALTS ")),
            vec![Token::Instruction(OpcodeIns::Trap(25))]
        );
        assert_ne!(
            lexer.run(String::from(" ADDI ")),
            vec![Token::Instruction(OpcodeIns::Add)]
        );
        assert_ne!(
            lexer.run(String::from(" ANDY ")),
            vec![Token::Instruction(OpcodeIns::And)]
        );
        assert_ne!(
            lexer.run(String::from(" JSRRR ")),
            vec![Token::Instruction(OpcodeIns::Jsrr)]
        );
    }

    #[test]
    fn test_registers() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" R0 ")),
            vec![Token::Register(0)]
        );
        assert_eq!(
            lexer.run(String::from(" R1 ")),
            vec![Token::Register(1)]
        );
        assert_eq!(
            lexer.run(String::from(" R2 ")),
            vec![Token::Register(2)]
        );
        assert_eq!(
            lexer.run(String::from(" R3 ")),
            vec![Token::Register(3)]
        );
        assert_eq!(
            lexer.run(String::from(" R4 ")),
            vec![Token::Register(4)]
        );
        assert_eq!(
            lexer.run(String::from(" R5 ")),
            vec![Token::Register(5)]
        );
        assert_eq!(
            lexer.run(String::from(" R6 ")),
            vec![Token::Register(6)]
        );
        assert_eq!(
            lexer.run(String::from(" R7 ")),
            vec![Token::Register(7)]
        );


        assert_ne!(
            lexer.run(String::from(" R8 ")),
            vec![Token::Register(8)]
        );
        assert_ne!(
            lexer.run(String::from(" RR1 ")),
            vec![Token::Register(1)]
        );
        assert_ne!(
            lexer.run(String::from(" GO_TO_R1 ")),
            vec![Token::Register(1)]
        );
        assert_ne!(
            lexer.run(String::from(" R1_FOREVER_IN_MY_HEART ")),
            vec![Token::Register(1)]
        );
        assert_ne!(
            lexer.run(String::from(" okay_R1_your_right ")),
            vec![Token::Register(1)]
        );
    }

    #[test]
    fn test_directives() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" .ORIG ")),
            vec![Token::Directive(Directive::ORIG)]
        );
        assert_eq!(
            lexer.run(String::from(" .FILL ")),
            vec![Token::Directive(Directive::FILL)]
        );
        assert_eq!(
            lexer.run(String::from(" .BLKW ")),
            vec![Token::Directive(Directive::BLKW)]
        );
        assert_eq!(
            lexer.run(String::from(" .STRINGZ ")),
            vec![Token::Directive(Directive::STRINGZ)]
        );
        assert_eq!(
            lexer.run(String::from(" .END ")),
            vec![Token::Directive(Directive::END)]
        );
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" #1 ")),
            vec![Token::Number(1)]
        );
        assert_eq!(
            lexer.run(String::from(" #2 ")),
            vec![Token::Number(2)]
        );
        assert_eq!(
            lexer.run(String::from(" #3 ")),
            vec![Token::Number(3)]
        );
        assert_eq!(
            lexer.run(String::from(" #128 ")),
            vec![Token::Number(128)]
        );
        assert_eq!(
            lexer.run(String::from(" #-128 ")),
            vec![Token::Number(-128)]
        );

        assert_eq!(
            lexer.run(String::from(" x0000 ")),
            vec![Token::Number(0)]
        );
        assert_eq!(
            lexer.run(String::from(" x1 ")),
            vec![Token::Number(1)]
        );
        assert_eq!(
            lexer.run(String::from(" x0001 ")),
            vec![Token::Number(1)]
        );
        assert_eq!(
            lexer.run(String::from(" xFFFF ")),
            vec![Token::Number(-1)]
        );
    }

    #[test]
    fn test_strings() {
        let file = String::from(r#".STRINGZ "Hello, World!"  "#);

        let mut lexer = Lexer::new();
        let tokens = lexer.run(file);

        println!("TOKENS: {:?}", tokens);

        assert!(tokens == vec![Token::Directive(Directive::STRINGZ), Token::String("Hello, World!".to_string())]);
        // assert!(tokens[5] == Token::Number(65535_u16 as i16));
    }
    
    #[test]
    fn test_labels() {
        todo!();
    }

    #[test]
    fn test_case_sensitivity() {
        let mut lexer = Lexer::new();

        assert_eq!(
            lexer.run(String::from(" this ")),
            vec![Token::Label("this".to_string())]
        );
        assert_eq!(
            lexer.run(String::from(" THIS ")),
            vec![Token::Label("THIS".to_string())]
        );
        assert_ne!(
            lexer.run(String::from(" this ")),
            vec![Token::Label("THIS".to_string())]
        );


        assert_eq!(
            lexer.run(String::from(" halt ")),
            lexer.run(String::from(" HALT "))
        );
        assert_eq!(
            lexer.run(String::from(" and ")),
            lexer.run(String::from(" AND "))
        );
        assert_eq!(
            lexer.run(String::from(" jsrR ")),
            lexer.run(String::from(" JSRr "))
        );
        assert_eq!(
            lexer.run(String::from(" LEA ")),
            lexer.run(String::from(" lea "))
        );
        assert_eq!(
            lexer.run(String::from(" GeTc ")),
            lexer.run(String::from(" gEtC "))
        );


        assert_ne!(
            lexer.run(String::from(r#" "string" "#)),
            lexer.run(String::from(r#" "STRING" "#))
        );

        assert_eq!(
            lexer.run(String::from(" r1 ")),
            lexer.run(String::from(" R1 "))
        );

        assert_eq!(
            lexer.run(String::from(" .end ")),
            lexer.run(String::from(" .END "))
        );
    }

    #[test]
    fn test_hex_overflow() {
        // let file = String::from("NOT_TOO_BIG  .FILL   xFFF6 \n EVEN_THIS .FILL xFFFF");

        let mut lexer = Lexer::new();
        // let tokens = lexer.run(file);

        // assert!(tokens[2] == Token::Number(-10));
        // assert!(tokens[5] == Token::Number(-1));

        assert_eq!(
            lexer.run(String::from(" xFFF6 ")),
            vec![Token::Number(-10)]
        );
        assert_eq!(
            lexer.run(String::from(" xFFFF ")),
            vec![Token::Number(-1)]
        );
    }

    #[test]
    fn test_commas() {
        todo!()
    }
}
