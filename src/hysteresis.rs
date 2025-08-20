use core::cmp::PartialOrd;
use num_traits::Num;

use crate::{NotDefinedError, TransferFunction};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    FromUpper,
    FromLower,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearFn<N> {
    pub m: N,
    pub n: N,
}

#[derive(Debug, PartialEq)]
pub struct Hysteresis<N> {
    upper_fn: LinearFn<N>,
    lower_fn: LinearFn<N>,
    upper: N,
    lower: N,
    direction: Direction,
}

impl<N: Num + Copy + Clone + PartialOrd> TransferFunction<N> for Hysteresis<N> {
    fn transfer(&mut self, u: N) -> Result<N, NotDefinedError> {
        if self.lower > u {
            self.direction = Direction::FromLower;
            return Ok(self.lower_fn.m * u + self.lower_fn.n);
        }
        if self.upper < u {
            self.direction = Direction::FromUpper;
            return Ok(self.upper_fn.m * u + self.upper_fn.n);
        }
        match self.direction {
            Direction::FromLower => Ok(self.lower_fn.m * u + self.lower_fn.n),
            Direction::FromUpper => Ok(self.upper_fn.m * u + self.upper_fn.n),
        }
    }
}

/// Build a hysteresis with
///
/// # Examples
/// ```
/// use cb_simulation_util::hysteresis::{HysteresisBuilder, LinearFn};
///
/// fn main() {
///     let _h= HysteresisBuilder::<f64>::new( LinearFn{ m: 1.0, n: 0.0},  LinearFn{ m: 1.0, n: 0.0})
///         .lower_x(0.5).upper_x(1.0).build();
///     let _h = HysteresisBuilder::<f64>::new( LinearFn{ m: 1.0, n: 0.0},  LinearFn{ m: 1.0, n: 0.0})
///         .lower_y(0.5).spread_x(1.0).build();
///     let _h = HysteresisBuilder::<f64>::new( LinearFn{ m: 1.0, n: 0.0},  LinearFn{ m: 1.0, n: 0.0})
///         .upper_x(0.5).spread_y(1.0).upper_direction().build();
///     let _h = HysteresisBuilder::<f64>::new( LinearFn{ m: 1.0, n: 0.0},  LinearFn{ m: 1.0, n: 0.0})
///         .cross().spread_y(1.0).upper_direction().build();
/// }
///
/// ```
#[derive(Debug)]
pub struct HysteresisBuilder<N> {
    upper_fn: LinearFn<N>,
    lower_fn: LinearFn<N>,
    upper: Option<N>,
    lower: Option<N>,
    midpoint: N,
    spread: N,
    direction: Direction,
}

impl<N> HysteresisBuilder<N>
where
    N: Default + Num + Copy + Clone,
{
    pub fn new(lower_fn: LinearFn<N>, upper_fn: LinearFn<N>) -> Self {
        HysteresisBuilder::<N> {
            upper_fn,
            lower_fn,
            upper: None,
            lower: None,
            spread: <N>::default(),
            midpoint: <N>::default(),
            direction: Direction::FromLower,
        }
    }

    pub fn build(&self) -> Hysteresis<N> {
        let lower = match self.lower {
            Some(x) => x,
            None => match self.upper {
                None => self.midpoint - self.spread / (<N>::one() + <N>::one()),
                Some(y) => y - self.spread,
            },
        };
        let upper = match self.upper {
            Some(x) => x,
            None => match self.lower {
                None => self.midpoint + self.spread / (<N>::one() + <N>::one()),
                Some(y) => y + self.spread,
            },
        };
        Hysteresis {
            lower_fn: self.lower_fn,
            upper_fn: self.upper_fn,
            upper,
            lower,
            direction: self.direction,
        }
    }

    pub fn spread_x(mut self, s: N) -> Self {
        self.spread = s;
        self
    }

    pub fn spread_y(mut self, s: N) -> Self {
        if self.lower_fn.m != self.upper_fn.m {
            self.spread = s / (self.upper_fn.m - self.lower_fn.m);
        }
        self
    }

    pub fn cross(mut self) -> Self {
        // m_lower * x  + n_lower = m_upper * x + n_upper
        if self.lower_fn.m != self.upper_fn.m {
            self.midpoint =
                (self.lower_fn.n - self.upper_fn.n) / (self.upper_fn.m - self.lower_fn.m);
        }
        self
    }

    pub fn lower_x(mut self, s: N) -> Self {
        self.lower = Some(s);
        self
    }

    pub fn upper_x(mut self, s: N) -> Self {
        self.upper = Some(s);
        self
    }

    pub fn lower_y(mut self, delta_y: N) -> Self {
        // m_lower * x  + n_lower + delta_y = m_upper * x + n_upper
        if self.lower_fn.m != self.upper_fn.m {
            self.lower = Some(
                (self.lower_fn.n - self.upper_fn.n + delta_y) / (self.upper_fn.m - self.lower_fn.m),
            );
        }
        self
    }

    pub fn upper_y(mut self, delta_y: N) -> Self {
        // m_lower * x  + n_lower + delta_y = m_upper * x + n_upper
        if self.lower_fn.m != self.upper_fn.m {
            self.upper = Some(
                (self.lower_fn.n - self.upper_fn.n + delta_y) / (self.upper_fn.m - self.lower_fn.m),
            );
        }
        self
    }

    pub fn upper_direction(mut self) -> Self {
        self.direction = Direction::FromUpper;
        self
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_HysteresisBuilder_default_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 1.0, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: 0.0,
            upper: 0.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 1.0, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_upper_direction_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 1.0, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: 0.0,
            upper: 0.0,
            direction: Direction::FromUpper,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 1.0, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .upper_direction()
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_spread_x_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 1.0, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: -0.5,
            upper: 0.5,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 1.0, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_x(1.0)
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_spread_y_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: -1.0,
            upper: 1.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_y(1.0)
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_cross_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: -2.0,
            upper: -2.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .cross()
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_lower_x_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: 1.0,
            upper: 2.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_x(1.0)
                .lower_x(1.0)
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_upper_x_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: 0.0,
            upper: 1.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_x(1.0)
                .upper_x(1.0)
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_lower_y_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: 0.0,
            upper: 1.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_x(1.0)
                .lower_y(1.0)
                .build();
        assert_eq!(expected, sut)
    }

    #[test]
    fn test_HysteresisBuilder_upper_y_build() {
        let expected = Hysteresis {
            lower_fn: LinearFn { m: 0.5, n: 0.0 },
            upper_fn: LinearFn { m: 1.0, n: 1.0 },
            lower: -1.0,
            upper: 0.0,
            direction: Direction::FromLower,
        };
        let sut =
            HysteresisBuilder::<f64>::new(LinearFn { m: 0.5, n: 0.0 }, LinearFn { m: 1.0, n: 1.0 })
                .spread_x(1.0)
                .upper_y(1.0)
                .build();
        assert_eq!(expected, sut)
    }
}
