use core::any::Any;

use core::fmt::Debug;
use core::fmt::Display;


use dyn_clone::DynClone; // DynClone is a trait with clones a Box
use std::boxed::Box;

pub mod pt1;


pub trait TypeIdentifier {
    /// Treated as a "dynamic type identifier"
    /// It should be one word including numbers, starting with a capital letter
    fn short_type_name(&self) -> &'static str;
}

pub trait TransferTimeDomain<N> : TypeIdentifier {
    /// Transfer function for time domain
    ///
    /// # Arguments
    /// * `u` - input signal a number
    /// # Returns
    /// * `N` - output signal a number
    ///
    /// # Safety
    /// The input signal must be within the defined range of the transfer function.
    /// If the input signal is outside the defined range, the function will return
    /// some border case of the output range. - This is the same behavior as a physical system.
    ///
    /// # Note
    /// For simplicity reasons input and output signal are of the same type.
    /// This is not a requirement of the transfer function.
    /// It is just to focus on the function itself and not on value ranges and units of measurement.
    fn transfer_td(&mut self, u: N) -> N;
}


pub trait DynTransferTimeDomain<S: Debug + Display + Clone + Copy + Sized + Send + Sync>:
    TransferTimeDomain<S> + Debug + Display + DynClone + 'static + Send + Sync
{
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_element(&self) -> &dyn DynTransferTimeDomain<S>;
    fn dyn_eq(&self, other: &dyn DynTransferTimeDomain<S>) -> bool;
}

impl<T, S> DynTransferTimeDomain<S> for T
where
    T: TransferTimeDomain<S> + Debug + Display + DynClone + Copy + 'static + PartialEq + Send + Sync,
    S: Debug + Display + Clone + Copy + Sized + 'static + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_element(&self) -> &dyn DynTransferTimeDomain<S> {
        self
    }

    fn dyn_eq(&self, other: &dyn DynTransferTimeDomain<S>) -> bool {
        if let Some(other_t) = other.as_any().downcast_ref::<T>() {
            self == other_t
        } else {
            false
        }
    }
}


pub type BoxedTransferTimeDomain<S> = Box<dyn DynTransferTimeDomain<S> + 'static>;

impl<S> Clone for BoxedTransferTimeDomain<S> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}


impl<S: Debug + Display + Clone + Copy + Sized + 'static + Send + Sync> PartialEq for BoxedTransferTimeDomain<S> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.clone().as_dyn_element())
    }
}

impl Default for BoxedTransferTimeDomain<f64> {
    fn default() -> Self {
        Box::new(pt1::PT1::<f64>::default())
    }
}

