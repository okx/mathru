//! Solves an ODE using the 5th order Runge-Kutta-Dormand-Prince algorithm.
use crate::algebra::abstr::Real;
use crate::analysis::differential_equation::ordinary::solver::runge_kutta::explicit::adaptive::{ExplicitRKEmbedded, ExplicitRKEmbeddedMethod};
use std::clone::Clone;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


/// Runge-Kutta-Dormand-Prince algorithm.
///
///<https://en.wikipedia.org/wiki/Dormand-Prince_method>
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DormandPrince54<T>
{
    butcher: ExplicitRKEmbedded<T>,
}

impl<T> Default for DormandPrince54<T> 
    where T: Real
{
    fn default() -> DormandPrince54<T>
    {
        let a: Vec<T> = vec![T::from_f64(1.0 / 5.0),
                            T::from_f64(3.0 / 40.0), T::from_f64(9.0 / 40.0),
                            T::from_f64(44.0 / 45.0), T::from_f64(-56.0 / 15.0), T::from_f64(32.0 / 9.0),
                            T::from_f64(19372.0 / 6561.0), T::from_f64(-25360.0 / 2187.0), T::from_f64(64448.0 / 6561.0), T::from_f64(-212.0 / 729.0),
                            T::from_f64(9017.0 / 3168.0), T::from_f64(-355.0 / 33.0), T::from_f64(46732.0 / 5247.0), T::from_f64(49.0 / 176.0), T::from_f64(-5103.0 / 18656.0),
                            T::from_f64(35.0 / 384.0), T::zero(), T::from_f64(500.0 / 1113.0), T::from_f64(125.0 / 192.0), T::from_f64(-2187.0 / 6784.0), T::from_f64(11.0 / 84.0)];
        let b: Vec<T> = vec![T::from_f64(35.0 / 384.0), T::zero(), T::from_f64(500.0 / 1113.0), T::from_f64(125.0 / 192.0), T::from_f64(-2187.0 / 6784.0), T::from_f64(11.0 / 84.0), T::zero()];
        let b_s: Vec<T> = vec![T::from_f64(5179.0 / 57600.0), T::zero(), T::from_f64(7571.0 / 16695.0), T::from_f64(393.0 / 640.0), T::from_f64(-92097.0 / 339200.0), T::from_f64(187.0 / 2100.0), T::from_f64(1.0 / 40.0)];
        let c: Vec<T> = vec![T::from_f64(1.0 / 5.0), T::from_f64(3.0 / 10.0), T::from_f64(4.0 / 5.0), T::from_f64(8.0 / 9.0), T::one(), T::one()];

        DormandPrince54 
        {
            butcher: ExplicitRKEmbedded::new(a, b, 5, b_s, 4, c),
        }
    }
}

impl<T> ExplicitRKEmbeddedMethod<T> for DormandPrince54<T>
{
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>
    {
       &self.butcher
    }
}