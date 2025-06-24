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

use num_traits::Num;
use core::ops::Add;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;


use std::{boxed::Box, borrow::ToOwned,string::String};


pub trait TimeSignal<S: Debug + Display> {

    fn time_to_signal(&self, time: f64 ) -> S;

}

pub trait TimeSignalSuperTrait<S: Debug + Display>:  TimeSignal<S> + Debug + Display
{}

pub mod step_fn;
pub use step_fn::*;

pub mod time_range;
#[allow(unused_imports)]
pub use time_range::*;

#[derive(Debug )]
pub struct SuperPosition<S: Num +  Debug + Display + Clone + PartialEq> ( pub Box<dyn TimeSignalSuperTrait<S>>, pub Box<dyn TimeSignalSuperTrait<S>>);

impl<S:Num +  Debug + Display + Clone + Copy + PartialEq> fmt::Display for SuperPosition<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperPosition({}, {})", self.0, self.1)
    }
}

impl <S: Add<Output = S> +  Num +  Debug + Display + Clone + Copy + PartialEq> TimeSignal<S> for SuperPosition<S>  {

    fn time_to_signal(&self, time: f64 ) -> S {
        self.0.time_to_signal(time) + self.1.time_to_signal(time)
    }
}


#[derive(Debug)]
pub struct NamedTimeSignal<S: Num +  Debug + Display + Clone + Copy + PartialEq>  {
    name: String,
    signal: Box<dyn TimeSignalSuperTrait<S> + 'static>,
}

impl<S:Num +  Debug + Display + Clone + Copy + PartialEq+ 'static> Default for NamedTimeSignal<S> {
    fn default() -> Self {
        NamedTimeSignal {
            name: "Default Step Function".to_owned(),
            signal: Box::new(StepFunction::<S>::default()),
        }
    }
}

impl<S:Num +  Debug + Display + Clone + Copy + PartialEq> fmt::Display for NamedTimeSignal<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Time Signal: {} = {}", self.name, self.signal)
    }
}

