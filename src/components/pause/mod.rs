// =============================================================================
//! - Pause Component for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-23
//! - Updated: 2023-02-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::functions::web_sys::add_change_handler_by_id;
use crate::engine::traits::Component;
use crate::messages::inputs::Inputs;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct PauseComponent {
  event_unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
  id: String,
  inputs: Rc<RefCell<Inputs>>,
}

impl PauseComponent {
  fn changed(&mut self) -> Option<Event> {
    let unbounded_receiver: &mut UnboundedReceiver<Event> =
      self.event_unbounded_receiver_option.as_mut()?;
    let result: Result<Option<Event>, TryRecvError> =
      unbounded_receiver.try_next();
    if let Ok(event_option) = result {
      return event_option;
    }
    None
  }

  pub fn new(
    id: &str,
    inputs: Rc<RefCell<Inputs>>,
  ) -> Self {
    Self {
      id: String::from(id),
      inputs,
      event_unbounded_receiver_option: None,
    }
  }
}

impl Component for PauseComponent {
  fn make_html(&self) -> String {
    format!("Pause <input id=\"{}\" type=\"checkbox\">", self.id)
  }
}

impl Initializer for PauseComponent {
  fn initialize(&mut self) {
    self.event_unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl Updater for PauseComponent {
  fn update(&mut self) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        self.inputs.borrow_mut().pause_change_requested =
          Some(html_input_element.checked());
      }
    }
  }
}
