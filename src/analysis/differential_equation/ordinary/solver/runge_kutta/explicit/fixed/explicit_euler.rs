//! Solves an ODE using Euler's method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using Euler's method.
///
/// <https://en.wikipedia.org/wiki/Euler_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExplicitEuler<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for ExplicitEuler<T> where T: Real
{
    /// Creates a Euler instance
    fn default() -> ExplicitEuler<T>
    {
        let a: Vec<T> = vec![];
        let b: Vec<T> = vec![T::from_f64(1.0)];
        let c: Vec<T> = vec![];

        ExplicitEuler 
        {
            butcher: ExplicitRK::new(a, b, 1, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for ExplicitEuler<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}
