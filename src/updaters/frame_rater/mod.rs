// =============================================================================
//! - Frame Rate Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-24
//! - Updated: 2023-02-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rater::FrameRater;
use crate::models::options::Options;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait FrameRaterUpdaterInputs {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct FrameRaterUpdater {
  frame_rater: Rc<RefCell<FrameRater>>,
  inputs: Rc<RefCell<dyn FrameRaterUpdaterInputs>>,
  options: Rc<RefCell<Options>>,
}

impl FrameRaterUpdater {
  pub fn new(
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn FrameRaterUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
  ) -> Self {
    Self {
      frame_rater,
      inputs,
      options,
    }
  }
}

impl Updater for FrameRaterUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn FrameRaterUpdaterInputs> = self.inputs.borrow();
    if let Some(frame_rate_display) =
      inputs.get_frame_rate_display_change_requested()
    {
      if frame_rate_display {
        self.frame_rater.borrow_mut().clear();
      }
    }
    if let Some(update_period_millis) =
      inputs.get_update_period_millis_changed()
    {
      self
        .frame_rater
        .borrow_mut()
        .update_frame_sample_size(update_period_millis);
    }
    if inputs.get_reset_requested() {
      self.frame_rater.borrow_mut().clear();
      return;
    }
    let options: Ref<Options> = self.options.borrow();
    if inputs.get_time_to_update() && options.frame_rate_display {
      self
        .frame_rater
        .borrow_mut()
        .sample(self.inputs.borrow().get_update_time_millis());
    }
  }
}