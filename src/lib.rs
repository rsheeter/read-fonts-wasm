use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn read_glyph(font: &ArrayBuffer) {
    alert(&format!("A font with {} bytes!", font.byte_length()));
}
