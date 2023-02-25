// =============================================================================
//! - Options Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-23
//! - Updated: 2023-02-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::options::Options;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait OptionsUpdaterInputs {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct OptionsUpdater {
  inputs: Rc<RefCell<dyn OptionsUpdaterInputs>>,
  options: Rc<RefCell<Options>>,
}

impl OptionsUpdater {
  pub fn new(
    inputs: Rc<RefCell<dyn OptionsUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
  ) -> Self {
    Self {
      inputs,
      options,
    }
  }
}

impl Updater for OptionsUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn OptionsUpdaterInputs> = self.inputs.borrow();
    if let Some(frame_rate_display) =
      inputs.get_frame_rate_display_change_requested()
    {
      self.options.borrow_mut().frame_rate_display = frame_rate_display;
    }
    if let Some(pause) = inputs.get_pause_change_requested() {
      self.options.borrow_mut().pause = pause;
    }
  }
}
