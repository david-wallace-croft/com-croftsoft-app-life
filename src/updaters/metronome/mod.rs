// =============================================================================
//! - Metronome Updater for CroftSoft Life
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

use crate::constants::MILLIS_PER_SECOND;
use crate::engine::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait MetronomeUpdaterEvents {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_tick(&mut self);
}

pub trait MetronomeUpdaterInputs {
  fn get_frequency_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_millis(&self) -> f64;
}

pub struct MetronomeUpdater {
  events: Rc<RefCell<dyn MetronomeUpdaterEvents>>,
  inputs: Rc<RefCell<dyn MetronomeUpdaterInputs>>,
  metronome: Metronome,
}

impl MetronomeUpdater {
  pub fn new(
    events: Rc<RefCell<dyn MetronomeUpdaterEvents>>,
    inputs: Rc<RefCell<dyn MetronomeUpdaterInputs>>,
    metronome: Metronome,
  ) -> Self {
    Self {
      events,
      inputs,
      metronome,
    }
  }
}

impl Updater for MetronomeUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn MetronomeUpdaterInputs> = self.inputs.borrow();
    if let Some(frequency) = inputs.get_frequency_change_requested() {
      self.metronome.period_millis = (MILLIS_PER_SECOND / frequency).trunc();
      self
        .events
        .borrow_mut()
        .set_period_millis_changed(self.metronome.period_millis);
      self.metronome.time_millis_next = 0.;
    }
    let time_millis = self.inputs.borrow().get_time_millis();
    if inputs.get_reset_requested() {
      self.metronome.time_millis_next =
        time_millis + self.metronome.period_millis;
    }
    if self.metronome.tick(time_millis) {
      self.events.borrow_mut().set_tick();
    }
  }
}
