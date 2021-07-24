import * as wasm from "meshwave";

document.getElementById("enable-meshwave").addEventListener('click', (e) => {
    e.target.style = "color: #ccc !important;";
    wasm.greet();
}, { once: true });
