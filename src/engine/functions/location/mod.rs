// =============================================================================
//! - Location coordinate conversion functions for CroftSoft Life
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

use crate::constants::SPACE_WIDTH;

pub fn to_index_from_xy(
  x: usize,
  y: usize,
) -> usize {
  SPACE_WIDTH * y + x
}

pub fn to_x_from_index(index: usize) -> usize {
  index % SPACE_WIDTH
}

pub fn to_y_from_index(index: usize) -> usize {
  index / SPACE_WIDTH
}
