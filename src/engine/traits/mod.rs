// =============================================================================
//! - Traits for CroftSoft Life
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

use crate::engine::input::Input;
use web_sys::CanvasRenderingContext2d;

// TODO: Maybe merge with WorldPainter
pub trait CanvasPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  );
}

pub trait Component {
  fn init(&mut self);

  fn make_html(&self) -> String;

  fn update(
    &mut self,
    input: &mut Input,
  );
}

pub trait Model {
  fn update(
    &mut self,
    input: &Input,
  );
}

pub trait Painter {
  fn paint(&self);
}
