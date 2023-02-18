// =============================================================================
//! - Metronome for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-17
//! - Updated: 2023-02-17
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub struct Metronome {
  pub period_millis: f64,
  pub time_millis_next: f64,
}

impl Metronome {
  // TODO: make Metronome a Trait for Strategy pattern in Updater
  pub fn tick(
    &mut self,
    time_millis: f64,
  ) -> bool {
    if time_millis < self.time_millis_next {
      return false;
    }
    self.time_millis_next = time_millis + self.period_millis;
    true
  }
}
