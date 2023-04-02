#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

use crate::algebra::linear::Matrix;

pub trait Inverse<T>
{
    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{matrix::Inverse, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv().unwrap();
    /// ```
    fn inv(&self) -> Result<Matrix<T>, ()>;
}
