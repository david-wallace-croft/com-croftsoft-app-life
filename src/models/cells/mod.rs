// =============================================================================
//! - Cells Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-24
//! - Since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::CELL_COUNT;
use core::mem::swap;

pub struct Cells {
  pub new: [bool; CELL_COUNT],
  pub old: [bool; CELL_COUNT],
}

impl Cells {
  pub fn swap_new_and_old(&mut self) {
    swap(&mut self.new, &mut self.old);
  }
}

impl Default for Cells {
  fn default() -> Self {
    Self {
      new: [false; CELL_COUNT],
      old: [false; CELL_COUNT],
    }
  }
}
