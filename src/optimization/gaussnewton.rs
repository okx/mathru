use crate::{
    algebra::{
        abstr::Real,
        linear::{Matrix, Vector},
    },
    optimization::{Optim, OptimResult},
};
use std::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Gauss-Newton method
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct GaussNewton<T>
{
    iters: u64,
    __phantom: PhantomData<T>,
}

impl<T> GaussNewton<T>
{
    pub fn new(iters: u64) -> GaussNewton<T>
    {
        GaussNewton { iters,
                      __phantom: PhantomData }
    }
}

impl<T> GaussNewton<T> where T: Real
{
    /// Minimize function func
    ///
    /// # Arguments
    ///
    /// * 'func0': Function to be minimized
    /// * 'x_0': Initial guess for the minimum
    ///
    /// # Return
    ///
    /// local minimum
    pub fn minimize<F>(&self, func: &F, x_0: &Vector<T>) -> Result<OptimResult<Vector<T>>, ()>
        where F: Optim<T>
    {
        let mut x_n: Vector<T> = x_0.clone();

        for _i in 0..self.iters
        {
            let jacobian_x_n: Matrix<T> = func.jacobian(&x_n);
            let f_x_n: Vector<T> = func.eval(&x_n);
            let pinv = match jacobian_x_n.pinv() {
                Ok(pinv) => pinv,
                Err(e) => return Err(e)
            };
            let delta_x_n: Vector<T> = pinv * f_x_n;
            x_n -= delta_x_n;
        }

        Ok(OptimResult::new(x_n))
    }
}
