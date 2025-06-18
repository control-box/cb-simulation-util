//! A common PT1 element aka first order lag element
//!
//! $ out[k]= out[k-1]+ \alpha (P * in[k]-out[k-1]) $
//!
//! where $\alpha =\frac{T_{1}}{T_{1}+T_{s}}$
//! and $T_{s}$ is the sample time constant
//! and $P$ is the amplification
//!
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PT1<N> {
    alpha: N,
    kp: N,
    previous_output: N,
}

const FIX_KOMMA_SHIFT_BITS: u8 = 10;
const FIX_KOMMA_SHIFT: i32 =  1 << FIX_KOMMA_SHIFT_BITS;

impl PT1<i32> {
    ///
    pub fn new(sample_time: f32, t1_time: f32, kp: f32) -> Self {
        assert!(sample_time > 0.0);
        assert!(t1_time >= sample_time);
        assert!(kp > 0.0);
        assert!(kp < 1000.0);
        PT1::<i32> {
            kp: (kp * FIX_KOMMA_SHIFT as f32) as i32,
            alpha: (t1_time * FIX_KOMMA_SHIFT as f32 / (t1_time + sample_time)) as i32,
            previous_output: 0,
        }
    }
    pub fn transfer(&mut self, input: i32) -> i32 {
        let out = self.previous_output
            + (self.alpha * (input * self.kp - self.previous_output )) >> FIX_KOMMA_SHIFT_BITS;
        self.previous_output = out;
        out >> FIX_KOMMA_SHIFT_BITS
    }
}

impl PT1<f64> {
    ///
    pub fn new(sample_time: f32, t1_time: f32, kp: f32) -> Self {
        assert!(sample_time > 0.0);
        assert!(t1_time >= sample_time);
        assert!(kp > 0.0);
        assert!(kp < 1000.0);
        PT1::<f64> {
            kp: kp as f64,
            alpha: (sample_time / t1_time ) as f64,
            previous_output: 0.,
        }
    }
    pub fn transfer(&mut self, input: f64) -> f64 {
        let out = self.previous_output
            + (self.alpha * (input * self.kp - self.previous_output )) ;
        self.previous_output = out;
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_PT1_new() {
        assert_eq!( -2048 >> FIX_KOMMA_SHIFT_BITS, -2 );
        assert_eq!( PT1::<i32> {
            kp: 2048,
            alpha: 512,
            previous_output: 0,
        }, PT1::<i32>::new(0.4, 0.4, 2.0) );
    }

    #[test]
    fn test_PT1_transfer() {
        let mut sut = PT1::<i32>::new(0.4, 0.4, 2.0);
        assert_eq!( 1000, sut.transfer(1000));
    }
}
