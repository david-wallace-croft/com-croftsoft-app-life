// =============================================================================
//! - World Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-24
//! - Since: 2023-01-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::{CellsUpdater, CellsUpdaterInput};
use super::clock::{ClockUpdater, ClockUpdaterInput};
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait WorldUpdaterInput {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_reset_requested(&self) -> bool;
}

struct WorldUpdaterInputAdapter {
  input: Rc<RefCell<dyn WorldUpdaterInput>>,
}

impl WorldUpdaterInputAdapter {
  fn new(input: Rc<RefCell<dyn WorldUpdaterInput>>) -> Self {
    Self {
      input,
    }
  }
}

impl CellsUpdaterInput for WorldUpdaterInputAdapter {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.input.borrow().get_cell_toggle_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

impl ClockUpdaterInput for WorldUpdaterInputAdapter {
  fn get_reset_requested(&self) -> bool {
    self.input.borrow().get_reset_requested()
  }
}

pub struct WorldUpdater {
  cells_updater: CellsUpdater,
  clock_updater: ClockUpdater,
}

impl WorldUpdater {
  pub fn new(
    input: Rc<RefCell<dyn WorldUpdaterInput>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let world_input_upcast_adapter =
      Rc::new(RefCell::new(WorldUpdaterInputAdapter::new(input)));
    let world: Ref<World> = world.borrow();
    let cells = world.cells.clone();
    let clock = world.clock.clone();
    let cells_updater =
      CellsUpdater::new(cells, world_input_upcast_adapter.clone());
    let clock_updater = ClockUpdater::new(clock, world_input_upcast_adapter);
    Self {
      cells_updater,
      clock_updater,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    self.clock_updater.update();
    self.cells_updater.update();
  }
}
