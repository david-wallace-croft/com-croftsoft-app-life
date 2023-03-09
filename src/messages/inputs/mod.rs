// =============================================================================
//! - Input model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-03-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::root::RootUpdaterInputs;

// TODO: move this to a different module
#[derive(Default)]
pub struct Inputs {
  pub cell_toggle_requested: Option<usize>,
  pub current_time_millis: f64,
  pub frame_rate_display_change_requested: Option<bool>,
  pub pause_change_requested: Option<bool>,
  pub period_millis_change_requested: Option<f64>,
  pub reset_requested: bool,
}

impl Inputs {
  pub fn clear(&mut self) {
    self.cell_toggle_requested = None;
    self.current_time_millis = 0.;
    self.frame_rate_display_change_requested = None;
    self.pause_change_requested = None;
    self.period_millis_change_requested = None;
    self.reset_requested = false;
  }
}

impl RootUpdaterInputs for Inputs {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.cell_toggle_requested
  }

  fn get_current_time_millis(&self) -> f64 {
    self.current_time_millis
  }

  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.frame_rate_display_change_requested
  }

  fn get_pause_change_requested(&self) -> Option<bool> {
    self.pause_change_requested
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.period_millis_change_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }
}
