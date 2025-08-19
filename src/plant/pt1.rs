//! A common PT1 element aka first order lag element
//!
//! $ out[k]= out[k-1]+ \alpha (P * in[k]-out[k-1]) $
//!
//! where $\alpha =\frac{T_{s}}{T_{1}}$
//! and $T_{s}$ is the sample time constant
//! and $P$ is the amplification
//! Euler forward method
//!

use num_traits::Zero;

use super::*;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PT1<N> {
    pub t1_time: f64,
    pub sample_time: f64,
    pub kp: N,
    previous_output: N,
}

impl<N: PartialOrd + Zero> PT1<N> {
    pub fn set_sample_time(self, sample_time: f64) -> Self {
        assert!(sample_time > 0.0);
        PT1::<N> {
            sample_time,
            ..self
        }
    }

    pub fn set_t1_time(self, t1_time: f64) -> Self {
        assert!(t1_time >= self.sample_time || t1_time == 0.0);
        PT1::<N> { t1_time, ..self }
    }
}

const FIX_KOMMA_SHIFT_BITS: u8 = 10;
const FIX_KOMMA_SHIFT: i32 = 1 << FIX_KOMMA_SHIFT_BITS;

impl PT1<i32> {
    // alpha is fixed point with 10 bits after the comma
    // alpha is used to overcome sampling rate / t1 time dependency
    fn alpha(&self) -> i32 {
        (self.sample_time * FIX_KOMMA_SHIFT as f64 / self.t1_time) as i32
    }

    pub fn set_kp(self, kp: i32) -> Self {
        assert!(kp > 0);
        PT1::<i32> {
            kp: kp * FIX_KOMMA_SHIFT,
            ..self
        }
    }
}

impl Default for PT1<i32> {
    fn default() -> Self {
        PT1::<i32> {
            sample_time: 1.0,
            t1_time: 1.0,
            kp: FIX_KOMMA_SHIFT,
            previous_output: 0,
        }
    }
}

impl<N> TypeIdentifier for PT1<N> {
    fn short_type_name(&self) -> &'static str {
        "PT1"
    }
}

impl<N: Display> Display for PT1<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PT1(sample_time: {}, t1_time {}, kp: {})",
            self.sample_time, self.t1_time, self.kp
        )
    }
}

impl TransferTimeDomain<i32> for PT1<i32> {
    fn transfer_td(&mut self, input: i32) -> i32 {
        let out = self.previous_output + (self.alpha() * (input * self.kp - self.previous_output))
            >> FIX_KOMMA_SHIFT_BITS;
        self.previous_output = out;
        out >> FIX_KOMMA_SHIFT_BITS
    }
}

impl PT1<f64> {
    // alpha is used to overcome sampling rate / t1 time dependency
    fn alpha(&self) -> f64 {
        self.sample_time / self.t1_time
    }
    pub fn set_kp(self, kp: f64) -> Self {
        assert!(kp > 0.0);
        PT1::<f64> { kp, ..self }
    }
}

impl Default for PT1<f64> {
    fn default() -> Self {
        PT1::<f64> {
            t1_time: 1.0,
            sample_time: 1.0,
            kp: 1.0,
            previous_output: 0.0,
        }
    }
}

impl TransferTimeDomain<f64> for PT1<f64> {
    fn transfer_td(&mut self, input: f64) -> f64 {
        let out = self.previous_output + (self.alpha() * (input * self.kp - self.previous_output));
        self.previous_output = out;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_PT1_new() {
        assert_eq!(-2048 >> FIX_KOMMA_SHIFT_BITS, -2);
        assert_eq!(
            PT1::<i32> {
                kp: 2048,
                t1_time: 1.0,
                sample_time: 1.0,
                previous_output: 0,
            },
            PT1::<i32>::default().set_kp(2)
        );
    }

    #[test]
    fn test_PT1_i32_transfer() {
        let mut sut = PT1::<i32>::default();
        assert_eq!(1000, sut.transfer_td(1000));
    }

    fn test_PT1_f64_default() {
        assert_eq!(
            PT1::<f64> {
                kp: 1.0,
                t1_time: 0.0,
                sample_time: 1.0,
                previous_output: 0.0,
            },
            PT1::<f64>::default()
        );
    }
}
