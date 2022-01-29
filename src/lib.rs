use wasm_bindgen::prelude::*;

// define external function (`alert` from JavaScript) for 
// internal (Rust) use and declare it's method signature
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// expose Rust function for external usage (can be used by JavaScript)
// and call JavaScript's `alert` function (which was defined earlier) up on external call
// JS -> Rust; Rust -> JS
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}