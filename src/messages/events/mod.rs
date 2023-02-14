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
  pub update_period_millis_changed: Option<f64>,
  pub updated_world: bool,
}

impl Events {
  pub fn clear(&mut self) {
    self.update_period_millis_changed = None;
    self.updated_world = false;
  }
}

impl WorldUpdaterEvents for Events {
  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.update_period_millis_changed
  }

  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self.update_period_millis_changed = Some(update_period_millis);
  }

  fn set_updated_world(&mut self) {
    self.updated_world = true;
  }
}
