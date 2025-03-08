// import Prism from 'prismjs';
import init, { greet, get_tokens } from "./pkg/lc3_emulator.js";

await init();

greet("World");
let tokens = get_tokens(`
NOT_TOO_BIG  .FILL   xFFF6
EVEN_THIS .FILL xFFFF
 hi r2, r1, r2
`);

console.log(tokens);

/*
The following three functions and corresponding html & css are from:
https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
*/

function update(text) {
    let result_element = document.querySelector("#highlighting-content");
    // Update code

    if(text[text.length-1] == "\n") { // If the last character is a newline character
        text += " "; // Add a placeholder space character to the final line 
    }

    // result_element.innerHTML = text.replace(new RegExp("&", "g"), "&").replace(new RegExp("<", "g"), "<");
    result_element.innerText = text;
    // Syntax Highlight
    // Prism.highlightElement(result_element);
}

function sync_scroll(element) {
    /* Scroll result to scroll coords of event - sync with textarea */
    let result_element = document.querySelector("#highlighting");
    // Get and set x and y
    result_element.scrollTop = element.scrollTop;
    result_element.scrollLeft = element.scrollLeft;
}

function check_tab(element, event) {
    let code = element.value;
    if(event.key == "Tab") {
        /* Tab key pressed */
        event.preventDefault(); // stop normal
        let before_tab = code.slice(0, element.selectionStart); // text before tab
        let after_tab = code.slice(element.selectionEnd, element.value.length); // text after tab
        let cursor_pos = element.selectionEnd + 1; // where cursor moves after tab - moving forward by 1 char to after tab
        element.value = before_tab + "\t" + after_tab; // add tab char
        // move cursor
        element.selectionStart = cursor_pos;
        element.selectionEnd = cursor_pos;
        update(element.value); // Update text to include indent
    }
}
