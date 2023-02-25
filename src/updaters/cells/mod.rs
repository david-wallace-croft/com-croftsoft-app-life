// =============================================================================
//! - Cells Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-02-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{CELL_COUNT, SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
use crate::models::cells::Cells;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::rc::Rc;
// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

pub trait CellsUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait CellsUpdaterInputs {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
}

pub trait CellsUpdaterOptions {
  fn get_pause(&self) -> bool;
}

pub struct CellsUpdater {
  cells: Rc<RefCell<Cells>>,
  events: Rc<RefCell<dyn CellsUpdaterEvents>>,
  inputs: Rc<RefCell<dyn CellsUpdaterInputs>>,
  options: Rc<RefCell<dyn CellsUpdaterOptions>>,
}

impl CellsUpdater {
  fn count_adjacent_alive(
    &self,
    index: usize,
  ) -> usize {
    let x = to_x_from_index(index);
    let y = to_y_from_index(index);
    let west = if x > 0 {
      x - 1
    } else {
      SPACE_WIDTH - 1
    };
    let east = if x < SPACE_WIDTH - 1 {
      x + 1
    } else {
      0
    };
    let north = if y > 0 {
      y - 1
    } else {
      SPACE_HEIGHT - 1
    };
    let south = if y < SPACE_HEIGHT - 1 {
      y + 1
    } else {
      0
    };
    let mut sum = 0;
    let column_array = [
      west, x, east,
    ];
    let row_array = [
      north, y, south,
    ];
    for (column_index, column_value) in column_array.iter().enumerate() {
      for (row_index, row_value) in row_array.iter().enumerate() {
        if !(column_index == 1 && row_index == 1)
          && self.cells.borrow().old
            [to_index_from_xy(*column_value, *row_value)]
        {
          sum += 1
        };
      }
    }
    sum
  }

  pub fn new(
    cells: Rc<RefCell<Cells>>,
    events: Rc<RefCell<dyn CellsUpdaterEvents>>,
    inputs: Rc<RefCell<dyn CellsUpdaterInputs>>,
    options: Rc<RefCell<dyn CellsUpdaterOptions>>,
  ) -> Self {
    Self {
      cells,
      events,
      inputs,
      options,
    }
  }

  fn randomize(&mut self) {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    for i in 0..CELL_COUNT {
      let roll: usize = thread_rng.gen_range(0..4);
      self.cells.borrow_mut().new[i] = roll == 0;
    }
  }
}

impl Updater for CellsUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_reset_requested() {
      self.randomize();
      self.events.borrow_mut().set_updated();
      return;
    }
    let time_to_update: bool = self.inputs.borrow().get_time_to_update();
    if time_to_update && !self.options.borrow().get_pause() {
      self.cells.borrow_mut().swap_new_and_old();
      for i in 0..CELL_COUNT {
        let count = self.count_adjacent_alive(i);
        let mut cells: RefMut<Cells> = self.cells.borrow_mut();
        if count < 2 {
          cells.new[i] = false;
        } else if count == 2 {
          cells.new[i] = cells.old[i];
        } else if count == 3 {
          cells.new[i] = true;
        } else {
          cells.new[i] = false;
        }
      }
      self.events.borrow_mut().set_updated();
    }
    if let Some(index) = self.inputs.borrow().get_cell_toggle_requested() {
      let mut cells: RefMut<Cells> = self.cells.borrow_mut();
      if time_to_update {
        cells.new[index] = !cells.old[index];
      } else {
        cells.new[index] = !cells.new[index];
        self.events.borrow_mut().set_updated();
      }
    }
  }
}
