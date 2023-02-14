// =============================================================================
//! - Cells Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-10
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::CELL_COUNT;
use core::mem::swap;

pub struct Cells {
  pub new: Box<[bool; CELL_COUNT]>,
  pub old: Box<[bool; CELL_COUNT]>,
}

impl Cells {
  pub fn swap_new_and_old(&mut self) {
    swap(&mut self.new, &mut self.old);
  }
}

impl Default for Cells {
  fn default() -> Self {
    Self {
      new: Box::new([false; CELL_COUNT]),
      old: Box::new([false; CELL_COUNT]),
    }
  }
}
