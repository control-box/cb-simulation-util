//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use control_box::signal::{TimeRange, StepFunction, TimeSignal, SuperPosition};
//!
//! fn main () {
//!   let time: Array<f64, Ix1> = TimeRange::default().collect();
//!
//!   let step_fn_0 = StepFunction::<f64>::default();
//!   let step_fn_1 = StepFunction::<f64>::default().pre(0.0).post(-1.0).step(1.0);
//!   let super_position = SuperPosition::<f64>(Box::new(step_fn_0), Box::new(step_fn_1));
//!
//!   let signal: Array<f64, Ix1> = time.iter().map(|v| super_position.time_to_signal(*v)).collect();
//! }
//! ```

use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use num_traits::Num;

use std::{borrow::ToOwned, boxed::Box, string::String};

pub use super::*;

#[derive(Debug, Clone)]
pub struct NamedTimeSignal<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> {
    pub name: String,
    pub signal: BoxedTimeSignal<S>,
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> NamedTimeSignal<S> {
    pub fn set_name(self, name: String) -> Self {
        NamedTimeSignal { name, ..self }
    }

    pub fn set_signal(self, signal: BoxedTimeSignal<S>) -> Self {
        NamedTimeSignal { signal, ..self }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static + Send + Sync> PartialEq
    for NamedTimeSignal<S>
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signal == other.signal.clone()
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static + Send + Sync> Default for NamedTimeSignal<S> {
    fn default() -> Self {
        NamedTimeSignal {
            name: "Signal".to_owned(),
            signal: Box::new(StepFunction::<S>::default()),
        }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> fmt::Display for NamedTimeSignal<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time Signal: {} = {}", self.name, self.signal)
    }
}
