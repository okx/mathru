//! Solves an ODE using the 3th order Runge-Kutta algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using the 3th order Runge-Kutta algorithm.
///
///<https://en.wikipedia.org/wiki/Rung-Kutta_methods>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Kutta3<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Kutta3<T> 
    where T: Real
{
    /// Creates a Kutta3 instance
    fn default() -> Kutta3<T>
    {
        let a: Vec<T> = vec![T::from_f64(0.5), -T::one(), T::from_f64(2.0)];
        let b: Vec<T> = vec![T::from_f64(1.0/6.0), T::from_f64(2.0/3.0), T::from_f64(1.0/6.0)];
        let c: Vec<T> = vec![T::from_f64(0.5), T::one()];

        Kutta3 
        {
            butcher: ExplicitRK::new(a, b, 3, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Kutta3<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}