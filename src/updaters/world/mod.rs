// =============================================================================
//! - World Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-02-24
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
use super::frame_rater::{FrameRaterUpdater, FrameRaterUpdaterInputs};
use super::options::{OptionsUpdater, OptionsUpdaterInputs};
use super::overlay::{
  OverlayUpdater, OverlayUpdaterEvents, OverlayUpdaterInputs,
};
use crate::engine::frame_rater::FrameRater;
use crate::models::options::Options;
use crate::models::overlay::Overlay;
use crate::models::world::World;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::updater::{
  MetronomeUpdater, MetronomeUpdaterEvents, MetronomeUpdaterInputs,
};
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct WorldUpdaterConfiguration {
  pub update_period_millis_initial: f64,
}

pub trait WorldUpdaterEvents {
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

struct WorldUpdaterEventsAdapter {
  events: Rc<RefCell<dyn WorldUpdaterEvents>>,
}

impl WorldUpdaterEventsAdapter {
  fn new(events: Rc<RefCell<dyn WorldUpdaterEvents>>) -> Self {
    Self {
      events,
    }
  }
}

impl CellsUpdaterEvents for WorldUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl ClockUpdaterEvents for WorldUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

impl MetronomeUpdaterEvents for WorldUpdaterEventsAdapter {
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

impl OverlayUpdaterEvents for WorldUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

pub trait WorldUpdaterInputs {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_current_time_millis(&self) -> f64;
}

struct WorldUpdaterInputsAdapter {
  events: Rc<RefCell<dyn WorldUpdaterEvents>>,
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
}

impl WorldUpdaterInputsAdapter {
  fn new(
    events: Rc<RefCell<dyn WorldUpdaterEvents>>,
    inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      inputs,
    }
  }
}

impl CellsUpdaterInputs for WorldUpdaterInputsAdapter {
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

impl ClockUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }
}

impl FrameRaterUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_frame_rate_display_change_requested()
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

impl OptionsUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_frame_rate_display_change_requested()
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

impl MetronomeUpdaterInputs for WorldUpdaterInputsAdapter {
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

impl OverlayUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_frame_rate_display_change_requested()
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

pub trait WorldUpdaterOptions {
  fn get_pause(&self) -> bool;
}

struct WorldUpdaterOptionsAdapter {
  options: Rc<RefCell<dyn WorldUpdaterOptions>>,
}

impl WorldUpdaterOptionsAdapter {
  fn new(options: Rc<RefCell<dyn WorldUpdaterOptions>>) -> Self {
    Self {
      options,
    }
  }
}

impl CellsUpdaterOptions for WorldUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

impl ClockUpdaterOptions for WorldUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }
}

pub struct WorldUpdater {
  child_updaters: [Box<dyn Updater>; 6],
}

impl WorldUpdater {
  pub fn new(
    configuration: WorldUpdaterConfiguration,
    events: Rc<RefCell<dyn WorldUpdaterEvents>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let world_updater_events_adapter =
      Rc::new(RefCell::new(WorldUpdaterEventsAdapter::new(events.clone())));
    let world_updater_inputs_adapter = Rc::new(RefCell::new(
      WorldUpdaterInputsAdapter::new(events.clone(), inputs.clone()),
    ));
    let world_updater_options_adapter = Rc::new(RefCell::new(
      WorldUpdaterOptionsAdapter::new(options.clone()),
    ));
    let world: Ref<World> = world.borrow();
    let cells = world.cells.clone();
    let clock = world.clock.clone();
    let cells_updater = CellsUpdater::new(
      cells,
      world_updater_events_adapter.clone(),
      world_updater_inputs_adapter.clone(),
      world_updater_options_adapter.clone(),
    );
    let clock_updater = ClockUpdater::new(
      clock.clone(),
      world_updater_events_adapter.clone(),
      world_updater_inputs_adapter.clone(),
      world_updater_options_adapter,
    );
    let frame_rater_updater = FrameRaterUpdater::new(
      frame_rater.clone(),
      world_updater_inputs_adapter.clone(),
      options.clone(),
    );
    let overlay: Rc<RefCell<Overlay>> = world.overlay.clone();
    let options_updater =
      OptionsUpdater::new(world_updater_inputs_adapter.clone(), options);
    let overlay_updater = OverlayUpdater::new(
      clock,
      world_updater_events_adapter.clone(),
      frame_rater,
      world_updater_inputs_adapter.clone(),
      overlay,
    );
    let metronome = Rc::new(RefCell::new(DeltaMetronome {
      period_millis: configuration.update_period_millis_initial,
      time_millis_next_tick: 0.,
    }));
    let metronome_updater = MetronomeUpdater::new(
      world_updater_events_adapter,
      world_updater_inputs_adapter,
      metronome,
    );
    let child_updaters: [Box<dyn Updater>; 6] = [
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

impl Updater for WorldUpdater {
  fn update(&mut self) {
    self.child_updaters.iter_mut().for_each(|updater| updater.update());
  }
}
