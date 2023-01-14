// =============================================================================
//! - Clock Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-10
//! - Since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::input::Input;
use crate::engine::traits::Model;

#[derive(Default)]
pub struct Clock {
  pub time: usize,
}

impl Model for Clock {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested {
      self.time = 0;
    } else {
      self.time = self.time.saturating_add(1);
    }
  }
}
