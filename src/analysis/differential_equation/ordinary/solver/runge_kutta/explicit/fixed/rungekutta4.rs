//! Solves an ODE using the 4th order Runge-Kutta algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using the 4th order Runge-Kutta algorithm.
///
///<https://en.wikipedia.org/wiki/Rung-Kutta_methods>
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RungeKutta4<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for RungeKutta4<T> 
    where T: Real
{
    /// Creates a Kutta3 instance
    fn default() -> RungeKutta4<T>
    {
        let a: Vec<T> = vec![T::from_f64(0.5),
                             T::zero(), T::from_f64(0.5),
                             T::zero(), T::zero(), T::one()];
        let b: Vec<T> = vec![T::from_f64(1.0 / 6.0), T::from_f64(1.0 / 3.0), T::from_f64(1.0 / 3.0), T::from_f64(1.0 / 6.0)];
        let c: Vec<T> = vec![T::from_f64(0.5), T::from_f64(0.5), T::one()];

        RungeKutta4 
        {
            butcher: ExplicitRK::new(a, b, 4, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for RungeKutta4<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}
