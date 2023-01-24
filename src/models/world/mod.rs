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

use super::cells::{Cells, CellsInput};
use super::clock::{Clock, ClockInput};
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub trait WorldInput {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_reset_requested(&self) -> bool;
}

struct WorldInputUpcastAdapter {
  input: Rc<RefCell<dyn WorldInput>>,
}

impl WorldInputUpcastAdapter {
  fn new(input: Rc<RefCell<dyn WorldInput>>) -> Self {
    Self {
      input,
    }
  }
}

impl CellsInput for WorldInputUpcastAdapter {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.input.borrow().get_cell_toggle_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

impl ClockInput for WorldInputUpcastAdapter {
  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

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

  pub fn new(input: Rc<RefCell<dyn WorldInput>>) -> Self {
    let world_input_upcast_adapter =
      Rc::new(RefCell::new(WorldInputUpcastAdapter::new(input)));
    let cells =
      Rc::new(RefCell::new(Cells::new(world_input_upcast_adapter.clone())));
    let clock = Rc::new(RefCell::new(Clock::new(world_input_upcast_adapter)));
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
