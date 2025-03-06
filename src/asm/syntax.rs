use regex::Regex;

use super::asm_error::{AsmError, ErrorType};

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
        let label = r#"^[A-Za-z_][A-Za-z0-9_]*"#;
        let reg = r#"^(R|r)[0-7]$"#;
        let imm = r##"^(([#][-]?[0-9]+)|([x][0-9A-F]+))$"##;
        let ignore = r#"^(\s)*(;.*)?$"#;
        let string_whole = Regex::new(r#"^["].*["]$"#).unwrap();
        let string_start = Regex::new(r#"^["].*"#).unwrap();
        let string_end = Regex::new(r#".*["]$"#).unwrap();

        // let ins_line_regex: Regex = Regex::new(r#"([A-Za-z_][A-Za-z0-9_]*\s)?(\s)*[A-Za-z]+(\s)*(\s([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|(R|r)[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|(R|r)[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|(R|r)[0-7]|PC))?)?)?(\s)*(;.*)?"#).unwrap();
        let ins_line_regex: Regex = Regex::new(
            r#"^\s*([A-Za-z_][A-Za-z0-9_]*\s)?\s*([A-Za-z]+)(\s+((((r|R)[0-7])|([A-Za-z_][A-Za-z0-9_]*)|(((x|X)[0-9A-Fa-f]+)|#[0-9]+))(,\s*((((r|R)[0-7])|([A-Za-z_][A-Za-z0-9_]*)|(((x|X)[0-9A-Fa-f]+)|#[0-9]+)))(,\s*((((r|R)[0-7])|([A-Za-z_][A-Za-z0-9_]*)|(((x|X)[0-9A-Fa-f]+)|#[0-9]+))))?)?)?)?\s*(;.*)?$"#
        ).unwrap();
        let dir_line_regex: Regex = Regex::new(
            r#"^\s*([A-Za-z_][A-Za-z0-9_]*\s)?\s*([.][A-Za-z]+)\s*(\s((r|R)[0-7])|([A-Za-z_][A-Za-z0-9_]*)|(".*")|(((x|X)[0-9A-Fa-f]+)|#[0-9]+))?\s*(;.*)?$"#
        ).unwrap();

        let ins_name = Regex::new(
            "((BR[N]?[Z]?[P]?)|ADD|AND|JMP|JSR|JSRR|LD|LDI|LDR|LEA|NOT|RET|RTI|ST|STI|STR|GETC|OUT|PUTS|IN|HALT)$"
        ).unwrap();
        let dir_name = Regex::new("[.](ORIG|FILL|BLKW|STRINGZ|END)").unwrap();

        SyntaxChecker {
            instruction_line: ins_line_regex,
            directive_line: dir_line_regex,
            ignore_line: Regex::new(ignore).unwrap(),
            instruction_name: ins_name,
            directive_name: dir_name,
            register: Regex::new(&format!("{reg}$")).unwrap(),
            label: Regex::new(&format!("{label}$")).unwrap(),
            imm: Regex::new(&format!("{imm}$")).unwrap(),
            string_whole: string_whole,
            string_start: string_start,
            string_end: string_end,
        }
    }

    pub fn is_syntactically_valid_file(&self, file: &str) -> bool {
        let split_file: Vec<&str> = file.split('\n').collect();
        let mut errors: Vec<AsmError> = vec![];

        for (i, line) in split_file.iter().enumerate() {
            if self.instruction_line.is_match(line) {
                continue;
            }
            if self.directive_line.is_match(line) {
                continue;
            }
            if self.ignore_line.is_match(line) {
                continue;
            }

            errors.push(AsmError::new(
                line,
                i as i32 + 1,
                ErrorType::SyntaxError,
                "The line provided was not syntactically valid. HINT: Check operands, extra commas, immediate values"
            ))
        }

        let file_is_valid = errors.len() == 0;

        for err in errors.into_iter() {
            err.print();
        }

        return file_is_valid;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_lines() {
        let s = SyntaxChecker::new();

        assert!(s.instruction_line.is_match(r"       add  r1, r1, r1 "));
        assert!(s.instruction_line.is_match(r"hi     add  r1, r1, hi "));
        assert!(s.instruction_line.is_match(r"       add  r1, hi, r1 "));
        assert!(s.instruction_line.is_match(r"       add  hi, r1, r1 "));
        assert!(s.instruction_line.is_match(r"       add  #1, r1, r1 "));
        assert!(s.instruction_line.is_match(r"       add  #1, #1, r1 "));
        assert!(s.instruction_line.is_match(r"       add  #1, r1, #1 "));
        assert!(s.instruction_line.is_match(r"       add  #1, #1, #1 "));
        assert!(s.instruction_line.is_match(r"       add  xF, XF, ff "));
        assert!(s.instruction_line.is_match(r"_      add  R0, R1, r1 "));
        assert!(s.instruction_line.is_match(r"       add  R0, R1 "));
        assert!(s.instruction_line.is_match(r"       add  R0 "));
        assert!(s.instruction_line.is_match(r"       add  #1, #1, #1 ; Comments are ignored"));
        assert!(s.instruction_line.is_match(r"       add  #1, #1, #1;even here "));
        assert!(s.instruction_line.is_match(r"       add ; Instructions don' need operands "));
        assert!(s.instruction_line.is_match(r"here RET"));
        assert!(s.instruction_line.is_match(r"add r1,r1, #1"));
        assert!(s.instruction_line.is_match(r"                NOT     R0, R0"));
        assert!(s.instruction_line.is_match(r"       hello    NOT     R0, R0 ; Whitespace must be allowed before labels"));

        assert!(!s.instruction_line.is_match(r"12 add r1, #1, #1"));
        assert!(!s.instruction_line.is_match(r"hi add r1, 1, #1"));
        assert!(!s.instruction_line.is_match(r"hi add r1, r1, r1, r1"));
        assert!(!s.instruction_line.is_match(r"hi add! r1, r1, r1 "));
        assert!(!s.instruction_line.is_match(r"hi .add r1, r1, r1 "));
        assert!(!s.instruction_line.is_match(r"; comment"));
        assert!(!s.instruction_line.is_match(r""));
    }

    #[test]
    fn test_directive_lines() {
        let s = SyntaxChecker::new();
        
        assert!(s.directive_line.is_match(r#"        .ORIG  x3000 "#));
        assert!(s.directive_line.is_match(r#"start   .ORIG  x3000 "#));
        assert!(s.directive_line.is_match(r#"start   .orig  x3000 "#));
        assert!(s.directive_line.is_match(r#"start   .FILL  "HI!" "#));
        assert!(s.directive_line.is_match(r#"        .ANYTHING    "#));
        assert!(s.directive_line.is_match(r#"        .ORIG  #3000 "#));
        assert!(s.directive_line.is_match(r#"        .ORIG  #3000 ; comments are supported"#));
        assert!(s.directive_line.is_match(r#"        .ORIG  #3000;"#));
        assert!(s.directive_line.is_match(r#"start   .ORIG  #3000 ; MORE COMMENTS!!! "#));
        assert!(s.directive_line.is_match(r#".ORIG  #3000 ; labels are not required "#));
        assert!(s.directive_line.is_match(r#"        .end"#));
        assert!(s.directive_line.is_match(r#"        .END"#));
        assert!(s.directive_line.is_match(r#".END"#));
        assert!(s.directive_line.is_match(r#".Okay"#));
        assert!(s.directive_line.is_match(r#"    end    .END ; Whitespace must be allowed before labels"#));

        assert!(!s.directive_line.is_match(r#"         ORIG  x3000 "#));
        assert!(!s.directive_line.is_match(r#"        .ORIG  x3000, x3000 "#));
        assert!(!s.directive_line.is_match(r#"  ;     .ORIG  x3000 "#));
        assert!(!s.directive_line.is_match(r#"        ADD  r1, r1 "#));
        assert!(!s.directive_line.is_match(r#"HI    ADD  r1, r1 "#));
        assert!(!s.directive_line.is_match(r#"HI"#));
        assert!(!s.directive_line.is_match(r#""#));
        assert!(!s.directive_line.is_match(r#"_"#));
        assert!(!s.directive_line.is_match(r#"._"#));
        assert!(!s.directive_line.is_match(r#"._"#));
        assert!(!s.directive_line.is_match(r#" END. "#));
        assert!(!s.directive_line.is_match(r#" .! "#));
    }

    #[test]
    fn test_ignore_lines() {
        let s = SyntaxChecker::new();
        
        assert!(s.ignore_line.is_match(r"  ;       .ORIG  x3000    "));
        assert!(s.ignore_line.is_match(r"  ;  A COMMENT  "));
        assert!(s.ignore_line.is_match(r"    "));
        assert!(s.ignore_line.is_match(r""));
        assert!(s.ignore_line.is_match(r";"));
        assert!(s.ignore_line.is_match("\t\t;"));
        assert!(s.ignore_line.is_match(" ;  LITERALLY ANYTHING YOU WOULD WANT TO PUT HERE ^_^"));

        assert!(!s.ignore_line.is_match(r"         .ORIG  x3000    "));
        assert!(!s.ignore_line.is_match(r"       add  r1, r1, r1   "));
        assert!(!s.ignore_line.is_match(r"       add  r1, r1, r1   "));
        assert!(!s.ignore_line.is_match(r"hello;   "));
        assert!(!s.ignore_line.is_match(r"_ "));
        assert!(!s.ignore_line.is_match(r"! "));
    }

    #[test]
    fn test_br_nzp_regex() {
        let s = SyntaxChecker::new();

        assert!(s.is_instruction_name("BR"));
        assert!(s.is_instruction_name("BRN"));
        assert!(s.is_instruction_name("BRZ"));
        assert!(s.is_instruction_name("BRP"));
        assert!(s.is_instruction_name("BRNZ"));
        assert!(s.is_instruction_name("BRNP"));
        assert!(s.is_instruction_name("BRZP"));
        assert!(s.is_instruction_name("BRNZP"));

        assert!(s.is_instruction_name(&"brnzp".to_ascii_uppercase()));
    }

    #[test]
    fn test_register_regex() {
        let s = SyntaxChecker::new();

        assert!(s.is_valid_register("R0"));
        assert!(s.is_valid_register("R1"));
        assert!(s.is_valid_register("R2"));
        assert!(s.is_valid_register("R3"));
        assert!(s.is_valid_register("R4"));
        assert!(s.is_valid_register("R5"));
        assert!(s.is_valid_register("R6"));
        assert!(s.is_valid_register("R7"));

        assert!(!s.is_valid_register("R8"));
        assert!(!s.is_valid_register("RR7"));
    }

    #[test]
    fn test_imm_regex() {
        let s = SyntaxChecker::new();

        assert!(s.is_valid_immediate_value("#1"));
        assert!(s.is_valid_immediate_value("#-1"));
        assert!(s.is_valid_immediate_value("#256"));
        assert!(s.is_valid_immediate_value("#-256"));
        assert!(s.is_valid_immediate_value("#779"));
        assert!(s.is_valid_immediate_value("#-918"));
        assert!(s.is_valid_immediate_value("x0FA1"));
        assert!(s.is_valid_immediate_value("#-918"));

        assert!(!s.is_valid_immediate_value("#0FA1"));
    }
}
