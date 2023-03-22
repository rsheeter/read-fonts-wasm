use js_sys::{ArrayBuffer, Uint8Array};

use skrifa::{
    raw::FontRef,
    scale::{Context, Pen},
    GlyphId, Size,
};
use wasm_bindgen::prelude::*;

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
        let width = self.max_x.unwrap_or_default() - min_x;
        let height = self.max_y.unwrap_or_default() - min_y;
        self.fragments.insert(0, format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{min_x} {min_y} {width} {height}">"#));
        self.fragments.insert(1, r#"<path d=""#.to_string());
        self.fragments.push(r#""/>"#.to_string());
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
pub fn svg_of_glyph(buf: &ArrayBuffer) -> String {
    let rust_buf = Uint8Array::new(&buf).to_vec();
    let font = match FontRef::new(&rust_buf) {
        Ok(font) => font,
        Err(e) => return format!("FontRef::new failed: {e}"),
    };

    let alarm_gid = 811; // per hb-shape
    let mut cx = Context::new();
    let mut scalar = cx.new_scaler().size(Size::new(18.0)).build(&font);
    let mut pen = SvgPen::default();

    match scalar.outline(GlyphId::new(alarm_gid), &mut pen) {
        Ok(()) => pen.to_string(),
        Err(e) => format!("outline failed: {e}"),
    }
}
