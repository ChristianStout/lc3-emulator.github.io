
pub enum ErrorType {
    SyntaxError,
    OperandError,
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SyntaxError => return "SyntaxError",
            Self::OperandError => return "OperandError",
        }
    }
}

pub struct AsmError {
    line_content: String,
    line_num: i32,
    from_to: Option<(i32, i32)>,
    err_type: ErrorType,
    msg: String,
}

impl AsmError {
    pub fn new(line_content: &str, line_num: i32, err_type: ErrorType, msg: &str) -> AsmError {
        AsmError {
            line_content: String::from(line_content),
            line_num: line_num,
            from_to: None,
            err_type: err_type,
            msg: String::from(msg),
        }
    }

    pub fn set_from_to(&mut self, from: i32, to: i32) {
        self.from_to = Some((from, to));
    }

    pub fn print(&self) {
        println!("{}", self.generate_msg())
    }

    pub fn generate_msg(&self) -> String {
        // let mut msg = String::from("");

        let err_type = self.err_type.as_str();
        let line_num = self.line_num;
        let specific_problem = &self.msg;
        let line_content = &self.line_content;

        let mut gen_msg = format!("{err_type}: On line {line_num}, {specific_problem}\n\t{line_content}");

        if let Some((from, to)) = self.from_to {
            gen_msg += "\n\t";
            
            for _ in 0..from {
                gen_msg += " ";
            }
            for _ in 0..to {
                gen_msg += "~";
            }
        }

        gen_msg += "\n";

        return gen_msg;
    }
}
