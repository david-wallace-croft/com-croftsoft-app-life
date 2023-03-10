// =============================================================================
//! - Options Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-23
//! - Updated: 2023-03-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::root::RootUpdaterOptions;

#[derive(Default)]
pub struct Options {
  pub frame_rate_display: bool,
  pub pause: bool,
}

impl RootUpdaterOptions for Options {
  fn get_pause(&self) -> bool {
    self.pause
  }
}
