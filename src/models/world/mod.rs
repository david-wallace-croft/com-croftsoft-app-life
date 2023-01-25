// =============================================================================
//! - World Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-01-24
//! - Since: 2023-01-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::Cells;
use super::clock::Clock;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
  pub cells: Rc<RefCell<Cells>>,
  pub clock: Rc<RefCell<Clock>>,
}
