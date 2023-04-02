//! Implicit Ordinary Differential Equation

use crate::algebra::{
    abstr::Real,
    linear::{Matrix, Vector},
};

/// Implicit ordinary differential equation
pub trait ImplicitODE<T>
    where T: Real
{
    fn func(&self, t: &T, x: &Vector<T>) -> Vector<T>;

    fn jacobian(&self, t: &T, x: &Vector<T>) -> Matrix<T>;

    fn time_span(&self) -> (T, T)
    {
        unimplemented!();
    }

    fn init_cond(&self) -> Vector<T>
    {
        unimplemented!();
    }
}
