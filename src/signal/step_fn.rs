//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use control_box::signal::{TimeRange, StepFunction, TimeSignal};
//!
//! fn main () {
//!   let time: Array<f32, Ix1> = TimeRange::default().collect();
//!   let step_fn = StepFunction::default().pre(2.0).post(3.0).step(1.1);
//!   let signal: Array<f32, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();
//!   assert_eq!(signal[0], 2.0);
//!   assert_eq!(signal[20], 3.0);
//! }
//! ```

pub use super::*;


#[derive(Debug, Clone, Copy)]
pub struct StepFunction {
    pub pre_value: f32,
    pub post_value: f32,
    pub step_time: f32,
}

impl StepFunction {

    pub fn pre(self, pre_value: f32) -> Self {
        StepFunction {
            pre_value,
            ..self
        }
    }

    pub fn post(self, post_value: f32) -> Self {
        StepFunction {
            post_value,
            ..self
        }
    }

    pub fn step(self, step_time: f32) -> Self {
        StepFunction {
            step_time,
            ..self
        }
    }
}

impl Default for StepFunction {
    fn default() -> Self {
        StepFunction {
            pre_value: 0.0,
            post_value: 1.0,
            step_time: 0.0,
        }
    }
}

impl TimeSignal<f32> for StepFunction {

        fn time_to_signal(&self, time: f32 ) -> f32 {
            if self.step_time > time { self.post_value }
            else { self.pre_value }
        }
}


