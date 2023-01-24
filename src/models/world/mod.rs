// =============================================================================
//! - World Model for CroftSoft Life
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

use super::cells::Cells;
use super::clock::Clock;
use crate::engine::input::Input;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub struct World {
  cells: Rc<RefCell<Cells>>,
  clock: Rc<RefCell<Clock>>,
  models: [Rc<RefCell<dyn Updater>>; 2],
}

impl World {
  pub fn cells_clone(&self) -> Rc<RefCell<Cells>> {
    self.cells.clone()
  }

  pub fn clock_clone(&self) -> Rc<RefCell<Clock>> {
    self.clock.clone()
  }

  pub fn new(input: Rc<RefCell<Input>>) -> Self {
    let cells = Rc::new(RefCell::new(Cells::new(input.clone())));
    let clock = Rc::new(RefCell::new(Clock::new(input)));
    let models: [Rc<RefCell<dyn Updater>>; 2] = [
      clock.clone(),
      cells.clone(),
    ];
    Self {
      cells,
      clock,
      models,
    }
  }
}

impl Updater for World {
  fn update(&mut self) {
    self.models.iter().for_each(|model| model.borrow_mut().update());
  }
}
