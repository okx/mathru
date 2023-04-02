//! Newton-Raphson's root finding algorithm
use crate::{
    algebra::{
        abstr::Real,
        linear::{matrix::Solve, Matrix, Vector},
    },
    analysis::{Function, Jacobian},
};
use std::default::Default;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Newton Raphson
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct NewtonRaphson<T>
{
    iters: u64,
    tolerance_abs: T,
}

impl<T> NewtonRaphson<T>
{
    /// Creates an instance of newtons method
    ///
    /// # Arguments
    ///
    /// * 'iters': Number of iterations
    pub fn new(iters: u64, tolerance_abs: T) -> NewtonRaphson<T>
    {
        NewtonRaphson { iters, tolerance_abs }
    }
}

impl<T> Default for NewtonRaphson<T> where T: Real
{
    fn default() -> NewtonRaphson<T>
    {
        NewtonRaphson::new(1000, T::from_f64(10e-7))
    }
}

impl<T> NewtonRaphson<T> where T: Real
{
    pub fn find_root<F>(&self, func: &F, x_0: &Vector<T>) -> Result<Vector<T>, &'static str>
        where F: Function<Vector<T>, Codomain = Vector<T>> + Jacobian<T>
    {
        let mut x = x_0.clone();

        for _i in 0..self.iters
        {
            let func_x: Vector<T> = func.eval(&x);

            let jacobian_x: Matrix<T> = func.jacobian(&x);

            let b: Vector<T> = jacobian_x.solve(&func_x).unwrap();

            let x_current: Vector<T> = &x - &b;

            if (&x - &x_current).p_norm(&T::from_f64(2.0)) < self.tolerance_abs
            {
                return Ok(x);
            }
            x = x_current;
        }

        Err("Maximum number of iterations reached")
    }
}
