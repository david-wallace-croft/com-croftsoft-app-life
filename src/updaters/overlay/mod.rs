// =============================================================================
//! - Overlay Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-13
//! - Updated: 2023-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::OVERLAY_REFRESH_PERIOD_MILLIS;
use crate::models::clock::Clock;
use crate::models::overlay::Overlay;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait OverlayUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait OverlayUpdaterInputs {
  fn get_current_time_millis(&self) -> f64;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_time_to_update(&self) -> bool;
  fn get_reset_requested(&self) -> bool;
}

pub trait OverlayUpdaterOptions {
  fn get_pause(&self) -> bool;
}

pub struct OverlayUpdater {
  clock: Rc<RefCell<Clock>>,
  events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  metronome: RefCell<DeltaMetronome>,
  options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayUpdater {
  fn make_update_rate_string(&self) -> String {
    format!(
      "Simulation updates per second: {:.3}",
      self.frame_rater.borrow().get_frames_per_second_sampled()
    )
  }

  fn make_updates_string(&self) -> String {
    format!("Updates: {}", self.clock.borrow().time)
  }

  pub fn new(
    clock: Rc<RefCell<Clock>>,
    events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let metronome = RefCell::new(DeltaMetronome {
      period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      time_millis_next_tick: 0.,
    });
    Self {
      clock,
      events,
      frame_rater,
      inputs,
      metronome,
      options,
      overlay,
    }
  }

  fn update_overlay(&self) {
    let mut overlay: RefMut<Overlay> = self.overlay.borrow_mut();
    overlay.clock_string = self.make_updates_string();
    overlay.frame_rate_string = self.make_update_rate_string();
    self.events.borrow_mut().set_updated();
  }
}

impl Updater for OverlayUpdater {
  fn update(&self) {
    let inputs: Ref<dyn OverlayUpdaterInputs> = self.inputs.borrow();
    if inputs.get_frame_rate_display_change_requested().is_some()
      || inputs.get_pause_change_requested().is_some()
      || inputs.get_reset_requested()
    {
      self.update_overlay();
      return;
    }
    if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
      return;
    }
    let current_time_millis = inputs.get_current_time_millis();
    if self.metronome.borrow_mut().tick(current_time_millis) {
      self.update_overlay();
    }
  }
}
