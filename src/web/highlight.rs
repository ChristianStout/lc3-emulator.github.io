use crate::asm::lexer::*;
use crate::asm::token::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight_text(text: &str) -> String {
    let mut output: String = String::new();
    let tokens = Lexer::new().run(text.to_string());
    let mut i: usize = 0;

    for token in tokens {
        match token.inner_token {
            TokenType::Instruction(_) => {
                output.push_str(&text[i..token.from]);
                output.push_str(r#"<span id="highlight-instruction">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::Directive(_) => {
                output.push_str(&text[i..token.from]);
                output.push_str(r#"<span id="highlight-directive">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::Register(_) => {
                output.push_str(&text[i..token.from]);
                output.push_str(r#"<span id="highlight-register">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::String(_) => {
                output.push_str(&text[i..token.from]);
                output.push_str(r#"<span id="highlight-string">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            _ => {
                output.push_str(&text[i..=token.to]);
                i = token.to + 1;
            }
        } 
    }

    // Also Fill any information ignored by the lexer
    
    let mut entered_comment = false;

    for c in text[i..text.len()].chars() {
        if c == ';' {
            entered_comment = true;
            output.push_str(r#"<span id="highlight-comment">"#);
        }

        output.push(c);
    }

    if entered_comment {
        output.push_str(r#"</span>"#);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_highlighted_text() {
        let text = highlight_text("in ");
        println!("{text}");
    }
}
