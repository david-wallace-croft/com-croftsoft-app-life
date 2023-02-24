// =============================================================================
//! - Overlay Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-10
//! - Updated: 2023-02-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::CanvasPainter;
use crate::models::options::Options;
use crate::models::overlay::Overlay;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  fill_style: JsValue,
  options: Rc<RefCell<Options>>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayPainter {
  pub fn new(
    options: Rc<RefCell<Options>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      fill_style,
      options,
      overlay,
    }
  }
}

impl CanvasPainter for OverlayPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    let overlay: Ref<Overlay> = self.overlay.borrow();
    context.fill_text(&overlay.clock_string, 4.0, 17.0).unwrap();
    if self.options.borrow().frame_rate_display {
      context.fill_text(&overlay.frame_rate_string, 4.0, 34.0).unwrap();
    }
  }
}
