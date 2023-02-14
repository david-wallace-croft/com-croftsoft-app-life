// =============================================================================
//! - Overlay Updater for CroftSoft Life
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

use crate::constants::OVERLAY_REFRESH_PERIOD_MILLIS;
use crate::engine::frame_rater::FrameRater;
use crate::engine::update_timer::UpdateTimer;
use crate::models::clock::Clock;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::rc::Rc;

pub trait OverlayUpdaterInputs {
  fn get_update_time_millis(&self) -> f64;
}

pub struct OverlayUpdater {
  clock: Rc<RefCell<Clock>>,
  frame_rater: Rc<RefCell<FrameRater>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  overlay: Rc<RefCell<Overlay>>,
  update_timer: UpdateTimer,
}

impl OverlayUpdater {
  fn make_clock_string(&self) -> String {
    format!("Time: {}", self.clock.borrow().time)
  }

  fn make_frame_rate_string(&self) -> String {
    format!(
      "Simulation updates per second: {:.3}",
      self.frame_rater.borrow().get_frames_per_second_sampled()
    )
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let update_timer = UpdateTimer {
      update_period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      update_time_millis_next: 0.,
    };
    Self {
      clock,
      frame_rater,
      inputs,
      overlay,
      update_timer,
    }
  }
}

impl Updater for OverlayUpdater {
  fn update(&mut self) {
    let update_time_millis = self.inputs.borrow().get_update_time_millis();
    if self.update_timer.before_next_update_time(update_time_millis) {
      return;
    }
    let mut overlay: RefMut<Overlay> = self.overlay.borrow_mut();
    overlay.clock_string = self.make_clock_string();
    overlay.frame_rate_string = self.make_frame_rate_string();
  }
}
