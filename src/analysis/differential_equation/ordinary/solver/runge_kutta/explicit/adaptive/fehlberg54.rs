//! Solves an ODE using the 4th order Runge-Kutta-Fehlberg algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::adaptive::{ExplicitRKEmbedded, ExplicitRKEmbeddedMethod};
use std::default::Default;
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Solves an ODE using the 4th order Runge-Kutta-Fehlberg algorithm.
///
///<https://en.wikipedia.org/wiki/Runge-Kutta-Fehlberg_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Fehlberg54<T>
{
    butcher: ExplicitRKEmbedded<T>,
}

impl<T> Default for Fehlberg54<T> 
    where T: Real
{
    fn default() -> Fehlberg54<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0/4.0),
                             T::from_f64(3.0/32.0), T::from_f64(9.0/32.0),
                             T::from_f64(1932.0/2197.0), T::from_f64(-7200.0/2197.0),T::from_f64(7296.0/2197.0),
                             T::from_f64(439.0/216.0), T::from_f64(-8.0), T::from_f64(3680.0/513.0), T::from_f64(-845.0/4104.0),
                             T::from_f64(-8.0/27.0), T::from_f64(2.0), T::from_f64(-3544.0/2565.0), T::from_f64(1859.0/4104.0), T::from_f64(-11.0/40.0)];
        let b: Vec<T> = vec![T::from_f64(16.0/135.0), T::zero(), T::from_f64(6656.0/12825.0), T::from_f64(28561.0/56430.0), T::from_f64(-9.0/50.0), T::from_f64(2.0/55.0)];
        let b_s: Vec<T> = vec![T::from_f64(25.0/216.0), T::zero(), T::from_f64(1408.0/2565.0), T::from_f64(2197.0/4104.0), T::from_f64(-1.0/5.0), T::zero()];
        let c: Vec<T> = vec![T::from_f64(1.0 /4.0), T::from_f64(3.0/8.0), T::from_f64(12.0/13.0), T::one(), T::from_f64(0.5)];

        Fehlberg54 
        {
            butcher: ExplicitRKEmbedded::new(a, b, 5, b_s, 4, c),
        }
    }
}

impl<T> ExplicitRKEmbeddedMethod<T> for Fehlberg54<T> 
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>
    {
       &self.butcher
    }
}