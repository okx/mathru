//! Solves an ODE using the Bogacki Shampine algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::adaptive::{ExplicitRKEmbedded, ExplicitRKEmbeddedMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using the Bogacki Shampine algorithm of 2nd order.
///
/// <https://en.wikipedia.org/wiki/Bogacki-Shampine_method>
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct BogackiShampine32<T>
{
    butcher: ExplicitRKEmbedded<T>
}

impl<T> Default for BogackiShampine32<T> where T: Real
{
    fn default() -> BogackiShampine32<T>
    {
        let a: Vec<T> = vec![T::from_f64(0.5),
                             T::zero(), T::from_f64(3.0/4.0),
                             T::from_f64(2.0/9.0), T::from_f64(1.0/3.0), T::from_f64(4.0/9.0)];
        let b: Vec<T> = vec![T::from_f64(2.0/9.0), T::from_f64(1.0/3.0), T::from_f64(4.0/9.0), T::zero()];
        let b_s: Vec<T> = vec![T::from_f64(7.0/24.0), T::from_f64(1.0/4.0), T::from_f64(1.0/3.0), T::from_f64(1.0/8.0)];
        let c: Vec<T> = vec![T::from_f64(0.5), T::from_f64(3.0/4.0), T::one()];

        BogackiShampine32 
        {
            butcher: ExplicitRKEmbedded::new(a, b, 3, b_s, 2, c)
        }
    }
}

impl<T> ExplicitRKEmbeddedMethod<T> for BogackiShampine32<T> 
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>
    {
       &self.butcher
    }
}