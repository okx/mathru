use crate::algebra::{
    abstr::{Field, Scalar, AbsDiffEq},
    linear::{Matrix, Vector},
};

use super::Solve;

impl<T> Solve<Vector<T>> for Matrix<T>
    where T: Field + Scalar + AbsDiffEq
{
    /// Solves Ax = y
    /// where A \in R^{m * n}, x \in R^n, y \in R^m
    fn solve(&self, rhs: &Vector<T>) -> Result<Vector<T>, ()>
    {
        self.dec_lu()?.solve(rhs)
    }
}

impl<T> Solve<Matrix<T>> for Matrix<T>
    where T: Field + Scalar + AbsDiffEq
{
    fn solve(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, ()>
    {
        self.dec_lu()?.solve(rhs)
    }
}
