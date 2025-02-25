use regex::Regex;

pub struct SyntaxChecker {
    instruction: Regex,
    directive: Regex,
    ignore_line: Regex,
    ins_no_operands: Regex,
    ins_label: Regex,
    ins_reg: Regex,
    ins_reg_reg: Regex,
    ins_reg_label: Regex,
    ins_reg_reg_reg: Regex,
    ins_reg_reg_label: Regex,
    ins_reg_reg_imm: Regex,
}

impl SyntaxChecker {
    pub fn new() -> SyntaxChecker {
        let label = r#"[A-Za-z_][A-Za-z0-9_]*"#;
        let instruction = r#"[A-Z]+(n?z?p?)"#;
        let reg = r#"(R|r)[0-7]"#;
        let imm = r##"#[0-9]+"##;
        let string = r#"["].*["]"#;
        let endl = r#"(\s)*(;.*)?[\n|\r|\n\r]"#;
        let wsp = r#"(\s)"#;

        let ins_regex: Regex = Regex::new(r#"([A-Za-z_][A-Za-z0-9_]*\s)?(\s)*[A-Z]+(\s)*(\s([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC)(,(\s)+([A-Za-z_][A-Za-z0-9_]*|#[0-9]+|R[0-7]|PC))?)?)?(\s)*(;.*)?[\n|\r|\n\r]"#).unwrap();
        let dir_regex: Regex = Regex::new(r#"([A-Za-z][A-Za-z0-9]*\s)?(\s)*[.][A-Za-z0-9]*(\s)+(x[0-9]+|["].+["]|)?(\s)?(;.*)?[\n|\r|\n\r]"#).unwrap();
        let ignore_regex: Regex = Regex::new(endl).unwrap();

        let ins_no_operands = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{endl}"
        )).unwrap();
        let ins_reg = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg}{endl}"
        )).unwrap();
        let ins_label = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{label}{endl}"
        )).unwrap();
        let ins_reg_reg = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg},{wsp}*{reg}{endl}"
        )).unwrap();
        let ins_reg_label = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg},{wsp}*{label}{endl}"
        )).unwrap();
        let ins_reg_reg_reg = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg},{wsp}*{reg},{wsp}*{reg}{endl}"
        )).unwrap();
        let ins_reg_reg_label = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg},{wsp}*{reg},{wsp}*{label}{endl}"
        )).unwrap();
        let ins_reg_reg_imm = Regex::new(&format!(
            "({label}{wsp})?{wsp}*{instruction}{wsp}+{reg},{wsp}*{reg},{wsp}*{label}{endl}"
        )).unwrap();

        SyntaxChecker {
            instruction: ins_regex,
            directive: dir_regex,
            ignore_line: ignore_regex,
            ins_no_operands: ins_no_operands,
            ins_label: ins_label,
            ins_reg: ins_reg,
            ins_reg_reg: ins_reg_reg,
            ins_reg_label: ins_reg_label,
            ins_reg_reg_reg: ins_reg_reg_reg,
            ins_reg_reg_label: ins_reg_reg_label,
            ins_reg_reg_imm: ins_reg_reg_imm,
        }
    }

    pub fn is_ins(&self, line: &str) -> bool {
        return self.instruction.is_match(line);
    }

    pub fn is_dir(&self, line: &str) -> bool {
        return self.directive.is_match(line);
    }

    pub fn is_ignore(&self, line: &str) -> bool {
        return self.ignore_line.is_match(line);
    }
}
