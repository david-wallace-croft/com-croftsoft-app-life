// =============================================================================
//! - Cells Painter for CroftSoft Life
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

use crate::constants::CELL_COUNT;
use crate::engine::functions::location::{to_x_from_index, to_y_from_index};
use crate::engine::traits::CanvasPainter;
use crate::models::cells::Cells;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct CellsPainter {
  fill_style: JsValue,
  cells: Rc<RefCell<Cells>>,
  cells_height: f64,
  cells_width: f64,
  scale_x: f64,
  scale_y: f64,
}

impl CellsPainter {
  pub fn new(
    cells: Rc<RefCell<Cells>>,
    scale_x: f64,
    scale_y: f64,
  ) -> Self {
    let fill_style = JsValue::from_str("white");
    let cells_height = scale_y / 2.0;
    let cells_width = scale_x / 2.0;
    Self {
      fill_style,
      cells_height,
      cells_width,
      cells,
      scale_x,
      scale_y,
    }
  }
}

impl CanvasPainter for CellsPainter {
  fn paint(
    &self,
    context: &CanvasRenderingContext2d,
  ) {
    context.set_fill_style(&self.fill_style);
    let cells: Ref<Cells> = self.cells.borrow();
    for index in 0..CELL_COUNT {
      if cells.new[index] {
        // TODO: replace with PlotLib.xy()
        let x: f64 = to_x_from_index(index) as f64;
        let y: f64 = to_y_from_index(index) as f64;
        let corner_x = self.scale_x * (x + 0.5);
        let corner_y = self.scale_y * (y + 0.5);
        context.fill_rect(
          corner_x,
          corner_y,
          self.cells_width,
          self.cells_height,
        );
      }
    }
  }
}