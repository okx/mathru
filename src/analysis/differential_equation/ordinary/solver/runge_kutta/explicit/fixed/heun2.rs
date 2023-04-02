//! Solves an ODE using Heun's method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using Heun's 2nd order method.
///
/// <https://en.wikipedia.org/wiki/Heun's_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Heun2<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Heun2<T> where T: Real
{
    /// Creates a Euler instance
    fn default() -> Heun2<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0)];
        let b: Vec<T> = vec![T::from_f64(0.5), T::from_f64(0.5)];
        let c: Vec<T> = vec![T::from_f64(1.0)];

        Heun2 {
            butcher: ExplicitRK::new(a, b, 2, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Heun2<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}