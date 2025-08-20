//! A common PT2 element aka second order lag element
//!
//! $ T_{1}, T_{2} = \frac{1}{\omega} (D \plusminus \sqrt{D^{2} - 1 }) $
//!
//! where $D$ is the damping factor
//! wehre $T_{1}$ is the time constant of the first order lag
//! and $T_{2}$ is the time constant of the second order lag
//! and $\omega$ is the angular frequency
//!
//! $ x2[k] = x2​[k−1] + h(−2D omega ​x2​[k−1]) − \omega^{2} ​x1​[k−1] + P \omega^{2} ​u[k]) $
//! $ x1[k] = x1​[k−1] + h omega ​x2​[k−1]
//!

//! and $T_{s}$ is the sample time constant
//! and $P$ is the amplification
//! (Euler Forward method)
//!
//! PT2 == PS2 element iff damping factor $D = 0.0 $
//!
//! $D  =  \frac{T_{1} + T_{2}}{2 \cdot T_{1} \cdot T_{2}} $
//!
//! $D = 0.0 $  *not damped oscillation* - not possible with $ T_{1} $ and $ T_{2} $
//! $D < 1.0 $  *underdamped oscillation* - over oscillation, slow response
//! $D = 1.0 $  *critically damped oscillation* - no over oscillation, fastest possible response
//! $D > 1.0 $  *overdamped oscillation* - no over oscillation

use num_traits::Zero;
use std::*;

use super::*;
use core::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PT2<N> {
    pub omega: f64,
    pub damping: f64,
    pub sample_time: f64,
    pub kp: N,
    previous_output: N,
    previous_diff_output: N,
}

impl<N: PartialOrd + Zero> PT2<N> {
    pub fn set_sample_time_or_default(self, sample_time: f64) -> Self {
        if sample_time > 0.0 {
            PT2::<N> {
                sample_time,
                ..self
            }
        } else {
            PT2::<N> {
                sample_time: 1.0,
                ..self
            }
        }
    }

    pub fn set_omega_or_default(self, omega: f64) -> Self {
        if 1.0 / omega >= self.sample_time {
            PT2::<N> { omega, ..self }
        } else {
            PT2::<N> { omega: 1.0, ..self }
        }
    }

    /// Set the damping factor
    ///
    /// $D = 0.0 $  *not damped oscillation* - not possible with $ T_{1} $ and $ T_{2} $
    /// $D < 1.0 $  *underdamped oscillation* - over oscillation, slow response
    /// $D = 1.0 $  *critically damped oscillation* - no over oscillation, fastest possible response
    /// $D > 1.0 $  *overdamped oscillation* - no over oscillation
    pub fn set_damping_or_default(self, damping: f64) -> Self {
        if damping >= 0.0 {
            PT2::<N> { damping, ..self }
        } else {
            PT2::<N> { damping: 1.0, ..self }
        }
    }

    /// Set the time constant of the first order lag
    ///
    /// - it must be greater than or equal to the sample time
    /// - is equivalent to set the period of angular frequency
    pub fn set_t1_time_or_default(self, t1_time: f64) -> Self {
        if t1_time >= self.sample_time {
            PT2::<N> { omega: 1.0 / t1_time, ..self }
        } else {
            PT2::<N> { omega: 1.0, ..self }
        }
    }

    /// Set the time constant of the second order lag
    /// - it must be greater than or equal to the sample time
    /// - modifies the angular frequency and damping factor
    /// - leads to a damping >= 1.0
    pub fn set_t2_time_or_default(self, t2_time: f64) -> Self {
        if t2_time >= self.sample_time {
            let omega = (1.0 / t2_time * self.omega).sqrt();
            PT2::<N> {
                omega,
                damping : (1.0 / self.omega + t2_time) / (2.0 * self.omega),
                ..self }
        } else {
            PT2::<N> {
                damping: 1.0, // t1 == t2 equivalent to critically damped oscillation
                ..self
            }
        }
    }
}

const FIX_KOMMA_SHIFT_BITS: u8 = 10;
const FIX_KOMMA_SHIFT: i64 = 1 << FIX_KOMMA_SHIFT_BITS;

impl PT2<i32> {
    pub fn set_kp(self, kp: i32) -> Self {
        PT2::<i32> {
            kp: kp * FIX_KOMMA_SHIFT as i32,
            ..self
        }
    }
}

