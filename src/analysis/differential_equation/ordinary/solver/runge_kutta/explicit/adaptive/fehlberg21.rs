//! Solves an ODE using  Runge-Kutta-Fehlberg1(2) algorithm.
use crate::algebra::abstr::Real;
use std::default::Default;
use std::clone::Clone;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::adaptive::{ExplicitRKEmbedded, ExplicitRKEmbeddedMethod};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using Runge-Kutta-Fehlberg21 algorithm.
///
///<https://en.wikipedia.org/wiki/Runge-Kutta-Fehlberg_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Fehlberg21<T>
{
    butcher: ExplicitRKEmbedded<T>
}

impl<T> Default for Fehlberg21<T> 
    where T: Real
{
    fn default() -> Fehlberg21<T>
    {
        let a: Vec<T> = vec![T::from_f64(0.5),
                             T::from_f64(1.0 / 256.0), T::from_f64(255.0 / 256.0)];
        let b: Vec<T> = vec![T::from_f64(1.0 / 512.0), T::from_f64(255.0 / 256.0), T::from_f64(1.0 / 512.0)];
        let b_s: Vec<T> = vec![T::from_f64(1.0 / 256.0), T::from_f64(255.0 / 256.0), T::zero()];
        let c: Vec<T> = vec![T::from_f64(0.5), T::one()];

        Fehlberg21
        {
            butcher: ExplicitRKEmbedded::new(a, b, 2, b_s, 1, c)
        }
    }
}

impl<T> ExplicitRKEmbeddedMethod<T> for Fehlberg21<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>
    {
       &self.butcher
    }
}