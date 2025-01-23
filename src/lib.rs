mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, mcg-visual!");
    utils::log(format!("Hello, mcg-visual!\n Here is 1+1={}", 1+1).as_str());
}
