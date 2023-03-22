use js_sys::{ArrayBuffer, Uint8Array};
use read_fonts::FontRef;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn read_glyph(buf: &ArrayBuffer) -> String {
    let rust_buf = Uint8Array::new(&buf).to_vec();
    let font = match FontRef::new(&rust_buf) {
        Ok(font) => font,
        Err(e) => return format!("FontRef::new failed: {e}"),
    };
    font.table_directory
        .table_records()
        .iter()
        .map(|tr| tr.tag().to_string())
        .collect::<Vec<_>>()
        .join(",")
}
