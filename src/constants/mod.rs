// =============================================================================
//! - CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-09
//! - Updated: 2024-02-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::configuration::Configuration;

pub static INFO: &str =
  "CroftSoft Life v0.4.2-SNAPSHOT Copyright 2023-2024 CroftSoft Inc";

pub const CELL_COUNT: usize = SPACE_HEIGHT * SPACE_WIDTH;
pub const CELL_PAINT_OFFSET: f64 = (1. - CELL_PAINT_SIZE) / 2.;
pub const CELL_PAINT_SIZE: f64 = 2. / 3.;
pub const FILL_STYLE_BACKGROUND: &str = "black";
pub const MILLIS_PER_SECOND: f64 = 1_000.;
pub const OVERLAY_REFRESH_PERIOD_MILLIS: f64 = 1_000.;
pub const SPACE_HEIGHT: usize = 100;
pub const SPACE_WIDTH: usize = 100;
pub const UPDATES_PER_SECOND: f64 = 1.;
pub const UPDATE_PERIOD_MILLIS: f64 = MILLIS_PER_SECOND / UPDATES_PER_SECOND;

pub const CONFIGURATION: Configuration = Configuration {
  update_period_millis_initial: UPDATE_PERIOD_MILLIS,
};
