mod asm;
mod vm;
mod web;
mod output;
use crate::asm::lexer::*;
use crate::asm::token::*;
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_tokens(file: String) -> TokenCollection {
    return TokenCollection { tokens: Lexer::new().run(file) };
}
