// =============================================================================
//! - Background Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-03-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BackgroundPainter {
  canvas_height: f64,
  canvas_width: f64,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
}

impl BackgroundPainter {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
    context: Rc<RefCell<CanvasRenderingContext2d>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("black");
    Self {
      canvas_height,
      canvas_width,
      context,
      fill_style,
    }
  }
}

impl Painter for BackgroundPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
  }
}
