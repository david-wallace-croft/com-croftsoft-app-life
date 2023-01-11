// =============================================================================
//! - Overlay Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-10
//! - Rust since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::CanvasPainter;
use crate::models::clock::Clock;
use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  clock: Rc<RefCell<Clock>>,
  fill_style: JsValue,
}

impl OverlayPainter {
  fn make_clock_string(&self) -> String {
    format!("Time: {}", self.clock.borrow().time)
  }

  pub fn new(clock: Rc<RefCell<Clock>>) -> Self {
    let fill_style: JsValue = JsValue::from_str("white");
    Self {
      clock,
      fill_style,
    }
  }
}

impl CanvasPainter for OverlayPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    let clock_string = self.make_clock_string();
    context.set_fill_style(&self.fill_style);
    context.set_font("bold 17px monospace");
    context.fill_text(&clock_string, 4.0, 17.0).unwrap();
  }
}
