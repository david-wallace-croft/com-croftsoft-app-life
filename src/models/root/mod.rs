// =============================================================================
//! - Root Model for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-10
//! - Updated: 2023-03-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::cells::Cells;
use super::clock::Clock;
use super::overlay::Overlay;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Root {
  pub cells: Rc<RefCell<Cells>>,
  pub clock: Rc<RefCell<Clock>>,
  pub overlay: Rc<RefCell<Overlay>>,
}
