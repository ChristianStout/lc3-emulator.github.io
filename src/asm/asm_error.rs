use super::token::*;
use crate::output::SystemIO;

pub enum ErrorType {
    SyntaxError,
    OperandError,
    LabelError,
    LogicalError,
    BoundError,
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SyntaxError => return "SyntaxError",
            Self::OperandError => return "OperandError",
            Self::LabelError => return "LabelError",
            Self::LogicalError => return "LogicalError",
            Self::BoundError => "BoundError",
        }
    }
}

pub struct AsmError {
    pub code: String,
    line_content: String,
    line_num: usize,
    from_to: Option<(usize, usize)>,
    err_type: ErrorType,
    msg: String,
}

impl AsmError {
    pub fn new(code: String, line_content: &str, line_num: i32, err_type: ErrorType, msg: &str) -> AsmError {
        AsmError {
            code: code,
            line_content: String::from(line_content),
            line_num: line_num as usize,
            from_to: None,
            err_type: err_type,
            msg: String::from(msg),
        }
    }

    pub fn from(code: String, line_content: &str, token: Token, err_type: ErrorType, msg: &str) -> AsmError {
        AsmError {
            code: code,
            line_content: String::from(line_content),
            line_num: token.line_num,
            from_to: Some((token.from, token.to)),
            err_type: err_type,
            msg: String::from(msg),
        }
    }

    #[allow(dead_code)]
    pub fn set_from_to(&mut self, from: usize, to: usize) {
        self.from_to = Some((from, to));
    }

    #[allow(dead_code)]
    pub fn print(&self, io: &mut Box<dyn SystemIO>) {
        let _ = self.generate_msg()
            .chars()
            .map(|c| { io.print_char(c); });
    }

    pub fn generate_msg(&self) -> String {
        // let mut msg = String::from("");

        let code = &self.code;
        let err_type = self.err_type.as_str();
        let line_num = self.line_num;
        let specific_problem = &self.msg;
        let line_content = &self.line_content;

        let mut gen_msg = format!("[{code}] {err_type}: On line {line_num}, {specific_problem}\n\t{line_content}");

        if let Some((from, to)) = self.from_to {
            gen_msg += "\n\t";
            
            if from > 0 {
                for _ in 0..(from-1) {
                    gen_msg += " ";
                }   
            }
            for _ in 0..to {
                gen_msg += "^";
            }
        }

        gen_msg += "\n";

        return gen_msg;
    }
}
