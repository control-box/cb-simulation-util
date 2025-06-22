//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use control_box::signal::{TimeRange, StepFunction, TimeSignal, SuperPosition};
//!
//! fn main () {
//!   let time: Array<f32, Ix1> = TimeRange::default().collect();
//!
//!   let step_fn_0 = StepFunction::default();
//!   let step_fn_1 = StepFunction::default().pre(0.0).post(-1.0).step(1.0);
//!   let super_position = SuperPosition(Box::new(step_fn_0), Box::new(step_fn_1));
//!
//!   let signal: Array<f32, Ix1> = time.iter().map(|v| super_position.time_to_signal(*v)).collect();
//! }
//! ```
pub mod time_range;
#[allow(unused_imports)]
pub use time_range::*;

use num_traits::Num;
use core::ops::Add;

use std::boxed::Box;

pub trait TimeSignal<S: Num > {

    fn time_to_signal(&self, time: f32 ) -> S;

}

pub mod step_fn;
pub use step_fn::*;

pub struct SuperPosition<S> ( Box<dyn TimeSignal<S>>, Box<dyn TimeSignal<S>>);


impl <S: Add<Output = S> + Num> TimeSignal<S> for SuperPosition<S>  {

    fn time_to_signal(&self, time: f32 ) -> S {
        self.0.time_to_signal(time) + self.1.time_to_signal(time)
    }
}


