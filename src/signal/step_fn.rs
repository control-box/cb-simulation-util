//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use control_box::signal::{TimeRange, StepFunction, TimeSignal};
//!
//! fn main () {
//!   let time: Array<f64, Ix1> = TimeRange::default().collect();
//!   let step_fn = StepFunction::default().pre(2.0).post(3.0).step(1.1);
//!   let signal: Array<f64, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();
//!   assert_eq!(signal[0], 2.0);
//!   assert_eq!(signal[20], 3.0);
//! }
//! ```

use num_traits::{Num, one, zero};


pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepFunction<S: Debug + Display + Clone + Copy + PartialEq> {
    pub pre_value: S,
    pub post_value: S,
    pub step_time: f64,
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> StepFunction<S> {
    pub fn pre(self, pre_value: S) -> Self {
        StepFunction::<S> { pre_value, ..self }
    }

    pub fn post(self, post_value: S) -> Self {
        StepFunction::<S> { post_value, ..self }
    }

    pub fn step(self, step_time: f64) -> Self {
        StepFunction::<S> { step_time, ..self }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> Default for StepFunction<S> {
    fn default() -> Self {
        StepFunction::<S> {
            pre_value: zero(),
            post_value: one(),
            step_time: 0.0,
        }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignal<S>
    for StepFunction<S>
{
    fn time_to_signal(&self, time: f64) -> S {
        if self.step_time < time {
            self.post_value
        } else {
            self.pre_value
        }
    }

    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> fmt::Display for StepFunction<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Step(step_time={}, pre={}, post={}",
            self.step_time, self.pre_value, self.post_value
        )
    }
}

// impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> DynTimeSignal<S>
//     for StepFunction<S>
// {
// }
