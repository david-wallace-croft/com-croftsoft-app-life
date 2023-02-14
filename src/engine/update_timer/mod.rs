// =============================================================================
//! - Updater Timer for CroftSoft Life
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

pub struct UpdateTimer {
  pub update_period_millis: f64,
  pub update_time_millis_next: f64,
}

impl UpdateTimer {
  pub fn before_next_update_time(
    &mut self,
    update_time_millis: f64,
  ) -> bool {
    if update_time_millis < self.update_time_millis_next {
      return true;
    }
    self.update_time_millis_next =
      update_time_millis + self.update_period_millis;
    false
  }
}
