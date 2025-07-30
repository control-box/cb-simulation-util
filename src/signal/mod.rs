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

use std::{borrow::ToOwned, boxed::Box, string::String};

pub trait TimeSignal<S: Debug + Display + Clone + Sized>: Any {
    fn time_to_signal(&self, time: f64) -> S;
    fn as_any(&self) -> &dyn Any;
}

pub trait TimeSignalSuperTrait<S: Debug + Display + Clone + Sized>:
    TimeSignal<S> + Debug + Display + DynClone + 'static
{
}

pub type BoxedTimeSignal<S> = Box<dyn TimeSignalSuperTrait<S> + 'static>;

impl<S> Clone for BoxedTimeSignal<S> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

impl<S: Debug + Display + Clone + Sized + 'static> PartialEq for BoxedTimeSignal<S> {
    fn eq(&self, other: &Self) -> bool {
        self.as_any().type_id() == other.as_any().type_id()
    }
}

pub mod impulse_fn;
pub mod step_fn;

pub use impulse_fn::*;
pub use step_fn::*;

pub mod time_range;
#[allow(unused_imports)]
pub use time_range::*;

#[derive(Debug, Clone)]
pub struct SuperPosition<S: Num + Debug + Display + Clone + PartialEq>(
    pub Box<dyn TimeSignalSuperTrait<S>>,
    pub Box<dyn TimeSignalSuperTrait<S>>,
);

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> fmt::Display for SuperPosition<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperPosition({}, {})", self.0, self.1)
    }
}

impl<S: Add<Output = S> + Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignal<S>
    for SuperPosition<S>
{
    fn time_to_signal(&self, time: f64) -> S {
        self.0.time_to_signal(time) + self.1.time_to_signal(time)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct NamedTimeSignal<S: Num + Debug + Display + Clone + Copy + PartialEq> {
    pub name: String,
    pub signal:  BoxedTimeSignal<S>,
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> NamedTimeSignal<S> {
    pub fn set_name(self, name: String) -> Self {
        NamedTimeSignal {
            name,
            ..self }
        }

    pub fn set_signal(self, signal:  BoxedTimeSignal<S>) -> Self {
        NamedTimeSignal {
            signal,
            ..self }
        }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> PartialEq
    for NamedTimeSignal<S>
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> Default for NamedTimeSignal<S> {
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
