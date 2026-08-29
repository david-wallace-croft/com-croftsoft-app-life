// =============================================================================
//! - CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-06
//! - Updated: 2026-08-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// TODO
#![expect(deprecated)]

use com_croftsoft_lib_animation::web_sys::log;
use constants::INFO;
use engine::looper::Looper;
use wasm_bindgen::prelude::*;

mod components;
mod constants;
mod engine;
mod messages;
mod models;
mod painters;
mod updaters;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Looper::launch();
  Ok(())
}
