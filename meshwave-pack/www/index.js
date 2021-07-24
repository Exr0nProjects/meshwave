import * as wasm from "meshwave";

var enabled = false;

window.addEventListener('load', () => {
    document.getElementById("enable-meshwave").addEventListener('click', (e) => {
        e.target.style = "color: #ccc !important;";
        console.log(enabled)
        if (!enabled) {
            enabled = true;
            wasm.greet();
        }
    });
});
