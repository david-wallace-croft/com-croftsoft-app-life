// =============================================================================
//! - Input model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterInputs;

// TODO: move this to a different module
#[derive(Default)]
pub struct Inputs {
  pub cell_toggle_requested: Option<usize>,
  pub frame_rate_display_change_requested: Option<bool>,
  pub reset_requested: bool,
  pub speed_change_requested: Option<usize>,
  pub update_time_millis: f64,
}

impl Inputs {
  pub fn clear(&mut self) {
    self.cell_toggle_requested = None;
    self.frame_rate_display_change_requested = None;
    self.reset_requested = false;
    self.speed_change_requested = None;
    self.update_time_millis = 0.;
  }
}

impl WorldUpdaterInputs for Inputs {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.cell_toggle_requested
  }

  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.frame_rate_display_change_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }

  fn get_speed_change_requested(&self) -> Option<usize> {
    self.speed_change_requested
  }

  fn get_update_time_millis(&self) -> f64 {
    self.update_time_millis
  }
}
