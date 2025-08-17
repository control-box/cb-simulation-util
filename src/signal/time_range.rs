//! # Time Range
//!
//! ## Example
//!
//! ```rust
//! use ndarray::{Array, Ix1};
//! use cb_simulation_util::signal::{TimeRange, StepFunction, TimeSignal};
//!
//! fn main () {
//!   let range = TimeRange::default().set_start(-5.0).set_end(15.0).set_number_of_samples(Some(10));
//!   assert_eq!(range.len(), 10);
//!   let time: Array<f64, Ix1> = range.collect();
//! }
//! ```

use core::default::Default;
use core::option::Option;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeRange {
    pub unit_of_measurement: &'static str,
    pub start: f64,
    pub end: f64,
    pub sampling_interval: f64,
    current: f64,
}

const DEFAULT_SAMPLES: usize = 100;

impl Default for TimeRange {
    fn default() -> Self {
        TimeRange {
            unit_of_measurement: "ms",
            start: 0.0,
            end: 100.0,
            sampling_interval: 1.0,
            current: 0.0,
        }
    }
}

impl TimeRange {
    pub fn set_unit_of_measurement(self, unit_of_measurement: &'static str) -> Self {
        TimeRange {
            unit_of_measurement,
            ..self
        }
    }

    pub fn set_start(self, start: f64) -> Self {
        if start > self.end {
            panic!("Start must be less than end")
        }
        TimeRange {
            start,
            current: start,
            ..self
        }
    }

    pub fn set_end(self, end: f64) -> Self {
        if self.start > end {
            panic!("Start must be less than end")
        }
        TimeRange { end, ..self }
    }

    pub fn set_number_of_samples(self, samples: Option<usize>) -> Self {
        let samples = match samples {
            None => DEFAULT_SAMPLES,
            Some(s) => s,
        };
        let sampling_interval = (self.end - self.start) / samples as f64;
        TimeRange {
            sampling_interval,
            ..self
        }
    }

    pub fn set_sampling_interval(self, sampling_interval: f64) -> Self {
        if self.end - self.start < sampling_interval {
            panic!("Sampling interval too small")
        }
        TimeRange {
            sampling_interval,
            ..self
        }
    }
}

impl Iterator for TimeRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }
        self.current += self.sampling_interval;
        Some(self.current)
    }
}

impl ExactSizeIterator for TimeRange {
    fn len(&self) -> usize {
        ((self.end - self.start) / self.sampling_interval) as usize
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn time_range_unit_of_measurement() {
        let sut = TimeRange::default();
        assert!("ms" == sut.unit_of_measurement);
        let sut = TimeRange::default().set_unit_of_measurement("sec");
        assert!("sec" == sut.unit_of_measurement);
    }

    #[test]
    fn time_range_start() {
        let sut = TimeRange::default();
        assert!(0.0 == sut.start);
        let sut = TimeRange::default().set_start(-5.1);
        assert!(-5.1 == sut.start);
    }

    #[test]
    #[should_panic]
    fn time_range_start_panic() {
        let _sut = TimeRange::default().set_start(5000.0);
    }

    #[test]
    fn time_range_end() {
        let sut = TimeRange::default();
        assert!(100.0 == sut.end);
        let sut = TimeRange::default().set_end(5.1);
        assert!(5.1 == sut.end);
    }

    #[test]
    #[should_panic]
    fn time_range_end_panic() {
        let _sut = TimeRange::default().set_end(-5000.0);
    }

    #[test]
    fn time_range_sampling_interval() {
        let sut = TimeRange::default();
        assert!(1.0 == sut.sampling_interval);
        let sut = TimeRange::default().set_sampling_interval(0.1);
        assert!(0.1 == sut.sampling_interval);
    }

    #[test]
    #[should_panic]
    fn time_range_sampling_interval_panic() {
        let _sut = TimeRange::default().set_sampling_interval(5000.0);
    }

    #[test]
    fn time_range_number_of_samples() {
        let sut = TimeRange::default().set_number_of_samples(Some(50));
        assert!(2.0 == sut.sampling_interval);
        let sut = sut.set_number_of_samples(None);
        assert!(1.0 == sut.sampling_interval);
    }
}
