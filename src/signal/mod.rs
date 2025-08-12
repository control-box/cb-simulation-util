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

use core::any::Any;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use core::ops::Add;
use dyn_clone::DynClone; // DynClone is a trait with clones a Box
use num_traits::Num;

use std::boxed::Box;

pub trait TimeSignal<S: Debug + Display + Clone + Copy + Sized>: Any {
    /// Mapping from time to signal
    fn time_to_signal(&self, time: f64) -> S;
    /// Treated as a "dynamic type identifier"
    /// It should be one word starting with a capital letter
    fn short_type_name(&self) -> &'static str;

}

pub trait DynTimeSignal<S: Debug + Display + Clone + Copy + Sized>:
    TimeSignal<S> + Debug + Display + DynClone + 'static
{
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_time_signal(&self) -> &dyn DynTimeSignal<S>;
    fn dyn_eq(&self, other: &dyn DynTimeSignal<S>) -> bool;
}

impl<T, S> DynTimeSignal<S> for T
where
    T: TimeSignal<S> + Debug + Display + DynClone + Copy + 'static + PartialEq,
    S: Debug + Display + Clone + Copy + Sized + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_time_signal(&self) -> &dyn DynTimeSignal<S> {
        self
    }

    fn dyn_eq(&self, other: &dyn DynTimeSignal<S>) -> bool {
        if let Some(other_t) = other.as_any().downcast_ref::<T>() {
            self == other_t
        } else {
            false
        }
    }
}


pub type BoxedTimeSignal<S> = Box<dyn DynTimeSignal<S> + 'static>;

impl<S> Clone for BoxedTimeSignal<S> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}


impl<S: Debug + Display + Clone + Copy + Sized + 'static> PartialEq for BoxedTimeSignal<S> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.clone().as_dyn_time_signal())
    }
}

pub mod impulse_fn;
pub mod named_time_signal;
pub mod step_fn;

pub use impulse_fn::*;
pub use named_time_signal::*;
pub use step_fn::*;

pub mod time_range;
#[allow(unused_imports)]
pub use time_range::*;

#[derive(Debug, Clone)]
pub struct SuperPosition<S: Num + Debug + Display + Clone + PartialEq>(
    pub Box<dyn DynTimeSignal<S>>,
    pub Box<dyn DynTimeSignal<S>>,
);

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> fmt::Display for SuperPosition<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}, {})", self.short_type_name(), self.0, self.1)
    }
}

impl<S: Add<Output = S> + Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignal<S>
    for SuperPosition<S>
{
    fn time_to_signal(&self, time: f64) -> S {
        self.0.time_to_signal(time) + self.1.time_to_signal(time)
    }

    fn short_type_name(&self) -> &'static str {
        "Superposition"
    }
}
