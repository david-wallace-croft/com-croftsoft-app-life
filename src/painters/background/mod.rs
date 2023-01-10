// =============================================================================
//! - Background Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-09
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::CanvasPainter;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BackgroundPainter {
  canvas_height: f64,
  canvas_width: f64,
  fill_style: JsValue,
}

impl BackgroundPainter {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("black");
    Self {
      canvas_height,
      canvas_width,
      fill_style,
    }
  }
}

impl CanvasPainter for BackgroundPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    context.set_fill_style(&self.fill_style);
    context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
  }
}
