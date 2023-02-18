// =============================================================================
//! - UpdateTimer Updater for CroftSoft Life
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

use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::constants::MILLIS_PER_SECOND;
use crate::engine::update_timer::UpdateTimer;

pub trait UpdateTimerUpdaterEvents {
  fn set_time_to_update(&mut self);
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
}

pub trait UpdateTimerUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
  fn get_speed_change_requested(&self) -> Option<usize>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct UpdateTimerUpdater {
  events: Rc<RefCell<dyn UpdateTimerUpdaterEvents>>,
  inputs: Rc<RefCell<dyn UpdateTimerUpdaterInputs>>,
  update_timer: UpdateTimer,
}

impl UpdateTimerUpdater {
  pub fn new(
    events: Rc<RefCell<dyn UpdateTimerUpdaterEvents>>,
    inputs: Rc<RefCell<dyn UpdateTimerUpdaterInputs>>,
    update_timer: UpdateTimer,
  ) -> Self {
    Self {
      events,
      inputs,
      update_timer,
    }
  }
}

impl Updater for UpdateTimerUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn UpdateTimerUpdaterInputs> = self.inputs.borrow();
    if let Some(speed) = inputs.get_speed_change_requested() {
      self.update_timer.update_period_millis =
        (MILLIS_PER_SECOND / speed as f64).trunc();
      self.events.borrow_mut().set_update_period_millis_changed(
        self.update_timer.update_period_millis,
      );
      self.update_timer.update_time_millis_next = 0.;
    }
    let update_time_millis = self.inputs.borrow().get_update_time_millis();
    if inputs.get_reset_requested() {
      self.update_timer.update_time_millis_next =
        update_time_millis + self.update_timer.update_period_millis;
    }
    if !self.update_timer.before_next_update_time(update_time_millis) {
      self.events.borrow_mut().set_time_to_update();
    }
  }
}
