// =============================================================================
//! - Clock Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-23
//! - Since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub trait ClockInput {
  fn get_reset_requested(&self) -> bool;
}

pub struct Clock {
  input: Rc<RefCell<dyn ClockInput>>,
  pub time: usize,
}

impl Clock {
  pub fn new(input: Rc<RefCell<dyn ClockInput>>) -> Self {
    Self {
      input,
      time: 0,
    }
  }
}

impl Updater for Clock {
  fn update(&mut self) {
    if self.input.borrow().get_reset_requested() {
      self.time = 0;
    } else {
      self.time = self.time.saturating_add(1);
    }
  }
}
