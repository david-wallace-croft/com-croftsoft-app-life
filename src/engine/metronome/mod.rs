// =============================================================================
//! - Metronome for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-17
//! - Updated: 2023-02-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait Metronome {
  fn reset(
    &mut self,
    current_time_millis: f64,
  );

  fn set_period_millis(
    &mut self,
    period_millis: f64,
  );

  fn set_time_millis_next_tick(
    &mut self,
    time_millis_next_tick: f64,
  );

  fn tick(
    &mut self,
    current_time_millis: f64,
  ) -> bool;
}

pub struct SimpleMetronome {
  pub period_millis: f64,
  pub time_millis_next_tick: f64,
}

impl Metronome for SimpleMetronome {
  fn reset(
    &mut self,
    current_time_millis: f64,
  ) {
    self.time_millis_next_tick = current_time_millis + self.period_millis
  }

  fn set_period_millis(
    &mut self,
    period_millis: f64,
  ) {
    self.period_millis = period_millis;
  }

  fn set_time_millis_next_tick(
    &mut self,
    time_millis_next_tick: f64,
  ) {
    self.time_millis_next_tick = time_millis_next_tick;
  }

  fn tick(
    &mut self,
    current_time_millis: f64,
  ) -> bool {
    if current_time_millis < self.time_millis_next_tick {
      return false;
    }
    self.time_millis_next_tick = current_time_millis + self.period_millis;
    true
  }
}
