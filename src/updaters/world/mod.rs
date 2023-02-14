// =============================================================================
//! - World Updater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-24
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::{CellsUpdater, CellsUpdaterInputs};
use super::clock::{ClockUpdater, ClockUpdaterInputs};
use super::frame_rate::{
  FrameRateUpdater, FrameRateUpdaterEvents, FrameRateUpdaterInputs,
};
use super::overlay::{OverlayUpdater, OverlayUpdaterInputs};
use crate::engine::frame_rater::FrameRater;
use crate::engine::update_timer::UpdateTimer;
use crate::models::frame_rate::FrameRate;
use crate::models::overlay::Overlay;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct WorldUpdaterConfiguration {
  pub update_period_millis_initial: f64,
}

pub trait WorldUpdaterEvents {
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated_world(&mut self);
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

impl FrameRateUpdaterEvents for WorldUpdaterEventsAdapter {
  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }
}

pub trait WorldUpdaterInputs {
  fn get_cell_toggle_requested(&self) -> Option<usize>;
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_speed_change_requested(&self) -> Option<usize>;
  fn get_update_time_millis(&self) -> f64;
}

struct WorldUpdaterInputsAdapter {
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
}

impl WorldUpdaterInputsAdapter {
  fn new(inputs: Rc<RefCell<dyn WorldUpdaterInputs>>) -> Self {
    Self {
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
}

impl ClockUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl FrameRateUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_frame_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_update_time_millis()
  }
}

impl OverlayUpdaterInputs for WorldUpdaterInputsAdapter {
  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_update_time_millis()
  }
}

pub struct WorldUpdater {
  events: Rc<RefCell<dyn WorldUpdaterEvents>>,
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
  update_timer_world: UpdateTimer,
  updaters: [Box<dyn Updater>; 4],
}

impl WorldUpdater {
  pub fn new(
    configuration: WorldUpdaterConfiguration,
    events: Rc<RefCell<dyn WorldUpdaterEvents>>,
    frame_rate: Rc<RefCell<FrameRate>>,
    frame_rater: Rc<RefCell<FrameRater>>,
    inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let world_updater_events_adapter =
      Rc::new(RefCell::new(WorldUpdaterEventsAdapter::new(events.clone())));
    let update_timer_world = UpdateTimer {
      update_period_millis: configuration.update_period_millis_initial,
      update_time_millis_next: 0.,
    };
    let world_updater_inputs_adapter =
      Rc::new(RefCell::new(WorldUpdaterInputsAdapter::new(inputs.clone())));
    let world: Ref<World> = world.borrow();
    let cells = world.cells.clone();
    let clock = world.clock.clone();
    let cells_updater =
      CellsUpdater::new(cells, world_updater_inputs_adapter.clone());
    let clock_updater =
      ClockUpdater::new(clock.clone(), world_updater_inputs_adapter.clone());
    let overlay: Rc<RefCell<Overlay>> = world.overlay.clone();
    let frame_rate_updater = FrameRateUpdater::new(
      world_updater_events_adapter,
      frame_rate,
      frame_rater.clone(),
      world_updater_inputs_adapter.clone(),
    );
    let overlay_updater = OverlayUpdater::new(
      clock,
      frame_rater,
      world_updater_inputs_adapter,
      overlay,
    );
    let updaters: [Box<dyn Updater>; 4] = [
      Box::new(clock_updater),
      Box::new(cells_updater),
      Box::new(overlay_updater),
      Box::new(frame_rate_updater),
    ];
    Self {
      events,
      inputs,
      update_timer_world,
      updaters,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    if let Some(speed) = self.inputs.borrow().get_speed_change_requested() {
      self.update_timer_world.update_period_millis =
        (1_000. / speed as f64).trunc();
      self.events.borrow_mut().set_update_period_millis_changed(
        self.update_timer_world.update_period_millis,
      );
      self.update_timer_world.update_time_millis_next = 0.;
    }
    let update_time_millis = self.inputs.borrow().get_update_time_millis();
    {
      let inputs: Ref<dyn WorldUpdaterInputs> = self.inputs.borrow();
      if self.update_timer_world.before_next_update_time(update_time_millis)
        && inputs.get_frame_rate_display_change_requested().is_none()
        && !inputs.get_reset_requested()
      {
        return;
      }
    }
    self.updaters.iter_mut().for_each(|updater| updater.update());
    self.events.borrow_mut().set_updated_world();
  }
}