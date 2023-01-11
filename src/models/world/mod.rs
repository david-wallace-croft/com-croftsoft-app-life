// =============================================================================
//! - World Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-10
//! - Since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::Cells;
use super::clock::Clock;
use crate::engine::input::Input;
use crate::engine::traits::Model;
use core::cell::RefCell;
use std::rc::Rc;

pub struct World {
  cells: Rc<RefCell<Cells>>,
  clock: Rc<RefCell<Clock>>,
  models: Vec<Rc<RefCell<dyn Model>>>,
}

// TODO: extract the trait?
impl World {
  pub fn cells_clone(&self) -> Rc<RefCell<Cells>> {
    self.cells.clone()
  }

  pub fn clock_clone(&self) -> Rc<RefCell<Clock>> {
    self.clock.clone()
  }
}

impl Default for World {
  fn default() -> Self {
    let cells = Rc::new(RefCell::new(Cells::default()));
    let clock = Rc::new(RefCell::new(Clock::default()));
    let models: Vec<Rc<RefCell<dyn Model>>> = vec![
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

impl Model for World {
  fn update(
    &mut self,
    input: &Input,
  ) {
    self.models.iter().for_each(|model| model.borrow_mut().update(input));
  }
}
