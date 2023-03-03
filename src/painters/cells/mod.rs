// =============================================================================
//! - Cells Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-10
//! - Updated: 2023-03-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{CELL_COUNT, CELL_PAINT_OFFSET, CELL_PAINT_SIZE};
use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::models::cells::Cells;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CellsPainter {
  cells: Rc<RefCell<Cells>>,
  cells_height: f64,
  cells_width: f64,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  scale_x: f64,
  scale_y: f64,
}

impl CellsPainter {
  pub fn new(
    cells: Rc<RefCell<Cells>>,
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let cells_height = (CELL_PAINT_SIZE * scale_y).trunc();
    let cells_width = (CELL_PAINT_SIZE * scale_x).trunc();
    let fill_style = JsValue::from_str("lightgreen");
    Self {
      cells_height,
      cells_width,
      cells,
      context,
      fill_style,
      scale_x,
      scale_y,
    }
  }
}

impl Painter for CellsPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    let cells: Ref<Cells> = self.cells.borrow();
    for index in 0..CELL_COUNT {
      if !cells.new[index] {
        continue;
      }
      let x: f64 = to_x_from_index(index) as f64;
      let y: f64 = to_y_from_index(index) as f64;
      let corner_x = (self.scale_x * (x + CELL_PAINT_OFFSET)).trunc();
      let corner_y = (self.scale_y * (y + CELL_PAINT_OFFSET)).trunc();
      context.fill_rect(
        corner_x,
        corner_y,
        self.cells_width,
        self.cells_height,
      );
    }
  }
}
