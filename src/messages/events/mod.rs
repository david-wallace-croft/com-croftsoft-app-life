// =============================================================================
//! - Events for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-13
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updaters::world::WorldUpdaterEvents;

#[derive(Default)]
pub struct Events {
  pub time_to_update: bool,
  pub update_period_millis_changed: Option<f64>,
  pub updated: bool,
}

impl Events {
  pub fn clear(&mut self) {
    self.time_to_update = false;
    self.update_period_millis_changed = None;
    self.updated = false;
  }
}

impl WorldUpdaterEvents for Events {
  fn get_time_to_update(&self) -> bool {
    self.time_to_update
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.update_period_millis_changed
  }

  fn get_updated(&self) -> bool {
    self.updated
  }

  fn set_time_to_update(&mut self) {
    self.time_to_update = true;
  }

  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self.update_period_millis_changed = Some(update_period_millis);
  }

  fn set_updated(&mut self) {
    self.updated = true;
  }
}
