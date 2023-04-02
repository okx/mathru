//! Solves an ODE using Ralston2's method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using Ralston's 2nd order method.
///
/// <https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Ralston2<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Ralston2<T> 
    where T: Real
{
    /// Creates a Ralston2 instance
    fn default() -> Ralston2<T>
    {
        let a: Vec<T> = vec![T::from_f64(2.0/3.0)];
        let b: Vec<T> = vec![T::from_f64(1.0/4.0), T::from_f64(3.0/4.0)];
        let c: Vec<T> = vec![T::from_f64(2.0/3.0)];

        Ralston2 {
            butcher: ExplicitRK::new(a, b, 2, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Ralston2<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}

