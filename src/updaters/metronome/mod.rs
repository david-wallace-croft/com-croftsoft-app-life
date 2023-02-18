// =============================================================================
//! - Metronome Updater for CroftSoft Life
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

// TODO: Move this file to the engine module

use crate::engine::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

const MILLIS_PER_SECOND: f64 = 1_000.;

pub trait MetronomeUpdaterEvents {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_tick(&mut self);
}

pub trait MetronomeUpdaterInputs {
  // TODO: Change this to get_period_millis_change_requested()
  fn get_frequency_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
  // TODO: Change this to get_current_time_millis()
  fn get_time_millis(&self) -> f64;
}

pub struct MetronomeUpdater {
  events: Rc<RefCell<dyn MetronomeUpdaterEvents>>,
  inputs: Rc<RefCell<dyn MetronomeUpdaterInputs>>,
  metronome: Rc<RefCell<dyn Metronome>>,
}

impl MetronomeUpdater {
  pub fn new(
    events: Rc<RefCell<dyn MetronomeUpdaterEvents>>,
    inputs: Rc<RefCell<dyn MetronomeUpdaterInputs>>,
    metronome: Rc<RefCell<dyn Metronome>>,
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
      let period_millis: f64 = (MILLIS_PER_SECOND / frequency).trunc();
      let mut metronome: RefMut<dyn Metronome> = self.metronome.borrow_mut();
      metronome.set_period_millis(period_millis);
      self.events.borrow_mut().set_period_millis_changed(period_millis);
      metronome.set_time_millis_next_tick(0.);
    }
    let time_millis = self.inputs.borrow().get_time_millis();
    if inputs.get_reset_requested() {
      self.metronome.borrow_mut().reset(time_millis);
    }
    if self.metronome.borrow_mut().tick(time_millis) {
      self.events.borrow_mut().set_tick();
    }
  }
}
