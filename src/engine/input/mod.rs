// =============================================================================
//! - Input model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-24
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterInput;

// TODO: move this to a different module
#[derive(Default)]
pub struct Input {
  pub cell_toggle_requested: Option<usize>,
  pub reset_requested: bool,
  pub speed_toggle_requested: bool,
}

impl Input {
  pub fn clear(&mut self) {
    self.cell_toggle_requested = None;
    self.reset_requested = false;
    self.speed_toggle_requested = false;
  }
}

impl WorldUpdaterInput for Input {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.cell_toggle_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }
}
