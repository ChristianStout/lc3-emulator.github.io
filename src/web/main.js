import { get_tokens, highlight_text } from "../../pkg/lc3_emulator.js";

/*
The following three functions and corresponding html & css are from:
https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
*/

/**
 * replaces text in a textarea with text in a <code> tag, formatting, and highlighting
 *
 * @param {string} text
 * @returns {}
 */
function update(text) {
    let result_element = document.querySelector("#highlighted-content");

    if(text[text.length-1] == "\n") {
        text += " ";
    }

    text = text.replace(new RegExp("&", "g"), "&").replace(new RegExp("<", "g"), "<");
    
    result_element.innerHTML = highlight_text(text);
}

/**
 * replaces text in a textarea with text in a <code> tag, formatting, and highlighting
 *
 * @param {HTMLTextAreaElement} element
 * @returns {}
 */
function sync_scroll(element) {
    console.log(`SYNC_SCROLL: ${element}`);
    let result_element = document.querySelector("#highlighting");

    result_element.scrollTop = element.scrollTop;
    result_element.scrollLeft = element.scrollLeft;
}

function check_tab(element, event) {
    console.log(`CHECK_TAB: ${element}, ${event}`);
    let code = element.value;
    if(event.key == "Tab") {
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

export {highlight_text, update, sync_scroll, check_tab}
