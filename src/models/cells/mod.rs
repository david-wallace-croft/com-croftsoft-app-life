// =============================================================================
//! - Cells Model for CroftSoft Life
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

use crate::constants::{CELL_COUNT, SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::{
  to_index_from_xy, to_x_from_index, to_y_from_index,
};
use crate::engine::input::Input;
use crate::engine::traits::Model;

// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

pub struct Cells {
  pub old: [bool; CELL_COUNT],
  pub new: [bool; CELL_COUNT],
}

impl Cells {
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
          && self.old[to_index_from_xy(*column_value, *row_value)]
        {
          sum += 1
        };
      }
    }
    sum
  }

  fn randomize(&mut self) {
    let mut thread_rng: ThreadRng = rand::thread_rng();
    for i in 0..CELL_COUNT {
      let roll: usize = thread_rng.gen_range(0..4);
      self.new[i] = roll == 0;
    }
  }
}

impl Default for Cells {
  fn default() -> Self {
    Self {
      old: [false; CELL_COUNT],
      new: [false; CELL_COUNT],
    }
  }
}

impl Model for Cells {
  fn update(
    &mut self,
    input: &Input,
  ) {
    if input.reset_requested {
      self.randomize();
      return;
    }
    for i in 0..CELL_COUNT {
      // TODO: Can I transfer this as a slice or use a buffer?
      self.old[i] = self.new[i];
    }
    for i in 0..CELL_COUNT {
      let count = self.count_adjacent_alive(i);
      if count < 2 {
        self.new[i] = false;
      } else if count == 2 {
        self.new[i] = self.old[i];
      } else if count == 3 {
        self.new[i] = true;
      } else {
        self.new[i] = false;
      }
    }
    if let Some(index) = input.cell_toggle_requested {
      self.new[index] = !self.old[index];
    }
  }
}
