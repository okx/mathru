use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{
        matrix::{Inverse, Solve, Substitute},
        Matrix, Vector,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use crate::algebra::abstr::AbsDiffEq;

/// Result of a LU decomposition
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LUDec<T>
{
    l: Matrix<T>,
    u: Matrix<T>,
    p: Matrix<T>,
}

impl<T> LUDec<T>
{
    pub(super) fn new(l: Matrix<T>, u: Matrix<T>, p: Matrix<T>) -> LUDec<T>
    {
        LUDec { l, u, p }
    }

    /// Return l matrix of LU decomposition
    pub fn l(self) -> Matrix<T>
    {
        self.l
    }

    /// Return u matrix of LU decomposition
    pub fn u(self) -> Matrix<T>
    {
        self.u
    }

    /// Return p matrix of LU decomposition
    pub fn p(self) -> Matrix<T>
    {
        self.p
    }

    /// Return l, u, and p matrix of the LU decomposition
    pub fn lup(self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        (self.l, self.u, self.p)
    }
}

impl<T> Solve<Vector<T>> for LUDec<T> where T: Field + Scalar + AbsDiffEq
{
    /// Solves Ax = y
    /// where A \in R^{m * n}, x \in R^n, y \in R^m
    fn solve(&self, rhs: &Vector<T>) -> Result<Vector<T>, ()>
    {
        let b_hat: Vector<T> = &self.p * rhs;
        let y: Vector<T> = self.l.substitute_forward(b_hat)?;
        self.u.substitute_backward(y)
    }
}

// TODO
impl<T> Inverse<T> for LUDec<T>
    where T: Field + Scalar + AbsDiffEq
{
    /// Inverse Matrix
    ///
    /// PAX = LUX = I
    /// X = (PA)^-1
    /// X = A^-1P^-1
    /// XP = A^-1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{matrix::Inverse, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    /// ```
    fn inv(&self) -> Result<Matrix<T>, ()>
    {
        let b = Matrix::one(self.p.nrows());
        let x: Matrix<T> = self.solve(&b)?;
        Ok(x)
    }
}

// TODO
impl<T> Solve<Matrix<T>> for LUDec<T>
    where T: Field + Scalar + AbsDiffEq
{
    fn solve(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, ()>
    {
        let b_hat: Matrix<T> = &self.p * rhs;

        let y: Matrix<T> = self.l.substitute_forward(b_hat)?;
        self.u.substitute_backward(y)
    }
}
