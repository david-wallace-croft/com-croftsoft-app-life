// =============================================================================
//! - CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-06
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use constants::INFO;
use engine::functions::web_sys::log;
use engine::looper::Looper;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

mod components;
mod constants;
mod engine;
mod messages;
mod models;
mod painters;
mod updaters;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Looper::launch();
  Ok(())
}
