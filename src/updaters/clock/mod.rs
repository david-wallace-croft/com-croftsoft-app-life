// =============================================================================
//! - Clock Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::clock::Clock;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait ClockUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait ClockUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
}

pub trait ClockUpdaterOptions {
  fn get_pause(&self) -> bool;
}

pub struct ClockUpdater {
  clock: Rc<RefCell<Clock>>,
  events: Rc<RefCell<dyn ClockUpdaterEvents>>,
  inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
  options: Rc<RefCell<dyn ClockUpdaterOptions>>,
}

impl ClockUpdater {
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      clock,
      events,
      inputs,
      options,
    }
  }
}

impl Updater for ClockUpdater {
  fn update(&self) {
    let mut clock: RefMut<Clock> = self.clock.borrow_mut();
    let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    if inputs.get_reset_requested() {
      clock.time = 0;
      self.events.borrow_mut().set_updated();
      return;
    }
    if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
      return;
    }
    clock.time = clock.time.saturating_add(1);
    self.events.borrow_mut().set_updated();
  }
}
