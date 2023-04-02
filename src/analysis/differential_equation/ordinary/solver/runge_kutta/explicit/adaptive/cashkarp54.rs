//! Solves an ODE using the 4th order Cash-Karp algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::adaptive::{ExplicitRKEmbedded, ExplicitRKEmbeddedMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using the 4th order Cash-Karp algorithm.
///
/// <https://en.wikipedia.org/wiki/Cash-Karp_method>
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CashKarp54<T>
{
    butcher: ExplicitRKEmbedded<T>
}

impl<T> Default for CashKarp54<T> 
    where T: Real
{
    fn default() -> CashKarp54<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0 / 5.0),
                             T::from_f64(3.0 / 40.0), T::from_f64(9.0 / 40.0),
                             T::from_f64(3.0 / 10.0), T::from_f64(-9.0 / 10.0), T::from_f64(6.0 / 5.0),
                             T::from_f64(-11.0 / 54.0), T::from_f64(5.0 / 2.0), T::from_f64(-70.0 / 27.0), T::from_f64(35.0 / 27.0),
                             T::from_f64(1631.0 / 55296.0), T::from_f64(175.0 / 512.0), T::from_f64(575.0 / 13824.0), T::from_f64(44275.0 / 110592.0), T::from_f64(253.0 / 4096.0)];
        let b_s: Vec<T> = vec![T::from_f64(37.0 / 378.0), T::zero(), T::from_f64(250.0 / 621.0), T::from_f64(125.0 / 594.0), T::zero(), T::from_f64(512.0 / 1771.0)];
        let b: Vec<T> = vec![T::from_f64(2825.0 / 27648.0), T::zero(), T::from_f64(18575.0 / 48384.0), T::from_f64(13525.0 / 55296.0), T::from_f64(277.0 / 14336.0), T::from_f64(1.0 / 4.0)];
        let c: Vec<T> = vec![T::from_f64(1.0 / 5.0), T::from_f64(3.0 / 10.0), T::from_f64(3.0 / 5.0), T::one(), T::from_f64(7.0 / 8.0)];

        CashKarp54 
        {
            butcher: ExplicitRKEmbedded::new(a, b, 5, b_s, 4, c)
        }
    }
}

impl<T> ExplicitRKEmbeddedMethod<T> for CashKarp54<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>
    {
       &self.butcher
    }
}