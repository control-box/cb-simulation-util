#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod hysteresis;
#[cfg(feature = "std")]
pub mod plant;

#[cfg(feature = "std")]
pub mod signal;

use core::fmt;

#[derive(Debug, Clone)]
pub struct NotDefinedError;
impl fmt::Display for NotDefinedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Outside of definition range")
    }
}

pub trait TransferFunction<T> {
    fn transfer(&mut self, u: T) -> Result<T, NotDefinedError>;
}
