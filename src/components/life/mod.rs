// =============================================================================
//! - Life Component for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2023-03-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::canvas::CanvasComponent;
use super::frame_rate::FrameRateComponent;
use super::pause::PauseComponent;
use super::reset::ResetComponent;
use super::speed::SpeedComponent;
use crate::engine::functions::web_sys::get_window;
use crate::engine::traits::Component;
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::models::options::Options;
use crate::models::world::World;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct LifeComponent {
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 5],
  events: Rc<RefCell<Events>>,
  frame_rate_component: Rc<RefCell<FrameRateComponent>>,
  pause_component: Rc<RefCell<PauseComponent>>,
  reset_component: Rc<RefCell<ResetComponent>>,
  speed_component: Rc<RefCell<SpeedComponent>>,
}

impl LifeComponent {
  pub fn new(
    events: Rc<RefCell<Events>>,
    // TODO: do something with the ID
    _id: &str,
    inputs: Rc<RefCell<Inputs>>,
    options: Rc<RefCell<Options>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let canvas_component = Rc::new(RefCell::new(CanvasComponent::new(
      "canvas",
      inputs.clone(),
      options,
      world,
    )));
    let frame_rate_component = Rc::new(RefCell::new(FrameRateComponent::new(
      "frame-rate",
      inputs.clone(),
    )));
    let pause_component =
      Rc::new(RefCell::new(PauseComponent::new("pause", inputs.clone())));
    let reset_component =
      Rc::new(RefCell::new(ResetComponent::new("reset", inputs.clone())));
    let speed_component =
      Rc::new(RefCell::new(SpeedComponent::new("speed", inputs)));
    let components: [Rc<RefCell<dyn Component>>; 5] = [
      canvas_component.clone(),
      frame_rate_component.clone(),
      pause_component.clone(),
      reset_component.clone(),
      speed_component.clone(),
    ];
    Self {
      canvas_component,
      components,
      events,
      frame_rate_component,
      pause_component,
      reset_component,
      speed_component,
    }
  }
}

impl Component for LifeComponent {
  fn make_html(&self) -> String {
    let canvas_html: String = self.canvas_component.borrow().make_html();
    let frame_rate_html: String =
      self.frame_rate_component.borrow().make_html();
    let pause_html: String = self.pause_component.borrow().make_html();
    let reset_html: String = self.reset_component.borrow().make_html();
    let speed_html: String = self.speed_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"life\">"),
      canvas_html,
      String::from("<br>"),
      reset_html,
      speed_html,
      frame_rate_html,
      pause_html,
      String::from("</div>"),
    ]
    .join("\n")
  }
}

impl Initializer for LifeComponent {
  fn initialize(&mut self) {
    let document: Document = get_window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-life");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let evolve_html: String = self.make_html();
    // TODO: Remove existing child nodes
    let _result = element.insert_adjacent_html("afterbegin", &evolve_html);
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().initialize());
  }
}

impl Painter for LifeComponent {
  fn paint(&mut self) {
    if !self.events.borrow().updated {
      return;
    }
    self.canvas_component.borrow_mut().paint();
  }
}

impl Updater for LifeComponent {
  fn update(&mut self) {
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().update());
  }
}
