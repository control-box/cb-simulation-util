//! A common PT0 element aka zero order lag element
//!
//! $ out[k] = P * in[k] * \sigma (k - \floor(T_{0}/T_s) $
//!
//! and $T_{s}$ is the sample time constant
//! amd $T_{0}$ is the time constant of the zero order lag
//! and $P$ is the amplification
//!
//! For t_0 = 0 it is equivalent to a simple gain element.
//!


use super::*;
use core::fmt::{self, Display};

use num_traits::{Num, Zero};
 use std::ops::Shr;


const MAX_BUFFER_SIZE: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PT0<N> {
    pub t0_time: f64,
    pub sample_time: f64,
    pub kp: N,
    buffered_output: [N; MAX_BUFFER_SIZE], // a fixed array meets the Copy trait requirements
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
            buffered_output:  [0.0; MAX_BUFFER_SIZE],
        }
    }
}

impl TransferTimeDomain<f64> for PT0<f64> {
    fn transfer_td(&mut self, input: f64) -> f64 {
        let length = (self.t0_time / self.sample_time) as usize ;
        assert!(length <= MAX_BUFFER_SIZE, "Buffer size exceeded");

        for i in 0..length {
            // Shift the buffer to the left
            self.buffered_output[i] = self.buffered_output[i + 1];
        }
        // Add the new input to the end of the buffer
        self.buffered_output[length] = input * self.kp;
        // The output is the first element of the buffer
        self.buffered_output[0]
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
            buffered_output: [0; MAX_BUFFER_SIZE],
        }
    }
}


impl TransferTimeDomain<i32> for PT0<i32> {
    fn transfer_td(&mut self, input: i32) -> i32 {
        let length = (self.t0_time / self.sample_time) as usize ;
        assert!(length <= MAX_BUFFER_SIZE, "Buffer size exceeded");

        for i in 0..length {
            // Shift the buffer to the left
            self.buffered_output[i] = self.buffered_output[i + 1];
        }
        // Add the new input to the end of the buffer
        self.buffered_output[length] = input * self.kp;
        // The output is the first element of the buffer
        self.buffered_output[0] >> FIX_KOMMA_SHIFT_BITS
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
                buffered_output: [0; MAX_BUFFER_SIZE],
            },
            PT0::<i32>::default().set_kp(2)
        );
    }

    #[test]
    fn test_PT0_i32_transfer_t0_is_null() {
        let mut sut = PT0::<i32>::default();
        assert_eq!(1000, sut.transfer_td(1000));
    }
    fn test_PT0_i32_transfer_t0_is_one() {
        let mut sut = PT0::<i32>::default().set_t0_time(1.0);
        assert_eq!(0, sut.transfer_td(100));
        assert_eq!(100, sut.transfer_td(200));
        assert_eq!(200, sut.transfer_td(300));
    }
    fn test_PT0_i32_transfer_t0_is_two() {
        let mut sut = PT0::<i32>::default().set_t0_time(2.0);
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
                buffered_output: [0.0; MAX_BUFFER_SIZE],
            },
            PT0::<f64>::default()
        );
    }
}
