// =============================================================================
//! - Metronome Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-17
//! - Updated: 2023-02-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use super::Metronome;

pub trait MetronomeUpdaterEvents {
  fn set_period_millis_changed(
    &mut self,
    period_millis: f64,
  );
  fn set_tick(&mut self);
}

pub trait MetronomeUpdaterInputs {
  fn get_current_time_millis(&self) -> f64;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
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
    if let Some(period_millis) = inputs.get_period_millis_change_requested() {
      let mut metronome: RefMut<dyn Metronome> = self.metronome.borrow_mut();
      metronome.set_period_millis(period_millis);
      self.events.borrow_mut().set_period_millis_changed(period_millis);
      metronome.set_time_millis_next_tick(0.);
    }
    let current_time_millis: f64 = self.inputs.borrow().get_current_time_millis();
    if inputs.get_reset_requested() {
      self.metronome.borrow_mut().reset(current_time_millis);
    }
    if self.metronome.borrow_mut().tick(current_time_millis) {
      self.events.borrow_mut().set_tick();
    }
  }
}
