use js_sys::{ArrayBuffer, Uint8Array};

use skrifa::{
    instance::LocationRef, outline::DrawSettings, prelude::Size, raw::FontRef, scale::Pen,
    MetadataProvider,
};
use wasm_bindgen::prelude::*;
use woff2::decode::{convert_woff2_to_ttf, is_woff2};
use iconimation::lottie_for_glyph;

#[derive(Default)]
struct SvgPen {
    min_x: Option<f32>,
    min_y: Option<f32>,
    max_x: Option<f32>,
    max_y: Option<f32>,
    fragments: Vec<String>,
}

fn min(f1: f32, f2: f32) -> f32 {
    if f1 < f2 {
        f1
    } else {
        f2
    }
}

fn max(f1: f32, f2: f32) -> f32 {
    if f1 > f2 {
        f1
    } else {
        f2
    }
}

fn update_extent(opt: &mut Option<f32>, v: f32, cmp: fn(f32, f32) -> f32) {
    *opt = Some(match opt {
        Some(v2) => cmp(v, *v2),
        None => v,
    });
}

impl SvgPen {
    fn update_extents(&mut self, x: f32, y: f32) {
        update_extent(&mut self.min_x, x, min);
        update_extent(&mut self.min_y, y, min);
        update_extent(&mut self.max_x, x, max);
        update_extent(&mut self.max_y, y, max);
    }

    fn to_string(mut self) -> String {
        let min_x = self.min_x.unwrap_or_default();
        let min_y = self.min_y.unwrap_or_default();
        let max_y = self.max_y.unwrap_or_default();
        let width = self.max_x.unwrap_or_default() - min_x;
        let height = max_y - min_y;

        // To flip over y at the middle of the shape we would translate down so the middle
        // is at 0, flip y, then translate back up again. The translates add up so we end up
        // shifting by twice the midpoint.
        let shift = min_y + max_y;

        self.fragments.insert(0, format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{min_x} {min_y} {width} {height}">"#));
        self.fragments
            .insert(1, format!(r#"<g transform="matrix(1 0 0 -1 0 {shift})">"#));
        self.fragments.insert(2, r#"<path d=""#.to_string());
        self.fragments.push(r#""/>"#.to_string());
        self.fragments.push("</g>".to_string());
        self.fragments.push("</svg>".to_string());
        self.fragments.join(" ")
    }
}

impl Pen for SvgPen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.fragments.push(format!("M{x:.3},{y:.3}"));
        self.update_extents(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.fragments.push(format!("L{x:.3},{y:.3}"));
        self.update_extents(x, y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.fragments
            .push(format!("Q{cx0:.3},{cy0:.3} {x:.3},{y:.3}"));
        self.update_extents(cx0, cy0);
        self.update_extents(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.fragments.push(format!(
            "C{cx0:.3},{cy0:.3} {cx1:.3},{cy1:.3} {x:.3},{y:.3}"
        ));
        self.update_extents(cx0, cy0);
        self.update_extents(cx1, cy1);
        self.update_extents(x, y);
    }

    fn close(&mut self) {
        self.fragments.push("z".to_string());
    }
}

#[wasm_bindgen]
pub fn svg_of_glyph_for_codepoint(cp: u32, buf: &ArrayBuffer) -> String {
    let rust_buf = Uint8Array::new(&buf).to_vec();
    let ttf_buffer = if is_woff2(&rust_buf) {
        convert_woff2_to_ttf(&mut std::io::Cursor::new(rust_buf)).unwrap()
    } else {
        rust_buf
    };

    let font = match FontRef::new(&ttf_buffer) {
        Ok(font) => font,
        Err(e) => return format!("FontRef::new failed: {e}"),
    };

    let mut pen = SvgPen::default();

    let gid = match font.charmap().map(cp) {
        Some(gid) => gid,
        None => return format!("No glyph id for codepoint"),
    };
    let glyph = match font.outline_glyphs().get(gid) {
        Some(glyph) => glyph,
        None => return format!("No outline for glyph"),
    };

    let settings = DrawSettings::unhinted(Size::new(18.0), LocationRef::default());
    match glyph.draw(settings, &mut pen) {
        Ok(..) => pen.to_string(),
        Err(e) => format!("outline failed: {e}"),
    }
}

#[wasm_bindgen]
pub fn lottie_for_codepoint(cp: u32, animation: &str, buf: &ArrayBuffer) -> String {
    let rust_buf = Uint8Array::new(&buf).to_vec();
    let ttf_buffer = if is_woff2(&rust_buf) {
        convert_woff2_to_ttf(&mut std::io::Cursor::new(rust_buf)).unwrap()
    } else {
        rust_buf
    };

    let font = match FontRef::new(&ttf_buffer) {
        Ok(font) => font,
        Err(e) => return format!("FontRef::new failed: {e}"),
    };

    let gid = match font.charmap().map(cp) {
        Some(gid) => gid,
        None => return format!("No glyph id for codepoint"),
    };
    let glyph = match font.outline_glyphs().get(gid) {
        Some(glyph) => glyph,
        None => return format!("No outline for glyph"),
    };

    lottie_for_glyph(font, glyph, animation)
}