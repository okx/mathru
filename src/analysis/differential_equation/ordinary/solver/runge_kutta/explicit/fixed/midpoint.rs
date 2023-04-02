//! Solves an ODE using midpoint method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using midpoint method.
///
/// <https://en.wikipedia.org/wiki/Midpoint_method>
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Midpoint<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Midpoint<T> 
    where T: Real
{
    /// Creates an Euler instance
    fn default() -> Midpoint<T>
    {
        let a: Vec<T> = vec![T::from_f64(0.5)];
        let b: Vec<T> = vec![T::zero(), T::one()];
        let c: Vec<T> = vec![T::from_f64(0.5)];

        Midpoint 
        {
            butcher: ExplicitRK::new(a, b, 2, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Midpoint<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}
