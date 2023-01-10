// =============================================================================
//! - WorldPainter for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-09
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// use super::overlay::OverlayPainter;
use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::traits::{CanvasPainter, Painter};
use crate::models::world::World;
use crate::painters::background::BackgroundPainter;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, Document, Element, HtmlCanvasElement,
};

pub struct WorldPainter {
  canvas_painters: Vec<Box<dyn CanvasPainter>>,
  context: CanvasRenderingContext2d,
}

impl WorldPainter {
  pub fn new(
    canvas_element_id: &str,
    world: &World,
  ) -> Self {
    let document: Document = window().unwrap().document().unwrap();
    let element: Element =
      document.get_element_by_id(canvas_element_id).unwrap();
    let html_canvas_element: HtmlCanvasElement = element.dyn_into().unwrap();
    let object: Object =
      html_canvas_element.get_context("2d").unwrap().unwrap();
    let context: CanvasRenderingContext2d = object.dyn_into().unwrap();
    let canvas_height: f64 = html_canvas_element.height() as f64;
    let canvas_width: f64 = html_canvas_element.width() as f64;
    let background_painter =
      BackgroundPainter::new(canvas_height, canvas_width);
    let scale_x = canvas_width / SPACE_WIDTH as f64;
    let scale_y = canvas_height / SPACE_HEIGHT as f64;
    let canvas_painters: Vec<Box<dyn CanvasPainter>> =
      vec![Box::new(background_painter)];
    Self {
      canvas_painters,
      context,
    }
  }
}

impl Painter for WorldPainter {
  fn paint(&self) {
    self
      .canvas_painters
      .iter()
      .for_each(|canvas_painter| canvas_painter.paint(&self.context));
  }
}
