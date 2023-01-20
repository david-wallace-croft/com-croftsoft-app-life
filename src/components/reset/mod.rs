// =============================================================================
//! - Reset Component for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-19
//! - Since: 2023-01-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::functions::web_sys::add_click_handler_by_id;
use crate::engine::input::Input;
use crate::engine::traits::Component;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use futures::channel::mpsc::UnboundedReceiver;
use std::rc::Rc;

pub struct ResetComponent {
  id: String,
  input: Rc<RefCell<Input>>,
  unbounded_receiver: Option<UnboundedReceiver<()>>,
}

impl ResetComponent {
  pub fn new(
    id: &str,
    input: Rc<RefCell<Input>>,
  ) -> Self {
    Self {
      id: String::from(id),
      input,
      unbounded_receiver: None,
    }
  }

  fn pressed(&mut self) -> bool {
    if self.unbounded_receiver.is_none() {
      return false;
    }
    matches!(
      self.unbounded_receiver.as_mut().unwrap().try_next(),
      Ok(Some(()))
    )
  }
}

impl Component for ResetComponent {
  fn init(&mut self) {
    self.unbounded_receiver = add_click_handler_by_id(&self.id);
  }

  fn make_html(&self) -> String {
    format!("<button id=\"{}\">Reset</button>", self.id)
  }
}

impl Updater for ResetComponent {
  fn update(&mut self) {
    if self.pressed() {
      self.input.borrow_mut().reset_requested = true;
    }
  }
}
