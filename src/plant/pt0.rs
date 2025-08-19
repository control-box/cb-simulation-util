//! A common PT0 element aka zero order lag element
//!
//! $ out[k] = P * in[k] * \sigma (k - \floor(T_{0}/T_s) $
//!
//! and $T_{s}$ is the sample time constant
//! amd $T_{0}$ is the time constant of the zero order lag
//! and $P$ is the amplification

//!


use super::*;
use core::fmt::{self, Display};

use std::vec;
use std::vec::Vec;
use num_traits::{Num, Zero, zero};

#[derive(Debug, Clone, PartialEq)]
pub struct PT0<N> {
    pub t0_time: f64,
    pub sample_time: f64,
    pub kp: N,
    buffered_output: Vec<N>,
}

impl<N: PartialOrd + Zero + Clone + Num> PT0<N> {
    pub fn set_sample_time(self, sample_time: f64) -> Self {
        assert!(sample_time > 0.0);
        PT0::<N> {
            sample_time,
            ..self
        }
    }

    pub fn set_t0_time(self, t0_time: f64) -> Self {
        assert!(t0_time >= 0.0);
        PT0::<N> { t0_time: t0_time + 1.0, ..self }
    }

    pub fn build(self) -> Self {
        // Adjust  buffered output
        let buffer_size = (self.t0_time / self.sample_time) as usize;

        PT0::<N> {
            buffered_output: vec![N::zero(); buffer_size],
            ..self
        }
    }

    fn is_buffer_size_ok(&self) -> bool {
        (self.t0_time / self.sample_time) as usize == self.buffered_output.len()
    }
}

const FIX_KOMMA_SHIFT_BITS: u8 = 10;
const FIX_KOMMA_SHIFT: i32 = 1 << FIX_KOMMA_SHIFT_BITS;

impl PT0<i32> {

    pub fn set_kp(self, kp: i32) -> Self {
        PT0::<i32> {
            kp: kp * FIX_KOMMA_SHIFT,
            ..self
        }
    }
}

impl Default for PT0<i32> {
    fn default() -> Self {
        PT0::<i32> {
            sample_time: 1.0,
            t0_time: 0.0,
            kp: FIX_KOMMA_SHIFT,
            buffered_output: vec![zero(); 1],
        }
    }
}

impl<N> TypeIdentifier for PT0<N> {
    fn short_type_name(&self) -> &'static str {
        "PT0"
    }
}

impl<N: Display> Display for PT0<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PT0(sample_time: {}, t0_time {}, kp: {})",
            self.sample_time, self.t0_time, self.kp
        )
    }
}

impl TransferTimeDomain<i32> for PT0<i32> {
    fn transfer_td(&mut self, input: i32) -> i32 {
        assert!(self.is_buffer_size_ok(), "Build method must be called before transfer_td");
        if self.buffered_output.len() > 0 {
            // Shift the buffer to the left
             self.buffered_output.remove(0);
        }
        self.buffered_output.push(input * self.kp);
        // The output is the first element of the buffer
        self.buffered_output[0] >> FIX_KOMMA_SHIFT_BITS
    }
}

impl PT0<f64> {

    pub fn set_kp(self, kp: f64) -> Self {
        assert!(kp > 0.0);
        PT0::<f64> { kp, ..self }
    }
}

impl Default for PT0<f64> {
    fn default() -> Self {
        PT0::<f64> {
            t0_time: 0.0,
            sample_time: 1.0,
            kp: 1.0,
            buffered_output: vec![zero(); 1],
        }
    }
}

impl TransferTimeDomain<f64> for PT0<f64> {
    fn transfer_td(&mut self, input: f64) -> f64 {
        assert!(self.is_buffer_size_ok(), "Build method must be called before transfer_td");
        if self.buffered_output.len() > 0 {
            // Shift the buffer to the left
             self.buffered_output.remove(0);
        }
        self.buffered_output.push(input * self.kp);
        // The output is the first element of the buffer
        self.buffered_output[0]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_PT0_new() {
        assert_eq!(-2048 >> FIX_KOMMA_SHIFT_BITS, -2);
        assert_eq!(
            PT0::<i32> {
                kp: 2048,
                t0_time: 0.0,
                sample_time: 1.0,
                buffered_output: vec![zero(); 1],
            },
            PT0::<i32>::default().set_kp(2)
        );
    }

    #[test]
    fn test_PT0_i32_transfer() {
        let mut sut = PT0::<i32>::default().set_t0_time(2.0).build();
        assert_eq!(0, sut.transfer_td(100));
        assert_eq!(0, sut.transfer_td(1000));
        assert_eq!(100, sut.transfer_td(2000));
        assert_eq!(1000, sut.transfer_td(2000));
        assert_eq!(2000, sut.transfer_td(2000));
    }

    fn test_PT0_f64_default() {
        assert_eq!(
            PT0::<f64> {
                kp: 1.0,
                t0_time: 2.0,
                sample_time: 1.0,
                buffered_output: vec![zero(); 2],
            },
            PT0::<f64>::default()
        );
    }
}
