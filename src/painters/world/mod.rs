// =============================================================================
//! - World Painter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-03-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::CellsPainter;
use super::overlay::OverlayPainter;
use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::models::options::Options;
use crate::models::world::World;
use crate::painters::background::BackgroundPainter;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use js_sys::Object;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement,
};

pub struct WorldPainter {
  painters: Vec<Box<dyn Painter>>,
}

impl WorldPainter {
  pub fn new(
    canvas_element_id: &str,
    options: Rc<RefCell<Options>>,
    world: &World,
  ) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let element: Element =
      document.get_element_by_id(canvas_element_id).unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let canvas_context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let context: Rc<RefCell<CanvasRenderingContext2d>> =
      Rc::new(RefCell::new(canvas_context));
    let canvas_height: f64 = html_canvas_element.height() as f64;
    let canvas_width: f64 = html_canvas_element.width() as f64;
    let background_painter =
      BackgroundPainter::new(canvas_height, canvas_width, context.clone());
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let cells_painter =
      CellsPainter::new(world.cells.clone(), context.clone(), scale_x, scale_y);
    let overlay_painter =
      OverlayPainter::new(context, options, world.overlay.clone());
    let painters: Vec<Box<dyn Painter>> = vec![
      Box::new(background_painter),
      Box::new(cells_painter),
      Box::new(overlay_painter),
    ];
    Self {
      painters,
    }
  }
}

impl Painter for WorldPainter {
  fn paint(&self) {
    self.painters.iter().for_each(|painter| painter.paint());
  }
}
