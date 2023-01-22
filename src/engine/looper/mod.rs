// =============================================================================
//! - Looper for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-22
//! - Since: 2023-01-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::configuration::Configuration;
use super::functions::web_sys::LoopUpdater;
use super::input::Input;
use crate::components::life::LifeComponent;
use crate::constants::{CONFIGURATION, FRAME_PERIOD_MILLIS_MINIMUM};
use crate::engine::functions::web_sys::spawn_local_loop;
use crate::models::world::World;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  configuration: Configuration,
  frame_period_millis: f64,
  input: Rc<RefCell<Input>>,
  life_component: LifeComponent,
  next_update_time: f64,
  world: Rc<RefCell<World>>,
}

impl Looper {
  pub fn launch() {
    let mut looper = Looper::default();
    looper.initialize();
    spawn_local_loop(looper);
  }

  pub fn new(configuration: Configuration) -> Self {
    let Configuration {
      frame_period_millis,
    } = configuration;
    let input = Rc::new(RefCell::new(Input::default()));
    let world = Rc::new(RefCell::new(World::new(input.clone())));
    let life_component =
      LifeComponent::new("life", input.clone(), world.clone());
    Self {
      configuration,
      input,
      frame_period_millis,
      life_component,
      next_update_time: 0.0,
      world,
    }
  }

  fn update_frame_rate(&mut self) {
    if !self.input.borrow().speed_toggle_requested {
      return;
    }
    if self.frame_period_millis == FRAME_PERIOD_MILLIS_MINIMUM {
      self.frame_period_millis = self.configuration.frame_period_millis;
    } else {
      self.frame_period_millis = FRAME_PERIOD_MILLIS_MINIMUM;
    }
  }
}

impl Default for Looper {
  fn default() -> Self {
    Looper::new(CONFIGURATION)
  }
}

impl Initializer for Looper {
  fn initialize(&mut self) {
    self.life_component.initialize();
    self.input.borrow_mut().reset_requested = true;
  }
}

impl LoopUpdater for Looper {
  fn update_loop(
    &mut self,
    update_time: f64,
  ) {
    if update_time < self.next_update_time {
      return;
    }
    self.life_component.update();
    self.world.borrow_mut().update();
    self.life_component.paint();
    self.update_frame_rate();
    self.next_update_time = update_time + self.frame_period_millis;
    self.input.borrow_mut().clear();
  }
}
