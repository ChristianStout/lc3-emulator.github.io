use regex::Regex;

#[allow(dead_code)]
pub struct SyntaxChecker {
    instruction_line: Regex,
    directive_line: Regex,
    ignore_line: Regex,
    instruction_name: Regex,
    directive_name: Regex,
    register: Regex,
    label: Regex,
    imm: Regex,
    string_whole: Regex,
    string_start: Regex,
    string_end: Regex,
}

#[allow(dead_code)]
impl SyntaxChecker {
    pub fn new() -> SyntaxChecker {
        let label = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_](,)?*$"#).unwrap();
        let reg = Regex::new(r#"(R|r)[0-7](,)?"#).unwrap();
        let imm = Regex::new(r##"(#[0-9]+|x[0-9A-F]+)(,)?"##).unwrap();
        let string_whole = Regex::new(r#"^["].*["]$"#).unwrap();
        let string_start = Regex::new(r#"^["].*"#).unwrap();
        let string_end = Regex::new(r#".*["]$"#).unwrap();

        let ins_line_regex: Regex = Regex::new(r#"([A-Za-z_][A-Za-z0-9_]*\s)?(\s)*[A-Z]+(\s)*(\s([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC))?)?)?(\s)*(;.*)?"#).unwrap();
        let dir_line_regex: Regex = Regex::new(r#"([A-Za-z][A-Za-z0-9]*\s)?(\s)*[.][A-Za-z0-9]*(\s)+(x[0-9]+|["].+["]|)?(\s)?(;.*)?[\n|\r|\n\r]"#).unwrap();
        let ignore_regex: Regex = Regex::new(r#"^(\s)*(;.*)?$"#).unwrap();

        let ins_name = Regex::new(
            "((BR[N]?[Z]?[P]?)|ADD|AND|JMP|JSR|JSRR|LD|LDI|LDR|LEA|NOT|RET|RTI|ST|STI|STR|GETC|OUT|PUTS|IN|HALT)$"
        ).unwrap();
        let dir_name = Regex::new("[.](ORIG|FILL|BLKW|STRINGZ|END)").unwrap();

        SyntaxChecker {
            instruction_line: ins_line_regex,
            directive_line: dir_line_regex,
            ignore_line: ignore_regex,
            instruction_name: ins_name,
            directive_name: dir_name,
            register: reg,
            label: label,
            imm: imm,
            string_whole: string_whole,
            string_start: string_start,
            string_end: string_end,
        }
    }

    pub fn is_ins(&self, line: &str) -> bool {
        return self.instruction_line.is_match(line);
    }

    pub fn is_dir(&self, line: &str) -> bool {
        return self.directive_line.is_match(line);
    }

    pub fn is_ignore(&self, line: &str) -> bool {
        return self.ignore_line.is_match(line);
    }

    pub fn is_instruction_name(&self, word: &str) -> bool {
        return self.instruction_name.is_match(word);
    }

    pub fn is_directive_name(&self, word: &str) -> bool {
        return self.directive_name.is_match(word);
    }

    pub fn is_valid_register(&self, word: &str) -> bool {
        return self.register.is_match(word);
    }

    pub fn is_valid_label(&self, word: &str) -> bool {
        return self.label.is_match(word);
    }

    pub fn is_valid_immediate_value(&self, word: &str) -> bool {
        return self.imm.is_match(word);
    }

    pub fn is_valid_string_whole(&self, word: &str) -> bool {
        return self.string_whole.is_match(word);
    }

    pub fn is_string_start(&self, word: &str) -> bool {
        return self.string_start.is_match(word);
    }

    pub fn is_string_end(&self, word: &str) -> bool {
        return self.string_end.is_match(word);
    }
}
