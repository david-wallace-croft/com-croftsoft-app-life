// =============================================================================
//! - Root Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::{
  CellsUpdater, CellsUpdaterEvents, CellsUpdaterInputs, CellsUpdaterOptions,
};
use super::clock::{
  ClockUpdater, ClockUpdaterEvents, ClockUpdaterInputs, ClockUpdaterOptions,
};
use super::options::{OptionsUpdater, OptionsUpdaterInputs};
use super::overlay::{
  OverlayUpdater, OverlayUpdaterEvents, OverlayUpdaterInputs,
  OverlayUpdaterOptions,
};
use crate::models::options::Options;
use crate::models::overlay::Overlay;
use crate::models::root::Root;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdaterInputs;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::updater::{
  MetronomeUpdater, MetronomeUpdaterEvents, MetronomeUpdaterInputs,
};
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct RootUpdaterConfiguration {
  pub update_period_millis_initial: f64,
}

pub trait RootUpdaterEvents {
  fn get_updated(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_time_to_update(&mut self);
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated(&mut self);
}

struct RootUpdaterEventsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
}

impl RootUpdaterEventsAdapter {
  fn new(events: Rc<RefCell<dyn RootUpdaterEvents>>) -> Self {
    Self {
      events,
    }
  }
}

impl CellsUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl ClockUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl MetronomeUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self
      .events
      .borrow_mut()
      .set_update_period_millis_changed(update_period_millis);
  }

  fn set_tick(&mut self) {
    self.events.borrow_mut().set_time_to_update();
  }
}

impl OverlayUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

pub trait RootUpdaterInputs {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_current_time_millis(&self) -> f64;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
}

struct RootUpdaterInputsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
  inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
}

impl RootUpdaterInputsAdapter {
  fn new(
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      inputs,
    }
  }
}

impl CellsUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_cell_toggle_requested(&self) -> Option<usize> {
    self.inputs.borrow().get_cell_toggle_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl ClockUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl FrameRaterUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl MetronomeUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.inputs.borrow().get_period_millis_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl OptionsUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }

  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl OverlayUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self
      .inputs
      .borrow()
      .get_frame_rate_display_change_requested()
  }

  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

pub trait RootUpdaterOptions {
  fn get_pause(&self) -> bool;
}

struct RootUpdaterOptionsAdapter {
  options: Rc<RefCell<dyn RootUpdaterOptions>>,
}

impl RootUpdaterOptionsAdapter {
  fn new(options: Rc<RefCell<dyn RootUpdaterOptions>>) -> Self {
    Self {
      options,
    }
  }
}

impl CellsUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl ClockUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl OverlayUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

pub struct RootUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
}

impl RootUpdater {
  pub fn new(
    configuration: RootUpdaterConfiguration,
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
    root_model: Rc<RefCell<Root>>,
  ) -> Self {
    let root_updater_events_adapter =
      Rc::new(RefCell::new(RootUpdaterEventsAdapter::new(events.clone())));
    let root_updater_inputs_adapter = Rc::new(RefCell::new(
      RootUpdaterInputsAdapter::new(events.clone(), inputs.clone()),
    ));
    let root_updater_options_adapter = Rc::new(RefCell::new(
      RootUpdaterOptionsAdapter::new(options.clone()),
    ));
    let root_model: Ref<Root> = root_model.borrow();
    let cells = root_model.cells.clone();
    let clock = root_model.clock.clone();
    let cells_updater = CellsUpdater::new(
      cells,
      root_updater_events_adapter.clone(),
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter.clone(),
    );
    let clock_updater = ClockUpdater::new(
      clock.clone(),
      root_updater_events_adapter.clone(),
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter.clone(),
    );
    let frame_rater_updater = FrameRaterUpdater::new(
      false,
      frame_rater.clone(),
      root_updater_inputs_adapter.clone(),
    );
    let overlay: Rc<RefCell<Overlay>> = root_model.overlay.clone();
    let options_updater =
      OptionsUpdater::new(root_updater_inputs_adapter.clone(), options);
    let overlay_updater = OverlayUpdater::new(
      clock,
      root_updater_events_adapter.clone(),
      frame_rater,
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter,
      overlay,
    );
    let metronome = Rc::new(RefCell::new(DeltaMetronome {
      period_millis: configuration.update_period_millis_initial,
      time_millis_next_tick: 0.,
    }));
    let metronome_updater = MetronomeUpdater::new(
      root_updater_events_adapter,
      root_updater_inputs_adapter,
      metronome,
    );
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(metronome_updater),
      Box::new(options_updater),
      Box::new(frame_rater_updater),
      Box::new(clock_updater),
      Box::new(cells_updater),
      Box::new(overlay_updater),
    ];
    Self {
      child_updaters,
    }
  }
}

impl Updater for RootUpdater {
  fn update(&self) {
    self
      .child_updaters
      .iter()
      .for_each(|updater| updater.update());
  }
}
