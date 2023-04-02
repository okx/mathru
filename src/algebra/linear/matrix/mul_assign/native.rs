use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};
use std::ops::MulAssign;

// Multiply matrix with a matrix
impl<T> MulAssign<Matrix<T>> for Matrix<T>
    where T: Field + Scalar
{
    /// Multiply a matrix with a matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![2.0, 3.0, -5.0, 2.0]);
    /// a *= b;
    /// ```
    fn mul_assign(&mut self, rhs: Matrix<T>)
    {
        let _ = self * &rhs;
    }
}

// Multiply matrix with a scalar
impl<T> MulAssign<T> for Matrix<T>
    where T: Field + Scalar
{
    /// Multiply matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// a *= -4.0;
    /// ```
    fn mul_assign(&mut self, rhs: T)
    {
        self.data.iter_mut().for_each(|a: &mut T|
            *a *= rhs
        );
    }
}
