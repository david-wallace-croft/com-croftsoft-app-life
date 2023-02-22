// =============================================================================
//! - Overlay Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-13
//! - Updated: 2023-02-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::OVERLAY_REFRESH_PERIOD_MILLIS;
use crate::engine::frame_rater::FrameRater;
use crate::models::clock::Clock;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_animation::metronome::simple::SimpleMetronome;
use com_croftsoft_lib_animation::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait OverlayUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait OverlayUpdaterInputs {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_time_to_update(&self) -> bool;
  fn get_reset_requested(&self) -> bool;
  fn get_update_time_millis(&self) -> f64;
}

pub struct OverlayUpdater {
  clock: Rc<RefCell<Clock>>,
  events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
  frame_rater: Rc<RefCell<FrameRater>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  metronome: SimpleMetronome,
  overlay: Rc<RefCell<Overlay>>,
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
    events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let metronome = SimpleMetronome {
      period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      time_millis_next_tick: 0.,
    };
    Self {
      clock,
      events,
      frame_rater,
      inputs,
      metronome,
      overlay,
    }
  }

  fn update_overlay(&self) {
    let mut overlay: RefMut<Overlay> = self.overlay.borrow_mut();
    overlay.clock_string = self.make_clock_string();
    overlay.frame_rate_string = self.make_frame_rate_string();
    self.events.borrow_mut().set_updated();
  }
}

impl Updater for OverlayUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn OverlayUpdaterInputs> = self.inputs.borrow();
    if inputs.get_reset_requested()
      || inputs.get_frame_rate_display_change_requested().is_some()
    {
      self.update_overlay();
      return;
    }
    if !inputs.get_time_to_update() {
      return;
    }
    let update_time_millis = inputs.get_update_time_millis();
    if self.metronome.tick(update_time_millis) {
      self.update_overlay();
    }
  }
}
