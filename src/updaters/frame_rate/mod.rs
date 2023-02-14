// =============================================================================
//! - Frame Rate Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-13
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::frame_rater::FrameRater;
use crate::models::frame_rate::FrameRate;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait FrameRateUpdaterEvents {
  fn get_update_period_millis_changed(&self) -> Option<f64>;
}

pub trait FrameRateUpdaterInputs {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_update_time_millis(&self) -> f64;
}

pub struct FrameRateUpdater {
  events: Rc<RefCell<dyn FrameRateUpdaterEvents>>,
  frame_rate: Rc<RefCell<FrameRate>>,
  frame_rater: Rc<RefCell<FrameRater>>,
  inputs: Rc<RefCell<dyn FrameRateUpdaterInputs>>,
}

impl FrameRateUpdater {
  pub fn new(
    events: Rc<RefCell<dyn FrameRateUpdaterEvents>>,
    frame_rate: Rc<RefCell<FrameRate>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn FrameRateUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      frame_rate,
      frame_rater,
      inputs,
    }
  }
}

impl Updater for FrameRateUpdater {
  fn update(&mut self) {
    if let Some(display) =
      self.inputs.borrow().get_frame_rate_display_change_requested()
    {
      self.frame_rate.borrow_mut().display = display;
      if display {
        self.frame_rater.borrow_mut().clear();
      }
    }
    let events: Ref<dyn FrameRateUpdaterEvents> = self.events.borrow();
    if let Some(update_period_millis) =
      events.get_update_period_millis_changed()
    {
      self
        .frame_rater
        .borrow_mut()
        .update_frame_sample_size(update_period_millis);
    }
    if self.inputs.borrow().get_reset_requested() {
      self.frame_rater.borrow_mut().clear();
      return;
    }
    if self.frame_rate.borrow().display {
      self
        .frame_rater
        .borrow_mut()
        .sample(self.inputs.borrow().get_update_time_millis());
    }
  }
}
