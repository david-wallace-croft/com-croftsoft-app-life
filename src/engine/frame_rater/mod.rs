// =============================================================================
//! - Frame Rater for CroftSoft Life
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-13
//! - Updated: 2023-02-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use std::collections::VecDeque;

const FRAME_SAMPLE_SIZE_MAX: usize = 1_000;
const MILLIS_PER_SECOND: f64 = 1_000.;
const SAMPLE_TIME_MILLIS: f64 = 1_000.;

pub struct FrameRater {
  frame_rate: f64,
  frame_sample_size_target: usize,
  update_time_millis_next: f64,
  update_times: VecDeque<f64>,
}

impl FrameRater {
  fn calculate_frame_sample_size_target(frame_period_millis: f64) -> usize {
    let mut frame_sample_size = if frame_period_millis > 0. {
      (SAMPLE_TIME_MILLIS / frame_period_millis) as usize
    } else {
      FRAME_SAMPLE_SIZE_MAX
    };
    if frame_sample_size < 1 {
      frame_sample_size = 1;
    } else if frame_sample_size > FRAME_SAMPLE_SIZE_MAX {
      frame_sample_size = FRAME_SAMPLE_SIZE_MAX;
    }
    frame_sample_size
  }

  pub fn clear(&mut self) {
    self.update_times.clear();
    self.frame_rate = 0.;
  }

  pub fn get_frames_per_second_sampled(&self) -> f64 {
    self.frame_rate
  }

  pub fn new(frame_period_millis_target: f64) -> Self {
    let mut frame_rater = Self {
      frame_rate: 0.,
      frame_sample_size_target: 0,
      update_time_millis_next: 0.,
      update_times: VecDeque::new(),
    };
    frame_rater.update_frame_sample_size(frame_period_millis_target);
    frame_rater
  }

  pub fn sample(
    &mut self,
    update_time_millis: f64,
  ) -> bool {
    let deltas = self.update_times.len();
    self.update_times.push_back(update_time_millis);
    if deltas < 1 {
      return false;
    }
    let mut frame_sample_size = self.frame_sample_size_target;
    if frame_sample_size > deltas {
      frame_sample_size = deltas;
    }
    let index = deltas - frame_sample_size;
    let first_update_time = self.update_times[index];
    let delta = update_time_millis - first_update_time;
    self.frame_rate = frame_sample_size as f64 * MILLIS_PER_SECOND / delta;
    if deltas >= FRAME_SAMPLE_SIZE_MAX {
      self.update_times.pop_front();
    }
    false
  }

  pub fn update_frame_sample_size(
    &mut self,
    mut frame_period_millis: f64,
  ) {
    if frame_period_millis < 0. {
      frame_period_millis = 0.;
    }
    self.update_time_millis_next = 0.;
    self.frame_sample_size_target =
      Self::calculate_frame_sample_size_target(frame_period_millis);
    self.update_times.clear();
    self.frame_rate = 0.;
  }
}
