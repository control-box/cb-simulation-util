#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod hysteresis;
pub mod pt1;

use core::fmt;

#[derive(Debug, Clone)]
pub struct NotDefinedError;
impl fmt::Display for NotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Outside of definition range")
    }
}

trait TransferFunction<T> {
    fn transfer(&mut self, u: T) -> Result<T, NotDefinedError>;
}
