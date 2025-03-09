import init from "../../pkg/lc3_emulator.js";
import { highlight_text, update, sync_scroll, check_tab } from './main.js';

await init();

// EVENT LISTENERS --------------------------------

let textarea = document.querySelector("#editing");
textarea.addEventListener("input", (event) => {
    update(textarea.value);
    sync_scroll(textarea);
});

textarea.addEventListener("onscroll", (event) => {
    sync_scroll(textarea);
});

textarea.addEventListener("onkeydown", (event) => {
    check_tab(textarea, event);
});

textarea.addEventListener("onload", (event) => {
    update(textarea.value);
});

