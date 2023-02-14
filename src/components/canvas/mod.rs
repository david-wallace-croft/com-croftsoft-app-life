// =============================================================================
//! - Canvas Component for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{SPACE_HEIGHT, SPACE_WIDTH};
use crate::engine::functions::location::to_index_from_xy;
use crate::engine::functions::web_sys::{
  add_mouse_down_handler_by_id, get_canvas_xy, get_html_canvas_element_by_id,
};
use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use crate::models::frame_rate::FrameRate;
use crate::models::world::World;
use crate::painters::world::WorldPainter;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use web_sys::{HtmlCanvasElement, MouseEvent};

pub struct CanvasComponent {
  frame_rate: Rc<RefCell<FrameRate>>,
  id: String,
  inputs: Rc<RefCell<Inputs>>,
  root_painter_option: Option<WorldPainter>,
  unbounded_receiver_option: Option<UnboundedReceiver<MouseEvent>>,
  world: Rc<RefCell<World>>,
}

impl CanvasComponent {
  fn get_scale_xy(canvas_element_id: &str) -> (f64, f64) {
    let html_canvas_element: HtmlCanvasElement =
      get_html_canvas_element_by_id(canvas_element_id);
    let canvas_height = html_canvas_element.height();
    let canvas_width = html_canvas_element.width();
    let scale_x = canvas_width as f64 / SPACE_WIDTH as f64;
    let scale_y = canvas_height as f64 / SPACE_HEIGHT as f64;
    (scale_x, scale_y)
  }

  pub fn new(
    frame_rate: Rc<RefCell<FrameRate>>,
    id: &str,
    inputs: Rc<RefCell<Inputs>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      frame_rate,
      id: String::from(id),
      inputs,
      root_painter_option: None,
      unbounded_receiver_option: None,
      world,
    }
  }

  fn poll_mouse_event(&mut self) -> Option<MouseEvent> {
    let unbounded_receiver: &mut UnboundedReceiver<MouseEvent> =
      self.unbounded_receiver_option.as_mut()?;
    let result: Result<Option<MouseEvent>, TryRecvError> =
      unbounded_receiver.try_next();
    if let Ok(mouse_event_option) = result {
      return mouse_event_option;
    }
    None
  }

  fn to_world_index_from_canvas_xy(
    &self,
    canvas_x: usize,
    canvas_y: usize,
  ) -> usize {
    let (scale_x, scale_y) = CanvasComponent::get_scale_xy(&self.id);
    let scaled_canvas_x: f64 = canvas_x as f64 / scale_x;
    let scaled_canvas_y: f64 = canvas_y as f64 / scale_y;
    let mut world_x = scaled_canvas_x as usize;
    let mut world_y = scaled_canvas_y as usize;
    if world_x >= SPACE_WIDTH {
      world_x = SPACE_WIDTH - 1;
    }
    if world_y >= SPACE_HEIGHT {
      world_y = SPACE_HEIGHT - 1;
    }
    to_index_from_xy(world_x, world_y)
  }
}

impl Component for CanvasComponent {
  fn make_html(&self) -> String {
    format!(
      "<canvas id=\"{}\" height=\"600\" style=\"cursor: crosshair\" width=\"600\"></canvas>",
      self.id
    )
  }
}

impl Initializer for CanvasComponent {
  fn initialize(&mut self) {
    self.unbounded_receiver_option = add_mouse_down_handler_by_id(&self.id);
    self.root_painter_option = Some(WorldPainter::new(
      "canvas",
      self.frame_rate.clone(),
      &self.world.borrow(),
    ));
  }
}

impl Painter for CanvasComponent {
  fn paint(&self) {
    if let Some(root_painter) = &self.root_painter_option {
      root_painter.paint();
    }
  }
}

impl Updater for CanvasComponent {
  fn update(&mut self) {
    let mouse_event_option = self.poll_mouse_event();
    if let Some(mouse_event) = mouse_event_option {
      let (canvas_x, canvas_y) = get_canvas_xy(&mouse_event);
      let index = self.to_world_index_from_canvas_xy(canvas_x, canvas_y);
      self.inputs.borrow_mut().cell_toggle_requested = Some(index);
    }
  }
}
