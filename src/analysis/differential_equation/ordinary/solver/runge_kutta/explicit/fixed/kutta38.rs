//! Solves an ODE using the 4th order 3/8-rule Runge-Kutta algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::fixed::{ExplicitRK, ExplicitRKMethod};
use std::default::Default;
use std::clone::Clone;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Kutta38<T>
{
    butcher: ExplicitRK<T>
}

impl<T> Default for Kutta38<T> 
    where T: Real
{
    fn default() -> Kutta38<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0/3.0),
                             T::from_f64(-1.0/3.0) ,T::one(),
                             T::one(), -T::one(), T::one()];
        let b: Vec<T> = vec![T::from_f64(1.0/8.0), T::from_f64(3.0/8.0), T::from_f64(3.0/8.0), T::from_f64(1.0/8.0)];
        let c: Vec<T> = vec![T::from_f64(1.0/3.0), T::from_f64(2.0/3.0), T::one()];

        Kutta38 
        {
            butcher: ExplicitRK::new(a, b, 4, c)
        }
    }
}

impl<T> ExplicitRKMethod<T> for Kutta38<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>
    {
       &self.butcher
    }
}
