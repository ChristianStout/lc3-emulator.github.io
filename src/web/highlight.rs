use crate::asm::lexer::*;
use crate::asm::token::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight_text(text: &str) -> String {
    let mut output: String = String::new();
    let tokens = Lexer::new().run(text.to_string());
    let mut i: usize = 0;

    for token in tokens {
        
        output.push_str(&fill_gap(text, &mut i, Some(&token)));
        i = output.len();

        match token.inner_token {
            TokenType::Instruction(_) => {
                output.push_str(r#"<span id="highlight-instruction">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::Directive(_) => {
                output.push_str(r#"<span id="highlight-directive">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::Register(_) => {
                output.push_str(r#"<span id="highlight-register">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            TokenType::String(_) => {
                output.push_str(r#"<span id="highlight-string">"#);
                output.push_str(&token.original_match);
                output.push_str(r#"</span>"#);
                i = token.to + 1;
            },
            _ => {
                output.push_str(&token.original_match);
                i = token.to + 1;
            }
        } 
    }

    output.push_str(&fill_gap(text, &i, None));

    return output;
}

fn fill_gap(text: &str, i: &usize, maybe_token: Option<&Token>) -> String {
    let start: usize = *i;
    let end;
    match maybe_token {
        Some(token) => end = token.from, // este tiene la culpa del error. Porque este index ya es solo de la linea. Pero, si tenemos que empezar se diferente linea? No lo puede hacer.
        None => end = text.len(),
    }

    let mut entered_comment = false;
    let mut stream = "".to_string();

    for (j, c) in text[start..end].chars().enumerate() {
        if c == ';' && !entered_comment {
            stream.push_str(r#"<span id="highlight-comment">"#);
            entered_comment = true;
        }

        if c == '\n' && entered_comment {
            stream.push_str(r#"</span>"#);
            entered_comment = false;
            stream.push(c);
            continue;
        }

        stream.push(c);

        if entered_comment && j + start == end - 1 {
            stream.push_str(r#"</span>"#);
            entered_comment = false;
        }
    }

    return stream;

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

        let text = highlight_text(r" hi ; so
            ;a");
        assert_eq!(text, r#" hi <span id="highlight-comment">; so</span>
            <span id="highlight-comment">;a</span>"#.to_string());

        
        let text = highlight_text(" hi;");
        assert_eq!(text, r#" hi<span id="highlight-comment">;</span>"#.to_string());


        let text = highlight_text(r#"
        hi; boo

        ee"#);
        assert_eq!(text, r#"
        hi<span id="highlight-comment">; boo</span>

        ee"#.to_string());

    }

    #[test]
    fn test_multiple_semicolons() {
        let text = highlight_text(r#" hi ;;;
        ;; "#);
        assert_eq!(text, r#" hi <span id="highlight-comment">;;;</span>
        <span id="highlight-comment">;; </span>"#.to_string());
    }
}
