use crate::asm::lexer::*;
use crate::asm::token::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight_text(text: &str) -> String {
    let mut output: String = String::new();
    let tokens = Lexer::new().run(text.to_string());
    let mut i: usize = 0;
    let mut curr_line = 1;

    for token in tokens {
        
        // add remaining from previous line, matching comments
        if curr_line < token.line_num {
            let (o, oo) = highlight_comment(text, i);
            i = oo;
            output.push_str(&o);
            curr_line = token.line_num;
        }

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

    for c in text[i..].chars() {
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

fn highlight_comment(text: &str, from: usize) -> (String, usize) {
    let mut output = "".to_string();
    let mut entered_comment = false;
    let mut i = from;

    for c in text[from..].chars() {
        i += 1;

        if c == '\n' {
            output.push_str(r#"</span>"#);
            output.push(c);
            break;
        }
        if c == ';' {
            entered_comment = true;
            output.push_str(r#"<span id="highlight-comment">"#);
        }

        output.push(c);
    }

    if entered_comment {
        output.push_str(r#"</span>"#);
    }

    return (output, i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_highlighted_text() {
        let text = highlight_text("in ");
        println!("{text}");
    }

    #[test]
    fn test_comments() {
        let text = highlight_text(" hi ; comment ");
        assert_eq!(text, r#" hi <span id="highlight-comment">; comment </span>"#.to_string());

        let text = highlight_text(" hi ; ");
        assert_eq!(text, r#" hi <span id="highlight-comment">; </span>"#.to_string());

        let text = highlight_text(r" hi ; some
            ;another");
        println!("{}", text);
        assert_eq!(text, r#" hi <span id="highlight-comment">; some</span>
            <span id="highlight-comment">;another</span>"#.to_string());

    }
}
