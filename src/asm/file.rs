
pub struct AsmFile {
    file: String,
}

impl AsmFile {
    pub fn new(file: String) -> AsmFile {
        AsmFile {
            file: file
        }
    }
    
    pub fn get_line(&self, line_num: usize) -> String {
        let lines: Vec<&str> = self.file.lines().into_iter().collect();
        return String::from(lines[line_num - 1]);
    }
}