impl Default for PT2<i32> {
    fn default() -> Self {
        PT2::<i32> {
            sample_time: 1.0,
            omega: 1.0,
            damping: 0.0,
            kp: FIX_KOMMA_SHIFT as i32,
            previous_output: 0,
            previous_diff_output: 0,
        }
    }
}

impl<N> TypeIdentifier for PT2<N> {
    fn short_type_name(&self) -> &'static str {
        "PT2"
    }
}

impl<N: Display> Display for PT2<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PT2(sample_time: {}, omega {}, damping {}, kp: {})",
            self.sample_time, self.omega, self.damping, self.kp
        )
    }
}

impl TransferTimeDomain<i32> for PT2<i32> {
    fn transfer_td(&mut self, input: i32) -> i32 {
        let omega: i64 = (self.omega * (FIX_KOMMA_SHIFT as f64)) as i64;
        let omega_squared = omega * omega / FIX_KOMMA_SHIFT;
        let damping: i64 = (self.damping * (FIX_KOMMA_SHIFT as f64)) as i64;

        // $ x2[k] = x2​[k−1] + h(−2D omega ​x2​[k−1]) − \omega^{2} ​x1​[k−1] + K \omega^{2} ​u[k]) $
        let diff_output: i64 = self.previous_diff_output as i64
            + (self.sample_time as i64
                * (-2 * damping * omega / FIX_KOMMA_SHIFT * self.previous_diff_output as i64
                    / FIX_KOMMA_SHIFT
                    - omega_squared * self.previous_output as i64
                    + self.kp as i64 * input as i64 * omega_squared / FIX_KOMMA_SHIFT)
                    as i64);
        // $ x1[k] = x1​[k−1] + h omega ​x2​[k−1]
        let output: i64 = self.previous_output as i64
            + (self.sample_time as i64 * omega * self.previous_diff_output as i64);
        self.previous_diff_output = diff_output.try_into().unwrap();
        self.previous_output = output.try_into().unwrap();
        self.previous_output >> FIX_KOMMA_SHIFT_BITS
    }
}

impl PT2<f64> {
    pub fn set_kp(self, kp: f64) -> Self {
        PT2::<f64> { kp, ..self }
    }
}

impl Default for PT2<f64> {
    fn default() -> Self {
        PT2::<f64> {
            omega: 1.0,
            damping: 1.0,
            sample_time: 1.0,
            kp: 1.0,
            previous_output: 0.0,
            previous_diff_output: 0.0,
        }
    }
}

impl TransferTimeDomain<f64> for PT2<f64> {
    fn transfer_td(&mut self, input: f64) -> f64 {
        let omega_squared = self.omega * self.omega;

        // $ x2[k] = x2​[k−1] + h(−2D omega ​x2​[k−1]) − \omega^{2} ​x1​[k−1] + K \omega^{2} ​u[k]) $
        let diff_output: f64 = self.previous_diff_output
            + self.sample_time
                * (-2.0 * self.damping * self.omega * self.previous_diff_output
                    - omega_squared * self.previous_output
                    + self.kp * omega_squared * input);
        // $ x1[k] = x1​[k−1] + h omega ​x2​[k−1]
        let output = self.previous_output + (self.sample_time * self.omega * self.previous_diff_output);
        self.previous_diff_output = diff_output;
        self.previous_output = output;
        output
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_PT2_new() {
        assert_eq!(-2048 >> FIX_KOMMA_SHIFT_BITS, -2);
        assert_eq!(
            PT2::<i32> {
                kp: 2048,
                omega: 1.0,
                damping: 0.0,
                sample_time: 1.0,
                previous_output: 0,
                previous_diff_output: 0
            },
            PT2::<i32>::default().set_kp(2)
        );
    }

    #[test]
    fn test_PT2_i32_transfer() {
        let mut sut = PT2::<i32>::default();
        assert_eq!(0, sut.transfer_td(1000));
    }

    fn test_PT2_f64_default() {
        assert_eq!(
            PT2::<f64> {
                kp: 1.0,
                omega: 0.0,
                sample_time: 1.0,
                damping: 1.0,
                previous_diff_output: 0.0,
                previous_output: 0.0,
            },
            PT2::<f64>::default()
        );
    }
}
