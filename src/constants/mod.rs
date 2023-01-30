// =============================================================================
//! - CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2023-01-29
//! - Rust since: 2023-01-09
//! - Java version: 2008-11-05
//! - Java since: 2008-11-04
//!
//! # History
//! - Adapted from the Java package com.croftsoft.apps.life
//!   - In the Java-based [`CroftSoft Apps Library`]
//!
//! [`CroftSoft Apps Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::configuration::Configuration;

pub static INFO: &str =
  "CroftSoft Life v0.2.2-SNAPSHOT Copyright 2023 CroftSoft Inc";

pub const CELL_COUNT: usize = SPACE_HEIGHT * SPACE_WIDTH;
pub const CELL_PAINT_OFFSET: f64 = (1. - CELL_PAINT_SIZE) / 2.;
pub const CELL_PAINT_SIZE: f64 = 2. / 3.;
pub const FRAME_PERIOD_MILLIS_MINIMUM: f64 = 10.0;
pub const FRAMES_PER_SECOND: f64 = 1.0;
pub const SPACE_HEIGHT: usize = 100;
pub const SPACE_WIDTH: usize = 100;

pub const CONFIGURATION: Configuration = Configuration {
  frame_period_millis: 1_000.0 / FRAMES_PER_SECOND,
};
