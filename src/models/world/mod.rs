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
use crate::engine::input::Input;
use crate::engine::traits::Model;
use core::cell::RefCell;
use std::rc::Rc;

pub struct World {
  cells: Rc<RefCell<Cells>>,
  models: Vec<Rc<RefCell<dyn Model>>>,
}

// TODO: extract the trait?
impl World {
  pub fn cells_clone(&self) -> Rc<RefCell<Cells>> {
    self.cells.clone()
  }
}

impl Default for World {
  fn default() -> Self {
    let cells = Rc::new(RefCell::new(Cells::default()));
    cells.borrow_mut().randomize();
    let models: Vec<Rc<RefCell<dyn Model>>> = vec![cells.clone()];
    Self {
      cells,
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
