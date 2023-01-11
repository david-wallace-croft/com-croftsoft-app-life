// =============================================================================
//! - Life Component for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-11
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::canvas::CanvasComponent;
use super::reset::ResetComponent;
use super::speed::SpeedComponent;
use crate::engine::functions::web_sys::get_window;
use crate::engine::input::Input;
use crate::engine::traits::{Component, Painter};
use crate::models::world::World;
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct LifeComponent {
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 3],
  reset_component: Rc<RefCell<ResetComponent>>,
  speed_component: Rc<RefCell<SpeedComponent>>,
}

impl LifeComponent {
  // TODO: do something with the ID
  pub fn new(
    _id: &str,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let canvas_component =
      Rc::new(RefCell::new(CanvasComponent::new("canvas", world)));
    let reset_component = Rc::new(RefCell::new(ResetComponent::new("reset")));
    let speed_component = Rc::new(RefCell::new(SpeedComponent::new("speed")));
    let components: [Rc<RefCell<dyn Component>>; 3] = [
      canvas_component.clone(),
      reset_component.clone(),
      speed_component.clone(),
    ];
    Self {
      canvas_component,
      components,
      reset_component,
      speed_component,
    }
  }
}

impl Component for LifeComponent {
  fn init(&mut self) {
    let document: Document = get_window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-life");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let evolve_html: String = self.make_html();
    // TODO: Remove existing child nodes
    let _result = element.insert_adjacent_html("afterbegin", &evolve_html);
    self.components.iter().for_each(|component| component.borrow_mut().init());
  }

  fn make_html(&self) -> String {
    let canvas_html: String = self.canvas_component.borrow().make_html();
    let reset_html: String = self.reset_component.borrow().make_html();
    let speed_html: String = self.speed_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"life\">"),
      canvas_html,
      String::from("<br>"),
      reset_html,
      speed_html,
      String::from("</div>"),
    ]
    .join("\n")
  }

  fn update(
    &mut self,
    input: &mut Input,
  ) {
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().update(input));
  }
}

impl Painter for LifeComponent {
  fn paint(&self) {
    self.canvas_component.borrow().paint();
  }
}
