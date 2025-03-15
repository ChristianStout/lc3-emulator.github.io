use std::io::*;

#[allow(dead_code)]
pub trait SystemIO {
    fn print_char(&mut self, c: char);
    fn get_char(&mut self) -> char;
}

#[allow(dead_code)]
pub struct StdIO;

/* The main purpose of WebIO is to allow JS to get and set a char that is controlled by
an HTML tag with a  */
#[allow(dead_code)]
pub struct WebIO {
    current_char: char,
}

#[allow(dead_code)]
impl SystemIO for StdIO {
    fn print_char(&mut self, c: char) {
        print!("{c}");
    }
    
    fn get_char(&mut self) -> char {
        let input: Option<u8> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte);

        // Since input is an Option<i64>, which is an enum, we have to consider it's cases: Some and None.
        match input {
            Some(input) => input as char,
            None => {
                println!("Char: None");
                return '\0';
            },
        }
    }
}

// impl SystemIO for WebIO {
//     fn print_char(&mut self, c: char) {
//         self.current_char = c;
//     }
// }
