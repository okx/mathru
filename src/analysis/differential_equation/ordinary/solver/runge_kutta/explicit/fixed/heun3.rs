//! Solves an ODE using Heun's method.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using Heun's 3rd order method.
///
/// <https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Heun's_third-order_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Heun3<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Heun3<T> 
    where T: Real
{
    /// Creates a Heun3 instance
    fn default() -> Heun3<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0 / 3.0),
                             T::zero(), T::from_f64(2.0 / 3.0)];
        let b: Vec<T> = vec![T::from_f64(1.0 / 4.0), T::zero(), T::from_f64(3.0 / 4.0)];
        let c: Vec<T> = vec![T::from_f64(1.0 / 3.0), T::from_f64(2.0 / 3.0)];

        Heun3 
        {
            butcher: ExplicitRK::new(a, b, 3, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Heun3<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}
