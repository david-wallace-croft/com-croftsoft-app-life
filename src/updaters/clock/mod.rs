// =============================================================================
//! - Clock Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::clock::Clock;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait ClockUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
}

pub struct ClockUpdater {
  clock: Rc<RefCell<Clock>>,
  input: Rc<RefCell<dyn ClockUpdaterInputs>>,
}

impl ClockUpdater {
  pub fn new(
    clock: Rc<RefCell<Clock>>,
    input: Rc<RefCell<dyn ClockUpdaterInputs>>,
  ) -> Self {
    Self {
      clock,
      input,
    }
  }
}

impl Updater for ClockUpdater {
  fn update(&mut self) {
    let mut clock: RefMut<Clock> = self.clock.borrow_mut();
    let input: Ref<dyn ClockUpdaterInputs> = self.input.borrow();
    if input.get_reset_requested() {
      clock.time = 0;
    } else {
      clock.time = clock.time.saturating_add(1);
    }
  }
}
