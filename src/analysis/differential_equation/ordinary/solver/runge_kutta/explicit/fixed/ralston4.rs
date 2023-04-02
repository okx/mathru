//! Solves an ODE using Ralston's 4th order method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Solves an ODE using Ralston's 4th order method.
///
/// <https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Ralston4<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Ralston4<T> 
    where T: Real
{
    /// Creates a Ralston4 instance
    fn default() -> Ralston4<T>
    {
        const SQRT5: f64 = 2.236_067_977_499_79;
        let a: Vec<T> = vec![T::from_f64(0.4),
                             T::from_f64((-2889.0 + 1428.0 * SQRT5) / 1024.0), T::from_f64((3785.0 - 1620.0 * SQRT5) / 1024.0),
                             T::from_f64((-3365.0 + 2094.0 * SQRT5) / 6040.0), T::from_f64((-975.0 - 3046.0 * SQRT5) / 2552.0), T::from_f64((467040.0 + 203968.0 * SQRT5) / 240845.0)];
        let b: Vec<T> = vec![T::from_f64((263.0 + 24.0 * SQRT5) / 1812.0), T::from_f64((125.0 - 1000.0 * SQRT5) / 3828.0), T::from_f64(1024.0 * (3356.0 + 1623.0 * SQRT5) / 5924787.0), T::from_f64((30.0 - 4.0 * SQRT5) / 123.0)];
        let c: Vec<T> = vec![T::from_f64(0.4), T::from_f64((14.0 - 3.0 * SQRT5) / 16.0), T::one()];

        Ralston4 {
            butcher: ExplicitRK::new(a, b, 4, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Ralston4<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}