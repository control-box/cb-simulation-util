//! # Impulse - Time Signal
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use cb_simulation_util::signal::{TimeRange, ImpulseFunction, TimeSignal};
//!
//! fn main () {
//!   let time: Array<f64, Ix1> = TimeRange::default().collect();
//!   let step_fn = ImpulseFunction::default().resting_level(2.0).amplitude(3.0).start(20.0);
//!   let signal: Array<f64, Ix1> = time.iter().map(|v| step_fn.time_to_signal(*v)).collect();
//!   assert_eq!(signal[0], 2.0);
//!   assert_eq!(signal[20], 3.0);
//! }
//! ```

use num_traits::{Num, one, zero};

pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImpulseFunction<S: Debug + Display + Clone + Copy + PartialEq> {
    pub out_value: S,
    pub in_value: S,
    pub start_time: f64,
    pub duration: f64,
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> ImpulseFunction<S> {
    pub fn resting_level(self, out_value: S) -> Self {
        ImpulseFunction::<S> { out_value, ..self }
    }

    pub fn amplitude(self, in_value: S) -> Self {
        ImpulseFunction::<S> { in_value, ..self }
    }

    pub fn start(self, start_time: f64) -> Self {
        ImpulseFunction::<S> { start_time, ..self }
    }

    pub fn duration(self, duration: f64) -> Self {
        ImpulseFunction::<S> { duration, ..self }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq> Default for ImpulseFunction<S> {
    fn default() -> Self {
        ImpulseFunction::<S> {
            out_value: zero(),
            in_value: one(),
            start_time: 0.0,
            duration: 1.0,
        }
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> TimeSignal<S>
    for ImpulseFunction<S>
{
    fn time_to_signal(&self, time: f64) -> S {
        if time < self.start_time {
            self.out_value
        } else {
            if time > self.start_time + self.duration {
                self.out_value
            } else {
                self.in_value
            }
        }
    }

    fn short_type_name(&self) -> &'static str {
        "Impulse"
    }
}

impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> fmt::Display
    for ImpulseFunction<S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}(amplitude={}, duration={}, start_time={}, rest_level={}",
            self.short_type_name(),
            self.in_value,
            self.duration,
            self.start_time,
            self.out_value,
        )
    }
}

// impl<S: Num + Debug + Display + Clone + Copy + PartialEq + 'static> DynTimeSignal<S>
//     for ImpulseFunction<S>
// {
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_impulse_build() {
        let sut = ImpulseFunction::<f64>::default()
            .resting_level(2.0)
            .amplitude(3.0)
            .start(1.0)
            .duration(2.0);
        let expected = ImpulseFunction::<f64> {
            out_value: 2.0,
            in_value: 3.0,
            start_time: 1.0,
            duration: 2.0,
        };
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_impulse_() {
        let sut = ImpulseFunction::<f64>::default();
        assert_eq!(sut.time_to_signal(-1.0), 0.0);
        assert_eq!(sut.time_to_signal(0.0), 1.0);
        assert_eq!(sut.time_to_signal(1.0), 1.0);
        assert_eq!(sut.time_to_signal(2.0), 0.0);
    }
}
