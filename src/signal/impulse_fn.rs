//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use control_box::signal::{TimeRange, ImpulsFunction, TimeSignal};
//!
//! fn main () {
//!   let time: Array<f64, Ix1> = TimeRange::default().collect();
//!   let step_fn = ImpulsFunction::default().pre(2.0).post(3.0).step(1.1);
//!   let signal: Array<f64, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();
//!   assert_eq!(signal[0], 2.0);
//!   assert_eq!(signal[20], 3.0);
//! }
//! ```

use num_traits::{Num, one, zero};

pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImpulsFunction<S: Debug + Display + Clone + Copy + PartialEq> {
    pub out_value: S,
    pub in_value: S,
    pub start_time: f64,
    pub duration: f64,
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> ImpulsFunction<S> {
    pub fn resting_level(self, out_value: S) -> Self {
        ImpulsFunction::<S> { out_value, ..self }
    }

    pub fn amplitude(self, in_value: S) -> Self {
        ImpulsFunction::<S> { in_value, ..self }
    }

    pub fn start(self, start_time: f64) -> Self {
        ImpulsFunction::<S> { start_time, ..self }
    }

    pub fn duration(self, duration: f64) -> Self {
        ImpulsFunction::<S> { duration, ..self }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> Default for ImpulsFunction<S> {
    fn default() -> Self {
        ImpulsFunction::<S> {
            out_value: zero(),
            in_value: one(),
            start_time: 0.0,
            duration: 1.0,
        }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignal<S>
    for ImpulsFunction<S>
{
    fn time_to_signal(&self, time: f64) -> S {
        if self.start_time < time {
            self.out_value
        } else {
            if self.start_time + self.duration > time {
                self.out_value
            } else {
                self.in_value
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> fmt::Display for ImpulsFunction<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Impulse(amplitude={}, duration={}, start_time={}, rest_level={}",
            self.in_value, self.duration, self.start_time, self.out_value,
        )
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignalSuperTrait<S>
    for ImpulsFunction<S>
{
}
