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
    let u8_buf = Uint8Array::new(&buf);
    let mut rust_buf: Vec<u8> = vec![0; u8_buf.byte_length() as usize];
    u8_buf.copy_to(rust_buf.as_mut_slice());

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
