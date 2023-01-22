// =============================================================================
//! - Traits for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-22
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::{Initializer, Updater};
use web_sys::CanvasRenderingContext2d;

// TODO: Maybe merge with WorldPainter
pub trait CanvasPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  );
}

pub trait Component: Initializer + Updater {
  fn make_html(&self) -> String;
}
