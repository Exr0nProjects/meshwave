import wasm from "./meshwave.js";

wasm().then(w => {
    w.greet();
});
